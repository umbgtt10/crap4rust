// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crap4rust::coverage_io::coverage_record::CoverageRecord;

#[test]
fn coverage_ratio_fully_covered_returns_one() {
    // Arrange
    let record = CoverageRecord::new(String::new(), 0, 10, 10);

    // Act
    let ratio = record.coverage_ratio();

    // Assert
    assert!((ratio - 1.0).abs() < 0.001);
}

#[test]
fn coverage_ratio_half_covered_returns_half() {
    // Arrange
    let record = CoverageRecord::new(String::new(), 0, 5, 10);

    // Act
    let ratio = record.coverage_ratio();

    // Assert
    assert!((ratio - 0.5).abs() < 0.001);
}

#[test]
fn coverage_ratio_zero_total_regions_returns_zero() {
    // Arrange
    let record = CoverageRecord::new(String::new(), 0, 0, 0);

    // Act
    let ratio = record.coverage_ratio();

    // Assert
    assert!((ratio - 0.0).abs() < 0.001);
}
