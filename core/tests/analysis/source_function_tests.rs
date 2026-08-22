// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crap4rust::analysis::source_function::SourceFunction;

fn sample() -> SourceFunction {
    SourceFunction {
        package_name: String::from("cargo-crap4rust"),
        name: String::from("App::run"),
        path_key: String::from("c:/workspace/crap4rust/src/app.rs"),
        relative_file: String::from("src/app.rs"),
        line: 10,
        end_line: 40,
        complexity: 7,
    }
}

#[test]
fn clone_preserves_every_field() {
    // Arrange
    let function = sample();

    // Act
    let cloned = function.clone();

    // Assert
    assert_eq!(cloned.package_name, function.package_name);
    assert_eq!(cloned.name, function.name);
    assert_eq!(cloned.path_key, function.path_key);
    assert_eq!(cloned.relative_file, function.relative_file);
    assert_eq!(cloned.line, function.line);
    assert_eq!(cloned.end_line, function.end_line);
    assert_eq!(cloned.complexity, function.complexity);
}

#[test]
fn end_line_is_greater_than_or_equal_to_line() {
    // Arrange & Act
    let function = sample();

    // Assert
    assert!(function.end_line >= function.line);
}
