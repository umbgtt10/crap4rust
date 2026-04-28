// Copyright 2025 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License or Apache License, Version 2.0
// SPDX-License-Identifier: MIT OR Apache-2.0

use crap4rust::cli::OutputFormat;
use crap4rust::manifest::resolve_packages;
use crap4rust::model::Config;

fn test_config() -> Config {
    Config {
        coverage_path: None,
        manifest_path: Some(std::path::PathBuf::from("Cargo.toml")),
        packages: vec![String::from("cargo-crap4rust")],
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

#[test]
fn resolve_packages_finds_cargo_crap4rust() {
    // Arrange
    let config = test_config();

    // Act
    let packages = resolve_packages(&config).expect("resolve packages");

    // Assert
    assert_eq!(packages.len(), 1);
    assert_eq!(packages[0].name, "cargo-crap4rust");
}

#[test]
fn resolve_packages_has_source_roots() {
    // Arrange
    let config = test_config();

    // Act
    let packages = resolve_packages(&config).expect("resolve packages");

    // Assert
    assert!(!packages[0].source_roots.is_empty());
}
