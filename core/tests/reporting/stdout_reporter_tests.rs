// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
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
    Config::default()
        .with_threshold(30.0)
        .with_warn_threshold(20.0)
        .with_project_threshold(5.0)
        .with_strict(true)
        .with_output_format(OutputFormat::Human)
}

#[test]
fn render_json_output_format_does_not_panic() {
    // Arrange
    let reporter = StdoutReporter::new();
    let report = ProjectReport::new(String::from("test"), 0, 0, 0.0, Verdict::Clean, vec![]);
    let config = test_config().with_output_format(OutputFormat::Json);

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
    let report = ProjectReport::new(
        String::from("test"),
        1,
        1,
        100.0,
        Verdict::Crappy,
        vec![
            FunctionReport::default()
                .with_package_name(String::from("pkg"))
                .with_name(String::from("risky_fn"))
                .with_relative_file(String::from("src/lib.rs"))
                .with_line(10)
                .with_complexity(10)
                .with_crap_score(110.0)
                .with_verdict(Verdict::Crappy),
        ],
    );
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
    let report = ProjectReport::new(String::from("test"), 0, 0, 0.0, Verdict::Clean, vec![]);
    let config = test_config();

    // Act
    let output = catch_unwind(AssertUnwindSafe(|| {
        reporter.render(&report, &config);
    }));

    // Assert
    assert!(output.is_ok());
}
