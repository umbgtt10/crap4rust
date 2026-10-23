// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crap4rust::invocation::package_context::PackageContext;
use std::path::Path;
use std::path::PathBuf;

fn sample() -> PackageContext {
    PackageContext::new(
        String::from("cargo-crap4rust"),
        PathBuf::from("/workspace/crap4rust"),
        PathBuf::from("/workspace"),
        vec![PathBuf::from("/workspace/crap4rust/src")],
        false,
        vec![String::from("tests/fixtures")],
    )
}

#[test]
fn clone_produces_an_independent_copy_of_vec_fields() {
    // Arrange
    let package = sample();

    // Act
    let cloned = package.clone();

    // Assert
    assert_eq!(cloned.source_roots(), package.source_roots());
    assert_eq!(cloned.exclude_paths(), package.exclude_paths());
    assert_ne!(
        cloned.source_roots().as_ptr(),
        package.source_roots().as_ptr()
    );
    assert_ne!(
        cloned.exclude_paths().as_ptr(),
        package.exclude_paths().as_ptr()
    );
}

#[test]
fn manifest_dir_returns_the_package_directory() {
    // Arrange
    let package = sample();

    // Act
    let manifest_dir = package.manifest_dir();

    // Assert
    assert_eq!(manifest_dir, Path::new("/workspace/crap4rust"));
}

#[test]
fn workspace_root_returns_the_workspace_directory() {
    // Arrange
    let package = sample();

    // Act
    let workspace_root = package.workspace_root();

    // Assert
    assert_eq!(workspace_root, Path::new("/workspace"));
}
