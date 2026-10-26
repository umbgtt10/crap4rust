// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::support::in_memory_source_tree::InMemorySourceTree;
use crate::support::verbatim_path_keys::VerbatimPathKeys;
use crap4rust::analysis::file_walker::FileWalker;
use crap4rust::analysis::source_path::SourcePath;
use crap4rust::invocation::package_context::PackageContext;

fn package(root: &str) -> PackageContext {
    let root = path(root);
    PackageContext {
        name: String::from("probe"),
        manifest_dir: root.clone(),
        workspace_root: root.clone(),
        source_roots: vec![root.join("src")],
        include_test_targets: false,
        exclude_paths: vec![],
    }
}

fn path(text: &str) -> SourcePath {
    SourcePath::new(String::from(text))
}

#[test]
fn is_excluded_relative_file_exact_match() {
    // Arrange
    let excludes = vec![String::from("build.rs")];

    // Act
    let result = FileWalker::is_excluded_relative_file("build.rs", &excludes);

    // Assert
    assert!(result);
}

#[test]
fn is_excluded_relative_file_matches_prefix() {
    // Arrange
    let excludes = vec![String::from("tests/fixtures")];

    // Act
    let result = FileWalker::is_excluded_relative_file("tests/fixtures/data.rs", &excludes);

    // Assert
    assert!(result);
}

#[test]
fn is_excluded_relative_file_non_matching_not_excluded() {
    // Arrange
    let excludes = vec![String::from("tests/fixtures")];

    // Act
    let result = FileWalker::is_excluded_relative_file("src/lib.rs", &excludes);

    // Assert
    assert!(!result);
}

#[test]
fn is_selected_relative_file_excludes_benches() {
    // Arrange & Act
    let result = FileWalker::is_selected_relative_file("benches/bench.rs", false);

    // Assert
    assert!(!result);
}

#[test]
fn is_selected_relative_file_excludes_build_rs() {
    // Arrange & Act
    let result = FileWalker::is_selected_relative_file("build.rs", false);

    // Assert
    assert!(!result);
}

#[test]
fn is_selected_relative_file_excludes_examples() {
    // Arrange & Act
    let result = FileWalker::is_selected_relative_file("examples/example.rs", false);

    // Assert
    assert!(!result);
}

#[test]
fn is_selected_relative_file_includes_src_files() {
    // Arrange & Act
    let result = FileWalker::is_selected_relative_file("src/lib.rs", false);

    // Assert
    assert!(result);
}

#[test]
fn is_selected_relative_file_with_test_targets_includes_tests() {
    // Arrange & Act
    let result = FileWalker::is_selected_relative_file("tests/test.rs", true);

    // Assert
    assert!(result);
}

#[test]
fn is_selected_relative_file_without_test_targets_excludes_tests() {
    // Arrange & Act
    let result = FileWalker::is_selected_relative_file("tests/test.rs", false);

    // Assert
    assert!(!result);
}

#[test]
fn process_source_root_finds_the_functions_of_its_files() {
    // Arrange
    let source_tree = InMemorySourceTree::new().with_file(
        "/probe/src/lib.rs",
        "pub fn alpha() {}\n\npub fn beta() {}\n",
    );
    let package = package("/probe");

    // Act
    let functions = FileWalker::new(&package, &source_tree, &VerbatimPathKeys)
        .process_source_root(&path("/probe/src"))
        .expect("walks");

    // Assert
    let names: Vec<&str> = functions
        .iter()
        .map(|function| function.name.as_str())
        .collect();
    assert_eq!(names, vec!["alpha", "beta"]);
}

// A package without the directory has nothing there to score, which is not a
// failure.
#[test]
fn process_source_root_of_a_missing_directory_finds_nothing() {
    // Arrange
    let source_tree = InMemorySourceTree::new();
    let package = package("/probe");

    // Act
    let functions = FileWalker::new(&package, &source_tree, &VerbatimPathKeys)
        .process_source_root(&path("/probe/src"))
        .expect("walks");

    // Assert
    assert!(functions.is_empty());
}

#[test]
fn relative_file_outside_base_dir_returns_full_path() {
    // Arrange
    let base = path("/project/src");
    let file = path("/other/lib.rs");

    // Act
    let result = FileWalker::relative_file(&base, &file);

    // Assert
    assert_eq!(result, "/other/lib.rs");
}

#[test]
fn relative_file_within_base_dir_strips_prefix() {
    // Arrange
    let base = path("/project/src");
    let file = path("/project/src/lib.rs");

    // Act
    let result = FileWalker::relative_file(&base, &file);

    // Assert
    assert_eq!(result, "lib.rs");
}
