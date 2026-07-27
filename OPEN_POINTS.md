# Open Points

## grip4rust self-analysis score remains below the confirmed threshold

`scripts/run_stage_2.ps1`'s `Invoke-GripGate` gates this repository's own
`cargo grip4rust` score at `70`. That threshold was a deliberate lowering
from the original `80` (`98faac7`) and has been confirmed as the accepted
value — the gate is not being raised back to `80`, nor dropped to
visibility-only. The score was last measured at `56` before the earlier
cleanup session, `64` after it, and is `63` now (post file_walker/
item_visitor/impl_collector restructuring and the `pub(crate)`→`pub`
visibility sweep), so the gate remains red, currently by 7 points.

The gap is structural, not a defect: `App::new()`, the composition root
that wires the five concrete trait implementations together (the one place
this repository's own `CLAUDE.md` permits that coupling), constructs five
concrete types by name in a single function. `grip`'s hidden-dependency
heuristic scores any function with that shape at zero contribution
regardless of purity, and `grip`'s own `App::new()` has the identical shape
for the identical reason — `grip`'s own README publishes its self-analysis
score as `59`, and `grip`'s own `scripts/run_stage_2.ps1` does not gate on
its own grip score at all (it only confirms `cargo run -- --json` exits
successfully). Splitting `App::new()`'s five constructor calls into
separate single-purpose factory functions would raise the measured score
further but was rejected as gaming the heuristic rather than improving the
design — see `CLAUDE.md`'s own "never increase cognitive complexity to pass
a test; find the root cause and fix it there," which applies here by the
same reasoning even though the metric in question is `grip`, not
`crap4rust` itself.

No further action is planned to close this specific gap: the threshold
decision is settled at `70`, and the remaining distance is expected to
narrow incidentally as `App::new()`'s shape changes for unrelated reasons,
not to be chased directly by gaming the heuristic.

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
