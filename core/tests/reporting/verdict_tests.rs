// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crap4rust::reporting::verdict::Verdict;

#[test]
fn as_str_clean_returns_clean() {
    // Arrange & Act
    let result = Verdict::Clean.as_str();

    // Assert
    assert_eq!(result, "clean");
}

#[test]
fn as_str_crappy_returns_crappy() {
    // Arrange & Act
    let result = Verdict::Crappy.as_str();

    // Assert
    assert_eq!(result, "crappy");
}

#[test]
fn as_str_warn_returns_warn() {
    // Arrange & Act
    let result = Verdict::Warn.as_str();

    // Assert
    assert_eq!(result, "warn");
}
