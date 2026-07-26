# Architecture Decision Records

Each ADR documents one load-bearing decision behind `cargo-crap4rust` —
succinct, self-contained, citable on its own. Unlike the larger `etheram`
ecosystem repos, these are not priority-tiered; `crap4rust` is a
single-crate CLI tool with a small enough decision surface that a flat list
is sufficient.

## Index

| ADR | Decision |
|---|---|
| [ADR-AstOnlyNoTypeResolution](ADR-AstOnlyNoTypeResolution.md) | `crap4rust` analyzes via `syn` AST parsing only, never type resolution — cognitive complexity and function/test discovery are structural, so the tool runs on any syntactically valid source whether or not it compiles. |
| [ADR-DynDispatchAppOverGenerics](ADR-DynDispatchAppOverGenerics.md) | `App` holds `Box<dyn Trait>` fields for its five dependencies (`PackageResolver`, `FunctionDiscovery`, `CoverageProvider`, `Scorer`, `Reporter`) rather than generic type parameters — unlike `grip`/`braintax`, this repository's `App` never had a generic-parameter phase to convert away from. |
| [ADR-CrossFileTestModuleExclusion](ADR-CrossFileTestModuleExclusion.md) | `TestModuleRegistry` resolves file-based `#[cfg(test)] mod name;` declarations to the file they gate, in a project-wide pre-pass before per-file function discovery, so a test-only module split into its own file is excluded the same way an inline `#[cfg(test)] mod name { ... }` block already was. |
| [ADR-SymmetricDuplicateCoverageHandling](ADR-SymmetricDuplicateCoverageHandling.md) | `CoverageIndex` discards a zero-count duplicate coverage record in favor of a non-zero one for the same `(path, line)` key regardless of which one arrives first — the original fix for this only handled one arrival order. |

## Template

```markdown
# ADR-<Name>

- **Status:** Accepted | Proposed | Superseded by <ADR>
- **Date:** YYYY-MM-DD

## Context
The forces and tension this resolves.

## Decision
The choice, in one quotable sentence.

## Forcing constraints / Evidence
Why this was forced, not freely chosen — the real evidence. `N/A` if none.

## Rejected alternatives
What we did not do, and why.

## Consequences
What it commits us to; what it costs; obligations pushed onto consumers.

## Enforcement
The specific test, gate, or structural mechanism that keeps it true.
`N/A` if purely structural.

## Related
Links to other ADRs (this repo, `grip`, or `braintax`) and architecture docs.
```

Fields that do not apply are marked `N/A` rather than padded. Each ADR is a
snapshot of the decision as it stands today, not a changelog — state the
current shape as fact, don't narrate what an earlier version of this
document used to say.
