// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crap4rust::coverage_io::export::Export;
use crap4rust::coverage_io::export_coverage_provider::ExportCoverageProvider;
use crap4rust::traits::coverage_provider::CoverageProvider;
use crap4rust::traits::coverage_source::CoverageSource;
use crap4rust_cli::invocation::cargo_config::CargoConfig;
use crap4rust_cli::system::canonical_path_keys::CanonicalPathKeys;
use crap4rust_cli::system::llvm_cov_source::LlvmCovSource;
use std::fs;
use std::path::{Path, PathBuf};
use tempfile::TempDir;

fn export_of_json(json: &str) -> Export {
    let dir = TempDir::new().expect("temp dir");
    let path = dir.path().join("coverage.json");
    fs::write(&path, json).expect("write coverage file");
    source(path).export(&[]).expect("deserialize export")
}

fn source(coverage_path: PathBuf) -> LlvmCovSource {
    LlvmCovSource::new(CargoConfig {
        coverage_path: Some(coverage_path),
        ..CargoConfig::default()
    })
}

#[test]
fn export_deserializes_empty_data_array() {
    // Arrange
    let json = r#"{"data": []}"#;

    // Act
    let export = export_of_json(json);

    // Assert
    assert!(export.data.is_empty());
}

#[test]
fn export_deserializes_multiple_functions_in_one_chunk() {
    // Arrange
    let json = r#"{
        "data": [
            {
                "functions": [
                    {"filenames": ["src/a.rs"], "regions": [[1, 1, 2, 2, 1, 0, 0, 0]]},
                    {"filenames": ["src/b.rs"], "regions": [[3, 1, 4, 2, 0, 0, 0, 0]]}
                ]
            }
        ]
    }"#;

    // Act
    let export = export_of_json(json);

    // Assert
    assert_eq!(export.data.len(), 1);
    assert_eq!(export.data[0].functions.len(), 2);
}

#[test]
fn export_function_deserializes_multiple_filenames() {
    // Arrange
    let json = r#"{
        "data": [
            {
                "functions": [
                    {"filenames": ["src/a.rs", "src/a_alias.rs"], "regions": [[1, 1, 2, 2, 1, 0, 0, 0]]}
                ]
            }
        ]
    }"#;

    // Act
    let export = export_of_json(json);

    // Assert
    assert_eq!(
        export.data[0].functions[0].filenames,
        vec!["src/a.rs", "src/a_alias.rs"]
    );
}

#[test]
fn export_returns_error_for_invalid_json() {
    // Arrange
    let dir = TempDir::new().expect("temp dir");
    let path = dir.path().join("coverage.json");
    fs::write(&path, "not valid json").expect("write coverage file");

    // Act
    let result = source(path).export(&[]);

    // Assert
    assert!(result.is_err());
}

#[test]
fn export_returns_error_for_nonexistent_file() {
    // Arrange
    let path = Path::new("/nonexistent/coverage.json");

    // Act
    let result = source(path.to_path_buf()).export(&[]);

    // Assert
    assert!(result.is_err());
}

#[test]
fn provide_reads_precomputed_coverage_file_without_generating() {
    // Arrange
    let dir = TempDir::new().expect("temp dir");
    let coverage_path = dir.path().join("coverage.json");
    fs::write(
        &coverage_path,
        r#"{"data":[{"functions":[{"filenames":["src/lib.rs"],"regions":[[10,1,20,2,1,0,0,0]]}]}]}"#,
    )
    .expect("write coverage file");
    let source = source(coverage_path);
    let provider = ExportCoverageProvider::new(&source, &CanonicalPathKeys);

    // Act
    let records = provider.provide(&[]).expect("provide coverage records");

    // Assert
    assert_eq!(records.len(), 1);
    assert_eq!(records[0].line(), 10);
}
