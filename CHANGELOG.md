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

## [0.7.0] - 2026-07-26

### Fixed

- `CoverageIndex::from_records` discards a zero-coverage duplicate record in
  favor of a non-zero one for the same `(path, line)` key regardless of
  which one arrives first. The `[0.6.2]` fix below only handled the case
  where the real record arrived first; when the zero-count "ghost" record
  arrived first instead, it was summed with the real record that followed
  it, silently reproducing the exact halved-ratio bug `[0.6.2]` was meant to
  eliminate. See `docs/ADRs/ADR-SymmetricDuplicateCoverageHandling.md`.
- File-based `#[cfg(test)] mod name;` test submodules (test code split into
  its own sibling file, as opposed to an inline `#[cfg(test)] mod name {
  ... }` block) are now excluded from function discovery. Previously only
  the inline form was recognized — the `#[cfg(test)]` gate lives on the
  `mod` statement in the *parent* file, which `FileWalker` had no way to see
  while walking the child file on its own, so a file-based test submodule's
  functions were counted as production code identical to real shipped
  functions. See `docs/ADRs/ADR-CrossFileTestModuleExclusion.md`.
- `while` conditions and `match` arm guards no longer count each `&&`/`||`
  operator twice toward cognitive complexity. A textually identical `if`
  condition was already scored correctly (once per operator); `while` and
  match guards separately re-added the same operators through a redundant
  code path. See `docs/FORMULA.md`.

### Changed

- **Breaking (library surface only, not the CLI):** `App` is now built from
  five injected traits — `PackageResolver`, `FunctionDiscovery`,
  `CoverageProvider`, `Scorer`, `Reporter` (all under `traits/`) — rather
  than calling `manifest`/`source`/`coverage`/`report` modules directly.
  `App::new(config)` wires the real implementations
  (`CargoPackageResolver`/`SourceFunctionDiscovery`/`LlvmCovProvider`/
  `DefaultScorer`/`StdoutReporter`); `App::with_deps(...)` takes all five as
  explicit parameters for callers that need to substitute one. `model.rs`
  (seven type definitions in one file) is now one file per type
  (`config.rs`, `package_context.rs`, `source_function.rs`,
  `coverage_record.rs`, `verdict.rs`, `function_report.rs`,
  `project_report.rs`), and `manifest.rs`/`source.rs`/`report.rs` are
  renamed to `cargo_package_resolver.rs`/`source_function_discovery.rs`/
  `stdout_reporter.rs` to match the struct each now holds. CLI flags,
  console/JSON output shape, and exit-code semantics are all unchanged.
- `app::compute_crap_score`/`app::classify` (bare free functions) are now
  `CrapFormula::score`/`CrapFormula::classify` (`crap_formula.rs`);
  `app::project_fails` is now `Config::fails` (`config.rs`); the
  project-metrics aggregation is now `DefaultScorer::project_metrics`
  (`default_scorer.rs`); exit-code selection is now
  `ProjectReport::exit_code` (`project_report.rs`). The formulas themselves
  are unchanged.

### Added

- `CLAUDE.md`, `docs/ADRs/` (four ADRs plus an index), `docs/FORMULA.md`,
  `docs/ARCHITECTURE.md`, and `OPEN_POINTS.md`, matching the `grip`/
  `braintax` sibling tools' documentation depth.
- Dedicated test files for every new/split source file, including direct
  unit coverage for `DefaultScorer`'s project-metrics aggregation and
  `TestModuleRegistry`'s resolution rules, neither of which had direct
  tests before (only indirect, CLI-level coverage). 164 tests total, up
  from 129 at the start of this cleanup.

## [0.6.2] - 2026-05-02

### Fixed

- `from_records` now skips zero-coverage duplicate function records when a
  non-zero record already exists for the same (path, line) key. LLVM coverage
  can emit multiple function records for the same source — one with actual
  execution counts and one with all-zero counts. Previously both were
  aggregated, halving the coverage ratio (e.g., 50% for `Protocol::decide`
  despite full branch coverage). The zero-count ghost record is now discarded.

## [0.6.1] - 2026-05-02

### Fixed

- `match_function_coverage` now aggregates all coverage regions within a
  function's line span instead of returning only the single nearest region.
  Previously, functions with multiple coverage regions (e.g., `if let` /
  `else if` / `else` chains) had inaccurate coverage ratios because only
  one region was counted.

## [0.6.0] - 2025-04-26

### Added

- `--output-format json` for structured CI-friendly output
- `--warn-threshold` CLI argument to set the warning threshold independently (default 20.0)
- `src/complexity.rs` — cognitive complexity scoring extracted from `source.rs` into its own module
- `tests/complexity_tests.rs` — 10 direct tests parsing Rust code via `syn`
- `tests/app_tests.rs` — 20 pure logic tests with AAA structure
- `tests/source_tests.rs` — full-stack complexity integration tests (renamed from `complexity_tests.rs`)
- `tests/all_tests.rs` plumbing with `autotests = false`
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

[Unreleased]: https://github.com/umbgtt10/crap4rust/compare/v0.7.0...HEAD
[0.7.0]: https://github.com/umbgtt10/crap4rust/releases/tag/v0.7.0
[0.6.2]: https://github.com/umbgtt10/crap4rust/releases/tag/v0.6.2
[0.6.1]: https://github.com/umbgtt10/crap4rust/releases/tag/v0.6.1
[0.6.0]: https://github.com/umbgtt10/crap4rust/releases/tag/v0.6.0
[0.5.1]: https://github.com/umbgtt10/crap4rust/releases/tag/v0.5.1
[0.5.0]: https://github.com/umbgtt10/crap4rust/releases/tag/v0.5.0
[0.4.0]: https://github.com/umbgtt10/crap4rust/releases/tag/v0.4.0
[0.3.0]: https://github.com/umbgtt10/crap4rust/releases/tag/v0.3.0
[0.2.0]: https://github.com/umbgtt10/crap4rust/releases/tag/v0.2.0
[0.1.0]: https://github.com/umbgtt10/crap4rust/releases/tag/v0.1.0
