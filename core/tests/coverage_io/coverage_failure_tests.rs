// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crap4rust::coverage_io::coverage_failure::CoverageFailure;
use std::error::Error;
use std::fmt;

fn failure() -> CoverageFailure {
    CoverageFailure {
        cause: Box::new(Outer { inner: Inner }),
    }
}

#[derive(Debug)]
struct Inner;

impl fmt::Display for Inner {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("no such file")
    }
}

impl Error for Inner {}

#[derive(Debug)]
struct Outer {
    inner: Inner,
}

impl fmt::Display for Outer {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("failed to read coverage file cov.json")
    }
}

impl Error for Outer {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        Some(&self.inner)
    }
}

#[test]
fn source_hands_on_the_causes_own_cause() {
    // Arrange
    let failure = failure();

    // Act
    let source = failure.source().map(ToString::to_string);

    // Assert
    assert_eq!(source, Some(String::from("no such file")));
}

#[test]
fn to_string_says_what_the_cause_says() {
    // Arrange
    let failure = failure();

    // Act
    let text = failure.to_string();

    // Assert
    assert_eq!(text, "failed to read coverage file cov.json");
}
