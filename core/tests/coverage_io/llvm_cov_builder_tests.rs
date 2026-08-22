// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crap4rust::coverage_io::llvm_cov_builder::LlvmCovBuilder;
use crap4rust::invocation::config::Config;
use crap4rust::reporting::output_format::OutputFormat;
use std::path::{Path, PathBuf};

fn bare_config() -> Config {
    Config {
        coverage_path: None,
        manifest_path: None,
        packages: vec![],
        features: None,
        all_features: false,
        no_default_features: false,
        include_test_targets: false,
        exclude_paths: vec![],
        threshold: 30.0,
        warn_threshold: 20.0,
        project_threshold: 5.0,
        strict: true,
        warn_only: false,
        output_format: OutputFormat::Human,
    }
}

fn builder() -> LlvmCovBuilder {
    LlvmCovBuilder::new(Path::new("out.json"))
}

#[test]
fn apply_config_with_a_manifest_path_forwards_it() {
    // Arrange
    let config = Config {
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
    let config = Config {
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
    let config = Config {
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
    let config = Config {
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
    let config = Config {
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
