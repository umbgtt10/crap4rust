# crap4rust

`crap4rust` computes CRAP scores for Rust functions by combining complexity and test coverage.

## Definition
What is a CRAP score?
CRAP (Change Risk Anti-Patterns) combines cognitive complexity and test coverage: CRAP(m) = comp(m)² × (1 − cov(m))³ + comp(m). Functions above a score of 30 are flagged as crappy — they are complex enough that their lack of test coverage makes them a maintenance risk.

Full derivation of every term — how cognitive complexity is scored construct
by construct, how coverage is matched and duplicate records resolved, and
how a project-level verdict is computed from every function's own — is in
[`docs/FORMULA.md`](docs/FORMULA.md).

It is published as the Cargo subcommand package `cargo-crap4rust`, so the command is `cargo crap4rust`.

## Documentation

| Doc | What's in it |
|---|---|
| [`docs/ARCHITECTURE.md`](docs/ARCHITECTURE.md) | How a `crap4rust` invocation flows through the code, module by module. |
| [`docs/FORMULA.md`](docs/FORMULA.md) | Every scoring term, in full, kept in sync with `src/`. |
| [`docs/ADRs/`](docs/ADRs/) | Why the codebase is shaped the way it is. |
| [`ROADMAP.md`](ROADMAP.md) | What's shipped, what's next. |
| [`OPEN_POINTS.md`](OPEN_POINTS.md) | Known gaps, deliberately deferred. |
| [`IMPLEMENTED-FEATURES.md`](IMPLEMENTED-FEATURES.md) | The full shipped feature set. |
| [`CHANGELOG.md`](CHANGELOG.md) | Release history. |

## Install

```powershell
cargo install cargo-crap4rust
```

## License

Licensed under:

- [LICENSE](LICENSE)

## What It Does

- Computes a CRAP score for each discovered Rust function
- Generates coverage automatically with `cargo llvm-cov` when `--coverage` is omitted
- Prints a single report to the console
- Supports multiple `--package` flags for one aggregated report
- Defaults to analysing all workspace members when `--package` is omitted in a multi-package workspace
- Does not count try-operator propagation with `?` as cognitive complexity
- Supports `--output-format json` for structured CI-friendly output
- Supports `--warn-threshold` to set the warning level independently from the crappy threshold
- Excludes test-only code from discovery whether it's an inline `#[cfg(test)] mod tests { ... }` block or a file-based `#[cfg(test)] mod tests;` submodule
- Cognitive complexity scoring lives in its own dedicated module

## Examples

Analyse the default scope for a manifest:

- single-package manifest: analyses the root package
- multi-package workspace: analyses all workspace members unless `--package` is provided

```powershell
cargo crap4rust --manifest-path C:\Projects\my-workspace\Cargo.toml
```

Analyse one specific package in a workspace and override the default all-members selection:

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

Pass Cargo feature flags to the coverage build:

```powershell
cargo crap4rust --manifest-path C:\Projects\my-workspace\Cargo.toml --package app-core --features host-tests
```

Disable default features and enable specific ones:

```powershell
cargo crap4rust --manifest-path C:\Projects\my-workspace\Cargo.toml --package app-core --no-default-features --features host-analysis
```

Include test targets in the analysis:

```powershell
cargo crap4rust --manifest-path C:\Projects\my-workspace\Cargo.toml --package app-validation --include-test-targets
```

Exclude specific source paths from analysis:

```powershell
cargo crap4rust --manifest-path C:\Projects\my-workspace\Cargo.toml --package app-core --exclude-path src/scenarios
```

Output as JSON for CI pipelines:

```powershell
cargo crap4rust --manifest-path C:\Projects\my-workspace\Cargo.toml --output-format json
```

Set a custom warning threshold:

```powershell
cargo crap4rust --manifest-path C:\Projects\my-workspace\Cargo.toml --warn-threshold 15
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
Production functions only — test code and generated code excluded by default.

The report above is abbreviated to the highest-scoring rows, with function names and file paths shortened for readability. When coverage is generated automatically, `cargo llvm-cov` also emits normal build and test output before the final crap4rust report.

Try-operator propagation with `?` is treated as error forwarding rather than decision-making complexity, so CRAP scoring reflects branching and control-flow structure instead of penalising straightforward `Result` propagation.

## Current Scope

The current implementation focuses on:

- console and JSON reporting
- automatic `cargo llvm-cov` integration
- internal cognitive-complexity scoring in a dedicated module
- workspace package selection and aggregation
- all-workspace-member default selection when `--package` is omitted in multi-package workspaces
- try-operator propagation excluded from cognitive-complexity scoring
- configurable warning threshold independent from the crappy threshold

See [IMPLEMENTED-FEATURES.md](IMPLEMENTED-FEATURES.md) for the shipped feature set and [ROADMAP.md](ROADMAP.md) for the broader plan.
