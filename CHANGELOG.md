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

[Unreleased]: https://github.com/umbgtt10/crap4rust/compare/v0.1.0...HEAD
[0.1.0]: https://github.com/umbgtt10/crap4rust/releases/tag/v0.1.0
