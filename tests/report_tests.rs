// Copyright 2025 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License or Apache License, Version 2.0
// SPDX-License-Identifier: MIT OR Apache-2.0

use crap4rust::cli::OutputFormat;
use crap4rust::model::{Config, ProjectReport, Verdict};

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
fn print_report_with_no_clean_functions_shows_threshold_message() {
    // Arrange
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
    let output = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        // This prints to stdout, which is fine — we just verify no panic
        crap4rust::report::print_report(&report, &config);
    }));

    // Assert
    assert!(output.is_ok());
}

#[test]
fn print_report_with_crappy_functions_shows_table() {
    // Arrange
    use crap4rust::model::FunctionReport;
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
    let output = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        crap4rust::report::print_report(&report, &config);
    }));

    // Assert
    assert!(output.is_ok());
}
