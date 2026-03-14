# crap4rust

`crap4rust` computes CRAP scores for Rust functions by combining complexity and test coverage.

It is published as the Cargo subcommand package `cargo-crap4rust`, so the command is `cargo crap4rust`.

Current status and release notes:

- [IMPLEMENTED-FEATURES.md](IMPLEMENTED-FEATURES.md) documents what `0.2.x` supports today
- [ROADMAP.md](ROADMAP.md) tracks planned capabilities beyond the first release
- [CHANGELOG.md](CHANGELOG.md) records released versions

## Install

```powershell
cargo install cargo-crap4rust
```

## License

Licensed under either of:

- [LICENSE-APACHE](LICENSE-APACHE)
- [LICENSE-MIT](LICENSE-MIT)

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

## Real Workspace Example

Example run against the Etheram workspace:

```powershell
cargo crap4rust --manifest-path C:\Projects\etheram\Cargo.toml --package etheram-node --package etheram-validation
```

Report excerpt:

`crap4rust report for etheram-node, etheram-validation`

| Package | Function | File | Line | Complexity | Coverage | CRAP | Verdict |
| --- | --- | --- | ---: | ---: | ---: | ---: | --- |
| `etheram-node` | `ConsensusWal::from_bytes` | `consensus_wal.rs` | 115 | 72 | 41.7% | 1099.3 | `crappy` |
| `etheram-node` | `execute_bytecode` | `tiny_evm_engine.rs` | 572 | 39 | 43.3% | 316.1 | `crappy` |
| `etheram-node` | `RecoveryImportValidator::validate_response` | `recovery_import_validator.rs` | 13 | 21 | 47.9% | 83.3 | `crappy` |
| `etheram-node` | `IbftProtocol::handle_client_message` | `ibft_protocol_dispatch.rs` | 82 | 19 | 47.1% | 72.6 | `crappy` |
| `etheram-node` | `exec_sha3` | `tiny_evm_engine.rs` | 345 | 12 | 30.3% | 60.7 | `crappy` |

Summary: `total_functions=388`, `crappy_functions=12`, `crappy_percent=3.1%`, `threshold=30.0`, `project_threshold=5.0%`, `verdict=warn`.

The report above is abbreviated to the highest-scoring rows, with function names and file paths shortened for readability. When coverage is generated automatically, `cargo llvm-cov` also emits normal build and test output before the final crap4rust report.

## Current Scope

The current implementation focuses on:

- console reporting
- automatic `cargo llvm-cov` integration
- internal cognitive-complexity scoring
- workspace package selection and aggregation

See [IMPLEMENTED-FEATURES.md](IMPLEMENTED-FEATURES.md) for the shipped feature set and [ROADMAP.md](ROADMAP.md) for the broader plan.
