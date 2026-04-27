// Copyright 2025 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License or Apache License, Version 2.0
// SPDX-License-Identifier: MIT OR Apache-2.0

use std::collections::HashMap;

use crap4rust::app::{classify, compute_crap_score, match_function_coverage, project_fails};
use crap4rust::cli::OutputFormat;
use crap4rust::model::{Config, CoverageRecord, SourceFunction, Verdict};

#[test]
fn compute_crap_score_zero_coverage_returns_complexity_squared_plus_complexity() {
    // Arrange
    let complexity = 5;
    let coverage = 0.0;

    // Act
    let score = compute_crap_score(complexity, coverage);

    // Assert
    assert!((score - 30.0).abs() < 0.001);
}

#[test]
fn compute_crap_score_full_coverage_returns_complexity_only() {
    // Arrange
    let complexity = 10;
    let coverage = 1.0;

    // Act
    let score = compute_crap_score(complexity, coverage);

    // Assert
    assert!((score - 10.0).abs() < 0.001);
}

#[test]
fn compute_crap_score_half_coverage_returns_expected_value() {
    // Arrange
    let complexity = 4;
    let coverage = 0.5;

    // Act
    let score = compute_crap_score(complexity, coverage);

    // Assert
    let expected = 16.0 * 0.125 + 4.0;
    assert!((score - expected).abs() < 0.001);
}

#[test]
fn compute_crap_score_zero_complexity_returns_zero() {
    // Arrange
    let complexity = 0;
    let coverage = 0.0;

    // Act
    let score = compute_crap_score(complexity, coverage);

    // Assert
    assert!((score - 0.0).abs() < 0.001);
}

#[test]
fn compute_crap_score_complexity_one_full_coverage_returns_one() {
    // Arrange
    let complexity = 1;
    let coverage = 1.0;

    // Act
    let score = compute_crap_score(complexity, coverage);

    // Assert
    assert!((score - 1.0).abs() < 0.001);
}

#[test]
fn classify_above_threshold_returns_crappy() {
    // Arrange & Act
    let verdict = classify(31.0, 30.0, 20.0);

    // Assert
    assert_eq!(verdict, Verdict::Crappy);
}

#[test]
fn classify_at_threshold_returns_warn() {
    // Arrange & Act
    let verdict = classify(30.0, 30.0, 20.0);

    // Assert
    assert_eq!(verdict, Verdict::Warn);
}

#[test]
fn classify_between_warn_and_threshold_returns_warn() {
    // Arrange & Act
    let verdict = classify(25.0, 30.0, 20.0);

    // Assert
    assert_eq!(verdict, Verdict::Warn);
}

#[test]
fn classify_at_warn_threshold_returns_warn() {
    // Arrange & Act
    let verdict = classify(20.0, 30.0, 20.0);

    // Assert
    assert_eq!(verdict, Verdict::Warn);
}

#[test]
fn classify_below_warn_threshold_returns_clean() {
    // Arrange & Act
    let verdict = classify(19.9, 30.0, 20.0);

    // Assert
    assert_eq!(verdict, Verdict::Clean);
}

#[test]
fn classify_zero_score_returns_clean() {
    // Arrange & Act
    let verdict = classify(0.0, 30.0, 20.0);

    // Assert
    assert_eq!(verdict, Verdict::Clean);
}

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
fn match_function_coverage_exact_match_returns_record() {
    // Arrange
    let mut index = HashMap::new();
    index.insert(
        (String::from("src/lib.rs"), 10),
        CoverageRecord {
            path_key: String::from("src/lib.rs"),
            line: 10,
            covered_regions: 3,
            total_regions: 5,
        },
    );
    let function = SourceFunction {
        package_name: String::from("test"),
        name: String::from("foo"),
        path_key: String::from("src/lib.rs"),
        relative_file: String::from("src/lib.rs"),
        line: 10,
        end_line: 20,
        complexity: 1,
    };

    // Act
    let result = match_function_coverage(&function, &index);

    // Assert
    assert!(result.is_some());
    assert_eq!(result.unwrap().covered_regions, 3);
}

#[test]
fn match_function_coverage_fuzzy_match_within_span_returns_nearest() {
    // Arrange
    let mut index = HashMap::new();
    index.insert(
        (String::from("src/lib.rs"), 12),
        CoverageRecord {
            path_key: String::from("src/lib.rs"),
            line: 12,
            covered_regions: 7,
            total_regions: 10,
        },
    );
    let function = SourceFunction {
        package_name: String::from("test"),
        name: String::from("foo"),
        path_key: String::from("src/lib.rs"),
        relative_file: String::from("src/lib.rs"),
        line: 10,
        end_line: 20,
        complexity: 1,
    };

    // Act
    let result = match_function_coverage(&function, &index);

    // Assert
    assert!(result.is_some());
    assert_eq!(result.unwrap().covered_regions, 7);
}

#[test]
fn match_function_coverage_no_match_returns_none() {
    // Arrange
    let mut index = HashMap::new();
    index.insert(
        (String::from("src/other.rs"), 10),
        CoverageRecord {
            path_key: String::from("src/other.rs"),
            line: 10,
            covered_regions: 1,
            total_regions: 1,
        },
    );
    let function = SourceFunction {
        package_name: String::from("test"),
        name: String::from("foo"),
        path_key: String::from("src/lib.rs"),
        relative_file: String::from("src/lib.rs"),
        line: 10,
        end_line: 20,
        complexity: 1,
    };

    // Act
    let result = match_function_coverage(&function, &index);

    // Assert
    assert!(result.is_none());
}

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
fn project_fails_strict_with_one_crappy_returns_true() {
    // Arrange
    let config = test_config();

    // Act
    let result = project_fails(1, 0.5, &config);

    // Assert
    assert!(result);
}

#[test]
fn project_fails_non_strict_below_threshold_returns_false() {
    // Arrange
    let mut config = test_config();
    config.strict = false;
    config.project_threshold = 5.0;

    // Act
    let result = project_fails(1, 4.9, &config);

    // Assert
    assert!(!result);
}

#[test]
fn project_fails_non_strict_above_threshold_returns_true() {
    // Arrange
    let mut config = test_config();
    config.strict = false;

    // Act
    let result = project_fails(2, 5.1, &config);

    // Assert
    assert!(result);
}
