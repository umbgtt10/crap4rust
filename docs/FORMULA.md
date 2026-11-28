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
| `match` | `1 + nesting` | arms, guards included, scored at `nesting + 1` |
| `for` | `1 + nesting` | yes |
| `while` | `1 + nesting` | yes |
| `loop` | `1 + nesting` | yes |
| `try { ... }` block (nightly `#![feature(try_blocks)]` syntax) | `1 + nesting` | yes |
| `?` (the `Try` operator) | `0` — recurses into the wrapped expression only | — |
| each `&&` or `\|\|`, in a condition, a guard or any other expression | `1` each | no |
| `\|`-separated or-pattern in a `for` loop's binding (`Pat::Or`) | `cases.len() - 1` | no |

Everything else `syn` parses from a source file (calls, casts, field
access, literals, `unsafe` and `const` blocks, closures, `&raw` borrows, and
so on) contributes `0` directly but is still walked recursively, so a
decision point nested inside a closure, a call argument or a cast is still
found and scored. A macro invocation is not expanded: what its arguments
hold is not scored.

Items declared inside a body are walked the same way wherever they sit: a
nested `fn`'s body, the methods of an `impl` and an inline `mod`'s items,
recursively, each scored at the nesting of the statement that declares them.
An `impl` written directly in a function body counts as it does inside a
`mod` there: one path, `score_nested_item`, scores both.

### Conditions and guards

`if` and `while` conditions -- `if let` and `while let` included -- and
`match` guards are walked by the general expression walker, `score_expr`,
like any other expression. Each `&&`/`\|\|` in them costs `1`, once, through
the walker's own `Expr::Binary` arm, exactly as in a function whose body is
just `a && b`; and a decision point inside them -- an `if` in a call's
argument, a `match` an `if let` binds from -- is found and scored. A
condition is scored at the `nesting` of the `if` or `while` it belongs to,
as a `match`'s scrutinee and a `for` loop's iterator are; a guard at the
nesting of its arm, one deeper than its `match`, as the arm's body is. syn
holds a guard in its arm's pattern, as a `Pat::Guard` around the pattern it
guards -- an or-pattern's included -- and `score_arm` takes it from there.

`score_if`, `score_while` and `score_arm` each hand their condition or guard
to `score_expr` exactly once, and to nothing else. Conditions once went
through a scorer of their own, `logical_expr_score`, which counted only
`&&`/`\|\|` and lost every other decision point in them; and calling both
scorers on the same condition is what once counted each `&&`/`\|\|` twice in
`while` conditions and match guards but once in `if`.
`core/tests/analysis/complexity_tests.rs` pins both: `if_with_*`,
`while_with_*` and `match_guard_with_*` count each operator once, and the
`cognitive_complexity_of_*_condition_*` and `*_match_guard_*` tests score a
decision point inside one.

### Worked examples

| Source | Score | Why |
|---|---|---|
| `if x { 1 } else { 0 }` | `1` | one `if`, nesting `0`; the `else` branch is a plain block, adding only what's inside it (`0`) |
| `if x { if y { 2 } else { 1 } } else { 0 }` | `3` | outer `if` (`1`, nesting `0`) + inner `if` (`1 + 1` nesting) |
| `match x { 0 => 0, 1 => 1, _ => 2 }` | `1` | one `match`, no arm bodies contain further decision points |
| `a && b \|\| c` (as a bare expression, no `if`) | `2` | one `&&` + one `\|\|`, via the general walker |
| `if a && b { }` | `2` | `1` for the `if` + `1` for the `&&` |
| `while a && b { }` | `2` | `1` for the `while` + `1` for the `&&` — same shape as the `if` case above |
| `if f(if a { 1 } else { 2 }) { }` | `2` | `1` for the outer `if` + `1` for the `if` in its condition, at the outer `if`'s nesting `0` |
| `if let Some(x) = match y { _ => z } { }` | `2` | `1` for the `if` + `1` for the `match` its condition binds from |
| `match x { n if f(if a { 1 } else { 2 }) => 1, _ => 0 }` | `3` | `1` for the `match` + `1 + 1` for the `if` in the guard, at the arm's nesting `1` |
| `let _ = (if a { 1 } else { 2 }) as u8;` | `1` | the cast's operand is walked |
| `let _ = const { if A { 1 } else { 2 } };` | `1` | the `const` block's body is walked |
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

`CrapFormula` (`crap_formula.rs`) is a stateless unit struct whose two
methods, `score` and `classify`, are the formula, mirroring `grip`'s own
`ContributionSchedule`/braintax's own formula structs: a struct, not a bag
of free functions.

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

`ExportCoverageProvider` (`export_coverage_provider.rs`) turns each function
of `cargo-llvm-cov`'s JSON export into a `CoverageRecord`: its file's key,
the line its first region starts on, and how many of its regions ran at
least once out of how many it has. `CoverageIndex` (`coverage_index.rs`)
indexes those records by `(path_key, line)`. `match_function_coverage` then
looks up every coverage record whose `line` falls within a given function's
own `[line, end_line]` span (not just its start line — a closure or a nested
function inside the span is a function of its own in the export, with a
record of its own, and every record inside the span is summed) and returns
the aggregate `covered_regions / total_regions` ratio, or `None` if nothing
in the index falls inside that span — a function with no matching coverage data at all defaults to `0.0`
coverage in `DefaultScorer::score_function`, not to being skipped.

Duplicate records for the same `(path_key, line)` — which `cargo-llvm-cov`
emits when a function compiles into more than one object file, one of them
often a zero-count "ghost" instantiation — are resolved by keeping one of
them, the same one whichever arrives first: the record with the most
covered regions; of two with as many, the one with fewer regions in all,
whose ratio is the higher. Two duplicates that are both zero are such a
tie: `0/3` and `0/2` keep `0/2`. Nothing is summed, so a ghost is the rule's
case rather than its exception, and one more covered region in either
duplicate never lowers the result when the two count the same regions:
`3/5` beside `0/5` and `3/5` beside `1/5` both keep `3/5`. See
`docs/ADRs/ADR-SymmetricDuplicateCoverageHandling.md`, and its amendment of
2026-11-25, for why this has to be order-independent and the bugs that
motivated it.

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
