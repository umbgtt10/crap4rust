// Copyright 2025 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crap4rust::reporting::function_report::FunctionReport;
use crap4rust::reporting::project_report::ProjectReport;
use crap4rust::reporting::verdict::Verdict;
use serde_json::to_string;

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
