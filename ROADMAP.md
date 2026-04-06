# crap4rust Roadmap

This document tracks the planned evolution of `cargo-crap4rust` beyond the currently shipped `0.3.x` release.

For what is available today, see [IMPLEMENTED-FEATURES.md](IMPLEMENTED-FEATURES.md). For released versions, see [CHANGELOG.md](CHANGELOG.md).

## Product Direction

crap4rust aims to be a practical Rust-native CRAP analysis tool that identifies functions that are both complex and under-tested.

The long-term direction is:

- strong Rust-aware complexity analysis
- low-friction Cargo integration
- useful local and CI reporting
- stable machine-readable outputs
- a reusable library surface for embedding and automation

## Guiding Principles

- Prefer cognitive complexity as the default Rust-facing metric
- Keep the first-class workflow centered on Cargo projects and workspaces
- Avoid noise from generated code, macro expansion, and compiler artifacts
- Keep reports actionable, not just descriptive
- Add ecosystem integrations only after the core ranking remains trustworthy on real Rust codebases

## Current Baseline

The shipped `0.3.x` line already provides:

- `cargo crap4rust` as a published Cargo subcommand
- per-function CRAP scoring
- internal cognitive-complexity scoring
- automatic `cargo llvm-cov` JSON generation when needed
- explicit coverage-file input
- multi-package workspace aggregation
- console reporting and threshold-based exit codes

That baseline is intentionally small. The roadmap below is about what comes next, not what is already complete.

## Planned Phases

### Phase 2: Richer Reporting

Goal: make local output more useful without changing the core analysis model.

Planned scope:

- JSON output for automation and downstream tooling
- HTML and Markdown reports for people-facing consumption
- output-file support instead of stdout-only reporting
- sorting, top-N filtering, and optional clean-function display
- clearer project summaries and report metadata

Exit criteria:

- one machine-readable format is stable enough for CI consumption
- one human-readable file format is useful enough to archive or share

### Phase 3: CI and Baselines

Goal: make crap4rust usable as a gating tool in automated pipelines.

Planned scope:

- baseline files for score comparison over time
- regression detection
- SARIF output for code-scanning workflows
- documented GitHub Actions examples
- stable non-zero exits for regression-specific failures

Exit criteria:

- a team can use crap4rust in CI to fail on CRAP regressions, not just absolute thresholds

### Phase 4: Configuration and Policy

Goal: let teams adopt the tool without passing every preference through the command line.

Planned scope:

- `crap4rust.toml` support
- per-workspace and per-crate defaults
- suppression rules with explicit reasons
- stricter policy controls around ignored functions
- optional risk modifiers such as `unsafe` weighting

Exit criteria:

- common project policy can live in versioned configuration rather than shell scripts

### Phase 5: Deeper Analysis Fidelity

Goal: improve the trustworthiness and flexibility of the analysis engine.

Planned scope:

- alternative complexity sources or engine integration
- alternative coverage-source support beyond `cargo llvm-cov` JSON
- better filtering for macros, tests, and generated code
- richer handling of closures and other Rust-specific units when justified
- versioned report schema and formula metadata where needed

Exit criteria:

- complexity and coverage inputs are flexible without making default usage harder

### Phase 6: Public API and Ecosystem Integrations

Goal: make crap4rust useful as a component, not only as a command.

Planned scope:

- a stable public library API
- formatter and provider extension points where they prove necessary
- editor and ecosystem integrations such as VS Code or code-scanning adapters
- optional compatibility outputs for external dashboards or legacy systems

Exit criteria:

- third-party tools can embed crap4rust without shelling out to the CLI for every workflow

## Deferred Ideas

These ideas are intentionally not prioritized until the core roadmap is further along:

- broad plugin architecture before the core data model stabilizes
- niche output formats before JSON and one strong human-readable file format land
- advanced policy systems before basic configuration exists
- ecosystem integrations that outrun the stability of the core analysis engine

## Success Measure

The roadmap is succeeding if each phase improves one of these outcomes:

- more trustworthy ranking of risky functions
- easier adoption in local development and CI
- clearer automation and reporting surfaces
- broader reuse without destabilizing the default CLI workflow

## Revision Policy

This roadmap is directional, not contractual. Phases may be reordered or narrowed if real-world use shows that a smaller, sharper scope is the better engineering decision.
