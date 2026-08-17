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

`powershell -File scripts\run_stage_1.ps1`
`powershell -File scripts\run_stage_2.ps1`

If either gate is not green, the work is not complete.

Stage 1 is formatting, clippy and tests. Stage 2 runs this tool against itself
and then `cargo twin4rust`, which requires every source file under `src/` to
have a mirrored `tests/<name>_tests.rs` beside it. Both must be installed:

`cargo install cargo-crap4rust`
`cargo install cargo-twin4rust`

The fixture crates under `tests/fixtures/` are analysis inputs, not sources of
this crate. The CRAP gate excludes them explicitly with `--exclude-path`;
`twin4rust` never sees them, because it resolves source roots from this
package's own cargo targets.

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

- one struct per file
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
