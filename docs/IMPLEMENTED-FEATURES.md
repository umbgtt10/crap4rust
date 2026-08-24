# Implemented Features

This document describes the feature set currently shipped by `cargo-crap4rust`.

## Version 0.7.0

### Correctness Fixes

- Coverage duplicate handling is now order-independent: a zero-count
  duplicate coverage record is discarded in favor of a non-zero one for the
  same file/line regardless of which one `cargo-llvm-cov` happens to list
  first (previously only one arrival order was handled correctly; the other
  silently reproduced the halved-coverage-ratio bug the original fix was
  meant to eliminate)
- A file-based `#[cfg(test)] mod name;` test submodule is now excluded from
  function discovery the same way an inline `#[cfg(test)] mod name { ... }`
  block already was, via a new project-wide `TestModuleRegistry` pre-pass
- `while` conditions and `match` arm guards no longer double-count `&&`/`||`
  operators against cognitive complexity (a textually identical condition on
  an `if` was already scored correctly; `while`/guards scored every logical
  operator twice)

### Structural

- `App` is now built from five injected traits (`PackageResolver`,
  `FunctionDiscovery`, `CoverageProvider`, `Scorer`, `Reporter`) rather than
  calling concrete modules directly, matching the `grip`/`braintax` sibling
  tools' own dependency-injection shape
- `model.rs` (seven type definitions in one file) split into one file per
  type, each with its own dedicated test file
- `docs/ADRs/`, `docs/FORMULA.md`, `docs/ARCHITECTURE.md`, `CLAUDE.md`, and
  `OPEN_POINTS.md` added
- License headers normalized to MIT-only across every source file (`Cargo.toml`
  and `LICENSE` were already MIT; the headers themselves had drifted to a
  `MIT OR Apache-2.0` dual-license form, or plain Apache-2.0 in the stage
  scripts)
- `pub(crate)` items widened to `pub` wherever that makes them directly
  testable from `tests/` — this crate's integration-tests-only policy means a
  `pub(crate)` item is otherwise unreachable from any test
- Free functions that were thin wrappers around, or exclusive helpers for,
  one struct consolidated into that struct's own `impl` block
  (`SourceRootCollector`, `CargoPackageResolver`, `CoverageIndex`,
  `SourceFunctionDiscovery`, `ImplCollector`, `FileWalker`); functions
  genuinely shared across multiple structs' files (`normalize_path`,
  `qualified_name`, `start_line`, `end_line`, `is_test_attrs`) deliberately
  stay free functions

See [CHANGELOG.md](../CHANGELOG.md) for the full entry.

## Version 0.6.0

### CLI and Packaging

- `--warn-threshold` sets the warning threshold independently from the crappy threshold (default 20.0, no longer hardcoded)
- `--output-format` accepts `human` (default) or `json` for structured output
- Published as `cargo-crap4rust`

### Metric Computation

- Cognitive complexity scoring extracted into its own `complexity` module
- Try-operator (`?`) is no longer counted as cognitive complexity
- Builder-style error propagation scores based on actual control-flow structure

### Reporting

- JSON output format via `--output-format json` — produces a structured report parseable by CI pipelines and dashboards
- JSON report includes all fields: scope name, total/crappy functions, crappy percentage, per-function details with complexity, coverage, CRAP score, and verdict

### Code Quality

- `source.rs` complexity scoring extracted to dedicated `src/complexity.rs` — 15 functions moved, `source.rs` reduced from 34 to ~20 functions
- Test infrastructure aligned with `faction` conventions:
  - `tests/all_tests.rs` plumbing with `autotests = false`
  - Test files named `<source>_tests.rs` per convention
  - CLI fixture tests under `tests/fixtures/` with `mod.rs`
  - `tests/complexity_tests.rs` — 10 direct tests parsing Rust code via `syn`
  - `tests/source_tests.rs` — full-stack complexity integration tests
  - `tests/app_tests.rs` — 20 pure logic tests with AAA structure
- All tests follow AAA (Arrange / Act / Assert) convention

### Test Count

- 69 tests total: 28 fixture integration + 20 app logic + 11 source integration + 10 complexity direct
- Single `all_tests` binary, no auto-discovery

## Version 0.3.0

The first release focuses on a minimal, usable CRAP workflow for Rust workspaces.

### CLI and Packaging

- Published as the Cargo subcommand package `cargo-crap4rust`
- Invoked as `cargo crap4rust`
- Supports `--manifest-path` for analysing a specific workspace or package manifest
- Supports repeated `--package` flags for selecting one or more workspace packages
- Supports `--features` for passing Cargo feature flags to the coverage build
- Supports `--all-features` to activate all features during the coverage build
- Supports `--no-default-features` to disable default features during the coverage build

### Metric Computation

- Computes a per-function CRAP score
- Uses the current internal cognitive-complexity scorer
- Uses line coverage from `cargo llvm-cov` JSON
- Matches coverage to functions by normalized source path and start line
- Aggregates duplicate coverage records emitted for the same source location

### Coverage Workflow

- Automatically runs `cargo llvm-cov --json` when `--coverage` is omitted, which
  requires `cargo-llvm-cov` and the `llvm-tools` rustup component
- Accepts a precomputed coverage file through `--coverage`, the one route that
  needs neither
- Reports an absent `cargo-llvm-cov` by name and with the install command,
  rather than as the exit code cargo returns for an unknown subcommand
- Carries `cargo llvm-cov`'s own stderr into the error when a coverage run
  fails for any other reason
- Produces one combined coverage input when multiple packages are requested

### Source Filtering

- Supports `--include-test-targets` to include test targets in function discovery (excluded by default)
- Supports repeatable `--exclude-path` to omit specific source paths from analysis

### Reporting

- Prints a single console table report
- Shows package, function name, file, line, complexity, coverage, CRAP score, and verdict
- Prints a project summary with total functions, crappy functions, crappy percentage, threshold values, and final verdict

### Exit Behavior

- Exit code `0` for pass or report-only mode with `--warn-only`
- Exit code `1` when the selected scope fails configured CRAP thresholds
- Exit code `2` for tool or input failures
- Supports `--strict`, `--warn-only`, `--threshold`, and `--project-threshold`

### Validation Status

- Validated locally against fixture workspaces through integration tests
- Validated against a larger real-world Rust workspace during Phase 1 development
- Published to crates.io as `cargo-crap4rust`

## Not Yet Implemented

These capabilities are planned but are not part of `0.6.x`:

- Additional output formats such as HTML, Markdown, XML, or SARIF
- Configuration file support
- Baseline or regression comparisons
- Alternative coverage formats beyond `cargo llvm-cov` JSON
- External complexity-engine integration
- Library API stabilization

See [ROADMAP.md](ROADMAP.md) for the planned expansion path.
