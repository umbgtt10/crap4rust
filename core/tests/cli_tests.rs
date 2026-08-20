// Copyright 2025 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crap4rust::cli::Args;
use crap4rust::output_format::OutputFormat;

#[test]
fn parse_args_all_features() {
    // Arrange & Act
    let args = Args::parse_from_args(["crap4rust", "--all-features"]);

    // Assert
    assert!(args.all_features);
}

#[test]
fn parse_args_coverage() {
    // Arrange & Act
    let args = Args::parse_from_args(["crap4rust", "--coverage", "cov.json"]);

    // Assert
    assert_eq!(args.coverage.unwrap().to_string_lossy(), "cov.json");
}

#[test]
fn parse_args_defaults() {
    // Arrange & Act
    let args = Args::parse_from_args(["crap4rust"]);

    // Assert
    assert_eq!(args.threshold, 30.0);
    assert_eq!(args.warn_threshold, 20.0);
    assert!(!args.strict);
    assert!(!args.warn_only);
    assert!(!args.all_features);
    assert!(!args.no_default_features);
    assert!(!args.include_test_targets);
    assert!(args.coverage.is_none());
    assert!(args.manifest_path.is_none());
    assert!(args.features.is_none());
    assert!(args.package.is_empty());
    assert!(args.exclude_path.is_empty());
}

#[test]
fn parse_args_exclude_path() {
    // Arrange & Act
    let args = Args::parse_from_args(["crap4rust", "--exclude-path", "tests/fixtures"]);

    // Assert
    assert_eq!(args.exclude_path, vec!["tests/fixtures"]);
}

#[test]
fn parse_args_features() {
    // Arrange & Act
    let args = Args::parse_from_args(["crap4rust", "--features", "feat1,feat2"]);

    // Assert
    assert_eq!(args.features.as_deref(), Some("feat1,feat2"));
}

#[test]
fn parse_args_include_test_targets() {
    // Arrange & Act
    let args = Args::parse_from_args(["crap4rust", "--include-test-targets"]);

    // Assert
    assert!(args.include_test_targets);
}

#[test]
fn parse_args_manifest_path() {
    // Arrange & Act
    let args = Args::parse_from_args(["crap4rust", "--manifest-path", "Cargo.toml"]);

    // Assert
    assert_eq!(args.manifest_path.unwrap().to_string_lossy(), "Cargo.toml");
}

#[test]
fn parse_args_no_default_features() {
    // Arrange & Act
    let args = Args::parse_from_args(["crap4rust", "--no-default-features"]);

    // Assert
    assert!(args.no_default_features);
}

#[test]
fn parse_args_output_format_json() {
    // Arrange & Act
    let args = Args::parse_from_args(["crap4rust", "--output-format", "json"]);

    // Assert
    assert_eq!(args.output_format, OutputFormat::Json);
}

#[test]
fn parse_args_package() {
    // Arrange & Act
    let args = Args::parse_from_args(["crap4rust", "--package", "foo", "--package", "bar"]);

    // Assert
    assert_eq!(args.package, vec!["foo", "bar"]);
}

#[test]
fn parse_args_project_threshold() {
    // Arrange & Act
    let args = Args::parse_from_args(["crap4rust", "--project-threshold", "10.0"]);

    // Assert
    assert_eq!(args.project_threshold, 10.0);
}

#[test]
fn parse_args_strict() {
    // Arrange & Act
    let args = Args::parse_from_args(["crap4rust", "--strict"]);

    // Assert
    assert!(args.strict);
}

#[test]
fn parse_args_threshold() {
    // Arrange & Act
    let args = Args::parse_from_args(["crap4rust", "--threshold", "15"]);

    // Assert
    assert_eq!(args.threshold, 15.0);
}

#[test]
fn parse_args_warn_only() {
    // Arrange & Act
    let args = Args::parse_from_args(["crap4rust", "--warn-only"]);

    // Assert
    assert!(args.warn_only);
}

#[test]
fn parse_args_warn_threshold() {
    // Arrange & Act
    let args = Args::parse_from_args(["crap4rust", "--warn-threshold", "6.0"]);

    // Assert
    assert_eq!(args.warn_threshold, 6.0);
}
