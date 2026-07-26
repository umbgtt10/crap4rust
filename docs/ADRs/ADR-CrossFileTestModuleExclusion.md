# ADR-CrossFileTestModuleExclusion

- **Status:** Accepted
- **Date:** 2026-07-26

## Context

`FileWalker` discovers every `.rs` file under a package's source roots and,
for each one it keeps, parses it independently and walks its items looking
for functions to score. Before this decision, that independence was total:
a file's own top-level items were the only thing `FileWalker` ever looked
at when deciding whether a given function belonged to shipped code or to
tests.

`#[cfg(test)] mod tests { ... }` — content and gate in the same file — was
already handled correctly: `is_test_attrs` checks the `mod` item's own
attributes, and a gated inline module's contents are skipped during the
walk. `#[cfg(test)] mod tests;` — a *file-based* declaration, where the gate
sits on the `mod` statement in the parent file and the actual test code
lives in a sibling `tests.rs` (or `tests/mod.rs`) — was not: `FileWalker`
finds and parses `tests.rs` as an independent file, with no way to see that
some *other* file's `mod` statement gates it behind `#[cfg(test)]`, since
that attribute is never repeated inside the child file itself.

Confirmed empirically before this decision was made: a probe crate with
`src/lib.rs` containing `#[cfg(test)] mod tests;` and a sibling
`src/tests.rs` defining `test_only_helper` reported `total_functions=2`,
counting the test-only helper as production code identical to a real
shipped function — the exact same class of miscount `CoverageIndex`'s
zero-duplicate handling (`ADR-SymmetricDuplicateCoverageHandling.md`) fixes
for the coverage side of CRAP, now on the complexity/discovery side.

## Decision

`TestModuleRegistry::build` runs once per source root, over every file's
already-parsed AST, before any file's functions are recorded. It walks each
file's items (recursing into inline `mod outer { ... }` blocks, tracking the
directory each nested module resolves to) looking for file-based `mod name;`
declarations that are themselves `#[cfg(test)]`-gated, and resolves each one
to the file it would compile — first `<dir>/name.rs`, falling back to
`<dir>/name/mod.rs` — checking existence on disk. `FileWalker` then skips
any parsed file whose path the registry resolved to, before that file's own
items are ever visited for function discovery.

This is one project-wide pass over already-in-memory parsed files, not a
second read-and-reparse: `FileWalker::collect_parsed_files` already reads
and parses every candidate file before scoring any of them (needed so a
later file's exclusion doesn't depend on file-system iteration order), and
`TestModuleRegistry::build` consumes that same `Vec<(PathBuf, syn::File)>`.

## Forcing constraints / Evidence

`tests/fixtures/file_based_test_module_fixture` is the fixture this decision
exists to fix: `src/lib.rs` declares `#[cfg(test)] mod tests;`, `src/tests.rs`
defines `test_only_helper`, and
`tests/fixtures/fixture_tests.rs::cfg_test_file_based_mod_declaration_is_excluded_from_discovery`
pins `summary: total_functions=1` end-to-end through the compiled binary —
this test fails without the registry (the probe crate above reproduces the
same miscount) and passes with it.

## Rejected alternatives

**Recognize `tests.rs`/`<name>/mod.rs` by filename convention alone, without
resolving an actual `#[cfg(test)]`-gated `mod` declaration pointing at it.**
Rejected: a file literally named `tests.rs` containing genuinely-shipped
code (an unusual but legal module name, no different from any other
identifier) would be silently excluded from every report with no way to
recover it short of renaming the file. Requiring a real gated `mod`
declaration pointing at it keeps exclusion tied to what the Rust compiler
itself would exclude from a non-test build, not to a naming guess.

**Chase the exclusion transitively** (a `mod nested;` declared, without its
own `#[cfg(test)]`, inside an already-excluded `tests.rs`, since it inherits
the gate from its already-excluded parent). Rejected for this pass, for the
same reason `grip`'s own `MethodPurityRegistry`
(`ADR-TwoPassProjectWideRegistries.md`) rejected recursive nested-accessor
trust: a single-level, non-recursive resolution already covers the dominant
real-world shape (one gated `mod` statement, one sibling file) without a
fixpoint or topological-sort build. A `mod` statement nested two or more
files deep inside an already-excluded test module is a narrow enough case
that it is left conservatively *included* rather than chased — not silently
wrong, just not yet proven, the same posture grip's own registries take.

## Consequences

`FileWalker::process_source_root` now builds one additional registry per
source root — a single linear pass over already-parsed files, far cheaper
than the discovery walk itself — before its existing per-file loop. Both
`FileWalker`'s AST walk (`visit_items`/`visit_item`/`visit_module`) and
`ImplCollector::collect` return `Vec<SourceFunction>` directly rather than
writing into a shared `&mut Vec<SourceFunction>` accumulator, a structural
side effect of restructuring `FileWalker` into a collect-then-visit
two-phase pipeline for this decision — both now satisfy this repository's
own "no `&mut` input parameters" rule as well.

## Enforcement

`tests/test_module_registry_tests.rs` exercises `TestModuleRegistry::build`
directly: sibling-file resolution, `mod.rs`-style directory resolution, a
non-`#[cfg(test)]` `mod` declaration correctly left unresolved, an
inline-nested test module resolving relative to its enclosing module's own
directory, and an unresolvable target leaving unrelated paths untouched.
`tests/fixtures/fixture_tests.rs::cfg_test_file_based_mod_declaration_is_excluded_from_discovery`
carries the end-to-end proof through the compiled binary.

## Related

- `grip`'s `ADR-TwoPassProjectWideRegistries.md` — the same shape (a
  project-wide pre-pass registry, built once before per-file scoring,
  single-level and non-recursive by deliberate scope choice) applied to a
  different gap in a sibling tool.
- `docs/ARCHITECTURE.md` — where `TestModuleRegistry` sits in
  `FileWalker`'s pipeline.
