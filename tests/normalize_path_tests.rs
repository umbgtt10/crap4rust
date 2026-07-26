// Copyright 2025 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use std::path::Path;

use crap4rust::normalize_path::normalize_path;

#[test]
fn normalize_path_replaces_backslashes() {
    // Arrange & Act
    let result = normalize_path(Path::new("C:\\project\\src\\lib.rs"));

    // Assert
    assert!(!result.contains('\\'));
}
