// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crap4rust::analysis::source_path::SourcePath;
use crap4rust::invocation::package_context::PackageContext;
use crap4rust_cli::invocation::cargo_config::CargoConfig;
use crap4rust_cli::system::llvm_cov_builder::LlvmCovBuilder;
use std::path::{Path, PathBuf};
use tempfile::TempDir;

fn bare_config() -> CargoConfig {
    CargoConfig::default()
}

fn builder() -> LlvmCovBuilder {
    LlvmCovBuilder::new(Path::new("out.json"))
}

fn package(name: &str) -> PackageContext {
    PackageContext {
        name: String::from(name),
        manifest_dir: SourcePath::new(String::from(".")),
        workspace_root: SourcePath::new(String::from(".")),
        source_roots: vec![],
        include_test_targets: false,
        exclude_paths: vec![],
    }
}

#[test]
fn add_packages_forwards_each_package_by_name() {
    // Arrange
    let packages = [package("alpha"), package("beta")];

    // Act
    let arguments = builder().add_packages(&packages).arguments();

    // Assert
    let expected: Vec<String> = ["--package", "alpha", "--package", "beta"]
        .into_iter()
        .map(String::from)
        .collect();
    assert!(arguments.ends_with(&expected), "got {arguments:?}");
}

#[test]
fn apply_config_with_a_manifest_path_forwards_it() {
    // Arrange
    let config = CargoConfig {
        manifest_path: Some(PathBuf::from("core").join("Cargo.toml")),
        ..bare_config()
    };

    // Act
    let arguments = builder().apply_config(&config).arguments();

    // Assert
    let manifest_path = PathBuf::from("core").join("Cargo.toml");
    assert_eq!(
        arguments[4..],
        [
            String::from("--manifest-path"),
            manifest_path.to_string_lossy().into_owned()
        ]
    );
}

#[test]
fn apply_config_with_all_features_forwards_the_flag() {
    // Arrange
    let config = CargoConfig {
        all_features: true,
        ..bare_config()
    };

    // Act
    let arguments = builder().apply_config(&config).arguments();

    // Assert
    assert_eq!(arguments[4..], ["--all-features"]);
}

#[test]
fn apply_config_with_every_option_set_forwards_them_in_declaration_order() {
    // Arrange
    let config = CargoConfig {
        manifest_path: Some(PathBuf::from("Cargo.toml")),
        features: Some(String::from("demo-feature")),
        all_features: true,
        no_default_features: true,
        ..bare_config()
    };

    // Act
    let arguments = builder().apply_config(&config).arguments();

    // Assert
    assert_eq!(
        arguments[4..],
        [
            "--manifest-path",
            "Cargo.toml",
            "--features",
            "demo-feature",
            "--all-features",
            "--no-default-features"
        ]
    );
}

#[test]
fn apply_config_with_features_forwards_them() {
    // Arrange
    let config = CargoConfig {
        features: Some(String::from("demo-feature")),
        ..bare_config()
    };

    // Act
    let arguments = builder().apply_config(&config).arguments();

    // Assert
    assert_eq!(arguments[4..], ["--features", "demo-feature"]);
}

#[test]
fn apply_config_with_no_default_features_forwards_the_flag() {
    // Arrange
    let config = CargoConfig {
        no_default_features: true,
        ..bare_config()
    };

    // Act
    let arguments = builder().apply_config(&config).arguments();

    // Assert
    assert_eq!(arguments[4..], ["--no-default-features"]);
}

#[test]
fn apply_config_with_nothing_set_adds_no_arguments() {
    // Arrange
    let config = bare_config();

    // Act
    let arguments = builder().apply_config(&config).arguments();

    // Assert
    assert_eq!(arguments, builder().arguments());
}

#[test]
fn builder_new_constructs_without_panicking() {
    // Arrange & Act
    let builder = LlvmCovBuilder::new(Path::new("out.json"));

    // Assert
    // Builder was created successfully — no panic
    drop(builder);
}

// What cargo wrote to stderr is the only account of a failed run, and it reaches
// the caller as the error rather than a bare exit code.
#[test]
fn execute_against_a_manifest_that_does_not_exist_fails() {
    // Arrange
    let dir = TempDir::new().expect("temp dir");
    let config = CargoConfig {
        manifest_path: Some(dir.path().join("Cargo.toml")),
        ..bare_config()
    };

    // Act
    let outcome = LlvmCovBuilder::new(&dir.path().join("out.json"))
        .apply_config(&config)
        .execute();

    // Assert
    assert!(outcome.is_err());
}

#[test]
fn new_puts_the_output_path_behind_the_json_flag() {
    // Arrange & Act
    let arguments = builder().arguments();

    // Assert
    assert_eq!(
        arguments,
        vec!["llvm-cov", "--json", "--output-path", "out.json"]
    );
}
