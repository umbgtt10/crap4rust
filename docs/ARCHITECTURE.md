# Architecture

How a `cargo crap4rust` invocation actually flows through the code. This is
a map of what exists today, not a decision record — see `docs/ADRs/` for the
"why" behind the shapes described here, and `docs/FORMULA.md` for how the
scores themselves are computed.

---

## Two crates

The workspace's two published crates split the tool along one line:

| crate | package | library | holds |
|---|---|---|---|
| `core/` | `crap4rust-core` | `crap4rust` | the analysis: `#![no_std]`, `#![forbid(unsafe_code)]`, depending on `proc-macro2` and `syn` only. Function discovery, cognitive complexity, the coverage arithmetic, the CRAP formula, scoring, the verdict and the human report |
| `cli/` | `cargo-crap4rust` | `crap4rust_cli` | the binary, `Args` (clap), the composition root `EntryPoint`, and every adapter: cargo metadata, cargo llvm-cov, the file system, the JSON report |

`cli` depends on `core`, never the reverse. Core touches no technology: what it
needs from the machine it asks of a trait in `core/src/traits/`, which `cli`
implements in `cli/src/system/` and core's own tests fake in memory under
`core/tests/support/`. See `docs/ADRs/ADR-TechnologyFreeCore.md`.

## Pipeline

```
cli/src/main.rs
  → EntryPoint::run(argv)               — strips cargo's repeated subcommand name
    → EntryPoint::run_from_args          — parses Args, builds the adapters and App
      → App
          resolver:          Box<dyn PackageResolver>    — resolves Cargo packages to score
          discovery:         Box<dyn FunctionDiscovery>   — finds functions inside each package
          coverage_provider: Box<dyn CoverageProvider>    — provides coverage data for them
          scorer:            Box<dyn Scorer>              — turns both into scored reports
          reporter:          Box<dyn Reporter>             — renders the final report
        → App::run() -> Result<RunOutcome, RunFailure>
            1. resolver.resolve(&config) -> Vec<PackageContext>
            2. coverage_provider.provide(&packages) -> Vec<CoverageRecord>
               (cargo llvm-cov generates an export first if --coverage was
               not given; either way, records are not checked for emptiness
               yet)
            3. discovery.discover(&package) -> Vec<SourceFunction>, once per
               package, accumulated; fail if the total is empty
            4. fail if the coverage records collected in (2) are empty
            5. CoverageIndex::from_records(records); fail if not a single
               discovered function matches any record by file + line span
            6. scorer.score_functions(functions, &coverage_index, &config)
               -> Vec<FunctionReport>, sorted by crap_score descending
            7. scorer.project_metrics(&reports, &config) -> ProjectMetrics
            8. reporter.render(&ProjectReport { .. }, &config) -> String
            9. Ok(RunOutcome { output, exit_code: report.exit_code(&config) })
      → print!(outcome.output); ExitCode::from(outcome.exit_code)
```

The failure points (empty functions, empty coverage, no matches, plus
whatever `resolve`/`provide`/`discover` themselves fail with) all surface as a
`RunFailure` from `App::run()`, each variant saying what went wrong and keeping
its cause. `EntryPoint::run` prints the whole chain as `error: {message}` with
exit code `2` -- distinct from a *successful* run that nonetheless found
crappy functions and exits `1`. Nothing in core prints.

`EntryPoint::run_from_args` (`cli/src/invocation/entry_point.rs`) is the only
place that picks concrete types: the adapters -- `CargoWorkspaceReader`,
`WalkdirSourceTree`, `CanonicalPathKeys`, `LlvmCovSource` -- and, by
`--output-format`, `HumanReporter` or `JsonReporter`; it hands core's
`CargoPackageResolver`, `SourceFunctionDiscovery`, `ExportCoverageProvider` and
`DefaultScorer`, built on those adapters, to `App::new`, `App`'s only
constructor. See `docs/ADRs/ADR-DynDispatchAppOverGenerics.md` for why these
are `Box<dyn Trait>` fields rather than generic parameters.

## The five injected seams (`core/src/traits/`)

| Trait | Implementation | Responsibility |
|---|---|---|
| `PackageResolver` | `CargoPackageResolver` (core, `cargo_package_resolver.rs`) | Apply cargo's rules to the workspace a `WorkspaceReader` describes -- the packages asked for, in that order; else the root package of a workspace of one; else every member -- and build each one's `PackageContext`: manifest dir, workspace root, and resolved source roots. |
| `FunctionDiscovery` | `SourceFunctionDiscovery` (core, `source_function_discovery.rs`) | Walk a single package's source roots through a `SourceTree` and return every production `SourceFunction` found, complexity already computed. |
| `CoverageProvider` | `ExportCoverageProvider` (core, `export_coverage_provider.rs`) | Take a `CoverageSource`'s export and turn each function in it into a `CoverageRecord`: its file's key, the line its first region starts on, and how many of its regions ran. |
| `Scorer` | `DefaultScorer` (core, `default_scorer.rs`) | Turn discovered functions plus a `CoverageIndex` into scored, sorted `FunctionReport`s, and fold those into project-level `ProjectMetrics`. |
| `Reporter` | `HumanReporter` (core, `human_reporter.rs`), `JsonReporter` (cli, `json_reporter.rs`) | Render a `ProjectReport` as the human-readable table or the JSON document, as a string. |

`core/tests/invocation/app_tests.rs` is the seam these five traits exist for:
it builds `App` via `App::new()` with fakes standing in for whichever one
dependency a given test wants to fail or return edge-case data from.

## The four ports (`core/src/traits/`)

What core needs from the machine, each implemented by `cli` and faked in
memory by core's tests:

| Port | cli adapter (`cli/src/system/`) | Answers |
|---|---|---|
| `SourceTree` | `WalkdirSourceTree` | the `.rs` files under a root, and a file's text |
| `PathKeys` | `CanonicalPathKeys` | the key a source file and a coverage record are matched by: the canonical path, with `/` separators, lower case on Windows |
| `WorkspaceReader` | `CargoWorkspaceReader` | the workspace's members, their targets and its root package, from `cargo metadata --no-deps` |
| `CoverageSource` | `LlvmCovSource` | cargo-llvm-cov's export for the selected packages: the `--coverage` file, or one generated under `target/crap4rust/` through `LlvmCovBuilder` -- whose failures `LlvmCovFailure` describes, telling an absent `cargo-llvm-cov` from a coverage run that genuinely failed -- read and parsed from JSON |

The line runs between the coverage arithmetic and its technology: cli runs
the process, places and reads the file and parses the JSON; core does
everything from the parsed export on. Paths cross it as `SourcePath`s, text
with `/` as the only separator, which `PortablePath` produces from the
platform's paths at the edge.

## Per-package function discovery: `FileWalker` and `TestModuleRegistry`

`SourceFunctionDiscovery::discover` delegates to `FileWalker::process_source_root`,
which runs in two phases per source root:

1. **`collect_parsed_files`** — take every `.rs` file the `SourceTree` lists
   under the source root, apply the path-based selection rules (`is_selected_relative_file`,
   `is_selected_source_file`, `is_excluded_relative_file` — `examples/`,
   `benches/`, `build.rs`, and, unless `--include-test-targets`, `tests/`),
   read and `syn::parse_file` every file that survives, and return the
   whole `Vec<(SourcePath, syn::File)>` — nothing is scored yet.
2. **`TestModuleRegistry::build`** runs once over that same in-memory set,
   resolving every file-based `#[cfg(test)] mod name;` declaration (inline
   `mod { ... }` blocks are recursed into for nested declarations) to the
   file it would compile to -- `name.rs` beside the declaring file, else
   `name/mod.rs`, whichever the tree listed, compared by `PathKeys` key --
   before any file's functions are recorded. The
   main loop then skips any parsed file the registry resolved to, and
   `visit_items`/`visit_item`/`visit_module` walk everything else, each
   *returning* the `Vec<SourceFunction>` it found rather than writing into a
   shared accumulator. See `docs/ADRs/ADR-CrossFileTestModuleExclusion.md`
   for why this needs to be a project-wide pass rather than a per-file
   check, and its scope boundary (single-level, non-recursive across
   files).

Both phases read every candidate file's contents once; `TestModuleRegistry`
and the actual function walk both operate on the same already-parsed
`syn::File`, not a second read-and-reparse.

## Per-impl-block discovery: `ImplCollector`

`ImplCollector` (`impl_collector.rs`) handles the one case `FileWalker`'s
own AST walk delegates out: an `impl Type { ... }` block's methods, each
qualified with the type's own name (`impl_type_name`) so `Foo::bar` and
`Baz::bar` are reported as distinct functions even when their method names
collide. `is_test_attrs`/`is_test_path` (also in this file) recognize
`#[test]` and `#[cfg(test)]` — by trailing path segment, so
`#[tokio::test]`-style attributes are caught too — and are shared by both
`FileWalker` and `ImplCollector` so a test function is excluded identically
whether it is a free function, a method, or (via `TestModuleRegistry`) an
entire file-based submodule.

## Data model

| Type | Scope | Carries |
|---|---|---|
| `WorkspaceDescription`, `PackageDescription`, `TargetDescription` | the workspace as a `WorkspaceReader` describes it | its root, its members' names, manifests and targets (kinds and source paths), and the root package |
| `Export`, `ExportChunk`, `ExportFunction` | one coverage export, as a `CoverageSource` hands it over | each function's file names and regions, as cargo-llvm-cov wrote them |
| `PackageContext` | one resolved Cargo package | name, manifest dir, workspace root, resolved source roots, and the `include_test_targets`/`exclude_paths` settings that applied when it was built |
| `SourceFunction` | one discovered function | qualified name, file path (both a normalized `path_key` for coverage matching and a human-readable `relative_file`), `[line, end_line]` span, and its own `complexity` |
| `CoverageRecord` | one llvm-cov region (post-merge) | `path_key`, `line`, `covered_regions`/`total_regions` |
| `FunctionReport` | one scored function | everything `SourceFunction` carries, plus `coverage`, `crap_score`, and `verdict` |
| `ProjectMetrics` | one `App::run()` invocation | `total_functions`, `crappy_functions`, `crappy_percent`, project-level `verdict` |
| `ProjectReport` | one `App::run()` invocation | `scope_name` (every scored package's name, joined) plus every field `ProjectMetrics` carries, plus the full `Vec<FunctionReport>` |

`ProjectReport` is what `Reporter::render` turns into either the
human-readable table or the `--output-format json` output — it is the
single shape both output modes are projections of, and the same shape
`ProjectReport::exit_code` reads to decide the process's exit code.

## CLI layer

`Args` (`cli/src/invocation/args.rs`, `clap::Parser`) parses argv into flags
and splits them in two: `Args::config()` is the `Config` `App` consumes -- the
packages, the selection options, the thresholds, `--strict` and
`--warn-only` -- and `Args::cargo_config()` the record `CargoConfig`, what
reaches cargo: the manifest path, the features and the coverage file, kept as
`PathBuf`s so a path that is not UTF-8 still works. `main.rs` hands argv to
`EntryPoint::run`, which strips a repeated leading `crap4rust` argument
(present when invoked as `cargo crap4rust ...` rather than the raw binary),
and `EntryPoint::run_from_args` parses the rest, builds the concrete parts,
runs `App` on them and prints its output -- all real logic lives in core.

## Related

- `docs/FORMULA.md` — how `cognitive_complexity`, the CRAP score, and
  project-level verdicts are actually computed.
- `docs/ADRs/` — why `App` is shaped this way, why classification is
  structural rather than type-resolved, why file-based test submodules need
  their own registry, and why duplicate coverage records have to merge
  order-independently.
- `docs/ROADMAP.md` — what's shipped and what's planned next.
