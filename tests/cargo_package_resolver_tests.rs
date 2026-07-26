// Copyright 2025 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crap4rust::cargo_package_resolver::CargoPackageResolver;
use crap4rust::config::Config;
use crap4rust::output_format::OutputFormat;
use crap4rust::traits::package_resolver::PackageResolver;

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
fn resolve_finds_cargo_crap4rust() {
    // Arrange
    let config = test_config();
    let resolver = CargoPackageResolver::new();

    // Act
    let packages = resolver.resolve(&config).expect("resolve packages");

    // Assert
    assert_eq!(packages.len(), 1);
    assert_eq!(packages[0].name, "cargo-crap4rust");
}

#[test]
fn resolve_has_source_roots() {
    // Arrange
    let config = test_config();
    let resolver = CargoPackageResolver::new();

    // Act
    let packages = resolver.resolve(&config).expect("resolve packages");

    // Assert
    assert!(!packages[0].source_roots.is_empty());
}

#[test]
fn resolve_via_dyn_package_resolver_finds_cargo_crap4rust() {
    // Arrange
    let config = test_config();
    let resolver: Box<dyn PackageResolver> = Box::new(CargoPackageResolver::new());

    // Act
    let packages = resolver.resolve(&config).expect("resolve packages");

    // Assert
    assert_eq!(packages.len(), 1);
    assert_eq!(packages[0].name, "cargo-crap4rust");
}
