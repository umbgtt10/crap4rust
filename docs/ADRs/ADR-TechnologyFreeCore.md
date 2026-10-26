# ADR-TechnologyFreeCore

- **Status:** Accepted
- **Date:** 2026-10-26

## Context

The tool is meant to be mutation-tested, and mutation testing pays for every
mutant with a build and a test run. While one crate held everything, a mutant
anywhere rebuilt a crate that linked cargo_metadata, clap, serde, serde_json
and walkdir, and built the `cargo-crap4rust` binary for the package's tests;
those tests ran `cargo metadata` nine times, spawned `cargo llvm-cov` once and
used seventeen temporary directories. The analysis -- which functions exist,
how complex they are, how much of each ran, and what that makes their CRAP
score and the project's verdict -- is what deserves mutating, and it was mixed
with code whose only job is to talk to the machine.

## Decision

The analysis lives in a technology-free crate, `crap4rust-core`: `#![no_std]`
and `#![forbid(unsafe_code)]`, depending on `proc-macro2` and `syn` only,
declaring a trait for every piece of technology it needs -- `SourceTree`,
`PathKeys`, `WorkspaceReader`, `CoverageSource` -- and returning its report as
a string with the exit code to use. A separate crate, `cargo-crap4rust`,
implements those traits with the real technology, parses the arguments,
renders the JSON report, prints, and exits.

For coverage, the line runs between the arithmetic and what feeds it: cli
runs `cargo llvm-cov`, decides where its export goes, reads the file and
parses the JSON; core does everything from the parsed export on -- regions
covered of the total per function, the duplicate merge and span matching, the
ratio, the formula, the classification and the verdict.

## Forcing constraints / Evidence

`cargo mutants --list` names 478 mutants, all in the core; the workspace held
600 before the split, 94 of the old core's 492 in technology. The core's 171
tests touch no disk and start no process. Measured on two fresh copies inside
one build slot, at a load average of six to seven, with `>` changed to `>=` in
`CrapFormula::classify` as cargo-mutants would, medians of three: a clean
build of the package went from 19.2 s to 3.5 s, its test build from 5.7 s to
1.3 s, and per mutant the rebuild from 0.67 s to 0.23 s and the test run from
0.62 s to 0.43 s.

The binary's behaviour did not move: stdout, stderr and exit codes were
compared byte for byte against 0.8.2 on twenty-six invocations after every
step, every error path and a coverage generation included. `f64::powi` is in
`core`, so every score is bit for bit what it was.

## Rejected alternatives

- **One crate, with cargo-mutants' `exclude_globs` on the technology.** Scopes
  the mutants but not the build or the tests, and nothing would stop the next
  change reaching for `std::fs` inside the analysis.
- **The coverage arithmetic in cli, with the JSON.** Counting regions and
  keying records is CRAP arithmetic; leaving it beside the file read would
  leave it unmutated.
- **The llvm-cov argument list and `LlvmCovFailure` in core.** They describe a
  process, not a score.
- **An output port for stdout.** The core returns a string; printing it is the
  composition root's one line.
- **`clippy.toml`'s `disallowed-*` lists.** Type-resolved, but blind to
  manifests, and with `#![no_std]` the compiler already guards the source.

## Consequences

Two published crates, `crap4rust-core` first, sharing one version. A new
technology need is a trait in `core/src/traits/`, an in-memory fake in
`core/tests/support/` and an adapter in `cli/src/system/`, composed in
`EntryPoint`. Paths in the core are `SourcePath`s with `/` as their only
separator, converted by `cli` at the edge; a `.rs` file whose path is not
UTF-8 fails the run instead of being scored under a name it does not have.
What reaches cargo -- the manifest path, the features, a coverage file -- stays
in cli as `PathBuf`s. The core's errors are typed and keep their causes, so
`EntryPoint` prints the chain it always printed.

## Enforcement

The "Technology-free core" gate, first in stage 2: core's manifest names only
`proc-macro2` and `syn` in any section; `core/src/lib.rs` carries `#![no_std]`
and `#![forbid(unsafe_code)]`; no file under `core/src` or `core/tests` reaches
`std::{env,fs,io,net,os,process}`, a printing macro, a file-system method of
`Path` or `extern crate std`. `.cargo/mutants.toml` scopes cargo-mutants to
`core/src`, each mutant tested by core's tests only.

## Related

- [ADR-DynDispatchAppOverGenerics](ADR-DynDispatchAppOverGenerics.md)
- [`../ARCHITECTURE.md`](../ARCHITECTURE.md)
- [`../history/[IMPLEMENTED] CORE_CLI_SPLIT.md`](../history/%5BIMPLEMENTED%5D%20CORE_CLI_SPLIT.md)
