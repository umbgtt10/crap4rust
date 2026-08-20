// Copyright 2025 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crap4rust::analysis::test_module_registry::TestModuleRegistry;
use std::fs;
use std::path::PathBuf;
use syn::parse_file;
use tempfile::TempDir;

fn parsed(source: &str) -> syn::File {
    parse_file(source).expect("parse source")
}

#[test]
fn build_directory_mod_rs_module_marks_target_excluded() {
    // Arrange
    let dir = TempDir::new().expect("temp dir");
    fs::create_dir_all(dir.path().join("tests")).expect("create tests dir");
    let mod_rs_path = dir.path().join("tests").join("mod.rs");
    fs::write(&mod_rs_path, "pub fn helper() {}").expect("write tests/mod.rs");
    let lib_path = dir.path().join("lib.rs");
    let files = vec![(lib_path, parsed("#[cfg(test)] mod tests;"))];

    // Act
    let registry = TestModuleRegistry::build(&files);

    // Assert
    assert!(registry.is_excluded(&mod_rs_path));
}

#[test]
fn build_inline_nested_test_module_resolves_relative_to_outer_dir() {
    // Arrange
    let dir = TempDir::new().expect("temp dir");
    fs::create_dir_all(dir.path().join("outer")).expect("create outer dir");
    let tests_path = dir.path().join("outer").join("tests.rs");
    fs::write(&tests_path, "pub fn helper() {}").expect("write outer/tests.rs");
    let lib_path = dir.path().join("lib.rs");
    let files = vec![(lib_path, parsed("mod outer { #[cfg(test)] mod tests; }"))];

    // Act
    let registry = TestModuleRegistry::build(&files);

    // Assert
    assert!(registry.is_excluded(&tests_path));
}

#[test]
fn build_non_test_file_module_leaves_it_not_excluded() {
    // Arrange
    let dir = TempDir::new().expect("temp dir");
    let helpers_path = dir.path().join("helpers.rs");
    fs::write(&helpers_path, "pub fn helper() {}").expect("write helpers.rs");
    let lib_path = dir.path().join("lib.rs");
    let files = vec![(lib_path, parsed("mod helpers;"))];

    // Act
    let registry = TestModuleRegistry::build(&files);

    // Assert
    assert!(!registry.is_excluded(&helpers_path));
}

#[test]
fn build_sibling_file_module_marks_target_excluded() {
    // Arrange
    let dir = TempDir::new().expect("temp dir");
    let tests_path = dir.path().join("tests.rs");
    fs::write(&tests_path, "pub fn helper() {}").expect("write tests.rs");
    let lib_path = dir.path().join("lib.rs");
    let files = vec![(lib_path, parsed("#[cfg(test)] mod tests;"))];

    // Act
    let registry = TestModuleRegistry::build(&files);

    // Assert
    assert!(registry.is_excluded(&tests_path));
}

#[test]
fn build_unresolvable_test_module_leaves_other_paths_not_excluded() {
    // Arrange
    let dir = TempDir::new().expect("temp dir");
    let lib_path = dir.path().join("lib.rs");
    let files = vec![(lib_path, parsed("#[cfg(test)] mod tests;"))];

    // Act
    let registry = TestModuleRegistry::build(&files);

    // Assert
    assert!(!registry.is_excluded(&dir.path().join("unrelated.rs")));
}

#[test]
fn is_excluded_path_not_in_registry_returns_false() {
    // Arrange
    let registry = TestModuleRegistry::build(&[]);

    // Act
    let result = registry.is_excluded(&PathBuf::from("/anywhere/lib.rs"));

    // Assert
    assert!(!result);
}
