// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use std::ffi::OsString;
use std::process::ExitCode;

use anyhow::Result;
use clap::Parser;
use crap4rust::analysis::source_function_discovery::SourceFunctionDiscovery;
use crap4rust::coverage_io::export_coverage_provider::ExportCoverageProvider;
use crap4rust::invocation::app::App;
use crap4rust::invocation::cargo_package_resolver::CargoPackageResolver;
use crap4rust::reporting::default_scorer::DefaultScorer;
use crap4rust::reporting::human_reporter::HumanReporter;
use crap4rust::traits::reporter::Reporter;

use crate::invocation::args::Args;
use crate::invocation::output_format::OutputFormat;
use crate::system::canonical_path_keys::CanonicalPathKeys;
use crate::system::cargo_workspace_reader::CargoWorkspaceReader;
use crate::system::json_reporter::JsonReporter;
use crate::system::llvm_cov_source::LlvmCovSource;
use crate::system::walkdir_source_tree::WalkdirSourceTree;

// Everything `main` used to do, moved somewhere a test can reach it.
//
// A binary entry point is not reachable from an integration test, so logic
// living in `main.rs` is logic nothing can check. What was sitting there is not
// glue: it is the argv fixup every cargo subcommand needs, and it has two
// properties that are easy to get wrong and impossible to notice.
//
// Cargo runs `cargo crap4rust ...` as `cargo-crap4rust crap4rust ...`, so the
// name arrives twice. Running the binary directly does not repeat it, which is
// why the strip is **conditional**. And it is **positional** -- only argv[1] is
// dropped -- because removing every occurrence would swallow
// `--package crap4rust`.
pub struct EntryPoint;

impl EntryPoint {
    pub const SUBCOMMAND: &'static str = "crap4rust";

    // An error is exit 2, the same code a broken rule uses, because a run that
    // could not finish must never read as a clean project.
    pub fn run(args: Vec<String>) -> ExitCode {
        match Self::run_from_args(Self::without_cargo_subcommand(args)) {
            Ok(code) => code,
            Err(error) => {
                println!("error: {error:#}");
                ExitCode::from(2)
            }
        }
    }

    // The composition root: App takes every part it runs on, and this is where
    // the concrete ones are chosen -- cargo metadata, the file system, cargo
    // llvm-cov and the report's format. The report reaches stdout here and
    // nowhere else.
    pub fn run_from_args<I, T>(args: I) -> Result<ExitCode>
    where
        I: IntoIterator<Item = T>,
        T: Into<OsString> + Clone,
    {
        let args = <Args as Parser>::parse_from(args);
        let cargo_config = args.cargo_config();
        let workspace_reader = CargoWorkspaceReader::new(cargo_config.manifest_path.clone());
        let source_tree = WalkdirSourceTree::new();
        let path_keys = CanonicalPathKeys::new();
        let coverage_source = LlvmCovSource::new(cargo_config);
        let reporter: Box<dyn Reporter> = match args.output_format() {
            OutputFormat::Json => Box::new(JsonReporter::new()),
            OutputFormat::Human => Box::new(HumanReporter::new()),
        };
        let outcome = App::new(
            Box::new(CargoPackageResolver::new(&workspace_reader)),
            Box::new(SourceFunctionDiscovery::new(&source_tree, &path_keys)),
            Box::new(ExportCoverageProvider::new(&coverage_source, &path_keys)),
            Box::new(DefaultScorer::new()),
            reporter,
            args.config(),
        )
        .run()?;

        print!("{}", outcome.output);
        Ok(ExitCode::from(outcome.exit_code))
    }

    // Conditional and positional: only a repeated name in argv[1] goes.
    pub fn without_cargo_subcommand(args: Vec<String>) -> Vec<String> {
        if args.get(1).map(String::as_str) != Some(Self::SUBCOMMAND) {
            return args;
        }
        let mut forwarded = Vec::with_capacity(args.len().saturating_sub(1));
        let mut rest = args.into_iter();
        if let Some(binary) = rest.next() {
            forwarded.push(binary);
        }
        forwarded.extend(rest.skip(1));
        forwarded
    }
}
