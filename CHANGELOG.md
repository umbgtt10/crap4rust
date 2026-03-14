# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/), and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Planned

- Additional output formats such as JSON, HTML, Markdown, XML, and SARIF
- Baseline and regression comparison support
- Configuration file support
- Broader coverage-source support
- A stable public library API

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

[Unreleased]: https://github.com/umbgtt10/crap4rust/compare/v0.2.0...HEAD
[0.2.0]: https://github.com/umbgtt10/crap4rust/releases/tag/v0.2.0
[0.1.0]: https://github.com/umbgtt10/crap4rust/releases/tag/v0.1.0
