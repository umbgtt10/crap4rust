# The crap4rust formula

Full reference for how `crap4rust` computes its scores, at every level: per
function and per project. `README.md` keeps a short summary and points here
for the complete picture — this is the document that stays in sync with
`src/`, not the other way around.

Every number here is read directly from the source that computes it
(`complexity.rs`, `crap_formula.rs`, `default_scorer.rs`, `config.rs`,
`coverage_index.rs`), not transcribed from memory — if this document and the
source ever disagree, the source is right and this file is stale.

---

## Per-function: cognitive complexity

`cognitive_complexity(block)` (`complexity.rs`) walks a function's body as a
`syn` AST and sums a cost per decision point, incrementing a running
`nesting` level as it descends into each one's own body:

| Construct | Cost | Nesting increments for its own body? |
|---|---|---|
| `if` (including each `else if` in a chain) | `1 + nesting` | yes |
| plain `else { ... }` block (no further `if`) | cost of whatever is inside it, scored at `nesting + 1` | yes |
| `match` | `1 + nesting` | arms scored at `nesting + 1` |
| `for` | `1 + nesting` | yes |
| `while` | `1 + nesting` | yes |
| `loop` | `1 + nesting` | yes |
| `try { ... }` block (nightly `#![feature(try_blocks)]` syntax) | `1 + nesting` | yes |
| `?` (the `Try` operator) | `0` — recurses into the wrapped expression only | — |
| each `&&` or `\|\|` inside a condition or guard | `1` each | no |
| `\|`-separated or-pattern in a `for` loop's binding (`Pat::Or`) | `cases.len() - 1` | no |

Everything else `syn` can produce (calls, field access, literals, `unsafe`
blocks, closures, and so on) contributes `0` directly but is still walked
recursively, so a decision point nested inside a closure or call argument is
still found and scored.

### Conditions and guards: `logical_expr_score`

`if`, `while`, and `match` guard conditions are *not* walked by the general
expression scorer for their `&&`/`\|\|` cost — they go through a dedicated
`logical_expr_score`, which recurses only through `Expr::Binary` (adding `1`
per `&&`/`\|\|` found, at any depth), `Expr::Paren`, and `Expr::Group`,
returning `0` for anything else. `score_if`, `score_while`, and `score_arm`
(match guards) each call `logical_expr_score` on their own condition/guard
exactly once — calling the general expression scorer on the *same*
condition as well, in addition to `logical_expr_score`, is what produced a
real, since-fixed bug where `while`/match-guard conditions counted every
`&&`/`\|\|` twice while a textually identical `if` condition counted it once;
see `docs/ADRs/` — no ADR was written for this one specifically since it is
a straightforward symmetry fix, not a design decision, but
`tests/complexity_tests.rs`'s `while_with_*`/`match_guard_with_*` tests pin
it.

A logical expression appearing *outside* any condition — a function whose
body is just `a && b`, for instance — is scored by the general expression
walker's own `Expr::Binary` handling instead, which adds the same `1` per
operator on its own; the two paths are mutually exclusive per expression,
not additive, because `if`/`while`/match-guard conditions are never handed
to the general walker at all.

### Worked examples

| Source | Score | Why |
|---|---|---|
| `if x { 1 } else { 0 }` | `1` | one `if`, nesting `0`; the `else` branch is a plain block, adding only what's inside it (`0`) |
| `if x { if y { 2 } else { 1 } } else { 0 }` | `3` | outer `if` (`1`, nesting `0`) + inner `if` (`1 + 1` nesting) |
| `match x { 0 => 0, 1 => 1, _ => 2 }` | `1` | one `match`, no arm bodies contain further decision points |
| `a && b \|\| c` (as a bare expression, no `if`) | `2` | one `&&` + one `\|\|`, via the general walker |
| `if a && b { }` | `2` | `1` for the `if` + `1` for the `&&` |
| `while a && b { }` | `2` | `1` for the `while` + `1` for the `&&` — same shape as the `if` case above |
| `let _ = try { 1 };` | `1` | `try` **block** syntax costs `1`; contrast with `?` below |
| `let val = x?;` | `0` | the `?` operator itself is free — see below |

### Try-operator propagation is not counted

`Expr::Try` (the `?` operator) scores `0` and recurses only into the
expression it wraps — `x?` costs exactly what `x` alone would cost.
Builder-style error propagation (`foo()?.bar()?`) is scored purely by its
actual control-flow structure (call chains, no decision points), not
penalized for using `?`. This is a deliberate choice, not an oversight: `?`
forwards an error to the caller, it does not introduce a branch a reader
needs to hold in their head the way an `if`/`match` does. `try { ... }`
**block** syntax (`Expr::TryBlock`, gated behind the nightly-only
`#![feature(try_blocks)]`) is a different AST node entirely and is scored
like any other nested block-with-a-cost (`1 + nesting`), since unlike bare
`?` it introduces a real block boundary a reader must track.

---

## Per-function: the CRAP score

`CrapFormula` (`crap_formula.rs`) is a stateless struct with two methods,
mirroring `grip`'s own `ContributionSchedule`/braintax's own formula
structs: a plain-data struct wrapping the formula's constants, not a bag of
free functions.

```
CrapFormula::score(complexity, coverage) =
    complexity² × (1 − coverage)³ + complexity
```

`coverage` is a ratio in `[0.0, 1.0]` (see "Coverage matching" below); at
`coverage = 1.0` the first term vanishes and the score equals `complexity`
exactly — a fully-covered function is only ever as risky as it is complex.
At `coverage = 0.0` the score is `complexity² + complexity`, the classic
CRAP amplification: complexity is squared before coverage even enters the
picture, so a totally untested complex function is penalized far more
steeply than a totally untested simple one.

```
CrapFormula::classify(score, threshold, warn_threshold) =
    Verdict::Crappy  if score > threshold
    Verdict::Warn    if score >= warn_threshold
    Verdict::Clean   otherwise
```

`threshold` defaults to `30.0` (`--threshold`), `warn_threshold` to `20.0`
(`--warn-threshold`); both are per-invocation CLI flags, not hardcoded.

---

## Per-project aggregation

`DefaultScorer` (`default_scorer.rs`, implementing the `Scorer` trait) turns
every discovered function into a `FunctionReport` — pairing each
`SourceFunction`'s complexity with its matched coverage ratio through
`CrapFormula` — then sorts the resulting `Vec<FunctionReport>` by
`crap_score` descending, breaking ties by function name ascending so output
is deterministic across runs.

`DefaultScorer::project_metrics` then folds that `Vec<FunctionReport>` into
a `ProjectMetrics`:

| Field | Definition |
|---|---|
| `total_functions` | `reports.len()` |
| `crappy_functions` | count of reports whose own `verdict` is `Verdict::Crappy` |
| `crappy_percent` | `crappy_functions / total_functions × 100`, or `0.0` when there are no functions |
| `verdict` | see below |

The **project-level** `verdict` is not simply "worst function's verdict" —
it goes through `Config::fails` first:

```
Config::fails(crappy_functions, crappy_percent) =
    crappy_functions > 0                     if strict
    crappy_percent > project_threshold        otherwise
```

```
project verdict =
    Verdict::Crappy  if Config::fails(...)
    Verdict::Warn    else if crappy_functions > 0 OR any report is individually Verdict::Warn
    Verdict::Clean   otherwise
```

So a project with exactly one `Crappy` function and `--strict` is itself
`Crappy`; the same project without `--strict`, below `--project-threshold`
crappy-percent, is only `Warn` — a single risky function among many clean
ones is visible but not, on its own, a failing build unless the project
crosses its own configured tolerance (or the tolerance is `--strict`, i.e.
zero).

`ProjectReport::exit_code(config)` (`project_report.rs`) is the last step:
`--warn-only` always exits `0`; otherwise the process exits `1` exactly when
`Config::fails` (the same predicate, evaluated again at the project's
overall counts) is true.

---

## Coverage matching

`CoverageIndex` (`coverage_index.rs`) indexes every `CoverageRecord` parsed
from `cargo-llvm-cov`'s JSON export (`coverage.rs::load_coverage_records`)
by `(path_key, line)`. `match_function_coverage` then looks up every
coverage record whose `line` falls within a given function's own
`[line, end_line]` span (not just its start line — a function with multiple
coverage regions, e.g. an `if let`/`else if`/`else` chain, gets every region
inside its span summed) and returns the aggregate `covered_regions /
total_regions` ratio, or `None` if nothing in the index falls inside that
span — a function with no matching coverage data at all defaults to `0.0`
coverage in `DefaultScorer::score_function`, not to being skipped.

Duplicate records for the same `(path_key, line)` — which `cargo-llvm-cov`
emits when a function compiles into more than one object file, one of them
a zero-count "ghost" instantiation — are resolved symmetrically regardless
of which one arrives first: whichever side has `covered_regions == 0` is
discarded in favor of the other if exactly one side is zero; otherwise
(both zero, or both non-zero) the two are summed. See
`docs/ADRs/ADR-SymmetricDuplicateCoverageHandling.md` for why this has to
be order-independent and the bug that motivated it.

---

## Related

- `docs/ADRs/ADR-AstOnlyNoTypeResolution.md` — why complexity scoring and
  function/test discovery are structural, name/shape-based, never
  type-resolved.
- `docs/ADRs/ADR-CrossFileTestModuleExclusion.md` — how a file-based
  `#[cfg(test)] mod name;` submodule is kept out of function discovery
  entirely, before any scoring happens.
- `docs/ADRs/ADR-SymmetricDuplicateCoverageHandling.md` — the coverage
  duplicate-handling rule above, in full.
- `docs/ARCHITECTURE.md` — how `App` wires resolution, discovery, coverage
  provisioning, scoring, and reporting together end to end.
