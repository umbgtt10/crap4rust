// Copyright 2025 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use cargo_metadata::Target;
use crap4rust::source_root_collector::SourceRootCollector;
use serde_json::from_str;
use std::path::Path;

fn make_target(kinds: &[&str], src_path: &str) -> Target {
    let kinds_json: Vec<String> = kinds.iter().map(|k| format!("\"{}\"", k)).collect();
    let json = format!(
        r#"{{"name":"test","kind":[{}],"crate_types":["lib"],"required_features":[],"src_path":"{}","edition":"2021","doctest":false}}"#,
        kinds_json.join(","),
        src_path
    );
    from_str(&json).unwrap()
}

#[test]
fn collect_excludes_bench_target_without_include_test_flag() {
    // Arrange
    let manifest_dir = Path::new("/project");
    let mut collector = SourceRootCollector::new(false, manifest_dir);
    let targets = vec![make_target(&["bench"], "/project/benches/bench.rs")];

    // Act
    collector.collect(&targets);
    let roots = collector.finalize();

    // Assert
    assert_eq!(roots, vec![Path::new("/project/src")]);
}

#[test]
fn collect_excludes_custom_build_target() {
    // Arrange
    let manifest_dir = Path::new("/project");
    let mut collector = SourceRootCollector::new(false, manifest_dir);
    let targets = vec![
        make_target(&["custom-build"], "/project/build.rs"),
        make_target(&["lib"], "/project/src/lib.rs"),
    ];

    // Act
    collector.collect(&targets);
    let roots = collector.finalize();

    // Assert
    assert_eq!(roots, vec![Path::new("/project/src")]);
}

#[test]
fn collect_regular_lib_target_adds_parent_dir() {
    // Arrange
    let manifest_dir = Path::new("/project");
    let mut collector = SourceRootCollector::new(false, manifest_dir);
    let targets = vec![make_target(&["lib"], "/project/src/lib.rs")];

    // Act
    collector.collect(&targets);
    let roots = collector.finalize();

    // Assert
    assert_eq!(roots, vec![Path::new("/project/src")]);
}

#[test]
fn collect_with_include_test_targets_includes_test_dir() {
    // Arrange
    let manifest_dir = Path::new("/project");
    let mut collector = SourceRootCollector::new(true, manifest_dir);
    let targets = vec![
        make_target(&["test"], "/project/tests/integration_test.rs"),
        make_target(&["lib"], "/project/src/lib.rs"),
    ];

    // Act
    collector.collect(&targets);
    let roots = collector.finalize();

    // Assert
    let mut expected: Vec<&Path> = vec![Path::new("/project/src"), Path::new("/project/tests")];
    expected.sort();
    let mut result: Vec<&Path> = roots.iter().map(|p| p.as_path()).collect();
    result.sort();
    assert_eq!(result, expected);
}

#[test]
fn finalize_with_no_targets_falls_back_to_src_dir() {
    // Arrange
    let manifest_dir = Path::new("/project");
    let mut collector = SourceRootCollector::new(false, manifest_dir);

    // Act
    collector.collect(&[]);
    let roots = collector.finalize();

    // Assert
    assert_eq!(roots, vec![Path::new("/project/src")]);
}

#[test]
fn is_selected_target_excludes_test_kind_without_include_test_targets() {
    // Arrange
    let target = make_target(&["test"], "/project/tests/integration_test.rs");

    // Act
    let result = SourceRootCollector::is_selected_target(&target, false);

    // Assert
    assert!(!result);
}

#[test]
fn is_selected_target_includes_proc_macro_kind() {
    // Arrange
    let target = make_target(&["proc-macro"], "/project/src/lib.rs");

    // Act
    let result = SourceRootCollector::is_selected_target(&target, false);

    // Assert
    assert!(result);
}
