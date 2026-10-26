// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::support::in_memory_coverage_source::InMemoryCoverageSource;
use crate::support::in_memory_coverage_source::export_of;
use crate::support::in_memory_coverage_source::exported_function;
use crate::support::verbatim_path_keys::VerbatimPathKeys;
use crap4rust::coverage_io::export_coverage_provider::ExportCoverageProvider;
use crap4rust::traits::coverage_provider::CoverageProvider;

#[test]
fn provide_counts_covered_regions() {
    // Arrange
    let source = InMemoryCoverageSource::new(export_of(vec![vec![exported_function(
        &["src/lib.rs"],
        &[
            &[10, 1, 15, 2, 5, 0, 0, 0],
            &[16, 1, 18, 2, 0, 0, 0, 0],
            &[19, 1, 22, 2, 3, 0, 0, 0],
        ],
    )]]));

    // Act
    let records = ExportCoverageProvider::new(&source, &VerbatimPathKeys)
        .provide(&[])
        .expect("load coverage records");

    // Assert
    assert_eq!(records.len(), 1);
    assert_eq!(records[0].total_regions(), 3);
    assert_eq!(records[0].covered_regions(), 2);
}

#[test]
fn provide_handles_multiple_data_chunks() {
    // Arrange
    let source = InMemoryCoverageSource::new(export_of(vec![
        vec![exported_function(
            &["src/lib.rs"],
            &[&[10, 1, 20, 2, 5, 0, 0, 0]],
        )],
        vec![exported_function(
            &["src/other.rs"],
            &[&[30, 1, 50, 2, 3, 0, 0, 0]],
        )],
    ]));

    // Act
    let records = ExportCoverageProvider::new(&source, &VerbatimPathKeys)
        .provide(&[])
        .expect("load coverage records");

    // Assert
    assert_eq!(records.len(), 2);
}

#[test]
fn provide_keys_each_record_by_its_file_and_first_line() {
    // Arrange
    let source = InMemoryCoverageSource::new(export_of(vec![vec![exported_function(
        &["src/lib.rs"],
        &[&[10, 1, 20, 2, 5, 0, 0, 0]],
    )]]));

    // Act
    let records = ExportCoverageProvider::new(&source, &VerbatimPathKeys)
        .provide(&[])
        .expect("load coverage records");

    // Assert
    assert_eq!(records.len(), 1);
    assert!(records[0].path_key().contains("src/lib.rs"));
    assert_eq!(records[0].line(), 10);
}

#[test]
fn provide_skips_a_function_whose_first_region_is_short() {
    // Arrange
    let source = InMemoryCoverageSource::new(export_of(vec![vec![
        exported_function(&["src/lib.rs"], &[&[10, 1, 20, 2]]),
        exported_function(&["src/lib.rs"], &[]),
    ]]));

    // Act
    let records = ExportCoverageProvider::new(&source, &VerbatimPathKeys)
        .provide(&[])
        .expect("load coverage records");

    // Assert
    assert!(records.is_empty());
}

#[test]
fn provide_skips_functions_without_filenames() {
    // Arrange
    let source = InMemoryCoverageSource::new(export_of(vec![vec![exported_function(
        &[],
        &[&[10, 1, 20, 2, 5, 0, 0, 0]],
    )]]));

    // Act
    let records = ExportCoverageProvider::new(&source, &VerbatimPathKeys)
        .provide(&[])
        .expect("load coverage records");

    // Assert
    assert!(records.is_empty());
}

#[test]
fn provide_with_a_source_that_fails_returns_its_failure() {
    // Arrange
    let source = InMemoryCoverageSource::failing();

    // Act
    let result = ExportCoverageProvider::new(&source, &VerbatimPathKeys).provide(&[]);

    // Assert
    let failure = result.expect_err("the source failed");
    assert_eq!(failure.to_string(), "not held");
}
