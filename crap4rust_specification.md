# crap4rust
## Change Risk Anti-Patterns for Rust

**Full Feature Specification — Version 1.0 — March 2026**
**Umberto Gotti**

---

> crap4rust computes the CRAP (Change Risk Anti-Patterns) score for every Rust function by combining cyclomatic or cognitive complexity with test coverage data. Functions with high complexity and low coverage score high — flagging the code most likely to harbour defects and resist safe maintenance. The default crappiness threshold is 30. A project is considered crappy if more than 5% of its functions exceed this threshold.

---

## Table of Contents

1. [Overview](#1-overview)
2. [Core Metric Engine](#2-core-metric-engine)
3. [Command Line Interface](#3-command-line-interface)
4. [Output Formats](#4-output-formats)
5. [Configuration File](#5-configuration-file)
6. [CI Integration](#6-ci-integration)
7. [Rust-Specific Considerations](#7-rust-specific-considerations)
8. [Library API](#8-library-api)
9. [Workspace Support](#9-workspace-support)
10. [Versioning and Stability](#10-versioning-and-stability)
11. [Implementation Roadmap](#11-implementation-roadmap)
- [Appendix A: Complete CLI Reference](#appendix-a-complete-cli-reference)
- [Appendix B: Formula Reference](#appendix-b-formula-reference)

---

## 1. Overview

crap4rust is a Rust-native implementation of the CRAP metric — Change Risk Anti-Patterns — originally defined by Alberto Savoia and Bob Evans for Java (crap4j, 2007). The metric combines complexity and test coverage to identify functions that are simultaneously hard to understand and poorly tested: the combination most strongly associated with defect introduction during maintenance.

The CRAP formula for a function `m`:

```
CRAP(m) = comp(m)² × (1 − cov(m))³ + comp(m)
```

Where `comp(m)` is cyclomatic or cognitive complexity and `cov(m)` is test coverage as a value between 0.0 and 1.0. A score above 30 means the function should be tested more thoroughly or refactored.

### 1.1 Design Goals

- First-class Rust support — understands idiomatic Rust, especially `match` expressions
- Cargo integration — invoked as `cargo crap` with zero friction
- CI-ready — structured output formats, configurable exit codes, baseline comparison
- Library API — embeddable in other tools, not just a CLI
- Honest metrics — cognitive complexity preferred over cyclomatic for Rust codebases
- No false positives from macros, generated code, or trivially simple functions

### 1.2 Relationship to Existing Tools

| Tool | Purpose | Relationship |
|---|---|---|
| rust-code-analysis (Mozilla) | Cyclomatic & cognitive complexity | Complexity input |
| cargo-llvm-cov | LLVM-based coverage data | Coverage input (primary) |
| grcov | lcov / cobertura coverage | Coverage input (alternative) |
| cargo-mutants | Mutation testing | Complementary — orthogonal tool |
| clippy | Linting | Orthogonal |

---

## 2. Core Metric Engine

### 2.1 CRAP Score Computation

The engine computes per-function CRAP scores. Inputs are complexity and coverage, both user-selectable. The formula version is tracked explicitly for reproducible CI baseline comparison.

### 2.2 Complexity / Coverage Threshold Table

Minimum coverage required to stay below the default threshold of 30, by function complexity:

| Cyclomatic Complexity | Coverage Required | Risk Level |
|---|---|---|
| 0–5 | 0% | Clean — no coverage required |
| 6–10 | 42% | Low risk |
| 11–15 | 57% | Moderate risk |
| 16–20 | 71% | Elevated risk |
| 21–25 | 80% | High risk |
| 26–30 | 100% | Critical — full coverage required |
| 31+ | N/A | Refactor regardless of coverage |

### 2.3 CRAP Load

For each crappy function, crap4rust computes the CRAP load — minimum work to bring it below threshold — expressed as two components:

- **Tests to write:** additional tests needed to raise coverage to the level required for the function's complexity
- **Extractions needed:** Extract Function refactorings (halving complexity each time) required if coverage alone is insufficient

Project-wide CRAP load is the sum of all per-function loads. It is a single number representing total testing and refactoring debt.

### 2.4 Complexity Sources

| Flag | Metric | Description |
|---|---|---|
| `--complexity cyclomatic` | Cyclomatic | Classic McCabe complexity — counts branches |
| `--complexity cognitive` | Cognitive | SonarSource cognitive — penalises nesting, rewards `match` |
| `--complexity both` | Both | Compute both, report side by side, use selected for CRAP |

> **Recommendation:** use cognitive complexity. Idiomatic Rust uses `match` extensively, which cyclomatic complexity penalises despite being easy to read. Cognitive complexity treats all `match` arms as a single branching unit.

### 2.5 Coverage Sources

| Source | Format | Flag |
|---|---|---|
| cargo-llvm-cov | JSON | `--coverage-source llvm-cov` |
| grcov | lcov | `--coverage-source lcov` |
| grcov | cobertura XML | `--coverage-source cobertura` |
| Custom | JSON mapping | `--coverage-source custom --coverage <file>` |

Coverage type is selectable via `--coverage-type line|branch|path`. Branch coverage is recommended as it aligns most closely with the original CRAP formula intent.

---

## 3. Command Line Interface

### 3.1 Invocation

```bash
cargo crap                               # analyse current workspace
cargo crap --manifest-path <path>        # specific Cargo.toml
cargo crap --coverage <file>             # pre-computed coverage file
cargo crap --package <name>              # single package in workspace
cargo crap --lib                         # lib targets only
cargo crap --tests                       # include test code in analysis
```

### 3.2 Complexity & Coverage Options

| Flag | Default | Description |
|---|---|---|
| `--complexity cyclomatic\|cognitive\|both` | `cognitive` | Complexity metric |
| `--coverage-type line\|branch\|path` | `branch` | Coverage measurement type |
| `--coverage-source <type>` | `llvm-cov` | Coverage data source |
| `--coverage <file>` | auto | Path to pre-computed coverage file |

### 3.3 Threshold Options

| Flag | Default | Description |
|---|---|---|
| `--threshold <n>` | `30` | Per-function CRAP score threshold |
| `--warn-threshold <n>` | `20` | Warn below this, fail at `--threshold` |
| `--project-threshold <pct>` | `5.0` | Max % crappy functions before project fails |
| `--max-complexity <n>` | off | Flag above this regardless of coverage |
| `--strict` | `false` | Fail on any single crappy function |
| `--warn-only` | `false` | Never exit non-zero — report only |

### 3.4 Filtering Options

| Flag | Default | Description |
|---|---|---|
| `--include-pattern <glob>` | all | Analyse only matching files |
| `--exclude-pattern <glob>` | none | Exclude matching files |
| `--exclude-tests` | `true` | Exclude `#[test]` functions |
| `--include-macros` | `false` | Include macro-generated functions |
| `--inline-closures` | `true` | Include closures as separate units |
| `--min-complexity <n>` | `1` | Only report complexity >= n |
| `--show-clean` | `false` | Also show functions below threshold |

### 3.5 Output Options

| Flag | Default | Description |
|---|---|---|
| `--format table\|json\|xml\|html\|markdown\|sarif` | `table` | Output format |
| `--output <file>` | stdout | Write report to file |
| `--output-dir <dir>` | `target/crap4rust` | Directory for report files |
| `--sort score\|complexity\|coverage\|name\|file` | `score` | Sort column |
| `--sort-order asc\|desc` | `desc` | Sort direction |
| `--top <n>` | all | Show only N crappiest functions |
| `--baseline <file>` | none | Previous report for delta comparison |
| `--fail-on-regression` | `false` | Fail if any score increased vs baseline |

### 3.6 Rust-Specific Options

| Flag | Default | Description |
|---|---|---|
| `--unsafe-multiplier <f>` | `1.0` | Complexity multiplier for functions with `unsafe` blocks |
| `--formula-version <v>` | latest | Pin formula version for reproducible CI |
| `--allow-formula-mismatch` | `false` | Allow baseline comparison across formula versions |
| `--strict-ignore` | `false` | Error on suppressions without reason |

### 3.7 Exit Codes

| Code | Meaning |
|---|---|
| `0` | Clean — no functions exceed threshold, project-wide % within limit |
| `1` | Project exceeds crappiness threshold |
| `2` | Tool error — missing coverage, parse failure, invalid configuration |
| `3` | Regression — a function's score increased vs baseline (`--fail-on-regression`) |

---

## 4. Output Formats

### 4.1 Terminal Table (default)

Colour-coded table. Columns: function name, file, line, complexity, coverage %, CRAP score, CRAP load, verdict. Green = clean, yellow = warn zone, red = crappy. Followed by a project summary line.

### 4.2 JSON

Machine-readable. Full per-function data plus project summary. Schema versioned alongside formula version.

```json
{
  "formula_version": "1.0",
  "threshold": 30,
  "project": {
    "total_functions": 142,
    "crappy_functions": 3,
    "crappy_percent": 2.1,
    "total_crap_load": 14,
    "verdict": "pass"
  },
  "functions": [
    {
      "name": "handle_message",
      "file": "src/protocol/ibft.rs",
      "line": 142,
      "complexity": 18,
      "coverage": 0.94,
      "crap_score": 18.6,
      "crap_load": 0,
      "verdict": "clean"
    }
  ]
}
```

### 4.3 XML (crap4j-compatible)

Follows the crap4j XML schema for compatibility with the Jenkins Crap4J plugin. Includes project-level stats and per-method detail.

### 4.4 HTML

Self-contained HTML report with sortable table, colour coding, per-function detail, and the complexity/coverage threshold reference table. No external dependencies — suitable for CI artifact archiving.

### 4.5 Markdown

Suitable for GitHub Actions PR comments. Summary table of crappy functions and a project verdict. Compact by default.

### 4.6 SARIF

Static Analysis Results Interchange Format — natively understood by GitHub Code Scanning. Each crappy function reported as a finding with severity, location, and remediation guidance. Uploadable via `actions/upload-sarif`.

### 4.7 Per-Function Report Fields

| Field | Description |
|---|---|
| `name` | Fully qualified function name including module path |
| `file` | Source file path relative to workspace root |
| `line` | Line number of function definition |
| `complexity` | Cyclomatic and/or cognitive complexity value |
| `coverage` | Test coverage as percentage |
| `crap_score` | Computed CRAP score |
| `crap_load` | Tests to write + extractions needed |
| `unsafe_blocks` | Number of `unsafe` blocks (flagged separately) |
| `verdict` | `clean` \| `warn` \| `crappy` |
| `delta` | Change in CRAP score vs baseline (if provided) |

---

## 5. Configuration File

All CLI options are expressible in `crap4rust.toml` placed in the project root. CLI flags take precedence.

```toml
[crap4rust]
threshold         = 30
warn_threshold    = 20
project_threshold = 5.0
complexity        = "cognitive"
coverage_type     = "branch"
coverage_source   = "llvm-cov"
format            = "html"
output_dir        = "target/crap4rust"
exclude_tests     = true
include_macros    = false
inline_closures   = true
unsafe_multiplier = 1.5

exclude_patterns  = ["**/generated/**", "**/vendor/**"]

[crap4rust.ignore]
functions = ["my_workspace::protocol::known_complex_fn"]
```

### 5.1 Per-Function Suppression via Doc Comments

Individual functions can be suppressed using a doc comment annotation. A reason is mandatory.

```rust
/// # crap4rust: ignore
/// Reason: all paths validated by TLA+ model checking.
fn handle_message(source: MessageSource, msg: Message, ctx: Context) -> Actions {
    // ...
}
```

> Suppressed functions are still reported but marked as ignored. The ignore reason is included in HTML and JSON reports. `--strict-ignore` causes suppression without a reason to be an error.

### 5.2 Per-Crate Thresholds in Workspaces

Each crate can override defaults in its own `Cargo.toml`:

```toml
[package.metadata.crap4rust]
threshold         = 20    # stricter than workspace default
project_threshold = 2.0
```

---

## 6. CI Integration

### 6.1 GitHub Actions — Minimal

```yaml
- name: Run crap4rust
  run: |
    cargo llvm-cov --json --output-path coverage.json
    cargo crap --coverage coverage.json --format sarif --output crap.sarif

- name: Upload to GitHub Code Scanning
  uses: github/codeql-action/upload-sarif@v3
  with:
    sarif_file: crap.sarif
```

### 6.2 GitHub Actions — With PR Comment and Regression Check

```yaml
- name: Run crap4rust
  run: |
    cargo llvm-cov --json --output-path coverage.json
    cargo crap --coverage coverage.json \
               --format markdown \
               --output crap_report.md \
               --baseline .crap4rust/baseline.json \
               --fail-on-regression

- name: Post CRAP report to PR
  uses: actions/github-script@v7
  with:
    script: |
      const report = require('fs').readFileSync('crap_report.md', 'utf8');
      github.rest.issues.createComment({
        issue_number: context.issue.number,
        owner: context.repo.owner,
        repo: context.repo.repo,
        body: report
      });
```

### 6.3 Baseline Management

- Generate: `cargo crap --format json --output .crap4rust/baseline.json`
- Commit the baseline file to the repository
- On each PR: `cargo crap --baseline .crap4rust/baseline.json --fail-on-regression`
- Update baseline on merge to main

### 6.4 Failure Modes

| Mode | Flag | Behaviour |
|---|---|---|
| Default | (none) | Fail if project-wide crappy % exceeds `--project-threshold` |
| Strict | `--strict` | Fail if any single function exceeds `--threshold` |
| Warn only | `--warn-only` | Never exit non-zero — report only |
| Regression | `--fail-on-regression` | Fail if any score increased vs baseline |

---

## 7. Rust-Specific Considerations

### 7.1 `match` Expressions

Cyclomatic complexity counts every `match` arm as a branch, producing inflated scores for idiomatic Rust. A `match` with 10 arms scores identically to 10 nested `if/else` blocks despite being far easier to read.

Cognitive complexity treats `match` as a single branching construct (+1) regardless of arm count, with an additional nesting penalty only if the `match` is itself nested. This is a more accurate reflection of maintenance risk.

> crap4rust defaults to cognitive complexity. When using cyclomatic, the report flags functions where the cyclomatic/cognitive gap is large — indicating inflation from `match` expressions.

### 7.2 Macros

Macro-generated functions are excluded by default. Macro-generated code typically has high apparent complexity but is not directly maintained. The `--include-macros` flag overrides this.

### 7.3 `unsafe` Blocks

Functions containing `unsafe` blocks are flagged separately in all report formats. An optional `--unsafe-multiplier` applies a complexity multiplier to these functions (default 1.0; 1.5 suggested for safety-critical codebases).

### 7.4 Closures

Inline closures are treated as separate units with their own CRAP scores. `--inline-closures false` treats closures as part of their enclosing function.

### 7.5 Generics and Monomorphisation

Generic functions are measured on the generic definition. Monomorphised instantiations are not counted separately — doing so would inflate function counts and distort project-wide statistics.

### 7.6 `async` Functions

`async` functions are measured at the source level, not on the lowered MIR state machine. This ensures the score reflects what a maintainer reads rather than compiler-generated complexity.

---

## 8. Library API

Beyond the CLI, crap4rust exposes a public Rust library API for embedding CRAP analysis in other tools, custom reporting pipelines, or IDE integrations.

### 8.1 Basic Usage

```rust
use crap4rust::{CrapAnalyser, Config, ComplexitySource, CoverageSource, CoverageType};

let config = Config::builder()
    .threshold(30)
    .complexity(ComplexitySource::Cognitive)
    .coverage_type(CoverageType::Branch)
    .project_threshold(5.0)
    .build()?;

let report = CrapAnalyser::new(config)
    .with_coverage(CoverageSource::LlvmCov("coverage.json"))
    .analyse("src/")?;

for f in report.crappy_functions() {
    println!("{}: score={:.1} load={}", f.name, f.crap_score, f.crap_load);
}
```

### 8.2 Key Traits

| Trait | Description |
|---|---|
| `ComplexityProvider` | Supply custom complexity data per function |
| `CoverageProvider` | Supply custom coverage data per function |
| `ReportFormatter` | Produce custom output formats |
| `CrapFilter` | Apply custom suppression logic |

### 8.3 Workspace Analysis

```rust
let workspace = CrapAnalyser::workspace("Cargo.toml", config)?
    .with_coverage(CoverageSource::LlvmCov("coverage.json"))
    .analyse_all()?;

for (crate_name, report) in workspace.crates() {
    println!("{}: {} crappy", crate_name, report.crappy_functions().count());
}

println!("Total CRAP load: {}", workspace.total_crap_load());
```

---

## 9. Workspace Support

crap4rust is workspace-aware. It analyses all crates by default and produces both per-crate and aggregate reports.

### 9.1 Workspace Commands

- `cargo crap` — analyses all crates
- `cargo crap --package <name>` — single crate
- `cargo crap --package a --package b` — multiple specific crates
- Per-crate thresholds respected from individual `Cargo.toml` files

### 9.2 Aggregate Report Contents

- Total functions across all crates
- Per-crate crappy function count and percentage
- Workspace-wide crappy percentage (used for `--project-threshold` evaluation)
- Workspace-wide total CRAP load
- Per-crate verdict and workspace verdict

### 9.3 Cross-Crate Trend Tracking

With a baseline, delta reporting covers all crates. Functions that moved between clean and crappy are highlighted. Regressions in any crate trigger `--fail-on-regression` regardless of aggregate workspace score.

---

## 10. Versioning and Stability

### 10.1 Semantic Versioning

crap4rust follows semantic versioning. Formula version is tracked independently — a formula change always bumps the formula major version, regardless of tool version.

### 10.2 Formula Versioning

The formula version is embedded in all JSON and XML reports. Comparison across formula versions is disabled by default. `--allow-formula-mismatch` overrides this.

```bash
# Pin to a specific formula version for reproducible CI
cargo crap --formula-version 1.0
```

### 10.3 Report Schema Versioning

JSON and XML report schemas are versioned independently. Schema versions are backward compatible within a major version. The schema version is embedded in the output alongside the formula version.

---

## 11. Implementation Roadmap

| Phase | Scope | Deliverable |
|---|---|---|
| 1 — MVP | CRAP formula, cognitive complexity, cargo-llvm-cov input, terminal table, exit codes | Early validation build proving the approach on a real Rust workspace |
| 2 — Output | JSON, HTML, Markdown formats, `--sort`, `--top`, `--show-clean`, project summary | Useful for local reporting |
| 3 — CI | SARIF output, `--baseline`, `--fail-on-regression`, GitHub Actions examples | CI-ready |
| 4 — Config | `crap4rust.toml`, per-function suppression, `--unsafe-multiplier`, workspace support | Production-ready |
| 5 — Extract | Standalone `crap4rust` crate on crates.io, `cargo-crap` subcommand, library API, full docs | Public crate |
| 6 — Ecosystem | VS Code extension, XML/crap4j-compatible output, SonarQube integration | Ecosystem integration |

> **Phase 1 is the validation gate.** If the scores on a real Rust codebase are meaningful — identifying the genuinely complex and undertested functions — the rest of the roadmap is justified. If not, adjust the formula or inputs before proceeding.

---

## Appendix A: Complete CLI Reference

| Flag | Default | Description |
|---|---|---|
| `--manifest-path <path>` | `Cargo.toml` | Path to workspace or crate manifest |
| `--package <name>` | all | Analyse specific package(s) |
| `--lib` | `false` | Analyse lib targets only |
| `--tests` | `false` | Include test code in analysis |
| `--complexity cyclomatic\|cognitive\|both` | `cognitive` | Complexity metric to use |
| `--coverage-type line\|branch\|path` | `branch` | Coverage measurement type |
| `--coverage-source llvm-cov\|lcov\|cobertura\|custom` | `llvm-cov` | Coverage data source |
| `--coverage <file>` | auto | Path to pre-computed coverage file |
| `--threshold <n>` | `30` | Per-function CRAP threshold |
| `--warn-threshold <n>` | `20` | Warning threshold |
| `--project-threshold <pct>` | `5.0` | Max % crappy functions |
| `--max-complexity <n>` | off | Flag above this regardless of coverage |
| `--strict` | `false` | Fail on any crappy function |
| `--warn-only` | `false` | Never exit non-zero |
| `--include-pattern <glob>` | all | Include only matching files |
| `--exclude-pattern <glob>` | none | Exclude matching files |
| `--exclude-tests` | `true` | Exclude `#[test]` functions |
| `--include-macros` | `false` | Include macro-generated functions |
| `--inline-closures` | `true` | Include closures as units |
| `--min-complexity <n>` | `1` | Only report complexity >= n |
| `--show-clean` | `false` | Show clean functions too |
| `--format table\|json\|xml\|html\|markdown\|sarif` | `table` | Output format |
| `--output <file>` | stdout | Write report to file |
| `--output-dir <dir>` | `target/crap4rust` | Directory for report files |
| `--sort score\|complexity\|coverage\|name\|file` | `score` | Sort column |
| `--sort-order asc\|desc` | `desc` | Sort direction |
| `--top <n>` | all | Show only N crappiest functions |
| `--baseline <file>` | none | Previous report for delta comparison |
| `--fail-on-regression` | `false` | Fail if any score increased vs baseline |
| `--unsafe-multiplier <f>` | `1.0` | Complexity multiplier for `unsafe` blocks |
| `--formula-version <v>` | latest | Pin formula version |
| `--allow-formula-mismatch` | `false` | Allow baseline comparison across versions |
| `--strict-ignore` | `false` | Error on suppression without reason |

---

## Appendix B: Formula Reference

### Standard CRAP Formula

```
CRAP(m) = comp(m)² × (1 − cov(m))³ + comp(m)
```

Where `comp(m)` = complexity of function m (cyclomatic or cognitive), and `cov(m)` = test coverage as a value between 0.0 and 1.0.

### CRAP Load — Tests Needed

```
tests_needed(m) = ceil(comp(m) × (1 − cov(m)))
```

### CRAP Load — Extractions Needed

```
extractions(m) = max(0, ceil(log₂(comp(m) / threshold)))
```

### Project-Wide CRAP Load

```
total_crap_load = Σ (tests_needed(m) + extractions(m))  for all crappy m
```

### Crappiness Percentage

```
crappy_pct = (crappy_function_count / total_function_count) × 100
```

A project is considered crappy if `crappy_pct` exceeds `--project-threshold` (default 5.0%).

---

*© 2026 Umberto Gotti — Apache License 2.0*
