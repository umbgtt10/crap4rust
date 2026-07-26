// Copyright 2025 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License or Apache License, Version 2.0
// SPDX-License-Identifier: MIT OR Apache-2.0

use tempfile::TempDir;

use crap4rust::cli::Args;
use crap4rust::config::Config;
use crap4rust::llvm_cov_provider::LlvmCovProvider;
use crap4rust::traits::coverage_provider::CoverageProvider;

#[test]
fn provide_reads_precomputed_coverage_file_without_generating() {
    // Arrange
    let dir = TempDir::new().expect("temp dir");
    let coverage_path = dir.path().join("coverage.json");
    std::fs::write(
        &coverage_path,
        r#"{"data":[{"functions":[{"filenames":["src/lib.rs"],"regions":[[10,1,20,2,1,0,0,0]]}]}]}"#,
    )
    .expect("write coverage file");
    let args = Args::parse_from_args(["crap4rust", "--coverage", &coverage_path.to_string_lossy()]);
    let config = Config::from_args(args);
    let provider = LlvmCovProvider::new();

    // Act
    let records = provider
        .provide(&config, &[])
        .expect("provide coverage records");

    // Assert
    assert_eq!(records.len(), 1);
    assert_eq!(records[0].line, 10);
}
