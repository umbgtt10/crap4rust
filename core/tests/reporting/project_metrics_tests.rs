// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crap4rust::reporting::project_metrics::ProjectMetrics;
use crap4rust::reporting::verdict::Verdict;

#[test]
fn eq_differing_verdict_are_not_equal() {
    // Arrange
    let left = ProjectMetrics::new(Verdict::Clean, 0, 5, 0.0);
    let right = ProjectMetrics::new(Verdict::Crappy, 0, 5, 0.0);

    // Act & Assert
    assert_ne!(left, right);
}

#[test]
fn eq_identical_metrics_are_equal() {
    // Arrange
    let left = ProjectMetrics::new(Verdict::Warn, 2, 10, 20.0);
    let right = left;

    // Act & Assert
    assert_eq!(left, right);
}
