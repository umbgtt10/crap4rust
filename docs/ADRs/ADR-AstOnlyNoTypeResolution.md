# ADR-AstOnlyNoTypeResolution

- **Status:** Accepted
- **Date:** 2026-07-26

## Context

`crap4rust` computes cognitive complexity by parsing each candidate source file
with `syn::parse_file` and walking the resulting AST (`src/complexity.rs`,
`src/file_walker.rs`, `src/impl_collector.rs`). It never performs type
resolution, borrow checking, or macro expansion — only syntactic pattern
matching over the parsed tree.

`cognitive_complexity` (`complexity.rs`) scores a function body purely from
its control-flow shape: `if`/`match`/`for`/`while`/`loop` arms, `&&`/`||`
short-circuit operators, and nesting depth, each identified by AST node kind,
never by what a branch's condition actually evaluates to or what type a
matched expression has. `FileWalker`/`ImplCollector` identify functions,
methods, and test-only items (`#[test]`, `#[cfg(test)]`) the same way: by
attribute and item-kind pattern, not by resolving what a macro expands to or
what a `cfg` predicate evaluates to on a given target.

Because analysis never requires type information, `crap4rust` also never
requires the analyzed code to compile. `syn::parse_file` only needs valid
syntax, so `crap4rust` runs unchanged against a branch with unresolved
imports, missing dependencies, or an incomplete refactor mid-flight — the same
property `cargo llvm-cov` itself does *not* have, since generating coverage
data requires a full, successful test build. `crap4rust`'s complexity side of
the CRAP formula stays available even when the coverage side cannot be
generated.

## Decision

`crap4rust` stays AST-only, built on `syn` alone, for the complexity and
function-discovery half of its pipeline. No dependency on `rustc`'s internals
(HIR/MIR/`rustc_middle::ty`) or on `rust-analyzer`'s semantic layer is taken
on, even though doing so would let cognitive complexity account for what a
condition or macro actually expands to rather than its surface AST shape.

## Forcing constraints / Evidence

`cognitive_complexity`'s own test suite (`tests/complexity_tests.rs`) proves
the scoring is structural, not semantic: `if a && b { }` scores `2`
(`tests/complexity_tests.rs::logical_and_scores_one` — one for the `if`, one
for `&&`) regardless of what `a`/`b` actually are, and a `try { }` block
(`tests/complexity_tests.rs::try_block_scores_one`) scores from its own AST
node kind, not from resolving what the block's implicit `Try` bound requires.
Neither case needs, or would benefit from, type information.

## Rejected alternatives

**Depend on `rustc_driver`/the compiler internals directly.** Rejected:
unstable API, tied to a specific toolchain version, and — critically for this
tool specifically — would require the analyzed crate to actually compile,
which is not guaranteed at the point a developer runs `crap4rust` locally
mid-refactor, and directly conflicts with the coverage side of the pipeline
already requiring a successful `cargo llvm-cov` run; forcing *both* halves of
CRAP to require a clean build removes the ability to at least see complexity
numbers when coverage generation is failing.

**Depend on `rust-analyzer`'s IDE crates for semantic resolution.** Rejected:
a much larger dependency surface and a fundamentally different architecture
(incremental salsa-based query engine vs. a one-shot `syn` walk) for a benefit
— slightly more semantically-aware complexity scoring — that is narrow
relative to cognitive complexity's own definition, which is explicitly a
*readability* metric scored from control-flow shape and nesting, not from
runtime semantics.

## Consequences

`crap4rust` has no compile requirement for the discovery/complexity half of
its pipeline, no toolchain-version coupling beyond `syn`'s own MSRV, and a
small dependency graph (`syn`, `proc-macro2`, `walkdir`). In exchange,
complexity scoring cannot distinguish a branch that is genuinely reachable
from one that is not (e.g. `if cfg!(...)`), and cannot see through macro
expansion to score what a macro actually generates rather than its
invocation site. Both are accepted: cognitive complexity is a readability
metric over the code as written, not over its expansion, so this ceiling
matches the metric's own definition rather than falling short of it.

## Enforcement

N/A — this is a foundational dependency choice, not a runtime-checkable
property. The check is `Cargo.toml` itself: no dependency on `rustc_*` crates
or `ra_ap_*`/`rust-analyzer` crates should ever appear.

## Related

- `docs/FORMULA.md` — the exact structural rules `cognitive_complexity` scores
  by.
- `grip`'s and `braintax`'s own `ADR-AstOnlyNoTypeResolution.md` — the same
  decision, independently applicable to each tool's own name/structure-based
  classification, which shares the identical `syn`-only shape.
