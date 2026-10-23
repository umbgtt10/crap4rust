// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crap4rust::analysis::source_function_discovery::SourceFunctionDiscovery;
use crap4rust::invocation::package_context::PackageContext;
use crap4rust::traits::function_discovery::FunctionDiscovery;
use std::fs;
use tempfile::TempDir;

#[test]
fn discover_finds_public_function_in_source_root() {
    // Arrange
    let dir = TempDir::new().expect("temp dir");
    let src_dir = dir.path().join("src");
    fs::create_dir_all(&src_dir).expect("create src dir");
    fs::write(src_dir.join("lib.rs"), "pub fn risky() { if true {} }").expect("write lib.rs");
    let package = PackageContext::new(
        String::from("probe"),
        dir.path().to_path_buf(),
        dir.path().to_path_buf(),
        vec![src_dir],
        false,
        vec![],
    );
    let discovery = SourceFunctionDiscovery::new();

    // Act
    let functions = discovery.discover(&package).expect("discover functions");

    // Assert
    assert_eq!(functions.len(), 1);
    assert_eq!(functions[0].name(), "risky");
}

#[test]
fn discover_missing_source_root_returns_empty() {
    // Arrange
    let dir = TempDir::new().expect("temp dir");
    let package = PackageContext::new(
        String::from("probe"),
        dir.path().to_path_buf(),
        dir.path().to_path_buf(),
        vec![dir.path().join("src")],
        false,
        vec![],
    );
    let discovery = SourceFunctionDiscovery::new();

    // Act
    let functions = discovery.discover(&package).expect("discover functions");

    // Assert
    assert!(functions.is_empty());
}
