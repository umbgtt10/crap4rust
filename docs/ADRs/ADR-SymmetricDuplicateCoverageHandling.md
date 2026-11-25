# ADR-SymmetricDuplicateCoverageHandling

- **Status:** Accepted
- **Date:** 2026-07-26

## Context

`cargo-llvm-cov`'s JSON export can emit more than one function record for
the same `(file, start_line)` — one instantiation compiled with the profile
data that actually ran, and, for a function monomorphized or inlined across
multiple object files, one or more "ghost" instantiations whose own profile
counters never incremented. `CoverageIndex::from_records` is where duplicate
records for the same key get merged into the one `CoverageRecord`
`match_function` will look up.

Before this decision, that merge was order-dependent. The rule was: sum a
new record into the existing one for that key *unless* the new record is an
all-zero ghost arriving after a real one — in which case discard the ghost.
Read the other way around, when the ghost happened to arrive *first*
(inserted before the real record was seen), the real record that followed
it did not replace the ghost — it was summed with it, exactly reproducing
the halved-ratio bug this same rule was written to fix (a fully-covered
function with one real region and one zero-count ghost region reported 50%
coverage, not 100%), just for the arrival order the original fix's own test
didn't happen to cover.

This was not a hypothetical gap: `tests/coverage_index_tests.rs` already
had a test (added when the original fix landed) pinning the *correct*
order — real record, then ghost, discarded — while a separate, older
CLI-level fixture test (`tests/fixtures/fixture_tests.rs`,
`duplicate_coverage_entries_are_aggregated`, present since this repository's
first commit) exercised the *other* order and asserted the halved,
50%-coverage result as correct — two tests, for what is conceptually one
contract, asserting two different numbers for it, each accidentally correct
only for the specific order its own scenario happened to construct.

## Decision

`CoverageIndex::from_records` merges each incoming record against whatever
is already stored for its key via a dedicated `merge_duplicate(existing,
incoming)` step, applied the same way regardless of which one is "already
there" and which one is "arriving": if exactly one side has zero covered
regions and the other has more than zero, the non-zero side wins outright
(the zero side is the ghost, discarded); otherwise — both non-zero, or both
zero — the two are summed, the same as two genuinely distinct real regions
that happen to start on the same line.

## Forcing constraints / Evidence

`tests/coverage_index_tests.rs::from_records_discards_zero_duplicate_when_nonzero_arrives_second`
is the regression test for the previously-uncovered order: real-record and
ghost supplied zero-then-real, asserting the merged record still has
`covered_regions == 1, total_regions == 1` (not `2`, which is what the
order-dependent rule produced).
`from_records_skips_zero_duplicate_when_nonzero_arrives_first` pins the
already-correct order the same way, so both directions are now pinned by
name, not by accident of which scenario a test happened to construct.
`tests/fixtures/fixture_tests.rs`'s two
`duplicate_coverage_entries_discard_zero_ghost_*` tests reconstruct both
arrival orders through the compiled binary and assert the corrected
`100.0%`, replacing the single stale assertion of `50.0%`.

## Rejected alternatives

**Special-case only the specific arrival order the original fix's test
happened to cover, and leave the other order as a documented limitation.**
Rejected outright: the whole reason this record shape exists is that
`cargo-llvm-cov`'s own ordering of duplicate function records within one
export is an implementation detail this tool has no control over and no
visibility into — a "documented limitation" here would mean coverage
accuracy silently depends on `cargo-llvm-cov`'s internal object-file
processing order, which is exactly the kind of instability the original
fix was written to eliminate, not to make order-dependent in a different
way.

**Treat any two records at the same key as always summed, reverting to the
pre-fix behavior.** Rejected: this is the original bug
(`CHANGELOG.md`'s `[0.6.2]` entry) the order-dependent rule was already
trying to fix — reverting to it would silently halve the reported coverage
of any function whose compiled form happens to produce a zero-count ghost
record, on both arrival orders instead of just one.

## Consequences

`CoverageIndex::from_records` no longer depends on the iteration order of
the `Vec<CoverageRecord>` it is given, which is itself sourced from
`Export.data`'s chunk order in `coverage.rs::load_coverage_records` — an
order this tool receives from `cargo-llvm-cov`'s JSON output as-is, never
sorts, and has no reason to assume is stable across runs or toolchain
versions. Two genuinely distinct non-zero regions that happen to share a
`(path, line)` key still sum, unchanged from the pre-existing (and separately
tested, `from_records_aggregates_duplicate_entries`) behavior for that case.

## Enforcement

`tests/coverage_index_tests.rs` pins both arrival orders by name, as does
`tests/fixtures/fixture_tests.rs` at the CLI level through the
`aggregation_fixture` crate.

## Amendment (2026-11-25)

The sum is gone. `merge_duplicate` keeps one of the two records: the one
with the most covered regions, and of two with as many, the one with fewer
regions in all -- the higher ratio of the two. `core::cmp::max_by_key` picks
it on `(covered_regions, Reverse(total_regions))`, so the choice still does
not depend on arrival order, which is what this decision exists for.

Summing was not monotone: one more covered region in a duplicate could
lower the merged ratio. A `3/5` record beside a `0/5` ghost merged to `3/5`
(0.6), the ghost discarded; beside a `1/5` duplicate the two were summed, to
`4/10` (0.4). sentinel4rust's round on f6f0ab7 (2026-11-25) found it. Keeping
the better-covered record makes a ghost a case of the rule rather than an
exception to it, and the merge monotone in either record's covered regions
when both count the same regions -- the same function compiled twice, which
is what a duplicate is.

What changes. Two non-zero duplicates no longer sum: `3/5` and `2/5` keep
`3/5`, where they made `5/10`. Two zero duplicates keep the smaller: `0/3`
and `0/2` keep `0/2`, where they made `0/5`. A function whose duplicates
both ran, or both never ran, counts its regions once in `match_function`'s
sum over its span, not twice. Two genuinely distinct functions starting on
one line, which the sum added up, now count as the better covered of the
two: the index keys a record by line, with no column to tell them apart.

The Decision, Consequences and Enforcement above stand except where they say
the two are summed. In `core/tests/coverage_io/coverage_index_tests.rs`,
`from_records_aggregates_duplicate_entries` is now
`from_records_keeps_the_duplicate_with_more_covered_regions` (0.6, was
0.5), and `from_records_sums_duplicates_that_are_both_zero` is now
`from_records_keeps_the_smaller_of_two_duplicates_that_are_both_zero` (`0/2`,
was `0/5`); `from_records_keeps_the_duplicate_with_more_covered_regions_when_it_arrives_second`
and the two `from_records_of_equally_covered_duplicates_keeps_the_smaller*`
tests pin the rest, each in both arrival orders.

## Related

- `CHANGELOG.md`'s `[0.6.1]`/`[0.6.2]` entries — the two earlier, narrower
  fixes (aggregating all regions within a function's span; discarding a
  zero-count duplicate for one arrival order) this decision completes.
- `docs/ARCHITECTURE.md` — where `CoverageIndex` sits between coverage
  loading and per-function scoring.
