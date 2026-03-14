# crap4rust

`crap4rust` computes CRAP scores for Rust functions by combining complexity and test coverage.

It is published as the Cargo subcommand package `cargo-crap4rust`, so the command is `cargo crap4rust`.

Current status and release notes:

- [IMPLEMENTED-FEATURES.md](IMPLEMENTED-FEATURES.md) documents what `0.1.x` supports today
- [ROADMAP.md](ROADMAP.md) tracks planned capabilities beyond the first release
- [CHANGELOG.md](CHANGELOG.md) records released versions

## Documentation Map

- Start with this README for installation and basic usage
- Read [IMPLEMENTED-FEATURES.md](IMPLEMENTED-FEATURES.md) for the current shipped scope
- Read [ROADMAP.md](ROADMAP.md) for planned phases and longer-term direction
- Read [CHANGELOG.md](CHANGELOG.md) for release history

## Install

```powershell
cargo install cargo-crap4rust
```

## License

Licensed under either of:

- [LICENSE-APACHE.md](LICENSE-APACHE.md)
- [LICENSE-MIT.md](LICENSE-MIT.md)

## What It Does

- Computes a CRAP score for each discovered Rust function
- Generates coverage automatically with `cargo llvm-cov` when `--coverage` is omitted
- Prints a single report to the console
- Supports multiple `--package` flags for one aggregated report

## Examples

Analyse the default package for a manifest:

```powershell
cargo crap4rust --manifest-path C:\Projects\my-workspace\Cargo.toml
```

Analyse one specific package in a workspace:

```powershell
cargo crap4rust --manifest-path C:\Projects\my-workspace\Cargo.toml --package app-core
```

Analyse multiple packages and produce one combined console report:

```powershell
cargo crap4rust --manifest-path C:\Projects\my-workspace\Cargo.toml --package app-core --package app-validation
```

Use a precomputed coverage export instead of generating coverage automatically:

```powershell
cargo crap4rust --manifest-path C:\Projects\my-workspace\Cargo.toml --package app-core --coverage C:\Projects\my-workspace\target\coverage.json
```

Use stricter project thresholds:

```powershell
cargo crap4rust --manifest-path C:\Projects\my-workspace\Cargo.toml --threshold 25 --project-threshold 3.0 --strict
```

## Current Scope

The current implementation focuses on:

- console reporting
- automatic `cargo llvm-cov` integration
- internal cognitive-complexity scoring
- workspace package selection and aggregation

See [IMPLEMENTED-FEATURES.md](IMPLEMENTED-FEATURES.md) for the shipped feature set and [ROADMAP.md](ROADMAP.md) for the broader plan.
