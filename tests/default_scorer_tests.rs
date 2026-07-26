// Copyright 2025 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License or Apache License, Version 2.0
// SPDX-License-Identifier: MIT OR Apache-2.0

use crap4rust::cli::Args;
use crap4rust::config::Config;
use crap4rust::coverage_index::CoverageIndex;
use crap4rust::coverage_record::CoverageRecord;
use crap4rust::default_scorer::DefaultScorer;
use crap4rust::source_function::SourceFunction;
use crap4rust::traits::scorer::Scorer;
use crap4rust::verdict::Verdict;

fn test_config(
    threshold: f64,
    warn_threshold: f64,
    strict: bool,
    project_threshold: f64,
) -> Config {
    let mut args = vec![
        "crap4rust".to_string(),
        "--threshold".to_string(),
        threshold.to_string(),
        "--warn-threshold".to_string(),
        warn_threshold.to_string(),
        "--project-threshold".to_string(),
        project_threshold.to_string(),
    ];
    if strict {
        args.push("--strict".to_string());
    }
    Config::from_args(Args::parse_from_args(args))
}

fn function(name: &str, complexity: u32, line: usize) -> SourceFunction {
    SourceFunction {
        package_name: String::from("pkg"),
        name: String::from(name),
        path_key: String::from("src/lib.rs"),
        relative_file: String::from("src/lib.rs"),
        line,
        end_line: line + 5,
        complexity,
    }
}

fn coverage(entries: &[(usize, u32, u32)]) -> CoverageIndex {
    let records = entries
        .iter()
        .map(|(line, covered, total)| CoverageRecord {
            path_key: String::from("src/lib.rs"),
            line: *line,
            covered_regions: *covered,
            total_regions: *total,
        })
        .collect();
    CoverageIndex::from_records(records)
}

#[test]
fn score_functions_sorts_by_crap_score_descending() {
    // Arrange
    let scorer = DefaultScorer::new();
    let config = test_config(30.0, 20.0, false, 5.0);
    let index = coverage(&[(10, 0, 1), (20, 1, 1)]);
    let functions = vec![function("low_risk", 2, 20), function("high_risk", 10, 10)];

    // Act
    let reports = scorer.score_functions(functions, &index, &config);

    // Assert
    assert_eq!(reports[0].name, "high_risk");
    assert_eq!(reports[1].name, "low_risk");
}

#[test]
fn score_functions_ties_break_by_name_ascending() {
    // Arrange
    let scorer = DefaultScorer::new();
    let config = test_config(30.0, 20.0, false, 5.0);
    let index = coverage(&[(10, 0, 1), (20, 0, 1)]);
    let functions = vec![function("zeta", 5, 20), function("alpha", 5, 10)];

    // Act
    let reports = scorer.score_functions(functions, &index, &config);

    // Assert
    assert_eq!(reports[0].name, "alpha");
    assert_eq!(reports[1].name, "zeta");
}

#[test]
fn score_functions_unmatched_coverage_defaults_to_zero() {
    // Arrange
    let scorer = DefaultScorer::new();
    let config = test_config(30.0, 20.0, false, 5.0);
    let index = coverage(&[]);
    let functions = vec![function("uncovered", 3, 10)];

    // Act
    let reports = scorer.score_functions(functions, &index, &config);

    // Assert
    assert!((reports[0].coverage - 0.0).abs() < 0.001);
}

#[test]
fn project_metrics_all_clean_returns_clean_verdict() {
    // Arrange
    let scorer = DefaultScorer::new();
    let config = test_config(30.0, 20.0, false, 5.0);
    let index = coverage(&[(10, 1, 1)]);
    let reports = scorer.score_functions(vec![function("safe", 1, 10)], &index, &config);

    // Act
    let metrics = scorer.project_metrics(&reports, &config);

    // Assert
    assert_eq!(metrics.verdict, Verdict::Clean);
    assert_eq!(metrics.crappy_functions, 0);
    assert_eq!(metrics.total_functions, 1);
}

#[test]
fn project_metrics_with_warn_function_returns_warn_verdict() {
    // Arrange
    let scorer = DefaultScorer::new();
    let config = test_config(30.0, 20.0, false, 5.0);
    let index = coverage(&[(10, 0, 1)]);
    let reports = scorer.score_functions(vec![function("borderline", 5, 10)], &index, &config);

    // Act
    let metrics = scorer.project_metrics(&reports, &config);

    // Assert
    assert_eq!(metrics.verdict, Verdict::Warn);
}

#[test]
fn project_metrics_exceeding_project_threshold_returns_crappy_verdict() {
    // Arrange
    let scorer = DefaultScorer::new();
    let config = test_config(5.0, 2.0, false, 5.0);
    let index = coverage(&[(10, 0, 1)]);
    let reports = scorer.score_functions(vec![function("risky", 10, 10)], &index, &config);

    // Act
    let metrics = scorer.project_metrics(&reports, &config);

    // Assert
    assert_eq!(metrics.verdict, Verdict::Crappy);
    assert_eq!(metrics.crappy_functions, 1);
}

#[test]
fn project_metrics_computes_crappy_percent() {
    // Arrange
    let scorer = DefaultScorer::new();
    let config = test_config(5.0, 2.0, false, 100.0);
    let index = coverage(&[(10, 0, 1), (20, 1, 1), (30, 1, 1), (40, 1, 1)]);
    let functions = vec![
        function("risky", 10, 10),
        function("ok_one", 1, 20),
        function("ok_two", 1, 30),
        function("ok_three", 1, 40),
    ];
    let reports = scorer.score_functions(functions, &index, &config);

    // Act
    let metrics = scorer.project_metrics(&reports, &config);

    // Assert
    assert_eq!(metrics.total_functions, 4);
    assert_eq!(metrics.crappy_functions, 1);
    assert!((metrics.crappy_percent - 25.0).abs() < 0.001);
}
