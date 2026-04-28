// Copyright 2025 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License or Apache License, Version 2.0
// SPDX-License-Identifier: MIT OR Apache-2.0

use crap4rust::model::{CoverageRecord, FunctionReport, ProjectReport, Verdict};

#[test]
fn coverage_ratio_zero_total_regions_returns_zero() {
    // Arrange
    let record = CoverageRecord {
        path_key: String::new(),
        line: 0,
        covered_regions: 0,
        total_regions: 0,
    };

    // Act
    let ratio = record.coverage_ratio();

    // Assert
    assert!((ratio - 0.0).abs() < 0.001);
}

#[test]
fn coverage_ratio_half_covered_returns_half() {
    // Arrange
    let record = CoverageRecord {
        path_key: String::new(),
        line: 0,
        covered_regions: 5,
        total_regions: 10,
    };

    // Act
    let ratio = record.coverage_ratio();

    // Assert
    assert!((ratio - 0.5).abs() < 0.001);
}

#[test]
fn coverage_ratio_fully_covered_returns_one() {
    // Arrange
    let record = CoverageRecord {
        path_key: String::new(),
        line: 0,
        covered_regions: 10,
        total_regions: 10,
    };

    // Act
    let ratio = record.coverage_ratio();

    // Assert
    assert!((ratio - 1.0).abs() < 0.001);
}

#[test]
fn verdict_as_str_clean() {
    // Arrange & Act
    let s = Verdict::Clean.as_str();

    // Assert
    assert_eq!(s, "clean");
}

#[test]
fn verdict_as_str_warn() {
    // Arrange & Act
    let s = Verdict::Warn.as_str();

    // Assert
    assert_eq!(s, "warn");
}

#[test]
fn verdict_as_str_crappy() {
    // Arrange & Act
    let s = Verdict::Crappy.as_str();

    // Assert
    assert_eq!(s, "crappy");
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
    let json = serde_json::to_string(&report).expect("serialization");

    // Assert
    assert!(json.contains(r#""scope_name":"test""#));
    assert!(json.contains(r#""verdict":"Clean""#));
    assert!(json.contains(r#""name":"foo""#));
}
