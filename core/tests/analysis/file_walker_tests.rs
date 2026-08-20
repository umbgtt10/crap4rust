// Copyright 2025 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crap4rust::analysis::file_walker::FileWalker;
use std::path::Path;

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
