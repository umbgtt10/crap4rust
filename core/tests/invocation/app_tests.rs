// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use anyhow::{Result, bail};
use crap4rust::analysis::source_function::SourceFunction;
use crap4rust::coverage_io::coverage_record::CoverageRecord;
use crap4rust::invocation::app::App;
use crap4rust::invocation::app::run_from_args;
use crap4rust::invocation::config::Config;
use crap4rust::invocation::package_context::PackageContext;
use crap4rust::reporting::default_scorer::DefaultScorer;
use crap4rust::reporting::output_format::OutputFormat;
use crap4rust::reporting::project_report::ProjectReport;
use crap4rust::traits::coverage_provider::CoverageProvider;
use crap4rust::traits::function_discovery::FunctionDiscovery;
use crap4rust::traits::package_resolver::PackageResolver;
use crap4rust::traits::reporter::Reporter;
use std::path::PathBuf;

struct FailingPackageResolver;

impl PackageResolver for FailingPackageResolver {
    fn resolve(&self, _config: &Config) -> Result<Vec<PackageContext>> {
        bail!("manifest not found")
    }
}

struct FakeCoverageProvider {
    records: Vec<CoverageRecord>,
}

impl CoverageProvider for FakeCoverageProvider {
    fn provide(
        &self,
        _config: &Config,
        _packages: &[PackageContext],
    ) -> Result<Vec<CoverageRecord>> {
        Ok(self.records.clone())
    }
}

struct FakeFunctionDiscovery {
    functions: Vec<SourceFunction>,
}

impl FunctionDiscovery for FakeFunctionDiscovery {
    fn discover(&self, _package: &PackageContext) -> Result<Vec<SourceFunction>> {
        Ok(self.functions.clone())
    }
}

struct FakePackageResolver {
    packages: Vec<PackageContext>,
}

impl PackageResolver for FakePackageResolver {
    fn resolve(&self, _config: &Config) -> Result<Vec<PackageContext>> {
        Ok(self.packages.clone())
    }
}

struct NoOpReporter;

impl Reporter for NoOpReporter {
    fn render(&self, _report: &ProjectReport, _config: &Config) {}
}

fn sample_function() -> SourceFunction {
    SourceFunction {
        package_name: String::from("probe"),
        name: String::from("risky"),
        path_key: String::from("src/lib.rs"),
        relative_file: String::from("src/lib.rs"),
        line: 10,
        end_line: 20,
        complexity: 5,
    }
}

fn sample_package() -> PackageContext {
    PackageContext {
        name: String::from("probe"),
        manifest_dir: PathBuf::from("/probe"),
        workspace_root: PathBuf::from("/probe"),
        source_roots: vec![],
        include_test_targets: false,
        exclude_paths: vec![],
    }
}

fn sample_record() -> CoverageRecord {
    CoverageRecord {
        path_key: String::from("src/lib.rs"),
        line: 10,
        covered_regions: 1,
        total_regions: 2,
    }
}

fn test_config() -> Config {
    Config {
        coverage_path: None,
        manifest_path: None,
        packages: vec![],
        features: None,
        all_features: false,
        no_default_features: false,
        include_test_targets: false,
        exclude_paths: vec![],
        threshold: 30.0,
        warn_threshold: 20.0,
        project_threshold: 5.0,
        strict: false,
        warn_only: false,
        output_format: OutputFormat::Human,
    }
}

// The one entry point the binary itself calls. Every other test here drives App
// directly, which left the argv-to-exit-code path -- the whole tool, as a user
// invokes it -- uncovered.
#[test]
fn run_from_args_against_a_manifest_that_does_not_exist_is_an_error() {
    // Arrange
    let args = [
        "cargo-crap4rust",
        "--manifest-path",
        "no/such/directory/Cargo.toml",
    ];

    // Act
    let outcome = run_from_args(args);

    // Assert
    assert!(
        outcome.is_err(),
        "a manifest that cannot be read must fail the run rather than report a clean project"
    );
}

#[test]
fn run_matched_functions_and_coverage_succeeds() {
    // Arrange
    let app = App::with_deps(
        Box::new(FakePackageResolver {
            packages: vec![sample_package()],
        }),
        Box::new(FakeFunctionDiscovery {
            functions: vec![sample_function()],
        }),
        Box::new(FakeCoverageProvider {
            records: vec![sample_record()],
        }),
        Box::new(DefaultScorer::new()),
        Box::new(NoOpReporter),
        test_config(),
    );

    // Act
    let result = app.run();

    // Assert
    assert!(result.is_ok());
}

#[test]
fn run_no_coverage_records_bails() {
    // Arrange
    let app = App::with_deps(
        Box::new(FakePackageResolver {
            packages: vec![sample_package()],
        }),
        Box::new(FakeFunctionDiscovery {
            functions: vec![sample_function()],
        }),
        Box::new(FakeCoverageProvider { records: vec![] }),
        Box::new(DefaultScorer::new()),
        Box::new(NoOpReporter),
        test_config(),
    );

    // Act
    let result = app.run();

    // Assert
    assert!(
        result
            .unwrap_err()
            .to_string()
            .contains("coverage file did not contain any function records")
    );
}

#[test]
fn run_no_functions_discovered_bails() {
    // Arrange
    let app = App::with_deps(
        Box::new(FakePackageResolver {
            packages: vec![sample_package()],
        }),
        Box::new(FakeFunctionDiscovery { functions: vec![] }),
        Box::new(FakeCoverageProvider {
            records: vec![sample_record()],
        }),
        Box::new(DefaultScorer::new()),
        Box::new(NoOpReporter),
        test_config(),
    );

    // Act
    let result = app.run();

    // Assert
    assert!(
        result
            .unwrap_err()
            .to_string()
            .contains("no Rust functions were discovered")
    );
}

#[test]
fn run_package_resolver_failure_propagates_error() {
    // Arrange
    let app = App::with_deps(
        Box::new(FailingPackageResolver),
        Box::new(FakeFunctionDiscovery { functions: vec![] }),
        Box::new(FakeCoverageProvider { records: vec![] }),
        Box::new(DefaultScorer::new()),
        Box::new(NoOpReporter),
        test_config(),
    );

    // Act
    let result = app.run();

    // Assert
    assert!(result.is_err());
    assert!(
        result
            .unwrap_err()
            .to_string()
            .contains("manifest not found")
    );
}

#[test]
fn run_unmatched_coverage_bails() {
    // Arrange
    let mut unmatched_record = sample_record();
    unmatched_record.line = 999;
    let app = App::with_deps(
        Box::new(FakePackageResolver {
            packages: vec![sample_package()],
        }),
        Box::new(FakeFunctionDiscovery {
            functions: vec![sample_function()],
        }),
        Box::new(FakeCoverageProvider {
            records: vec![unmatched_record],
        }),
        Box::new(DefaultScorer::new()),
        Box::new(NoOpReporter),
        test_config(),
    );

    // Act
    let result = app.run();

    // Assert
    assert!(
        result
            .unwrap_err()
            .to_string()
            .contains("coverage data could not be matched")
    );
}
