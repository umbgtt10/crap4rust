// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::support::in_memory_source_tree::InMemorySourceTree;
use crate::support::verbatim_path_keys::VerbatimPathKeys;
use crap4rust::analysis::source_function_discovery::SourceFunctionDiscovery;
use crap4rust::analysis::source_path::SourcePath;
use crap4rust::invocation::package_context::PackageContext;
use crap4rust::traits::function_discovery::FunctionDiscovery;

fn package() -> PackageContext {
    let root = SourcePath::new(String::from("/probe"));
    PackageContext {
        name: String::from("probe"),
        manifest_dir: root.clone(),
        workspace_root: root.clone(),
        source_roots: vec![root.join("src")],
        include_test_targets: false,
        exclude_paths: vec![],
    }
}

#[test]
fn discover_finds_public_function_in_source_root() {
    // Arrange
    let source_tree =
        InMemorySourceTree::new().with_file("/probe/src/lib.rs", "pub fn risky() { if true {} }");
    let package = package();
    let discovery = SourceFunctionDiscovery::new(&source_tree, &VerbatimPathKeys);

    // Act
    let functions = discovery.discover(&package).expect("discover functions");

    // Assert
    assert_eq!(functions.len(), 1);
    assert_eq!(functions[0].name, "risky");
}

#[test]
fn discover_missing_source_root_returns_empty() {
    // Arrange
    let source_tree = InMemorySourceTree::new();
    let package = package();
    let discovery = SourceFunctionDiscovery::new(&source_tree, &VerbatimPathKeys);

    // Act
    let functions = discovery.discover(&package).expect("discover functions");

    // Assert
    assert!(functions.is_empty());
}
