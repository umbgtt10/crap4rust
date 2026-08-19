// Copyright 2025 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use std::process::ExitCode;

use anyhow::{Context, Result, bail};

use crate::cargo_package_resolver::CargoPackageResolver;
use crate::cli::Args;
use crate::config::Config;
use crate::coverage_index::CoverageIndex;
use crate::default_scorer::DefaultScorer;
use crate::llvm_cov_provider::LlvmCovProvider;
use crate::package_context::PackageContext;
use crate::project_report::ProjectReport;
use crate::source_function::SourceFunction;
use crate::source_function_discovery::SourceFunctionDiscovery;
use crate::stdout_reporter::StdoutReporter;
use crate::traits::coverage_provider::CoverageProvider;
use crate::traits::function_discovery::FunctionDiscovery;
use crate::traits::package_resolver::PackageResolver;
use crate::traits::reporter::Reporter;
use crate::traits::scorer::Scorer;

pub struct App {
    resolver: Box<dyn PackageResolver>,
    discovery: Box<dyn FunctionDiscovery>,
    coverage_provider: Box<dyn CoverageProvider>,
    scorer: Box<dyn Scorer>,
    reporter: Box<dyn Reporter>,
    config: Config,
}

impl App {
    #[must_use]
    pub fn new(config: Config) -> Self {
        Self {
            resolver: Box::new(CargoPackageResolver::new()),
            discovery: Box::new(SourceFunctionDiscovery::new()),
            coverage_provider: Box::new(LlvmCovProvider::new()),
            scorer: Box::new(DefaultScorer::new()),
            reporter: Box::new(StdoutReporter::new()),
            config,
        }
    }

    #[must_use]
    pub fn with_deps(
        resolver: Box<dyn PackageResolver>,
        discovery: Box<dyn FunctionDiscovery>,
        coverage_provider: Box<dyn CoverageProvider>,
        scorer: Box<dyn Scorer>,
        reporter: Box<dyn Reporter>,
        config: Config,
    ) -> Self {
        Self {
            resolver,
            discovery,
            coverage_provider,
            scorer,
            reporter,
            config,
        }
    }

    pub fn run(&self) -> Result<ExitCode> {
        let packages = self.resolver.resolve(&self.config)?;
        let coverage_records = self.coverage_provider.provide(&self.config, &packages)?;

        let functions = self.discover_all_functions(&packages)?;
        if functions.is_empty() {
            bail!("no Rust functions were discovered in the selected packages");
        }

        if coverage_records.is_empty() {
            bail!("coverage file did not contain any function records");
        }
        let coverage_index = CoverageIndex::from_records(coverage_records);

        let matched_count = functions
            .iter()
            .filter(|function| coverage_index.match_function(function).is_some())
            .count();
        if matched_count == 0 {
            bail!(
                "coverage data could not be matched to any discovered function by file path and line"
            );
        }

        let reports = self
            .scorer
            .score_functions(functions, &coverage_index, &self.config);
        let metrics = self.scorer.project_metrics(&reports, &self.config);

        let report_data = ProjectReport {
            scope_name: packages
                .iter()
                .map(|package| package.name.clone())
                .collect::<Vec<_>>()
                .join(", "),
            total_functions: metrics.total_functions,
            crappy_functions: metrics.crappy_functions,
            crappy_percent: metrics.crappy_percent,
            verdict: metrics.verdict,
            functions: reports,
        };

        self.reporter.render(&report_data, &self.config);

        Ok(report_data.exit_code(&self.config))
    }

    fn discover_all_functions(&self, packages: &[PackageContext]) -> Result<Vec<SourceFunction>> {
        let mut functions = Vec::new();
        for package in packages {
            let mut package_functions = self.discovery.discover(package).with_context(|| {
                format!("failed to discover functions in package {}", package.name)
            })?;
            functions.append(&mut package_functions);
        }
        Ok(functions)
    }
}

pub fn run(args: Args) -> Result<ExitCode> {
    App::new(Config::from_args(args)).run()
}
