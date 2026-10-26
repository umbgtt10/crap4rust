// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crap4rust::analysis::source_path::SourcePath;
use crap4rust::invocation::package_context::PackageContext;

fn sample() -> PackageContext {
    PackageContext {
        name: String::from("cargo-crap4rust"),
        manifest_dir: SourcePath::new(String::from("/workspace/crap4rust")),
        workspace_root: SourcePath::new(String::from("/workspace")),
        source_roots: vec![SourcePath::new(String::from("/workspace/crap4rust/src"))],
        include_test_targets: false,
        exclude_paths: vec![String::from("tests/fixtures")],
    }
}

#[test]
fn clone_produces_an_independent_copy_of_vec_fields() {
    // Arrange
    let package = sample();

    // Act
    let cloned = package.clone();

    // Assert
    assert_eq!(cloned.source_roots, package.source_roots);
    assert_eq!(cloned.exclude_paths, package.exclude_paths);
    assert_ne!(cloned.source_roots.as_ptr(), package.source_roots.as_ptr());
    assert_ne!(
        cloned.exclude_paths.as_ptr(),
        package.exclude_paths.as_ptr()
    );
}
