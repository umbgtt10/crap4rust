// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::support::verbatim_path_keys::VerbatimPathKeys;
use crap4rust::analysis::source_path::SourcePath;
use crap4rust::analysis::test_module_registry::TestModuleRegistry;
use syn::File;
use syn::parse_file;

fn parsed(source: &str) -> File {
    parse_file(source).expect("parse source")
}

fn path(text: &str) -> SourcePath {
    SourcePath::new(String::from(text))
}

#[test]
fn build_directory_mod_rs_module_marks_target_excluded() {
    // Arrange
    let mod_rs_path = path("/crate/tests/mod.rs");
    let lib_path = path("/crate/lib.rs");
    let listed = vec![lib_path.clone(), mod_rs_path.clone()];
    let files = vec![(lib_path, parsed("#[cfg(test)] mod tests;"))];

    // Act
    let registry = TestModuleRegistry::build(&files, &listed, &VerbatimPathKeys);

    // Assert
    assert!(registry.is_excluded(&mod_rs_path));
}

#[test]
fn build_inline_nested_test_module_resolves_relative_to_outer_dir() {
    // Arrange
    let tests_path = path("/crate/outer/tests.rs");
    let lib_path = path("/crate/lib.rs");
    let listed = vec![lib_path.clone(), tests_path.clone()];
    let files = vec![(lib_path, parsed("mod outer { #[cfg(test)] mod tests; }"))];

    // Act
    let registry = TestModuleRegistry::build(&files, &listed, &VerbatimPathKeys);

    // Assert
    assert!(registry.is_excluded(&tests_path));
}

#[test]
fn build_non_test_file_module_leaves_it_not_excluded() {
    // Arrange
    let helpers_path = path("/crate/helpers.rs");
    let lib_path = path("/crate/lib.rs");
    let listed = vec![lib_path.clone(), helpers_path.clone()];
    let files = vec![(lib_path, parsed("mod helpers;"))];

    // Act
    let registry = TestModuleRegistry::build(&files, &listed, &VerbatimPathKeys);

    // Assert
    assert!(!registry.is_excluded(&helpers_path));
}

#[test]
fn build_sibling_file_module_marks_target_excluded() {
    // Arrange
    let tests_path = path("/crate/tests.rs");
    let lib_path = path("/crate/lib.rs");
    let listed = vec![lib_path.clone(), tests_path.clone()];
    let files = vec![(lib_path, parsed("#[cfg(test)] mod tests;"))];

    // Act
    let registry = TestModuleRegistry::build(&files, &listed, &VerbatimPathKeys);

    // Assert
    assert!(registry.is_excluded(&tests_path));
}

#[test]
fn build_unresolvable_test_module_leaves_other_paths_not_excluded() {
    // Arrange
    let lib_path = path("/crate/lib.rs");
    let listed = vec![lib_path.clone(), path("/crate/unrelated.rs")];
    let files = vec![(lib_path, parsed("#[cfg(test)] mod tests;"))];

    // Act
    let registry = TestModuleRegistry::build(&files, &listed, &VerbatimPathKeys);

    // Assert
    assert!(!registry.is_excluded(&path("/crate/unrelated.rs")));
}

#[test]
fn is_excluded_path_not_in_registry_returns_false() {
    // Arrange
    let registry = TestModuleRegistry::build(&[], &[], &VerbatimPathKeys);

    // Act
    let result = registry.is_excluded(&path("/anywhere/lib.rs"));

    // Assert
    assert!(!result);
}
