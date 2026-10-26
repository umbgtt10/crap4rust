// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use anyhow::{Context, Result, bail};

use crate::analysis::source_function::SourceFunction;
use crate::coverage_io::coverage_index::CoverageIndex;
use crate::invocation::config::Config;
use crate::invocation::package_context::PackageContext;
use crate::invocation::run_outcome::RunOutcome;
use crate::reporting::project_report::ProjectReport;
use crate::traits::coverage_provider::CoverageProvider;
use crate::traits::function_discovery::FunctionDiscovery;
use crate::traits::package_resolver::PackageResolver;
use crate::traits::reporter::Reporter;
use crate::traits::scorer::Scorer;

pub struct App<'a> {
    resolver: Box<dyn PackageResolver + 'a>,
    discovery: Box<dyn FunctionDiscovery + 'a>,
    coverage_provider: Box<dyn CoverageProvider + 'a>,
    scorer: Box<dyn Scorer + 'a>,
    reporter: Box<dyn Reporter + 'a>,
    config: Config,
}

impl<'a> App<'a> {
    #[must_use]
    pub fn new(
        resolver: Box<dyn PackageResolver + 'a>,
        discovery: Box<dyn FunctionDiscovery + 'a>,
        coverage_provider: Box<dyn CoverageProvider + 'a>,
        scorer: Box<dyn Scorer + 'a>,
        reporter: Box<dyn Reporter + 'a>,
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

    pub fn run(&self) -> Result<RunOutcome> {
        let packages = self.resolver.resolve(&self.config)?;
        let coverage_records = self.coverage_provider.provide(&packages)?;

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

        let report_data = ProjectReport::new(
            packages
                .iter()
                .map(|package| package.name.clone())
                .collect::<Vec<_>>()
                .join(", "),
            metrics.total_functions,
            metrics.crappy_functions,
            metrics.crappy_percent,
            metrics.verdict,
            reports,
        );

        Ok(RunOutcome {
            output: self.reporter.render(&report_data, &self.config),
            exit_code: report_data.exit_code(&self.config),
        })
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
