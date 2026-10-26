// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crap4rust::invocation::cargo_package_resolver::CargoPackageResolver;
use crap4rust::invocation::config::Config;
use crap4rust::invocation::package_context::PackageContext;
use crap4rust::invocation::run_failure::RunFailure;
use crap4rust::traits::package_resolver::PackageResolver;
use crap4rust_cli::system::cargo_workspace_reader::CargoWorkspaceReader;
use std::fs;
use std::path::PathBuf;
use tempfile::TempDir;

fn resolve(manifest_path: PathBuf, config: &Config) -> Result<Vec<PackageContext>, RunFailure> {
    let reader = CargoWorkspaceReader::new(Some(manifest_path));
    CargoPackageResolver::new(&reader).resolve(config)
}

fn solo_package_manifest(temp_dir: &TempDir) -> PathBuf {
    let source_dir = temp_dir.path().join("src");
    fs::create_dir_all(&source_dir).expect("create src dir");
    fs::write(source_dir.join("lib.rs"), "").expect("write lib.rs");
    let manifest_path = temp_dir.path().join("Cargo.toml");
    fs::write(
        &manifest_path,
        "[package]\nname = \"solo\"\nversion = \"0.1.0\"\nedition = \"2021\"\n",
    )
    .expect("write manifest");
    manifest_path
}

fn test_config() -> Config {
    Config::default()
        .with_packages(vec![String::from("cargo-crap4rust")])
        .with_threshold(30.0)
        .with_warn_threshold(20.0)
        .with_project_threshold(5.0)
        .with_strict(true)
}

#[test]
fn resolve_at_a_single_package_manifest_without_packages_selects_the_root() {
    // Arrange
    let temp_dir = TempDir::new().expect("temp dir");
    let manifest_path = solo_package_manifest(&temp_dir);
    let config = test_config().with_packages(vec![]);

    // Act
    let packages = resolve(manifest_path, &config).expect("resolve packages");

    // Assert
    assert_eq!(packages.len(), 1);
    assert_eq!(packages[0].name, "solo");
}

#[test]
fn resolve_at_a_workspace_root_without_packages_selects_every_member() {
    // Arrange
    let manifest_path = PathBuf::from("..").join("Cargo.toml");
    let config = test_config().with_packages(vec![]);

    // Act
    let packages = resolve(manifest_path, &config).expect("resolve packages");

    // Assert
    let mut names = packages
        .iter()
        .map(|package| package.name.to_string())
        .collect::<Vec<_>>();
    names.sort();
    assert_eq!(
        names,
        vec![
            String::from("cargo-crap4rust"),
            String::from("crap4rust-core"),
            String::from("validation"),
            String::from("xtask"),
        ]
    );
}

#[test]
fn resolve_finds_cargo_crap4rust() {
    // Arrange
    let config = test_config();

    // Act
    let packages = resolve(PathBuf::from("Cargo.toml"), &config).expect("resolve packages");

    // Assert
    assert_eq!(packages.len(), 1);
    assert_eq!(packages[0].name, "cargo-crap4rust");
}

#[test]
fn resolve_has_source_roots() {
    // Arrange
    let config = test_config();

    // Act
    let packages = resolve(PathBuf::from("Cargo.toml"), &config).expect("resolve packages");

    // Assert
    assert!(!packages[0].source_roots.is_empty());
}

#[test]
fn resolve_via_dyn_package_resolver_finds_cargo_crap4rust() {
    // Arrange
    let config = test_config();
    let reader = CargoWorkspaceReader::new(Some(PathBuf::from("Cargo.toml")));
    let resolver: Box<dyn PackageResolver> = Box::new(CargoPackageResolver::new(&reader));

    // Act
    let packages = resolver.resolve(&config).expect("resolve packages");

    // Assert
    assert_eq!(packages.len(), 1);
    assert_eq!(packages[0].name, "cargo-crap4rust");
}

#[test]
fn resolve_with_an_unknown_package_returns_an_error() {
    // Arrange
    let config = test_config().with_packages(vec![String::from("no-such-package")]);

    // Act
    let result = resolve(PathBuf::from("Cargo.toml"), &config);

    // Assert
    let error = result.expect_err("unknown package must not resolve");
    assert_eq!(
        format!("{error:#}"),
        "package no-such-package was not found in the manifest"
    );
}

#[test]
fn resolve_with_two_requested_packages_returns_them_in_request_order() {
    // Arrange
    let manifest_path = PathBuf::from("..").join("Cargo.toml");
    let config = test_config().with_packages(vec![
        String::from("validation"),
        String::from("cargo-crap4rust"),
    ]);

    // Act
    let packages = resolve(manifest_path, &config).expect("resolve packages");

    // Assert
    let names = packages
        .iter()
        .map(|package| package.name.to_string())
        .collect::<Vec<_>>();
    assert_eq!(
        names,
        vec![String::from("validation"), String::from("cargo-crap4rust")]
    );
}
