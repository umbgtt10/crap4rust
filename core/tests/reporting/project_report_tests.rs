// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crap4rust::invocation::config::Config;
use crap4rust::reporting::function_report::FunctionReport;
use crap4rust::reporting::output_format::OutputFormat;
use crap4rust::reporting::project_report::ProjectReport;
use crap4rust::reporting::verdict::Verdict;
use serde_json::to_string;
use std::process::ExitCode;

fn config(warn_only: bool) -> Config {
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
        warn_only,
        output_format: OutputFormat::Human,
    }
}

fn report(crappy_functions: usize) -> ProjectReport {
    ProjectReport {
        scope_name: String::from("probe"),
        total_functions: 4,
        crappy_functions,
        crappy_percent: crappy_functions as f64 * 25.0,
        verdict: Verdict::Clean,
        functions: vec![],
    }
}

#[test]
fn exit_code_of_a_clean_report_is_success() {
    // Arrange & Act
    let code = report(0).exit_code(&config(false));

    // Assert
    assert_eq!(code, ExitCode::SUCCESS);
}

#[test]
fn exit_code_of_a_report_with_a_crappy_function_is_one() {
    // Arrange & Act
    let code = report(1).exit_code(&config(false));

    // Assert
    assert_eq!(code, ExitCode::from(1));
}

// Warn-only reports and never fails the build, whatever it found.
#[test]
fn exit_code_with_warn_only_is_success_whatever_it_finds() {
    // Arrange & Act
    let code = report(2).exit_code(&config(true));

    // Assert
    assert_eq!(code, ExitCode::SUCCESS);
}

#[test]
fn project_report_serialization_contains_expected_fields() {
    // Arrange
    let report = ProjectReport {
        scope_name: String::from("test"),
        total_functions: 1,
        crappy_functions: 0,
        crappy_percent: 0.0,
        verdict: Verdict::Clean,
        functions: vec![FunctionReport {
            package_name: String::from("test"),
            name: String::from("foo"),
            relative_file: String::from("src/lib.rs"),
            line: 10,
            complexity: 1,
            coverage: 1.0,
            crap_score: 1.0,
            verdict: Verdict::Clean,
        }],
    };

    // Act
    let json = to_string(&report).expect("serialization");

    // Assert
    assert!(json.contains(r#""scope_name":"test""#));
    assert!(json.contains(r#""verdict":"Clean""#));
    assert!(json.contains(r#""name":"foo""#));
}
