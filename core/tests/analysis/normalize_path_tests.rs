// Copyright 2025 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crap4rust::analysis::normalize_path::normalize_path;
use std::path::Path;

#[test]
fn normalize_path_replaces_backslashes() {
    // Arrange & Act
    let result = normalize_path(Path::new("C:\\project\\src\\lib.rs"));

    // Assert
    assert!(!result.contains('\\'));
}
