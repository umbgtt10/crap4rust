# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/), and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Planned

- Additional output formats such as HTML, Markdown, XML, or SARIF
- Baseline and regression comparison support
- Configuration file support
- Broader coverage-source support
- A stable public library API

## [0.6.0] - 2025-04-26

### Added

- `--output-format json` for structured CI-friendly output
- `--warn-threshold` CLI argument to set the warning threshold independently (default 20.0)
- `src/complexity.rs` — cognitive complexity scoring extracted from `source.rs` into its own module
- `tests/complexity_tests.rs` — 10 direct tests parsing Rust code via `syn`
- `tests/app_tests.rs` — 20 pure logic tests with AAA structure
- `tests/source_tests.rs` — full-stack complexity integration tests (renamed from `complexity_tests.rs`)
- `tests/all_tests.rs` plumbing with `autotests = false`, following `faction` conventions
- `tests/fixtures/mod.rs` — CLI fixture tests organized under `tests/fixtures/`

### Changed

- JSON output format via `--output-format json` produces structured report with all fields
- `source.rs` reduced from 34 to ~20 functions after complexity extraction
- Test files follow `<source>_tests.rs` naming convention
- All tests follow AAA (Arrange / Act / Assert) structure
- 69 tests total: 28 fixture + 20 app logic + 11 source + 10 complexity

### Fixed

- License link in README (was broken `LICENS`, now `LICENSE`)

## [0.5.1] - 2025-04-26

### Added

- `--warn-threshold` CLI argument to set the warning threshold independently of the crappy threshold
- Integration test verifying custom `--warn-threshold` value appears in the output message

### Changed

- `--warn-threshold` is no longer hardcoded at 20.0; defaults to 20.0 but is now configurable via CLI

## [0.5.0] - 2026-04-20

Fifth public release.

### Added

- Regression coverage ensuring try-operator propagation does not contribute to reported complexity

### Changed

- Try-operator propagation with `?` is no longer counted as cognitive complexity in CRAP scoring
- Builder-style error propagation now scores based on actual control-flow structure rather than `?` usage

## [0.4.0] - 2026-04-20

Fourth public release.

### Added

- Regression coverage ensuring workspace manifests default to analysing all workspace members when `--package` is omitted
- Regression coverage ensuring explicit `--package` selection still overrides the all-members workspace default
- Regression coverage for aggregate workspace reporting and automatic coverage generation under the new default package-selection behavior

### Changed

- Default package resolution now selects all workspace members for multi-package workspaces when `--package` is not provided
- Workspace-scope reporting and automatic coverage generation now follow the resolved all-members default instead of requiring explicit package selection

## [0.3.0] - 2026-04-06

Third public release.

### Added

- `--features` flag for passing Cargo feature flags to the coverage build
- `--all-features` flag to activate all features during the coverage build
- `--no-default-features` flag to disable default features during the coverage build
- `--include-test-targets` flag to include test targets in function discovery
- `--exclude-path` flag (repeatable) to omit specific source paths from analysis

## [0.2.0] - 2026-03-14

Second public release.

### Added

- Regression coverage for root-package-only automatic coverage generation
- Regression coverage ensuring non-production test targets are excluded by default
- Regression coverage ensuring `#[cfg(test)]` modules inside `src/` are excluded from discovery
- Regression coverage ensuring normal successful runs remain silent on stderr

### Changed

- Automatic `cargo llvm-cov` generation now follows the resolved package selection instead of raw CLI package flags
- Coverage matching now falls back from exact function start-line matches to the nearest matching line within the discovered function span
- Source discovery now filters out non-production targets and excludes test-only code paths more aggressively

## [0.1.0] - 2026-03-14

First public release.

### Added

- Cargo subcommand packaging as `cargo-crap4rust`
- Console CRAP report for Rust functions
- Internal cognitive-complexity scoring
- Automatic `cargo llvm-cov` JSON generation when coverage input is omitted
- Support for explicit precomputed coverage files
- Workspace package selection with repeated `--package`
- Combined multi-package reporting
- Threshold-based exit behavior with `--strict` and `--warn-only`
- Integration-test coverage for the Phase 1 command-line workflow

### Published

- Initial crates.io release of `cargo-crap4rust`

[Unreleased]: https://github.com/umbgtt10/crap4rust/compare/v0.6.0...HEAD
[0.6.0]: https://github.com/umbgtt10/crap4rust/releases/tag/v0.6.0
[0.5.1]: https://github.com/umbgtt10/crap4rust/releases/tag/v0.5.1
[0.5.0]: https://github.com/umbgtt10/crap4rust/releases/tag/v0.5.0
[0.4.0]: https://github.com/umbgtt10/crap4rust/releases/tag/v0.4.0
[0.3.0]: https://github.com/umbgtt10/crap4rust/releases/tag/v0.3.0
[0.2.0]: https://github.com/umbgtt10/crap4rust/releases/tag/v0.2.0
[0.1.0]: https://github.com/umbgtt10/crap4rust/releases/tag/v0.1.0