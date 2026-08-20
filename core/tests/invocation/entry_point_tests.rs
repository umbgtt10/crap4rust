// Copyright 2025 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crap4rust::invocation::entry_point::EntryPoint;
use std::process::ExitCode;

#[test]
fn run_against_a_manifest_that_does_not_exist_returns_the_error_code() {
    // Arrange
    let args = [
        "cargo-crap4rust",
        "crap4rust",
        "--manifest-path",
        "no/such/directory/Cargo.toml",
    ]
    .map(String::from)
    .to_vec();

    // Act
    let code = EntryPoint::run(args);

    // Assert
    assert_eq!(format!("{code:?}"), format!("{:?}", ExitCode::from(2)));
}

#[test]
fn without_cargo_subcommand_drops_only_the_first_repeat() {
    // Arrange
    let args = ["cargo-crap4rust", "crap4rust", "--package", "crap4rust"]
        .map(String::from)
        .to_vec();

    // Act
    let forwarded = EntryPoint::without_cargo_subcommand(args);

    // Assert
    assert_eq!(forwarded, vec!["cargo-crap4rust", "--package", "crap4rust"]);
}

#[test]
fn without_cargo_subcommand_drops_the_name_cargo_repeats() {
    // Arrange
    let args = ["cargo-crap4rust", "crap4rust", "--threshold", "15"]
        .map(String::from)
        .to_vec();

    // Act
    let forwarded = EntryPoint::without_cargo_subcommand(args);

    // Assert
    assert_eq!(forwarded, vec!["cargo-crap4rust", "--threshold", "15"]);
}

#[test]
fn without_cargo_subcommand_keeps_a_package_named_after_the_tool() {
    // Arrange
    let args = ["cargo-crap4rust", "--package", "crap4rust"]
        .map(String::from)
        .to_vec();

    // Act
    let forwarded = EntryPoint::without_cargo_subcommand(args);

    // Assert
    assert_eq!(forwarded, vec!["cargo-crap4rust", "--package", "crap4rust"]);
}

#[test]
fn without_cargo_subcommand_leaves_a_direct_invocation_untouched() {
    // Arrange
    let args = ["cargo-crap4rust", "--threshold", "15"]
        .map(String::from)
        .to_vec();

    // Act
    let forwarded = EntryPoint::without_cargo_subcommand(args);

    // Assert
    assert_eq!(forwarded, vec!["cargo-crap4rust", "--threshold", "15"]);
}
