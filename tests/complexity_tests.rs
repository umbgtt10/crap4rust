// Copyright 2025 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License or Apache License, Version 2.0
// SPDX-License-Identifier: MIT OR Apache-2.0

use std::fs;
use std::path::{Path, PathBuf};

use assert_cmd::Command;
use serde_json::json;
use tempfile::TempDir;

const FUNCTION_NAMES: &[&str] = &[
    "empty_function",
    "single_if",
    "nested_if",
    "match_three_arms",
    "for_loop",
    "logical_and_or",
    "try_operator",
];

fn fixture_dir() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("fixtures")
        .join("complexity")
}

fn named_function_line(path: &Path, function_name: &str) -> usize {
    let needle = format!("pub fn {function_name}");

    fs::read_to_string(path)
        .expect("read fixture source")
        .lines()
        .enumerate()
        .find_map(|(index, line)| line.trim_start().starts_with(&needle).then_some(index + 1))
        .expect("fixture source contains the named public function")
}

fn write_coverage_file(temp_dir: &Path, entries: &[(PathBuf, usize, u64)]) -> PathBuf {
    let coverage_path = temp_dir.join("coverage.json");
    let functions = entries
        .iter()
        .map(|(path, line, count)| {
            json!({
                "filenames": [path.canonicalize().expect("canonical source path").to_string_lossy().to_string()],
                "regions": [[*line, 1, *line + 6, 2, *count, 0, 0, 0]],
            })
        })
        .collect::<Vec<_>>();
    let coverage_json = json!({
        "data": [
            {
                "functions": functions,
            }
        ]
    });

    fs::write(
        &coverage_path,
        serde_json::to_vec(&coverage_json).expect("serialize coverage json"),
    )
    .expect("write coverage file");

    coverage_path
}

fn build_coverage_entries(source_path: &Path, count: u64) -> Vec<(PathBuf, usize, u64)> {
    FUNCTION_NAMES
        .iter()
        .map(|name| {
            (
                source_path.to_path_buf(),
                named_function_line(source_path, name),
                count,
            )
        })
        .collect()
}

fn run_report(coverage_path: &Path, threshold: &str) -> String {
    let manifest_path = fixture_dir().join("Cargo.toml");

    let output = Command::cargo_bin("cargo-crap4rust")
        .expect("binary")
        .arg("--manifest-path")
        .arg(&manifest_path)
        .arg("--coverage")
        .arg(coverage_path)
        .arg("--warn-only")
        .arg("--threshold")
        .arg(threshold)
        .arg("--project-threshold")
        .arg("100.0")
        .output()
        .expect("failed to run crap4rust");

    String::from_utf8(output.stdout).expect("non-UTF-8 output")
}

fn extract_report_line<'a>(output: &'a str, function_name: &str) -> Option<&'a str> {
    output.lines().find(|line| {
        line.split_whitespace()
            .nth(1)
            .is_some_and(|field| field == function_name)
    })
}

fn extract_complexity(output: &str, function_name: &str) -> Option<u32> {
    extract_report_line(output, function_name).and_then(|line| {
        line.split_whitespace()
            .nth(4)
            .and_then(|field| field.parse().ok())
    })
}

fn extract_crap_score(output: &str, function_name: &str) -> Option<f64> {
    extract_report_line(output, function_name).and_then(|line| {
        line.split_whitespace()
            .nth(6)
            .and_then(|field| field.parse().ok())
    })
}

#[test]
fn full_coverage_report_runs_successfully_and_contains_expected_functions() {
    // Arrange
    let source_path = fixture_dir().join("src").join("lib.rs");
    let temp_dir = TempDir::new().expect("temp dir");
    let entries = build_coverage_entries(&source_path, 1);
    let coverage_path = write_coverage_file(temp_dir.path(), &entries);

    // Act
    let output = run_report(&coverage_path, "0");

    // Assert
    assert!(output.contains("crap4rust report for complexity-fixture"));
    assert!(output.contains("single_if"));
    assert!(output.contains("nested_if"));
    assert!(output.contains("match_three_arms"));
    assert!(output.contains("for_loop"));
    assert!(output.contains("logical_and_or"));
    assert!(output.contains("try_operator"));
}

#[test]
fn empty_function_excluded_from_report_because_complexity_is_zero() {
    // Arrange
    let source_path = fixture_dir().join("src").join("lib.rs");
    let temp_dir = TempDir::new().expect("temp dir");
    let entries = build_coverage_entries(&source_path, 1);
    let coverage_path = write_coverage_file(temp_dir.path(), &entries);

    // Act
    let output = run_report(&coverage_path, "0");

    // Assert
    assert!(
        extract_report_line(&output, "empty_function").is_none(),
        "empty_function should not appear in report (complexity 0, CRAP 0.0)"
    );
}

#[test]
fn single_if_has_complexity_one() {
    // Arrange
    let source_path = fixture_dir().join("src").join("lib.rs");
    let temp_dir = TempDir::new().expect("temp dir");
    let entries = build_coverage_entries(&source_path, 1);
    let coverage_path = write_coverage_file(temp_dir.path(), &entries);

    // Act
    let output = run_report(&coverage_path, "0");

    // Assert
    assert_eq!(
        extract_complexity(&output, "single_if"),
        Some(1),
        "single if/else should score complexity 1"
    );
}

#[test]
fn nested_if_has_complexity_three_due_to_nesting_increment() {
    // Arrange
    let source_path = fixture_dir().join("src").join("lib.rs");
    let temp_dir = TempDir::new().expect("temp dir");
    let entries = build_coverage_entries(&source_path, 1);
    let coverage_path = write_coverage_file(temp_dir.path(), &entries);

    // Act
    let output = run_report(&coverage_path, "0");

    // Assert
    assert_eq!(
        extract_complexity(&output, "nested_if"),
        Some(3),
        "nested if: outer (1+0) + inner (1+1) = 3"
    );
}

#[test]
fn match_three_arms_has_complexity_one() {
    // Arrange
    let source_path = fixture_dir().join("src").join("lib.rs");
    let temp_dir = TempDir::new().expect("temp dir");
    let entries = build_coverage_entries(&source_path, 1);
    let coverage_path = write_coverage_file(temp_dir.path(), &entries);

    // Act
    let output = run_report(&coverage_path, "0");

    // Assert
    assert_eq!(
        extract_complexity(&output, "match_three_arms"),
        Some(1),
        "match with literal arms should score complexity 1"
    );
}

#[test]
fn for_loop_has_complexity_one() {
    // Arrange
    let source_path = fixture_dir().join("src").join("lib.rs");
    let temp_dir = TempDir::new().expect("temp dir");
    let entries = build_coverage_entries(&source_path, 1);
    let coverage_path = write_coverage_file(temp_dir.path(), &entries);

    // Act
    let output = run_report(&coverage_path, "0");

    // Assert
    assert_eq!(
        extract_complexity(&output, "for_loop"),
        Some(1),
        "simple for loop should score complexity 1"
    );
}

#[test]
fn logical_and_or_has_complexity_two() {
    // Arrange
    let source_path = fixture_dir().join("src").join("lib.rs");
    let temp_dir = TempDir::new().expect("temp dir");
    let entries = build_coverage_entries(&source_path, 1);
    let coverage_path = write_coverage_file(temp_dir.path(), &entries);

    // Act
    let output = run_report(&coverage_path, "0");

    // Assert
    assert_eq!(
        extract_complexity(&output, "logical_and_or"),
        Some(2),
        "&& and || each contribute 1 to complexity"
    );
}

#[test]
fn try_operator_has_complexity_one() {
    // Arrange
    let source_path = fixture_dir().join("src").join("lib.rs");
    let temp_dir = TempDir::new().expect("temp dir");
    let entries = build_coverage_entries(&source_path, 1);
    let coverage_path = write_coverage_file(temp_dir.path(), &entries);

    // Act
    let output = run_report(&coverage_path, "0");

    // Assert
    assert_eq!(
        extract_complexity(&output, "try_operator"),
        Some(1),
        "? operator at nesting 0 should score complexity 1"
    );
}

#[test]
fn full_coverage_crap_score_equals_complexity() {
    // Arrange
    let source_path = fixture_dir().join("src").join("lib.rs");
    let temp_dir = TempDir::new().expect("temp dir");
    let entries = build_coverage_entries(&source_path, 1);
    let coverage_path = write_coverage_file(temp_dir.path(), &entries);

    // Act
    let output = run_report(&coverage_path, "0");

    // Assert
    assert_eq!(extract_crap_score(&output, "nested_if"), Some(3.0));
    assert_eq!(extract_crap_score(&output, "logical_and_or"), Some(2.0));
    assert_eq!(extract_crap_score(&output, "single_if"), Some(1.0));
}

#[test]
fn zero_coverage_amplifies_crap_to_complexity_squared_plus_complexity() {
    // Arrange
    let source_path = fixture_dir().join("src").join("lib.rs");
    let temp_dir = TempDir::new().expect("temp dir");
    let entries = build_coverage_entries(&source_path, 0);
    let coverage_path = write_coverage_file(temp_dir.path(), &entries);

    // Act
    let output = run_report(&coverage_path, "0");

    // Assert
    assert_eq!(
        extract_crap_score(&output, "nested_if"),
        Some(12.0),
        "nested_if: 3^2 + 3 = 12"
    );
    assert_eq!(
        extract_crap_score(&output, "logical_and_or"),
        Some(6.0),
        "logical_and_or: 2^2 + 2 = 6"
    );
    assert_eq!(
        extract_crap_score(&output, "single_if"),
        Some(2.0),
        "single_if: 1^2 + 1 = 2"
    );
}

#[test]
fn nested_control_flow_scores_higher_than_flat_control_flow() {
    // Arrange
    let source_path = fixture_dir().join("src").join("lib.rs");
    let temp_dir = TempDir::new().expect("temp dir");
    let entries = build_coverage_entries(&source_path, 1);
    let coverage_path = write_coverage_file(temp_dir.path(), &entries);

    // Act
    let output = run_report(&coverage_path, "0");

    // Assert
    let nested = extract_complexity(&output, "nested_if").expect("nested_if in report");
    let flat = extract_complexity(&output, "single_if").expect("single_if in report");
    assert!(
        nested > flat,
        "nested_if ({nested}) should have higher complexity than single_if ({flat})"
    );
}
