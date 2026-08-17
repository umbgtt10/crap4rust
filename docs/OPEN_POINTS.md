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
