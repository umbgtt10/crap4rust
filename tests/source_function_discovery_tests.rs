// Copyright 2025 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License or Apache License, Version 2.0
// SPDX-License-Identifier: MIT OR Apache-2.0

use std::fs;

use tempfile::TempDir;

use crap4rust::package_context::PackageContext;
use crap4rust::source_function_discovery::SourceFunctionDiscovery;
use crap4rust::traits::function_discovery::FunctionDiscovery;

#[test]
fn discover_finds_public_function_in_source_root() {
    // Arrange
    let dir = TempDir::new().expect("temp dir");
    let src_dir = dir.path().join("src");
    fs::create_dir_all(&src_dir).expect("create src dir");
    fs::write(src_dir.join("lib.rs"), "pub fn risky() { if true {} }").expect("write lib.rs");
    let package = PackageContext {
        name: String::from("probe"),
        manifest_dir: dir.path().to_path_buf(),
        workspace_root: dir.path().to_path_buf(),
        source_roots: vec![src_dir],
        include_test_targets: false,
        exclude_paths: vec![],
    };
    let discovery = SourceFunctionDiscovery::new();

    // Act
    let functions = discovery.discover(&package).expect("discover functions");

    // Assert
    assert_eq!(functions.len(), 1);
    assert_eq!(functions[0].name, "risky");
}

#[test]
fn discover_missing_source_root_returns_empty() {
    // Arrange
    let dir = TempDir::new().expect("temp dir");
    let package = PackageContext {
        name: String::from("probe"),
        manifest_dir: dir.path().to_path_buf(),
        workspace_root: dir.path().to_path_buf(),
        source_roots: vec![dir.path().join("src")],
        include_test_targets: false,
        exclude_paths: vec![],
    };
    let discovery = SourceFunctionDiscovery::new();

    // Act
    let functions = discovery.discover(&package).expect("discover functions");

    // Assert
    assert!(functions.is_empty());
}
