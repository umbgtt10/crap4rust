// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crap4rust::analysis::file_walker::FileWalker;
use crap4rust::invocation::package_context::PackageContext;
use std::fs;
use std::path::Path;
use tempfile::TempDir;

fn package(root: &Path) -> PackageContext {
    PackageContext {
        name: String::from("probe"),
        manifest_dir: root.to_path_buf(),
        workspace_root: root.to_path_buf(),
        source_roots: vec![root.join("src")],
        include_test_targets: false,
        exclude_paths: vec![],
    }
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
    let dir = TempDir::new().expect("temp dir");
    let src = dir.path().join("src");
    fs::create_dir_all(&src).expect("create src dir");
    fs::write(
        src.join("lib.rs"),
        "pub fn alpha() {}\n\npub fn beta() {}\n",
    )
    .expect("write lib.rs");
    let package = package(dir.path());

    // Act
    let functions = FileWalker::new(&package)
        .process_source_root(&src)
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
    let dir = TempDir::new().expect("temp dir");
    let package = package(dir.path());

    // Act
    let functions = FileWalker::new(&package)
        .process_source_root(&dir.path().join("src"))
        .expect("walks");

    // Assert
    assert!(functions.is_empty());
}

#[test]
fn relative_file_outside_base_dir_returns_full_path() {
    // Arrange
    let base = Path::new("/project/src");
    let file = Path::new("/other/lib.rs");

    // Act
    let result = FileWalker::relative_file(base, file);

    // Assert
    assert_eq!(result, "/other/lib.rs");
}

#[test]
fn relative_file_within_base_dir_strips_prefix() {
    // Arrange
    let base = Path::new("/project/src");
    let file = Path::new("/project/src/lib.rs");

    // Act
    let result = FileWalker::relative_file(base, file);

    // Assert
    assert_eq!(result, "lib.rs");
}
