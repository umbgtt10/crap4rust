// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crap4rust::reporting::output_format::OutputFormat;
use crap4rust_cli::invocation::args::Args;
use std::path::Path;

#[test]
fn cargo_config_keeps_the_all_features_flag() {
    // Arrange
    let args = Args::parse_from_args(["crap4rust", "--all-features"]);

    // Act
    let config = args.cargo_config();

    // Assert
    assert!(config.all_features);
}

#[test]
fn cargo_config_keeps_the_coverage_path() {
    // Arrange
    let args = Args::parse_from_args(["crap4rust", "--coverage", "coverage.json"]);

    // Act
    let config = args.cargo_config();

    // Assert
    assert_eq!(
        config.coverage_path.as_deref(),
        Some(Path::new("coverage.json"))
    );
}

#[test]
fn cargo_config_keeps_the_features() {
    // Arrange
    let args = Args::parse_from_args(["crap4rust", "--features", "demo-feature"]);

    // Act
    let config = args.cargo_config();

    // Assert
    assert_eq!(config.features.as_deref(), Some("demo-feature"));
}

#[test]
fn cargo_config_keeps_the_manifest_path() {
    // Arrange
    let args = Args::parse_from_args(["crap4rust", "--manifest-path", "Cargo.toml"]);

    // Act
    let config = args.cargo_config();

    // Assert
    assert_eq!(
        config.manifest_path.as_deref(),
        Some(Path::new("Cargo.toml"))
    );
}

#[test]
fn cargo_config_keeps_the_no_default_features_flag() {
    // Arrange
    let args = Args::parse_from_args(["crap4rust", "--no-default-features"]);

    // Act
    let config = args.cargo_config();

    // Assert
    assert!(config.no_default_features);
}

#[test]
fn config_carries_the_selection_options() {
    // Arrange
    let args = Args::parse_from_args([
        "crap4rust",
        "--include-test-targets",
        "--exclude-path",
        "benches",
        "--project-threshold",
        "9",
    ]);

    // Act
    let config = args.config();

    // Assert
    assert!(config.include_test_targets());
    assert_eq!(config.exclude_paths(), ["benches"]);
    assert_eq!(config.project_threshold(), 9.0);
}

#[test]
fn config_preserves_packages() {
    // Arrange
    let args = Args::parse_from_args(["crap4rust", "--package", "foo", "--package", "bar"]);

    // Act
    let config = args.config();

    // Assert
    assert_eq!(config.packages(), vec!["foo", "bar"]);
}

#[test]
fn config_preserves_strict_and_warn_only() {
    // Arrange
    let args = Args::parse_from_args(["crap4rust", "--strict", "--warn-only"]);

    // Act
    let config = args.config();

    // Assert
    assert!(config.strict());
    assert!(config.warn_only());
}

#[test]
fn config_preserves_threshold_fields() {
    // Arrange
    let args = Args::parse_from_args(["crap4rust", "--threshold", "42", "--warn-threshold", "7"]);

    // Act
    let config = args.config();

    // Assert
    assert_eq!(config.threshold(), 42.0);
    assert_eq!(config.warn_threshold(), 7.0);
}

#[test]
fn parse_args_all_features() {
    // Arrange & Act
    let args = Args::parse_from_args(["crap4rust", "--all-features"]);

    // Assert
    assert!(args.all_features());
}

#[test]
fn parse_args_coverage() {
    // Arrange & Act
    let args = Args::parse_from_args(["crap4rust", "--coverage", "cov.json"]);

    // Assert
    assert_eq!(args.coverage().unwrap().to_string_lossy(), "cov.json");
}

#[test]
fn parse_args_defaults() {
    // Arrange & Act
    let args = Args::parse_from_args(["crap4rust"]);

    // Assert
    assert_eq!(args.threshold(), 30.0);
    assert_eq!(args.warn_threshold(), 20.0);
    assert!(!args.strict());
    assert!(!args.warn_only());
    assert!(!args.all_features());
    assert!(!args.no_default_features());
    assert!(!args.include_test_targets());
    assert!(args.coverage().is_none());
    assert!(args.manifest_path().is_none());
    assert!(args.features().is_none());
    assert!(args.package().is_empty());
    assert!(args.exclude_path().is_empty());
}

#[test]
fn parse_args_exclude_path() {
    // Arrange & Act
    let args = Args::parse_from_args(["crap4rust", "--exclude-path", "tests/fixtures"]);

    // Assert
    assert_eq!(args.exclude_path(), vec!["tests/fixtures"]);
}

#[test]
fn parse_args_features() {
    // Arrange & Act
    let args = Args::parse_from_args(["crap4rust", "--features", "feat1,feat2"]);

    // Assert
    assert_eq!(args.features(), Some("feat1,feat2"));
}

#[test]
fn parse_args_include_test_targets() {
    // Arrange & Act
    let args = Args::parse_from_args(["crap4rust", "--include-test-targets"]);

    // Assert
    assert!(args.include_test_targets());
}

#[test]
fn parse_args_manifest_path() {
    // Arrange & Act
    let args = Args::parse_from_args(["crap4rust", "--manifest-path", "Cargo.toml"]);

    // Assert
    assert_eq!(
        args.manifest_path().unwrap().to_string_lossy(),
        "Cargo.toml"
    );
}

#[test]
fn parse_args_no_default_features() {
    // Arrange & Act
    let args = Args::parse_from_args(["crap4rust", "--no-default-features"]);

    // Assert
    assert!(args.no_default_features());
}

#[test]
fn parse_args_output_format_json() {
    // Arrange & Act
    let args = Args::parse_from_args(["crap4rust", "--output-format", "json"]);

    // Assert
    assert_eq!(args.output_format(), OutputFormat::Json);
}

#[test]
fn parse_args_package() {
    // Arrange & Act
    let args = Args::parse_from_args(["crap4rust", "--package", "foo", "--package", "bar"]);

    // Assert
    assert_eq!(args.package(), vec!["foo", "bar"]);
}

#[test]
fn parse_args_project_threshold() {
    // Arrange & Act
    let args = Args::parse_from_args(["crap4rust", "--project-threshold", "10.0"]);

    // Assert
    assert_eq!(args.project_threshold(), 10.0);
}

#[test]
fn parse_args_strict() {
    // Arrange & Act
    let args = Args::parse_from_args(["crap4rust", "--strict"]);

    // Assert
    assert!(args.strict());
}

#[test]
fn parse_args_threshold() {
    // Arrange & Act
    let args = Args::parse_from_args(["crap4rust", "--threshold", "15"]);

    // Assert
    assert_eq!(args.threshold(), 15.0);
}

#[test]
fn parse_args_warn_only() {
    // Arrange & Act
    let args = Args::parse_from_args(["crap4rust", "--warn-only"]);

    // Assert
    assert!(args.warn_only());
}

#[test]
fn parse_args_warn_threshold() {
    // Arrange & Act
    let args = Args::parse_from_args(["crap4rust", "--warn-threshold", "6.0"]);

    // Assert
    assert_eq!(args.warn_threshold(), 6.0);
}
