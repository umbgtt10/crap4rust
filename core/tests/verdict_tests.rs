// Copyright 2025 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crap4rust::verdict::Verdict;
use serde_json::to_string;

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
fn as_str_serializes_to_json_as_pascal_case_variant_name() {
    // Arrange
    let verdict = Verdict::Warn;

    // Act
    let json = to_string(&verdict).expect("serialize verdict");

    // Assert
    assert_eq!(json, "\"Warn\"");
}

#[test]
fn as_str_warn_returns_warn() {
    // Arrange & Act
    let result = Verdict::Warn.as_str();

    // Assert
    assert_eq!(result, "warn");
}
