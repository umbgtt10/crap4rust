// Copyright 2025 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crap4rust::reporting::project_metrics::ProjectMetrics;
use crap4rust::reporting::verdict::Verdict;

#[test]
fn eq_differing_verdict_are_not_equal() {
    // Arrange
    let left = ProjectMetrics {
        verdict: Verdict::Clean,
        crappy_functions: 0,
        total_functions: 5,
        crappy_percent: 0.0,
    };
    let right = ProjectMetrics {
        verdict: Verdict::Crappy,
        ..left
    };

    // Act & Assert
    assert_ne!(left, right);
}

#[test]
fn eq_identical_metrics_are_equal() {
    // Arrange
    let left = ProjectMetrics {
        verdict: Verdict::Warn,
        crappy_functions: 2,
        total_functions: 10,
        crappy_percent: 20.0,
    };
    let right = left;

    // Act & Assert
    assert_eq!(left, right);
}
