// Copyright 2025 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crap4rust::reporting::crap_formula::CrapFormula;
use crap4rust::reporting::verdict::Verdict;

#[test]
fn classify_above_threshold_returns_crappy() {
    // Arrange & Act
    let verdict = CrapFormula::new().classify(31.0, 30.0, 20.0);

    // Assert
    assert_eq!(verdict, Verdict::Crappy);
}

#[test]
fn classify_at_threshold_returns_warn() {
    // Arrange & Act
    let verdict = CrapFormula::new().classify(30.0, 30.0, 20.0);

    // Assert
    assert_eq!(verdict, Verdict::Warn);
}

#[test]
fn classify_at_warn_threshold_returns_warn() {
    // Arrange & Act
    let verdict = CrapFormula::new().classify(20.0, 30.0, 20.0);

    // Assert
    assert_eq!(verdict, Verdict::Warn);
}

#[test]
fn classify_below_warn_threshold_returns_clean() {
    // Arrange & Act
    let verdict = CrapFormula::new().classify(19.9, 30.0, 20.0);

    // Assert
    assert_eq!(verdict, Verdict::Clean);
}

#[test]
fn classify_between_warn_and_threshold_returns_warn() {
    // Arrange & Act
    let verdict = CrapFormula::new().classify(25.0, 30.0, 20.0);

    // Assert
    assert_eq!(verdict, Verdict::Warn);
}

#[test]
fn classify_zero_score_returns_clean() {
    // Arrange & Act
    let verdict = CrapFormula::new().classify(0.0, 30.0, 20.0);

    // Assert
    assert_eq!(verdict, Verdict::Clean);
}

#[test]
fn score_complexity_one_full_coverage_returns_one() {
    // Arrange
    let formula = CrapFormula::new();
    let complexity = 1;
    let coverage = 1.0;

    // Act
    let score = formula.score(complexity, coverage);

    // Assert
    assert!((score - 1.0).abs() < 0.001);
}

#[test]
fn score_full_coverage_returns_complexity_only() {
    // Arrange
    let formula = CrapFormula::new();
    let complexity = 10;
    let coverage = 1.0;

    // Act
    let score = formula.score(complexity, coverage);

    // Assert
    assert!((score - 10.0).abs() < 0.001);
}

#[test]
fn score_half_coverage_returns_expected_value() {
    // Arrange
    let formula = CrapFormula::new();
    let complexity = 4;
    let coverage = 0.5;

    // Act
    let score = formula.score(complexity, coverage);

    // Assert
    let expected = 16.0 * 0.125 + 4.0;
    assert!((score - expected).abs() < 0.001);
}

#[test]
fn score_zero_complexity_returns_zero() {
    // Arrange
    let formula = CrapFormula::new();
    let complexity = 0;
    let coverage = 0.0;

    // Act
    let score = formula.score(complexity, coverage);

    // Assert
    assert!((score - 0.0).abs() < 0.001);
}

#[test]
fn score_zero_coverage_returns_complexity_squared_plus_complexity() {
    // Arrange
    let formula = CrapFormula::new();
    let complexity = 5;
    let coverage = 0.0;

    // Act
    let score = formula.score(complexity, coverage);

    // Assert
    assert!((score - 30.0).abs() < 0.001);
}
