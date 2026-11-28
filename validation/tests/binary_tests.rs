// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use std::env::consts::EXE_SUFFIX;
use std::fs;
use std::panic::catch_unwind;
use tempfile::TempDir;
use validation::binary::Binary;
use validation::profile_constants::PROFILE;
use validation::target_constants::TARGET_DIRECTORY;

#[test]
fn path_of_a_binary_not_built_panics_naming_the_file_it_looked_for() {
    // Arrange
    let target_dir = TempDir::new().expect("temp dir");
    let binary = Binary::new(target_dir.path(), "debug");

    // Act & Assert
    let panic = catch_unwind(|| binary.path()).expect_err("a binary not built");
    let message = panic.downcast_ref::<String>().expect("a formatted message");
    let expected = target_dir
        .path()
        .join("debug")
        .join(format!("cargo-crap4rust{EXE_SUFFIX}"));
    assert_eq!(
        message,
        &format!(
            "{} is not built: `cargo build -p cargo-crap4rust` builds it, as `cargo test --workspace` does",
            expected.display()
        )
    );
}

#[test]
fn path_of_a_built_binary_returns_it_under_the_profile_directory_of_the_target_directory() {
    // Arrange
    let target_dir = TempDir::new().expect("temp dir");
    let profile_dir = target_dir.path().join("release");
    fs::create_dir_all(&profile_dir).expect("profile dir");
    let built = profile_dir.join(format!("cargo-crap4rust{EXE_SUFFIX}"));
    fs::write(&built, b"").expect("binary file");
    let binary = Binary::new(target_dir.path(), "release");

    // Act
    let path = binary.path();

    // Assert
    assert_eq!(path, built);
}

#[test]
fn path_of_the_binary_these_tests_run_returns_a_file_cargo_built() {
    // Arrange
    let binary = Binary::new(&TARGET_DIRECTORY, PROFILE);

    // Act
    let path = binary.path();

    // Assert
    assert!(path.is_file());
}
