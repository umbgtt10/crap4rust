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
    Config::default()
        .with_threshold(30.0)
        .with_warn_threshold(20.0)
        .with_project_threshold(5.0)
        .with_strict(true)
        .with_warn_only(warn_only)
        .with_output_format(OutputFormat::Human)
}

fn report(crappy_functions: usize) -> ProjectReport {
    ProjectReport::new(
        String::from("probe"),
        4,
        crappy_functions,
        crappy_functions as f64 * 25.0,
        Verdict::Clean,
        vec![],
    )
}

#[test]
fn crappy_functions_returns_how_many_functions_are_crappy() {
    // Arrange
    let report = report(1);

    // Act
    let crappy_functions = report.crappy_functions();

    // Assert
    assert_eq!(crappy_functions, 1);
}

#[test]
fn crappy_percent_returns_the_share_of_crappy_functions() {
    // Arrange
    let report = report(2);

    // Act
    let crappy_percent = report.crappy_percent();

    // Assert
    assert!((crappy_percent - 50.0).abs() < f64::EPSILON);
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
fn functions_lists_every_function_the_report_covers() {
    // Arrange
    let report = ProjectReport::new(
        String::from("probe"),
        1,
        0,
        0.0,
        Verdict::Clean,
        vec![FunctionReport {
            name: String::from("foo"),
            ..Default::default()
        }],
    );

    // Act
    let functions = report.functions();

    // Assert
    assert_eq!(functions.len(), 1);
    assert_eq!(functions[0].name, "foo");
}

#[test]
fn project_report_serialization_contains_expected_fields() {
    // Arrange
    let report = ProjectReport::new(
        String::from("test"),
        1,
        0,
        0.0,
        Verdict::Clean,
        vec![FunctionReport {
            package_name: String::from("test"),
            name: String::from("foo"),
            relative_file: String::from("src/lib.rs"),
            line: 10,
            complexity: 1,
            coverage: 1.0,
            crap_score: 1.0,
            verdict: Verdict::Clean,
        }],
    );

    // Act
    let json = to_string(&report).expect("serialization");

    // Assert
    assert!(json.contains(r#""scope_name":"test""#));
    assert!(json.contains(r#""verdict":"Clean""#));
    assert!(json.contains(r#""name":"foo""#));
}

#[test]
fn scope_name_returns_the_scope_the_report_covers() {
    // Arrange
    let report = report(0);

    // Act
    let scope_name = report.scope_name();

    // Assert
    assert_eq!(scope_name, "probe");
}

#[test]
fn total_functions_returns_how_many_functions_the_report_covers() {
    // Arrange
    let report = report(0);

    // Act
    let total_functions = report.total_functions();

    // Assert
    assert_eq!(total_functions, 4);
}

#[test]
fn verdict_returns_the_verdict_of_the_project() {
    // Arrange
    let report = report(0);

    // Act
    let verdict = report.verdict();

    // Assert
    assert_eq!(verdict, Verdict::Clean);
}
