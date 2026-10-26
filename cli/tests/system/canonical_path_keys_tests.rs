// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crap4rust::analysis::source_path::SourcePath;
use crap4rust::traits::path_keys::PathKeys;
use crap4rust_cli::system::canonical_path_keys::CanonicalPathKeys;
use std::fs;
use tempfile::TempDir;

#[test]
fn key_of_replaces_backslashes() {
    // Arrange & Act
    let result =
        CanonicalPathKeys::new().key_of(&SourcePath::new(String::from("C:\\project\\src\\lib.rs")));

    // Assert
    assert!(!result.contains('\\'));
}

#[test]
fn key_of_two_spellings_of_one_file_is_the_same_key() {
    // Arrange
    let dir = TempDir::new().expect("temp dir");
    fs::create_dir_all(dir.path().join("src")).expect("create src dir");
    fs::write(dir.path().join("src").join("lib.rs"), "").expect("write lib.rs");
    let root = dir
        .path()
        .to_str()
        .expect("UTF-8 temp dir")
        .replace('\\', "/");
    let direct = SourcePath::new(format!("{root}/src/lib.rs"));
    let roundabout = SourcePath::new(format!("{root}/src/../src/lib.rs"));

    // Act
    let keys = [
        CanonicalPathKeys::new().key_of(&direct),
        CanonicalPathKeys::new().key_of(&roundabout),
    ];

    // Assert
    assert_eq!(keys[0], keys[1]);
}
