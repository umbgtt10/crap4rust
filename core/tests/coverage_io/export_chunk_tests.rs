// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crap4rust::coverage_io::export_chunk::ExportChunk;
use serde_json::from_str;

#[test]
fn functions_lists_every_function_of_the_chunk() {
    // Arrange
    let chunk: ExportChunk = from_str(
        r#"{"functions": [
            {"filenames": ["src/a.rs"], "regions": [[1, 1, 2, 2, 1, 0, 0, 0]]},
            {"filenames": ["src/b.rs"], "regions": [[3, 1, 4, 2, 0, 0, 0, 0]]}
        ]}"#,
    )
    .expect("deserialize chunk");

    // Act
    let functions = chunk.functions();

    // Assert
    let filenames: Vec<&[String]> = functions
        .iter()
        .map(|function| function.filenames())
        .collect();
    assert_eq!(filenames, [["src/a.rs"], ["src/b.rs"]]);
}
