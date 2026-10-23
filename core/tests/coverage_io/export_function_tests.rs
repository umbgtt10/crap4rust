// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crap4rust::coverage_io::export_function::ExportFunction;
use serde_json::from_str;

#[test]
fn filenames_lists_every_filename_in_order() {
    // Arrange
    let function: ExportFunction =
        from_str(r#"{"filenames": ["src/a.rs", "src/a_alias.rs"], "regions": []}"#)
            .expect("deserialize function");

    // Act
    let filenames = function.filenames();

    // Assert
    assert_eq!(filenames, ["src/a.rs", "src/a_alias.rs"]);
}

#[test]
fn regions_keeps_every_region_in_order() {
    // Arrange
    let function: ExportFunction = from_str(
        r#"{"filenames": ["src/a.rs"], "regions": [[1, 1, 2, 2, 1, 0, 0, 0], [3, 5, 3, 9, 0, 0, 0, 0]]}"#,
    )
    .expect("deserialize function");

    // Act
    let regions = function.regions();

    // Assert
    assert_eq!(
        regions,
        [vec![1, 1, 2, 2, 1, 0, 0, 0], vec![3, 5, 3, 9, 0, 0, 0, 0]]
    );
}
