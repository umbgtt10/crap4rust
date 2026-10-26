# [IMPLEMENTED] A technology-free core and a cli crate

Status: **implemented**, 2026-10-26, in eleven signed commits on `main`; the
record below is the plan as it landed, row by row.

| slice | commit |
|---|---|
| S0 the record | `e33a6be` |
| S1 one dry4rust run, exclusions by path | `11b4073` |
| S2 the split | `792bf0a` |
| S3 `SourceTree` and `PathKeys` | `50a1741` |
| S4 `WorkspaceReader` | `1df0f01` |
| S5 `CoverageSource` | `11c3472` |
| S6 the report as a string | `6dc5412` |
| S7 `#![no_std]` | `0eaf6b8` |
| S8 the technology-free gate | `97f1b63` |
| S9 `.cargo/mutants.toml` | `329f01f` |
| S10 docs, and this move | the commit that added this file |

---

## A technology-free core and a cli crate

Status: **implemented** -- one commit per slice, in the order below.

## Why

Mutation testing pays for every mutant with a build and a test run. A mutant in
code that runs `cargo metadata` or `cargo llvm-cov`, reads a coverage file or
prints is slow to kill, and a test that needs a workspace to run in is slow to
run: today nine of core's tests spawn `cargo metadata`, one spawns
`cargo llvm-cov`, seventeen use a temporary directory, and a core mutant
rebuilds the `cargo-crap4rust` binary for the package's tests. So the CRAP
analysis -- complexity, the coverage arithmetic, the formula, scoring and the
human report -- moves into a crate that touches no technology at all, and
mutation testing is scoped to that crate alone.

## The shape

The workspace grows a fourth member:

| member | package | library | what it holds |
|---|---|---|---|
| `core/` | `crap4rust-core` | `crap4rust` | `#![no_std]`, `#![forbid(unsafe_code)]`; function discovery, cognitive complexity, the coverage arithmetic, the CRAP formula, scoring, the verdict and the human report. Depends on `proc-macro2` and `syn` only |
| `cli/` | `cargo-crap4rust` | `crap4rust_cli` | the binary, `Args` (clap), the composition root `EntryPoint`, and every adapter: cargo metadata, cargo llvm-cov, the file system, JSON |
| `validation/` | `validation` | `validation` | unchanged: the end-to-end tests, driving the binary cli builds |
| `xtask/` | `xtask` | `xtask` | the gates |

`cli` depends on `core`, never the reverse. `fixture/` stays excluded analysis
input. Both published crates share one workspace version.

### Ports

Core declares a trait for everything technological it needs, implemented in
`cli/src/system/` and, for core's own tests, by in-memory fakes under
`core/tests/support/`:

| port (`core/src/traits/`) | what it answers | cli adapter |
|---|---|---|
| `SourceTree` | the `.rs` files under a root, and a file's text | `WalkdirSourceTree` |
| `PathKeys` | the key a source file and a coverage record are matched by | `CanonicalPathKeys` (the canonical path, `/` separators, lower case on Windows) |
| `WorkspaceReader` | the workspace's packages and targets | `CargoWorkspaceReader` (`cargo metadata --no-deps`) |
| `CoverageSource` | the coverage export for the selected packages | `LlvmCovSource` (the `--coverage` file, or one `cargo llvm-cov` generates) |

They sit beside the five seams `App` already runs on (`PackageResolver`,
`FunctionDiscovery`, `CoverageProvider`, `Scorer`, `Reporter`), whose
implementations stay in core and use the ports. Where the line runs for
coverage: cli runs `cargo llvm-cov`, decides where its export goes, reads it and
parses its JSON; core does everything from the parsed export on -- the regions
covered of the total per function, the duplicate merge and span matching, the
ratio, the formula, the classification and the project's verdict.

Core works on its own records (`WorkspaceDescription`, `PackageDescription`,
`TargetDescription`, the export records), on `SourcePath` -- a path held as
text with `/` as its only separator, which cli converts to at the edge -- and
fails with typed errors. It returns the report as a string with the exit code
to use; printing it is `EntryPoint`'s. The human report is rendered in core,
the JSON report in cli.

The binary's behaviour does not change: stdout, stderr and exit codes are
compared byte for byte against 0.8.2 after every slice that touches them. One
exception is deliberate: a `.rs` file whose path is not UTF-8 fails the run
naming it, where 0.8.2 scored it under a name it does not have; and on Windows
paths in error texts show `/`.

### The gate

A stage-2 gate, first in the run, keeps core technology-free: its manifest may
name only `proc-macro2` and `syn` in any section, its crate root must carry
`#![no_std]` and `#![forbid(unsafe_code)]`, and no file under `core/src` or
`core/tests` may reach `std::{env,fs,io,net,os,process}`, the printing macros,
the file-system methods or `extern crate std`.

## The slices

| # | slice | status |
|---|---|---|
| S0 | this record | done |
| S1 | `DryGate`: one run over the workspace, with the trees it must not judge excluded by full path; the ignore file moves to the root it scans || done |
| S2 | the split: `cli` takes `main.rs`, `Args` and `EntryPoint`; core is renamed; `[workspace.lints]` forbids unsafe code; `Config::from_args` becomes `Args::config` || done |
| S3 | the `SourceTree` and `PathKeys` ports and `SourcePath`; discovery reads through them; the walk becomes cli's `WalkdirSourceTree`, `normalize_path` cli's `CanonicalPathKeys` || done |
| S4 | the `WorkspaceReader` port and core's workspace records; `CargoWorkspaceReader` in cli || done |
| S5 | the `CoverageSource` port: the region arithmetic stays in core, cargo llvm-cov, the coverage file and its JSON go to cli; `Config`'s cargo-facing options become cli's `CargoConfig` || done |
| S6 | the report as a string: `HumanReporter` in core, `JsonReporter` in cli; printing and the exit code go to cli; clap and serde leave core || done |
| S7 | `#![no_std]`, typed errors; anyhow leaves core || done |
| S8 | the technology-free gate, first in stage 2 || done |
| S9 | `.cargo/mutants.toml` scopes mutation testing to core || done |
| S10 | documentation; this record moves to `docs/history/` || done |

## Tests

No test is deleted and no assertion is relaxed. Tests that exercise
technology move to `cli/` with their names; tests that move to a new method are
renamed after it (`load_coverage_records_*` to `provide_*` and `export_*`,
`normalize_path_replaces_backslashes` to `key_of_replaces_backslashes`,
`from_args_*` to `config_*`, the three serialisation tests to the JSON
reporter's `render_*`). `with_coverage_path_keeps_the_path` becomes
`cargo_config_keeps_the_coverage_path`, with the same assertion, because the
cargo-facing options become a record with no methods.

When the `Args` tests moved to cli in S2, eleven of `Config`'s getters had no
caller left in core's tests; a test per getter pinned them. Five of them
followed their fields out of `Config`: in S5 `with_all_features_keeps_the_flag`,
`with_features_keeps_them`, `with_manifest_path_keeps_the_path` and
`with_no_default_features_keeps_the_flag` became `args_tests`'
`cargo_config_keeps_the_all_features_flag`, `_features`, `_manifest_path` and
`_no_default_features_flag`, same assertion values through
`Args::cargo_config()`; in S6 `with_output_format_keeps_the_format` became
`entry_point_tests`' `run_with_the_json_output_format_renders_the_json_report`,
which runs the built binary with `--output-format json` and reads its stdout as
the JSON report. The reporter tests that
asserted only "does not panic" assert the exact text. One fake's message, and
the assertion that reads it, change with the typed errors:
`run_package_resolver_failure_propagates_error` checks for "package probe was
not found in the manifest".
