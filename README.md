# crap4rust

`crap4rust` computes CRAP scores for Rust functions by combining complexity and test coverage.

It is published as the Cargo subcommand package `cargo-crap4rust`, so the command is `cargo crap4rust`.

## Install

```powershell
cargo install cargo-crap4rust
```

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

See [Phase1.md](Phase1.md) and [crap4rust_specification.md](crap4rust_specification.md) for the broader roadmap.
