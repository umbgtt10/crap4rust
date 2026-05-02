// Copyright 2025 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License or Apache License, Version 2.0
// SPDX-License-Identifier: MIT OR Apache-2.0

use std::path::Path;

use tempfile::TempDir;

use crap4rust::coverage::load_coverage_records;
use crap4rust::coverage_index::CoverageIndex;
use crap4rust::model::CoverageRecord;

#[test]
fn load_coverage_records_returns_error_for_nonexistent_file() {
    // Arrange
    let path = Path::new("/nonexistent/coverage.json");

    // Act
    let result = load_coverage_records(path);

    // Assert
    assert!(result.is_err());
}

#[test]
fn load_coverage_records_parses_valid_json() {
    // Arrange
    let dir = TempDir::new().expect("temp dir");
    let path = dir.path().join("coverage.json");
    let json = r#"{
        "data": [
            {
                "functions": [
                    {
                        "filenames": ["src/lib.rs"],
                        "regions": [[10, 1, 20, 2, 5, 0, 0, 0]]
                    }
                ]
            }
        ]
    }"#;
    std::fs::write(&path, json).expect("write coverage file");

    // Act
    let records = load_coverage_records(&path).expect("load coverage records");

    // Assert
    assert_eq!(records.len(), 1);
    assert!(records[0].path_key.contains("src/lib.rs"));
    assert_eq!(records[0].line, 10);
}

#[test]
fn load_coverage_records_counts_covered_regions() {
    // Arrange
    let dir = TempDir::new().expect("temp dir");
    let path = dir.path().join("coverage.json");
    let json = r#"{
        "data": [
            {
                "functions": [
                    {
                        "filenames": ["src/lib.rs"],
                        "regions": [
                            [10, 1, 15, 2, 5, 0, 0, 0],
                            [16, 1, 18, 2, 0, 0, 0, 0],
                            [19, 1, 22, 2, 3, 0, 0, 0]
                        ]
                    }
                ]
            }
        ]
    }"#;
    std::fs::write(&path, json).expect("write coverage file");

    // Act
    let records = load_coverage_records(&path).expect("load coverage records");

    // Assert
    assert_eq!(records.len(), 1);
    assert_eq!(records[0].total_regions, 3);
    assert_eq!(records[0].covered_regions, 2);
}

#[test]
fn load_coverage_records_skips_functions_without_filenames() {
    // Arrange
    let dir = TempDir::new().expect("temp dir");
    let path = dir.path().join("coverage.json");
    let json = r#"{
        "data": [
            {
                "functions": [
                    {
                        "filenames": [],
                        "regions": [[10, 1, 20, 2, 5, 0, 0, 0]]
                    }
                ]
            }
        ]
    }"#;
    std::fs::write(&path, json).expect("write coverage file");

    // Act
    let records = load_coverage_records(&path).expect("load coverage records");

    // Assert
    assert!(records.is_empty());
}

#[test]
fn load_coverage_records_handles_multiple_data_chunks() {
    // Arrange
    let dir = TempDir::new().expect("temp dir");
    let path = dir.path().join("coverage.json");
    let json = r#"{
        "data": [
            {
                "functions": [
                    {
                        "filenames": ["src/lib.rs"],
                        "regions": [[10, 1, 20, 2, 5, 0, 0, 0]]
                    }
                ]
            },
            {
                "functions": [
                    {
                        "filenames": ["src/other.rs"],
                        "regions": [[30, 1, 50, 2, 3, 0, 0, 0]]
                    }
                ]
            }
        ]
    }"#;
    std::fs::write(&path, json).expect("write coverage file");

    // Act
    let records = load_coverage_records(&path).expect("load coverage records");

    // Assert
    assert_eq!(records.len(), 2);
}

#[test]
fn from_records_skips_zero_duplicate_when_nonzero_exists() {
    // Arrange
    let records = vec![
        CoverageRecord {
            path_key: String::from("src/lib.rs"),
            line: 10,
            covered_regions: 1,
            total_regions: 1,
        },
        CoverageRecord {
            path_key: String::from("src/lib.rs"),
            line: 10,
            covered_regions: 0,
            total_regions: 1,
        },
    ];

    // Act
    let index = CoverageIndex::from_records(records);

    // Assert — find the function in the index
    let record = index.get("src/lib.rs", 10).unwrap();
    assert_eq!(record.covered_regions, 1);
    assert_eq!(record.total_regions, 1);
}

#[test]
fn load_coverage_records_returns_error_for_invalid_json() {
    // Arrange
    let dir = TempDir::new().expect("temp dir");
    let path = dir.path().join("coverage.json");
    std::fs::write(&path, "not valid json").expect("write coverage file");

    // Act
    let result = load_coverage_records(&path);

    // Assert
    assert!(result.is_err());
}
