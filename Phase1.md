# crap4rust Phase 1

## Goal

Deliver the smallest useful first release that proves the CRAP metric is valuable on real Rust code.

Phase 1 should answer one question well: can crap4rust reliably identify Rust functions that are both complex and under-tested?

## In Scope

### 1. Core Analysis Engine

- Compute per-function CRAP scores using the formula from the specification.
- Use cognitive complexity only.
- Use function-level coverage as a value from 0.0 to 1.0.
- Compute project-wide crappiness percentage.
- Compute project verdict from the default thresholds.

### 2. Coverage Input

- Support one coverage source only: cargo-llvm-cov JSON.
- Require coverage to be supplied explicitly with a coverage file path.
- Fail clearly when coverage data is missing, malformed, or cannot be mapped to functions.

### 3. Rust Code Analysis

- Analyse named Rust functions.
- Report fully qualified function name when available.
- Report source file and definition line.
- Exclude test functions by default.
- Ignore macro-generated functions for Phase 1.
- Treat async functions at source level.
- Measure generic functions once at the generic definition level.

### 4. CLI Surface

- Support analysis of the current crate.
- Support analysis via manifest path.
- Support package selection when the target manifest is a workspace.
- Support the following flags only:
  - --coverage <file>
  - --manifest-path <path>
  - --package <name>
  - --threshold <n>
  - --project-threshold <pct>
  - --strict
  - --warn-only

### 5. Output

- Provide one default terminal table output.
- Show at least these columns:
  - function name
  - file
  - line
  - complexity
  - coverage
  - CRAP score
  - verdict
- Include a project summary line with:
  - total functions
  - crappy functions
  - crappy percentage
  - threshold values
  - final verdict

### 6. Exit Codes

- 0: clean or warn-only mode
- 1: project fails threshold rules
- 2: tool or input error

## Out of Scope

- Cyclomatic complexity
- Dual complexity reporting
- lcov support
- Cobertura support
- Custom coverage adapters
- HTML output
- JSON output
- XML output
- Markdown output
- SARIF output
- Baseline comparison
- Regression detection
- crap4rust.toml
- Per-function suppressions
- Unsafe multiplier
- Closure-level reporting
- Macro inclusion toggles
- Sorting and top-N options
- Library API
- Cargo subcommand packaging polish
- Full workspace aggregate reporting beyond selected package analysis

## Success Criteria

Phase 1 is complete when the tool can:

1. Read a Cargo manifest and resolve the crate or selected package to analyse.
2. Parse Rust source well enough to enumerate functions with names and locations.
3. Load cargo-llvm-cov JSON and map coverage to functions.
4. Compute cognitive complexity and CRAP scores for those functions.
5. Print a readable terminal report that highlights functions above threshold.
6. Return stable exit codes suitable for local use and CI.
7. Produce results on a real Rust codebase that are defensible enough to guide testing or refactoring.

## Recommended Implementation Order

1. Cargo target discovery and CLI argument parsing.
2. Rust function discovery and source locations.
3. cargo-llvm-cov JSON ingestion.
4. Cognitive complexity calculation.
5. CRAP formula and project summary.
6. Terminal table formatting and exit-code logic.

## Notes

This phase is a validation release, not a completeness release.

If the ranking of risky functions is not credible on a real codebase, the tool should be adjusted before adding more formats, configuration layers, or ecosystem integrations.
