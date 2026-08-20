// Copyright 2025 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crap4rust::package_context::PackageContext;
use std::path::PathBuf;

fn sample() -> PackageContext {
    PackageContext {
        name: String::from("cargo-crap4rust"),
        manifest_dir: PathBuf::from("/workspace/crap4rust"),
        workspace_root: PathBuf::from("/workspace"),
        source_roots: vec![PathBuf::from("/workspace/crap4rust/src")],
        include_test_targets: false,
        exclude_paths: vec![String::from("tests/fixtures")],
    }
}

#[test]
fn clone_produces_an_independent_copy_of_vec_fields() {
    // Arrange
    let package = sample();
    let mut cloned = package.clone();

    // Act
    cloned
        .source_roots
        .push(PathBuf::from("/workspace/crap4rust/extra"));
    cloned.exclude_paths.push(String::from("benches"));

    // Assert
    assert_eq!(package.source_roots.len(), 1);
    assert_eq!(package.exclude_paths.len(), 1);
    assert_eq!(cloned.source_roots.len(), 2);
    assert_eq!(cloned.exclude_paths.len(), 2);
}
