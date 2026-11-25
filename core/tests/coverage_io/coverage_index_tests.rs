// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crap4rust::analysis::source_function::SourceFunction;
use crap4rust::coverage_io::coverage_index::CoverageIndex;
use crap4rust::coverage_io::coverage_record::CoverageRecord;
use std::collections::BTreeMap;

#[test]
fn from_records_discards_zero_duplicate_when_nonzero_arrives_second() {
    // Arrange
    let records = vec![
        CoverageRecord::new(String::from("src/lib.rs"), 10, 0, 1),
        CoverageRecord::new(String::from("src/lib.rs"), 10, 1, 1),
    ];

    // Act
    let index = CoverageIndex::from_records(records);

    // Assert
    let record = index.get("src/lib.rs", 10).unwrap();
    assert_eq!(record.covered_regions(), 1);
    assert_eq!(record.total_regions(), 1);
}

#[test]
fn from_records_keeps_the_duplicate_with_more_covered_regions() {
    // Arrange
    let records = vec![
        CoverageRecord::new(String::from("src/lib.rs"), 10, 3, 5),
        CoverageRecord::new(String::from("src/lib.rs"), 10, 2, 5),
    ];
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
    let index = CoverageIndex::from_records(records);
    let ratio = index.match_function(&function);

    // Assert
    assert!(ratio.is_some());
    assert!((ratio.unwrap() - 0.6).abs() < 0.001);
}

#[test]
fn from_records_keeps_the_duplicate_with_more_covered_regions_when_it_arrives_second() {
    // Arrange
    let records = vec![
        CoverageRecord::new(String::from("src/lib.rs"), 10, 1, 5),
        CoverageRecord::new(String::from("src/lib.rs"), 10, 3, 5),
    ];

    // Act
    let index = CoverageIndex::from_records(records);

    // Assert
    let record = index.get("src/lib.rs", 10).unwrap();
    assert_eq!(record.covered_regions(), 3);
    assert_eq!(record.total_regions(), 5);
}

#[test]
fn from_records_keeps_the_smaller_of_two_duplicates_that_are_both_zero() {
    // Arrange
    let records = vec![
        CoverageRecord::new(String::from("src/lib.rs"), 10, 0, 3),
        CoverageRecord::new(String::from("src/lib.rs"), 10, 0, 2),
    ];

    // Act
    let index = CoverageIndex::from_records(records);

    // Assert
    let record = index.get("src/lib.rs", 10).unwrap();
    assert_eq!(record.covered_regions(), 0);
    assert_eq!(record.total_regions(), 2);
}

#[test]
fn from_records_of_equally_covered_duplicates_keeps_the_smaller() {
    // Arrange
    let records = vec![
        CoverageRecord::new(String::from("src/lib.rs"), 10, 2, 4),
        CoverageRecord::new(String::from("src/lib.rs"), 10, 2, 3),
    ];

    // Act
    let index = CoverageIndex::from_records(records);

    // Assert
    let record = index.get("src/lib.rs", 10).unwrap();
    assert_eq!(record.covered_regions(), 2);
    assert_eq!(record.total_regions(), 3);
}

#[test]
fn from_records_of_equally_covered_duplicates_keeps_the_smaller_when_it_arrives_first() {
    // Arrange
    let records = vec![
        CoverageRecord::new(String::from("src/lib.rs"), 10, 2, 3),
        CoverageRecord::new(String::from("src/lib.rs"), 10, 2, 4),
    ];

    // Act
    let index = CoverageIndex::from_records(records);

    // Assert
    let record = index.get("src/lib.rs", 10).unwrap();
    assert_eq!(record.covered_regions(), 2);
    assert_eq!(record.total_regions(), 3);
}

#[test]
fn from_records_skips_zero_duplicate_when_nonzero_arrives_first() {
    // Arrange
    let records = vec![
        CoverageRecord::new(String::from("src/lib.rs"), 10, 1, 1),
        CoverageRecord::new(String::from("src/lib.rs"), 10, 0, 1),
    ];

    // Act
    let index = CoverageIndex::from_records(records);

    // Assert
    let record = index.get("src/lib.rs", 10).unwrap();
    assert_eq!(record.covered_regions(), 1);
    assert_eq!(record.total_regions(), 1);
}

#[test]
fn match_function_coverage_aggregates_all_regions_within_span() {
    // Arrange
    let mut index = BTreeMap::new();
    index.insert(
        (String::from("src/lib.rs"), 12),
        CoverageRecord::new(String::from("src/lib.rs"), 12, 2, 4),
    );
    index.insert(
        (String::from("src/lib.rs"), 15),
        CoverageRecord::new(String::from("src/lib.rs"), 15, 3, 6),
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
    let result = CoverageIndex::match_function_coverage(&function, &index);

    // Assert
    assert!(result.is_some());
    let record = result.unwrap();
    assert_eq!(record.covered_regions(), 5);
    assert_eq!(record.total_regions(), 10);
}

#[test]
fn match_function_coverage_exact_match_returns_record() {
    // Arrange
    let mut index = BTreeMap::new();
    index.insert(
        (String::from("src/lib.rs"), 10),
        CoverageRecord::new(String::from("src/lib.rs"), 10, 3, 5),
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
    let result = CoverageIndex::match_function_coverage(&function, &index);

    // Assert
    assert!(result.is_some());
    assert_eq!(result.unwrap().covered_regions(), 3);
}

#[test]
fn match_function_coverage_fuzzy_match_within_span_returns_nearest() {
    // Arrange
    let mut index = BTreeMap::new();
    index.insert(
        (String::from("src/lib.rs"), 12),
        CoverageRecord::new(String::from("src/lib.rs"), 12, 7, 10),
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
    let result = CoverageIndex::match_function_coverage(&function, &index);

    // Assert
    assert!(result.is_some());
    assert_eq!(result.unwrap().covered_regions(), 7);
}

#[test]
fn match_function_coverage_no_match_returns_none() {
    // Arrange
    let mut index = BTreeMap::new();
    index.insert(
        (String::from("src/other.rs"), 10),
        CoverageRecord::new(String::from("src/other.rs"), 10, 1, 1),
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
    let result = CoverageIndex::match_function_coverage(&function, &index);

    // Assert
    assert!(result.is_none());
}
