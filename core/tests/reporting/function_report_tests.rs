// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crap4rust::reporting::function_report::FunctionReport;
use crap4rust::reporting::verdict::Verdict;

fn sample() -> FunctionReport {
    FunctionReport {
        package_name: String::from("cargo-crap4rust"),
        name: String::from("compute_crap_score"),
        relative_file: String::from("src/app.rs"),
        line: 42,
        complexity: 5,
        coverage: 0.5,
        crap_score: 22.5,
        verdict: Verdict::Warn,
    }
}

#[test]
fn clone_produces_an_equal_copy() {
    // Arrange
    let report = sample();

    // Act
    let cloned = report.clone();

    // Assert
    assert_eq!(cloned.name, report.name);
    assert_eq!(cloned.crap_score, report.crap_score);
}
