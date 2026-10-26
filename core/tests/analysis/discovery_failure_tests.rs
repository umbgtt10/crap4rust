// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::support::in_memory_source_tree::NotHeld;
use crap4rust::analysis::discovery_failure::DiscoveryFailure;
use crap4rust::analysis::source_path::SourcePath;
use crap4rust::analysis::source_read_failure::SourceReadFailure;
use std::error::Error;
use syn::parse_file;

fn parse_failure() -> DiscoveryFailure {
    let Err(cause) = parse_file("fn (") else {
        panic!("`fn (` is not Rust");
    };
    DiscoveryFailure::Parse {
        path: SourcePath::new(String::from("/p/src/lib.rs")),
        cause,
    }
}

fn read_failure() -> DiscoveryFailure {
    DiscoveryFailure::from(SourceReadFailure {
        path: SourcePath::new(String::from("/p/src/lib.rs")),
        cause: Box::new(NotHeld),
    })
}

#[test]
fn source_of_a_parse_failure_is_what_syn_said() {
    // Arrange
    let failure = parse_failure();
    let Err(expected) = parse_file("fn (") else {
        panic!("`fn (` is not Rust");
    };

    // Act
    let cause = failure.source().map(ToString::to_string);

    // Assert
    assert_eq!(cause, Some(expected.to_string()));
}

#[test]
fn source_of_a_read_failure_is_the_trees_own_cause() {
    // Arrange
    let failure = read_failure();

    // Act
    let cause = failure.source().map(ToString::to_string);

    // Assert
    assert_eq!(cause, Some(String::from("not held")));
}

#[test]
fn to_string_of_a_parse_failure_names_the_file() {
    // Arrange
    let failure = parse_failure();

    // Act
    let text = failure.to_string();

    // Assert
    assert_eq!(text, "failed to parse source file /p/src/lib.rs");
}

#[test]
fn to_string_of_a_read_failure_is_the_read_failure() {
    // Arrange
    let failure = read_failure();

    // Act
    let text = failure.to_string();

    // Assert
    assert_eq!(text, "failed to read source file /p/src/lib.rs");
}
