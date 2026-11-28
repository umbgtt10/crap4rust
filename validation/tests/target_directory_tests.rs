// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use std::fs;
use std::panic::catch_unwind;
use std::path::PathBuf;
use tempfile::TempDir;
use validation::target_directory::TargetDirectory;

fn panic_message(target_directory: &TargetDirectory) -> String {
    let panic = catch_unwind(|| target_directory.locate()).expect_err("a failed lookup");
    panic
        .downcast_ref::<String>()
        .expect("a formatted message")
        .clone()
}

fn scratch_package(cargo_config: &str) -> TempDir {
    let package = TempDir::new().expect("temp dir");
    fs::write(
        package.path().join("Cargo.toml"),
        "[package]\nname = \"scratch\"\nversion = \"0.1.0\"\nedition = \"2024\"\n",
    )
    .expect("manifest");
    fs::create_dir_all(package.path().join("src")).expect("src dir");
    fs::write(package.path().join("src").join("lib.rs"), "").expect("lib.rs");
    fs::create_dir_all(package.path().join(".cargo")).expect(".cargo dir");
    fs::write(
        package.path().join(".cargo").join("config.toml"),
        cargo_config,
    )
    .expect("cargo config");
    package
}

#[test]
fn locate_for_a_manifest_that_does_not_exist_panics_naming_it() {
    // Arrange
    let package = TempDir::new().expect("temp dir");
    let manifest_path = package.path().join("Cargo.toml");
    let target_directory =
        TargetDirectory::new(PathBuf::from(env!("CARGO")), manifest_path.clone());

    // Act
    let message = panic_message(&target_directory);

    // Assert
    assert!(message.starts_with(&format!(
        "cargo metadata failed for {}: ",
        manifest_path.display()
    )));
}

#[test]
fn locate_for_a_package_whose_build_directory_is_moved_returns_its_target_directory() {
    // Arrange
    let package =
        scratch_package("[build]\ntarget-dir = \"artefacts\"\nbuild-dir = \"intermediate\"\n");
    let target_directory = TargetDirectory::new(
        PathBuf::from(env!("CARGO")),
        package.path().join("Cargo.toml"),
    );

    // Act
    let located = target_directory.locate();

    // Assert
    assert_eq!(located, package.path().join("artefacts"));
}

#[test]
fn locate_with_a_cargo_that_cannot_run_panics_naming_it() {
    // Arrange
    let package = scratch_package("");
    let cargo = package.path().join("no-such-cargo");
    let target_directory = TargetDirectory::new(cargo.clone(), package.path().join("Cargo.toml"));

    // Act
    let message = panic_message(&target_directory);

    // Assert
    assert!(message.starts_with(&format!("{} could not run: ", cargo.display())));
}
