// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crap4rust::reporting::function_report::FunctionReport;
use crap4rust::reporting::verdict::Verdict;
use serde_json::to_string;

fn sample() -> FunctionReport {
    FunctionReport::default()
        .with_package_name(String::from("cargo-crap4rust"))
        .with_name(String::from("compute_crap_score"))
        .with_relative_file(String::from("src/app.rs"))
        .with_line(42)
        .with_complexity(5)
        .with_coverage(0.5)
        .with_crap_score(22.5)
        .with_verdict(Verdict::Warn)
}

#[test]
fn clone_produces_an_equal_copy() {
    // Arrange
    let report = sample();

    // Act
    let cloned = report.clone();

    // Assert
    assert_eq!(cloned.name(), report.name());
    assert_eq!(cloned.crap_score(), report.crap_score());
}

#[test]
fn to_string_serializes_all_fields_to_json() {
    // Arrange
    let report = sample();

    // Act
    let json = to_string(&report).expect("serialize function report");

    // Assert
    assert!(json.contains(r#""package_name":"cargo-crap4rust""#));
    assert!(json.contains(r#""name":"compute_crap_score""#));
    assert!(json.contains(r#""relative_file":"src/app.rs""#));
    assert!(json.contains(r#""line":42"#));
    assert!(json.contains(r#""complexity":5"#));
    assert!(json.contains(r#""coverage":0.5"#));
    assert!(json.contains(r#""crap_score":22.5"#));
    assert!(json.contains(r#""verdict":"Warn""#));
}
