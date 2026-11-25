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
