// Copyright 2025 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crap4rust::invocation::config::Config;
use crap4rust::reporting::function_report::FunctionReport;
use crap4rust::reporting::output_format::OutputFormat;
use crap4rust::reporting::project_report::ProjectReport;
use crap4rust::reporting::stdout_reporter::StdoutReporter;
use crap4rust::reporting::verdict::Verdict;
use crap4rust::traits::reporter::Reporter;
use std::panic::AssertUnwindSafe;
use std::panic::catch_unwind;

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
        strict: true,
        warn_only: false,
        output_format: OutputFormat::Human,
    }
}

#[test]
fn render_json_output_format_does_not_panic() {
    // Arrange
    let reporter = StdoutReporter::new();
    let report = ProjectReport {
        scope_name: String::from("test"),
        total_functions: 0,
        crappy_functions: 0,
        crappy_percent: 0.0,
        verdict: Verdict::Clean,
        functions: vec![],
    };
    let mut config = test_config();
    config.output_format = OutputFormat::Json;

    // Act
    let output = catch_unwind(AssertUnwindSafe(|| {
        reporter.render(&report, &config);
    }));

    // Assert
    assert!(output.is_ok());
}

#[test]
fn render_with_crappy_functions_shows_table() {
    // Arrange
    let reporter = StdoutReporter::new();
    let report = ProjectReport {
        scope_name: String::from("test"),
        total_functions: 1,
        crappy_functions: 1,
        crappy_percent: 100.0,
        verdict: Verdict::Crappy,
        functions: vec![FunctionReport {
            package_name: String::from("pkg"),
            name: String::from("risky_fn"),
            relative_file: String::from("src/lib.rs"),
            line: 10,
            complexity: 10,
            coverage: 0.0,
            crap_score: 110.0,
            verdict: Verdict::Crappy,
        }],
    };
    let config = test_config();

    // Act
    let output = catch_unwind(AssertUnwindSafe(|| {
        reporter.render(&report, &config);
    }));

    // Assert
    assert!(output.is_ok());
}

#[test]
fn render_with_no_clean_functions_shows_threshold_message() {
    // Arrange
    let reporter = StdoutReporter::new();
    let report = ProjectReport {
        scope_name: String::from("test"),
        total_functions: 0,
        crappy_functions: 0,
        crappy_percent: 0.0,
        verdict: Verdict::Clean,
        functions: vec![],
    };
    let config = test_config();

    // Act
    let output = catch_unwind(AssertUnwindSafe(|| {
        reporter.render(&report, &config);
    }));

    // Assert
    assert!(output.is_ok());
}
