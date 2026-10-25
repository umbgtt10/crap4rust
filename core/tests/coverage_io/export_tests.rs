// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crap4rust::coverage_io::export::Export;
use serde_json::from_str;

#[test]
fn export_deserializes_empty_data_array() {
    // Arrange
    let json = r#"{"data": []}"#;

    // Act
    let export: Export = from_str(json).expect("deserialize export");

    // Assert
    assert!(export.data.is_empty());
}

#[test]
fn export_deserializes_multiple_functions_in_one_chunk() {
    // Arrange
    let json = r#"{
        "data": [
            {
                "functions": [
                    {"filenames": ["src/a.rs"], "regions": [[1, 1, 2, 2, 1, 0, 0, 0]]},
                    {"filenames": ["src/b.rs"], "regions": [[3, 1, 4, 2, 0, 0, 0, 0]]}
                ]
            }
        ]
    }"#;

    // Act
    let export: Export = from_str(json).expect("deserialize export");

    // Assert
    assert_eq!(export.data.len(), 1);
    assert_eq!(export.data[0].functions.len(), 2);
}

#[test]
fn export_function_deserializes_multiple_filenames() {
    // Arrange
    let json = r#"{
        "data": [
            {
                "functions": [
                    {"filenames": ["src/a.rs", "src/a_alias.rs"], "regions": [[1, 1, 2, 2, 1, 0, 0, 0]]}
                ]
            }
        ]
    }"#;

    // Act
    let export: Export = from_str(json).expect("deserialize export");

    // Assert
    assert_eq!(
        export.data[0].functions[0].filenames,
        vec!["src/a.rs", "src/a_alias.rs"]
    );
}
