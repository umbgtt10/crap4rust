// Copyright 2025 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License or Apache License, Version 2.0
// SPDX-License-Identifier: MIT OR Apache-2.0

use std::collections::HashMap;

use crap4rust::coverage_index::match_function_coverage;
use crap4rust::coverage_record::CoverageRecord;
use crap4rust::source_function::SourceFunction;

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

#[test]
fn match_function_coverage_aggregates_all_regions_within_span() {
    // Arrange
    let mut index = HashMap::new();
    index.insert(
        (String::from("src/lib.rs"), 12),
        CoverageRecord {
            path_key: String::from("src/lib.rs"),
            line: 12,
            covered_regions: 2,
            total_regions: 4,
        },
    );
    index.insert(
        (String::from("src/lib.rs"), 15),
        CoverageRecord {
            path_key: String::from("src/lib.rs"),
            line: 15,
            covered_regions: 3,
            total_regions: 6,
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
    let record = result.unwrap();
    assert_eq!(record.covered_regions, 5);
    assert_eq!(record.total_regions, 10);
}

#[test]
fn from_records_aggregates_duplicate_entries() {
    // Arrange
    use crap4rust::coverage_index::CoverageIndex;
    let records = vec![
        CoverageRecord {
            path_key: String::from("src/lib.rs"),
            line: 10,
            covered_regions: 3,
            total_regions: 5,
        },
        CoverageRecord {
            path_key: String::from("src/lib.rs"),
            line: 10,
            covered_regions: 2,
            total_regions: 5,
        },
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
    assert!((ratio.unwrap() - 0.5).abs() < 0.001);
}

#[test]
fn from_records_skips_zero_duplicate_when_nonzero_arrives_first() {
    // Arrange
    use crap4rust::coverage_index::CoverageIndex;
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

    // Assert
    let record = index.get("src/lib.rs", 10).unwrap();
    assert_eq!(record.covered_regions, 1);
    assert_eq!(record.total_regions, 1);
}

#[test]
fn from_records_discards_zero_duplicate_when_nonzero_arrives_second() {
    // Arrange
    use crap4rust::coverage_index::CoverageIndex;
    let records = vec![
        CoverageRecord {
            path_key: String::from("src/lib.rs"),
            line: 10,
            covered_regions: 0,
            total_regions: 1,
        },
        CoverageRecord {
            path_key: String::from("src/lib.rs"),
            line: 10,
            covered_regions: 1,
            total_regions: 1,
        },
    ];

    // Act
    let index = CoverageIndex::from_records(records);

    // Assert
    let record = index.get("src/lib.rs", 10).unwrap();
    assert_eq!(record.covered_regions, 1);
    assert_eq!(record.total_regions, 1);
}
