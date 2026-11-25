# Open Points

## TestModuleRegistry does not chase transitive test-module nesting

`TestModuleRegistry` (`ADR-CrossFileTestModuleExclusion.md`) resolves a
file-based `#[cfg(test)] mod name;` declaration to the file it gates, but
only one level: a further `mod nested;` declared *inside* that
already-excluded file, without repeating `#[cfg(test)]` on its own
declaration (which it does not need to, since it already inherits the gate
from its parent), is not itself resolved, so `nested`'s file stays
conservatively included. This mirrors the same non-recursive scope
boundary `grip`'s own `MethodPurityRegistry` deliberately accepted for
nested custom-accessor trust. Not started; the dominant real-world shape
(one gated `mod` statement, one sibling file) is already covered.

## Coverage region "kind" is not distinguished

`coverage.rs::load_coverage_records` treats every region in
`cargo-llvm-cov`'s JSON export identically when computing
`covered_regions`/`total_regions` — it does not read each region's `kind`
field (index 7 of the 8-element region array; `cargo-llvm-cov`'s own format
distinguishes Code/Expansion/Skipped/Gap/Branch regions). Every fixture and
real-world coverage file exercised so far has `kind == 0` (Code) throughout,
so this has not been observed to produce an incorrect ratio, and changing it
without a concrete, reproduced discrepancy would be speculative. Not
started; flagged here rather than silently assumed correct.

## `is_selected_relative_file` repeats `is_selected_source_file`

`FileWalker::is_selected` asks `is_selected_relative_file` -- the raw path's
`examples/`, `benches/`, `tests/` and `build.rs` at the package root -- and
then `is_selected_source_file`, the same questions asked of the `PathKeys`
keys, which answer every case the raw path does and the spellings it misses
(`Tests/` on a file system that ignores case). The build script is left out
by `is_build_script`, against the `custom-build` target cargo names. So the
raw check decides nothing of its own but one case it gets wrong: a `build.rs`
at the package root that is not the build script (`build = false`, or
`build = "other.rs"`), which it still leaves out. Its retirement -- the
function, its `pub` surface and its six tests in `file_walker_tests.rs` --
waits for the owner's word, since it removes tests. Found by sentinel4rust's
round on f6f0ab7 (2026-11-25).

## One shared "is this test code?" check

Deferred by the owner 2026-11-25. `ImplCollector::is_test_attr` takes any
attribute to mark test code when its path, or a path one or two levels
inside its arguments, ends in `test`: so `#[cfg(not(test))]` -- code that
exists only *outside* tests -- and `#[cfg_attr(test, derive(..))]` -- code
that exists in every build -- are both left out of discovery as test code,
and their functions are never scored. sentinel4rust's round on f6f0ab7
(2026-11-25) reported it.

The same defect exists in crap4rust, grip4rust, braintax4rust, iceberg4rust
and slotgate, so it is not fixed here alone: the check is to be written once
with shared test vectors and adopted by every tool. The vectors:

- an attribute whose path ends in `test` (`#[test]`, `#[tokio::test]`) marks
  test code;
- `cfg(P)` is test-only when `P` is test-only: `test` is; `all(..)` is when
  any of its parts is; `any(..)` is when every one of its parts is; `not(..)`
  never is;
- `cfg_attr(..)` never makes an item test-only.
