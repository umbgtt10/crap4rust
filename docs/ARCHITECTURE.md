# Architecture

How a `cargo crap4rust` invocation actually flows through the code. This is
a map of what exists today, not a decision record — see `docs/ADRs/` for the
"why" behind the shapes described here, and `docs/FORMULA.md` for how the
scores themselves are computed.

---

## Pipeline

```
Args (clap)
  → Config::from_args(args)
    → App
        resolver:          Box<dyn PackageResolver>    — resolves Cargo packages to score
        discovery:         Box<dyn FunctionDiscovery>   — finds functions inside each package
        coverage_provider: Box<dyn CoverageProvider>    — provides coverage data for them
        scorer:            Box<dyn Scorer>              — turns both into scored reports
        reporter:          Box<dyn Reporter>             — renders the final report
      → App::run()
          1. resolver.resolve(&config) -> Vec<PackageContext>
          2. coverage_provider.provide(&config, &packages) -> Vec<CoverageRecord>
             (generates coverage via `cargo llvm-cov` first if --coverage
             was not given; either way, records are not checked for
             emptiness yet)
          3. discovery.discover(&package) -> Vec<SourceFunction>, once per
             package, accumulated; bail if the total is empty
          4. bail if the coverage records collected in (2) are empty
          5. CoverageIndex::from_records(records); bail if not a single
             discovered function matches any record by file + line span
          6. scorer.score_functions(functions, &coverage_index, &config)
             -> Vec<FunctionReport>, sorted by crap_score descending
          7. scorer.project_metrics(&reports, &config) -> ProjectMetrics
          8. reporter.render(&ProjectReport { .. }, &config)
          9. Ok(report.exit_code(&config))
```

The four bail points (empty functions, empty coverage, no matches, plus
whatever error `resolve`/`provide`/`discover` themselves return) all
surface as `Result::Err` from `App::run()`, printed by `main.rs` as
`error: {message}` with exit code `2` — distinct from a *successful* run
that nonetheless found crappy functions and exits `1`.

`App` (`app.rs`) is the only place that wires concrete types to trait
objects — everywhere else in the codebase depends on the `traits/`
interfaces, never on `CargoPackageResolver`/`SourceFunctionDiscovery`/
`LlvmCovProvider`/`DefaultScorer`/`StdoutReporter` directly. See
`docs/ADRs/ADR-DynDispatchAppOverGenerics.md` for why these are `Box<dyn
Trait>` fields rather than generic parameters.

## The five injected seams (`traits/`)

| Trait | Concrete impl | Responsibility |
|---|---|---|
| `PackageResolver` | `CargoPackageResolver` (`cargo_package_resolver.rs`) | Run `cargo_metadata`, select the requested (or, for a multi-member workspace with none requested, every) package, and build each one's `PackageContext` — manifest dir, workspace root, and resolved source roots. |
| `FunctionDiscovery` | `SourceFunctionDiscovery` (`source_function_discovery.rs`) | Walk a single package's source roots and return every production `SourceFunction` found, complexity already computed. |
| `CoverageProvider` | `LlvmCovProvider` (`llvm_cov_provider.rs`) | Resolve a coverage JSON path (generating one via `cargo llvm-cov` if none was given) and parse it into `CoverageRecord`s. |
| `Scorer` | `DefaultScorer` (`default_scorer.rs`) | Turn discovered functions plus a `CoverageIndex` into scored, sorted `FunctionReport`s, and fold those into project-level `ProjectMetrics`. |
| `Reporter` | `StdoutReporter` (`stdout_reporter.rs`) | Render a `ProjectReport` as human-readable text or JSON. |

Every trait's real implementation keeps its underlying logic as plain,
directly-unit-testable functions in the same file (`resolve_packages`,
`discover_functions`, `ensure_coverage_path`/`load_coverage_records`) —
the trait `impl` itself is a thin delegator, not a rewrite. `tests/app_tests.rs`
is the seam these five traits exist for: it builds `App` via
`App::with_deps()` with fakes standing in for whichever one dependency a
given test wants to fail or return edge-case data from, without touching a
real Cargo workspace, filesystem, or `cargo llvm-cov` subprocess for the
other four.

## Per-package function discovery: `FileWalker` and `TestModuleRegistry`

`SourceFunctionDiscovery::discover` delegates to `FileWalker::process_source_root`,
which runs in two phases per source root:

1. **`collect_parsed_files`** — walk every `.rs` file under the source root,
   apply the path-based selection rules (`is_selected_relative_file`,
   `is_selected_source_file`, `is_excluded_relative_file` — `examples/`,
   `benches/`, `build.rs`, and, unless `--include-test-targets`, `tests/`),
   read and `syn::parse_file` every file that survives, and return the
   whole `Vec<(PathBuf, syn::File)>` — nothing is scored yet.
2. **`TestModuleRegistry::build`** runs once over that same in-memory set,
   resolving every file-based `#[cfg(test)] mod name;` declaration (inline
   `mod { ... }` blocks are recursed into for nested declarations) to the
   file it would compile to, before any file's functions are recorded. The
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

`Args` (`cli.rs`, `clap::Parser`) parses argv into flags; `Config`
(`config.rs`) is the plain-data form `App` actually consumes, built once via
`Config::from_args(args)`. `main.rs` strips a leading `crap4rust` argument
(present when invoked as `cargo crap4rust ...` rather than the raw binary)
and `lib.rs::run()`/`run_from_args()` are thin entry points — all real logic
lives in `App` and below.

## Related

- `docs/FORMULA.md` — how `cognitive_complexity`, the CRAP score, and
  project-level verdicts are actually computed.
- `docs/ADRs/` — why `App` is shaped this way, why classification is
  structural rather than type-resolved, why file-based test submodules need
  their own registry, and why duplicate coverage records have to merge
  order-independently.
- `docs/ROADMAP.md` — what's shipped and what's planned next.
