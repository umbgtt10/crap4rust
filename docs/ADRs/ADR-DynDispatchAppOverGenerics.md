# ADR-DynDispatchAppOverGenerics

- **Status:** Accepted
- **Date:** 2026-07-26

## Context

`App`, the top-level orchestrator that resolves packages, provisions
coverage, discovers functions, scores them, and renders a report, originally
had no trait seam at all: `app::run(args)` called `manifest::resolve_packages`,
`coverage::ensure_coverage_path`/`load_coverage_records`,
`source::discover_functions`, and `report::print_report` directly, as
concrete free functions. Nothing about the pipeline's five external
dependencies — Cargo metadata resolution, coverage generation/loading,
source-function discovery, per-function/per-project scoring, and rendering —
could be substituted for a test double; every test of `App`'s own
orchestration logic had to go through the real `cargo_metadata` subprocess,
the real filesystem, and (for coverage generation) the real `cargo llvm-cov`
subprocess, or not run at all as a unit.

This is a different starting point than `grip`'s and `braintax`'s own
version of this same decision: neither of those tools ever shipped a
generic-parameter `App<W, S, R>` that this repository's `App` had to be
converted *from*. `crap4rust`'s `App` went directly from "no trait seam,
free functions called inline" to "five injected `Box<dyn Trait>` fields" —
skipping the intermediate generic-parameter stage entirely, since no version
of this codebase had App as a struct with dependencies until this decision
was made.

## Decision

`App` is a struct holding `resolver: Box<dyn PackageResolver>`,
`discovery: Box<dyn FunctionDiscovery>`,
`coverage_provider: Box<dyn CoverageProvider>`, `scorer: Box<dyn Scorer>`,
`reporter: Box<dyn Reporter>`, and `config: Config`. `App::new(config)` wires
the five real implementations (`CargoPackageResolver`,
`SourceFunctionDiscovery`, `LlvmCovProvider`, `DefaultScorer`,
`StdoutReporter`); `App::with_deps(...)` takes all five boxes directly as
parameters for tests. No generic type parameters were introduced at any
point — `Box<dyn Trait>` was the first and only shape this repository's `App`
has had as a struct.

## Forcing constraints / Evidence

`tests/app_tests.rs` is the direct evidence this seam is load-bearing, not
decorative: `run_package_resolver_failure_propagates_error`,
`run_no_functions_discovered_bails`, `run_no_coverage_records_bails`, and
`run_unmatched_coverage_bails` each construct an `App` via `with_deps` with a
fake that fails or returns empty/mismatched data for exactly one of the five
dependencies, and assert on `App::run()`'s specific error message — none of
these four error paths could be exercised in isolation, without a real
Cargo workspace and a real (or faked) coverage file on disk, before this
decision.

## Rejected alternatives

**Introduce generics first (`App<R, D, C, S, P>`), matching grip's own
history literally.** Rejected: grip's own ADR for this decision (`ADR-
DynDispatchAppOverGenerics.md`) documents that its generic phase was never
about a real need for static dispatch — it was simply the first shape
anyone reached for, later simplified once it turned out to add call-site
noise (`App<W, S, R, C>` spelled out in every test helper) for zero
practical benefit at this call frequency. Reproducing that same detour here,
knowing in advance where it ends, would be manufacturing churn rather than
avoiding it.

**Keep `App` as free functions, add trait parameters only to the functions
that needed test doubles.** Rejected: this only pushes the same problem
down a level — `app::run(args)` would still need to construct concrete
resolver/discovery/coverage/scorer/reporter values somewhere, and every
test of `run`'s own orchestration (as opposed to any one dependency's
behavior) would still require either the real subprocesses or a parallel
"test version of run" duplicating its control flow. A single struct with
injected dependencies and one orchestration method (`App::run`) avoids
that duplication.

## Consequences

`App` is a single, nameable, ordinary type — usable in any signature without
generic parameters or turbofish, and constructible with fakes for exactly
the dependency a given test wants to exercise while using real
implementations (or simpler fakes) for the rest. The five trait definitions
now form the actual seam between `crap4rust`'s orchestration and everything
external to it: any future alternative package resolver, coverage source,
function discoverer, scoring strategy, or output renderer plugs in at
`App::with_deps`/`App::new` without `App::run` itself changing.

## Enforcement

N/A — structural; enforced only by `App`'s own type signature (there is no
generic parameter to reintroduce accidentally) and by `tests/app_tests.rs`
exercising `with_deps` directly.

## Related

- `grip`'s and `braintax`'s own `ADR-DynDispatchAppOverGenerics.md` — the
  same end shape, reached by each tool independently; this repository's
  version differs only in never having passed through a generic-parameter
  stage first.
- `docs/ARCHITECTURE.md` — where each of the five traits sits in `App`'s
  pipeline.
