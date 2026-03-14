# Implemented Features

This document describes the feature set currently shipped by `cargo-crap4rust`.

## Version 0.1.x

The first release focuses on a minimal, usable CRAP workflow for Rust workspaces.

### CLI and Packaging

- Published as the Cargo subcommand package `cargo-crap4rust`
- Invoked as `cargo crap4rust`
- Supports `--manifest-path` for analysing a specific workspace or package manifest
- Supports repeated `--package` flags for selecting one or more workspace packages

### Metric Computation

- Computes a per-function CRAP score
- Uses the current internal cognitive-complexity scorer
- Uses line coverage from `cargo llvm-cov` JSON
- Matches coverage to functions by normalized source path and start line
- Aggregates duplicate coverage records emitted for the same source location

### Coverage Workflow

- Automatically runs `cargo llvm-cov --json` when `--coverage` is omitted
- Accepts a precomputed coverage file through `--coverage`
- Produces one combined coverage input when multiple packages are requested

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

These capabilities are planned but are not part of `0.1.x`:

- Additional output formats such as JSON, HTML, Markdown, XML, or SARIF
- Configuration file support
- Baseline or regression comparisons
- Alternative coverage formats beyond `cargo llvm-cov` JSON
- External complexity-engine integration
- Library API stabilization

See [ROADMAP.md](ROADMAP.md) for the planned expansion path.
