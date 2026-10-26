# Crap4Rust

## Meaning

`crap4rust` is a cargo subcommand that computes CRAP (Change Risk Anti-Patterns)
scores for Rust functions — a composite of cognitive complexity and test
coverage that flags code which is both complex and under-tested.

It is self-contained.

## Boundary Rule

This repository is **SELF-CONTAINED**.

The LLM **SHALL NOT cross its boundaries without asking**.

That means:
- do not inspect, edit, or rely on files outside `crap4rust/` unless the user explicitly asks
- do not pull assumptions from sibling repositories or crates
- do not propose cross-repository changes by default

## Quality Gates

### Mandatory after every change to `src/` or `tests/` of any crate in the workspace

Run gates:

`just stage1`
`just stage2`

If either gate is not green, the work is not complete.

Stage 1 is formatting, clippy and tests. It is cargo built-ins plus
`cargo-llvm-cov`: this repository is the coverage tool, so its validation tests
drive the built binary through the path that generates coverage, and a checkout
without that install fails stage 1 rather than stage 2. Stage 2 is
`cargo xtask stage2`, which runs the xtask's own technology-free gate and then
six installed cargo subcommands, in this order:

| gate | asks |
|---|---|
| Technology-free core | does `core` still touch no technology |
| `cargo stern4rust` | do the house coding rules hold |
| `cargo shear` | is every declared dependency used, in the section that uses it |
| `cargo dry4rust` | did this change add duplicated code |
| `cargo crap4rust` | is any function complex and untested |
| `cargo twin4rust` | does every source file have a mirrored test file |
| `cargo iceberg4rust` | is any file's private implementation risk too high |

The technology-free gate runs **first**. It is the xtask's own -- no tool to
install -- and asks three things of `core`: that its manifest names only
`proc-macro2` and `syn`, in any section (read from `cargo metadata --no-deps`;
a package missing from the metadata fails); that `core/src/lib.rs` carries
`#![no_std]` and `#![forbid(unsafe_code)]`, which hand `core/src` to the
compiler; and that no file under `core/src` or `core/tests` reaches
`std::{env,fs,io,net,os,process}`, a printing macro, a file-system method of
`Path` or `extern crate std`, read with syn so a string literal is data. The
`core/src` scan is a backstop to the compiler; `core/tests`, where std is
linked, depends on it. Calls inside a macro's arguments are not seen, and a
method of one's own named like a file-system one would be flagged. Its fix is
moving code into `cli`, which changes what every gate behind it measures, and
it takes a second.

stern4rust runs **second** because its corrections are renames, file moves and
directory splits: a layout it is about to reject is a layout the others would
have measured for nothing. Its findings are also the cheapest to act on.

cargo-shear runs **third**: its corrections edit manifests only -- a
dependency removed, or moved to the section that uses it -- so they change
nothing the gates behind it measure, and it takes a second. It runs `--locked`,
reading the lock file and never writing it, and `--deny-warnings`, so an
optional dependency no feature's code uses fails as an unused one does. A
dependency it cannot see used -- linked only for what it provides, declared
only to switch on a feature, used only by generated code -- goes in
`[package.metadata.cargo-shear] ignored`, and the gate refuses an entry without
its reason in the same table's `reasons`; there are none today. Pinned at
1.14.0, as CI installs it.

`dry4rust` runs **fourth** for the same reason: removing a duplicate moves code
between files, which changes what the three behind it measure. It makes one run
from the workspace root over both published crates' sources, so a copy from one
into the other is seen, with `xtask`, `validation`, `fixture`, `core/tests` and
`cli/tests` excluded -- each by its full path, joined a component at a time,
because dry4rust matches an exclusion as a substring of a file's path; tests
repeat their arrangement by design -- and checks with zero ceilings against
`dry4rust-baseline.json`, the duplication already there
when the gate arrived. So it fails on what a change adds, not on what it
inherited; a duplicate removed is admitted, a copy added to a recorded group is
not.

It counts only code units of 25 AST nodes or more, the floor the family gates
at. The baseline holds no group. The one it was recorded with --
`App::with_deps`, now `App::new`, and `ImplCollector::new`, two constructors
that each store their arguments field by field -- no longer matches:
`ProjectReport::new` joined it, and the three are the first ignore entry below.

Re-record the baseline only to drop groups that are gone, never to admit new
ones, and at the same floor -- a baseline matches only at the floor it was
recorded at: `cargo dry4rust --path "$PWD" --exclude "$PWD/xtask" --exclude "$PWD/validation" --exclude "$PWD/fixture" --exclude "$PWD/core/tests" --exclude "$PWD/cli/tests" --min-nodes 25 --baseline "$PWD/dry4rust-baseline.json" baseline`.

`.dry4rust-ignore.toml`, at the root dry4rust scans from -- the only place it
reads one -- holds two fingerprints, with their reason:
plain constructors -- `Self { a, b, c, d }` -- which look alike when they fill
as many fields. What is left of them belongs to objects: stern4rust's
`records-and-objects` lets an object keep its constructor and builds a record
by literal. `App::new`, `ImplCollector::new` and `ProjectReport::new` are one
group, `CoverageRecord::new` and `ItemVisitor::new` the other. dry4rust
leaves plain constructors out from its next release
(ADR-PlainConstructorsAreTheStructsShape, in dry4rust); until the gate installs
it, these entries bridge, and the file goes when the release does.

All twenty-four of its rules apply to every workspace member, with nothing
skipped, nothing selected and nothing unconfigured: `stern4rust.toml` holds no
`skip`, no `rules` and no per-package section. `docs/header.txt` holds the
three-line header every `.rs` file carries and `stern4rust.toml` names it -- in
the config rather than the gate script, so a hand-run of `cargo stern4rust`
checks exactly what the gate checks. The gate passes all four members by name,
which is the same set a bare `cargo stern4rust` at the root scans.

`cargo install just`
`cargo install cargo-stern4rust`
`cargo install cargo-dry4rust`
`cargo install cargo-crap4rust`
`cargo install cargo-twin4rust`
`cargo install cargo-iceberg4rust`
`cargo install --locked cargo-shear --version 1.14.0`
`cargo install cargo-llvm-cov`
`rustup component add llvm-tools`

`cargo-llvm-cov` is crap4rust's own coverage backend rather than a house tool,
and stage 1 needs it too, not only stage 2: the validation tests drive the built
binary through the path that generates coverage. A missing install now names
itself -- `LlvmCovFailure` reads cargo's own "no such command" off stderr and
answers with the install line, instead of the bare
`cargo llvm-cov failed with exit code Some(101)` it used to report.

The gates are a `justfile` plus an `xtask` workspace member, not scripts: one
entry point that behaves the same on Linux, Windows and macOS, and gate
orchestration in Rust rather than shell text-parsing. `xtask` reads crap4rust's
`--output-format json` instead of matching a regex against its table, and names
every offending function when it fails. `.github/workflows/ci.yml` runs both
stages on all three platforms for every pull request and every push to `main`.

The CRAP gate measures this repository with the *published* crap4rust rather
than the binary in the working tree, which is what the PowerShell gate did and
what keeps a regression in the tree from hiding itself.

The measuring gates -- CRAP, mirrored tests, file risk -- take the two
published crates by name, `--package crap4rust-core --package cargo-crap4rust`
against the workspace manifest, which is what keeps validation and xtask out of
them. The house-rules gate is not: it holds every workspace member to every
rule.

- `fixture/` holds deliberately-crappy analysis inputs. They are `exclude`d
  from the workspace, so no gate and no `cargo test` ever compiles them.
- `validation/` holds the end-to-end tests that drive the built binary against
  those fixtures. It **is** a workspace member, so the root `cargo test` runs
  all of it. Its tests answer to the whole tool rather than to any one source
  file in `core/` or `cli/`, so the measuring gates leave it alone; the binary
  they drive is cli's. Its `src/` is the
  apparatus those tests share -- `Fixture` locates a crate under `fixture/`,
  `Source` finds the line a fixture function starts on -- and each test file is
  named for the apparatus it is built on, which is what lets `paired-test-file`
  hold there without the skip it used to need.

## Technology-free core

The workspace has four members:

| member | package | what it holds |
|---|---|---|
| `core/` | `crap4rust-core`, library `crap4rust` | the analysis: function discovery, cognitive complexity, the coverage arithmetic, the CRAP formula, scoring, the verdict and the human report. `#![no_std]`, `#![forbid(unsafe_code)]`, depending on `proc-macro2` and `syn` only. Published |
| `cli/` | `cargo-crap4rust`, library `crap4rust_cli`, binary `cargo-crap4rust` | `Args`, the composition root `EntryPoint`, and every adapter: cargo metadata, cargo llvm-cov, the file system, the JSON report. Published |
| `validation/` | `validation` | the end-to-end tests of the binary, above. `publish = false` |
| `xtask/` | `xtask` | gate orchestration, run as `cargo xtask`. `publish = false` |

`cli` depends on `core`, never the reverse; core and cli share the
workspace's `version`, and `[workspace.lints.rust]` forbids `unsafe_code` in
every member and target. Core reaches the machine only through the traits in
`core/src/traits/`: `SourceTree`, `PathKeys`, `WorkspaceReader` and
`CoverageSource`, beside the five seams `App` runs on. A new technology need is
a trait there, an in-memory fake in `core/tests/support/` and an adapter in
`cli/src/system/`, composed in `EntryPoint::run_from_args`. Paths in the core
are `SourcePath`s, `/` their only separator, which cli's `PortablePath`
produces at the edge; what reaches cargo -- manifest path, features, a
coverage file -- stays in cli's `CargoConfig` as `PathBuf`s.

`.cargo/mutants.toml` scopes cargo-mutants, and sentinel4rust, to `core/src`,
each mutant tested by core's own tests only.

A release publishes both crates, `crap4rust-core` first: `cargo publish
--workspace` orders them and skips the members marked `publish = false`.

## Orthogonality, trait surface and cognitive complexity

**When changing productive code, always maximize orthogonality and testable surface through traits, and minimize cognitive complexity.**

Specifically:
- prefer extracting behavior behind traits so individual pieces can be tested and swapped independently
- prefer small, focused methods with a single responsibility over large methods with many branches
- prefer named structs with methods over free functions operating on external state
- when `crap4rust` or a reviewer flags a function as too complex, reduce it by extracting internal structs with methods and adding integration coverage — not by extracting standalone helper functions
- never increase cognitive complexity to pass a test; find the root cause and fix it there
- when introducing a new protocol dependency seam, place the contract in `traits/`, place the protocol-facing state/data model parallel to the protocol, and place the concrete implementation in its own dedicated implementation area
- make constructors depend on traits, not directly on concrete implementations
- ALL dependencies are injected through the SINGLE constructor and stored in the struct
- apply the same split recursively to nested dependencies: trait first, state/data model second, concrete implementation third

## User coding standards

- one object per file; records where record-home puts them
- no unnecessary comments in code
- unit tests are not allowed. Only integration tests are
- consolidate scattered functions inside structs as appropriate
- no `&mut` input parameters; prefer return values
- only use `pub mod` in `mod.rs` and `lib.rs`
- split test files so there is one test file per source file, named `<source file name>_tests.rs`
- in `all_tests.rs`, reference test files one by one without `#[path = ...]`
- apply AAA (`Arrange`, `Act`, `Assert`) structure to tests with blank-line separation between the three sections
- use `// Arrange & Act` if there is no separate `Arrange`
- use `// Act & Assert` if there is no separate `Act`
- add the repository copyright and license header to every Rust source file
- tests should be named as follows `<method under test>_<test description>_<result>`
- do not use fully qualified paths; use `use` imports instead
