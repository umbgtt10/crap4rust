// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::support::in_memory_source_tree::NotHeld;
use crap4rust::analysis::discovery_failure::DiscoveryFailure;
use crap4rust::analysis::source_path::SourcePath;
use crap4rust::analysis::source_read_failure::SourceReadFailure;
use crap4rust::coverage_io::coverage_failure::CoverageFailure;
use crap4rust::invocation::run_failure::RunFailure;
use crap4rust::invocation::workspace_read_failure::WorkspaceReadFailure;
use std::error::Error;

fn chain(failure: &RunFailure) -> Vec<String> {
    let mut chain = vec![failure.to_string()];
    let mut source = failure.source();
    while let Some(cause) = source {
        chain.push(cause.to_string());
        source = cause.source();
    }
    chain
}

#[test]
fn from_a_coverage_failure_says_what_the_coverage_source_said() {
    // Arrange
    let failure = RunFailure::from(CoverageFailure {
        cause: Box::new(NotHeld),
    });

    // Act
    let chain = chain(&failure);

    // Assert
    assert_eq!(chain, vec![String::from("not held")]);
}

#[test]
fn from_a_workspace_read_failure_keeps_its_cause() {
    // Arrange
    let failure = RunFailure::from(WorkspaceReadFailure {
        cause: Box::new(NotHeld),
    });

    // Act
    let chain = chain(&failure);

    // Assert
    assert_eq!(
        chain,
        vec![
            String::from("failed to read Cargo metadata"),
            String::from("not held")
        ]
    );
}

#[test]
fn source_of_a_discovery_failure_names_the_package_then_the_file_then_the_cause() {
    // Arrange
    let failure = RunFailure::Discovery {
        package: String::from("probe"),
        cause: DiscoveryFailure::SourceRead(SourceReadFailure {
            path: SourcePath::new(String::from("/p/src/lib.rs")),
            cause: Box::new(NotHeld),
        }),
    };

    // Act
    let chain = chain(&failure);

    // Assert
    assert_eq!(
        chain,
        vec![
            String::from("failed to discover functions in package probe"),
            String::from("failed to read source file /p/src/lib.rs"),
            String::from("not held"),
        ]
    );
}

#[test]
fn to_string_of_each_failure_without_a_cause_says_what_went_wrong() {
    // Arrange
    let failures = [
        RunFailure::NoCoverageRecords,
        RunFailure::NoFunctions,
        RunFailure::NoMatchedCoverage,
        RunFailure::NoParentDirectory,
        RunFailure::NoWorkspaceMembers,
        RunFailure::PackageNotFound(String::from("probe")),
    ];

    // Act
    let chains = failures.iter().map(chain).collect::<Vec<_>>();

    // Assert
    assert_eq!(
        chains,
        vec![
            vec![String::from(
                "coverage file did not contain any function records"
            )],
            vec![String::from(
                "no Rust functions were discovered in the selected packages"
            )],
            vec![String::from(
                "coverage data could not be matched to any discovered function by file path and line"
            )],
            vec![String::from("package manifest has no parent directory")],
            vec![String::from(
                "manifest contains no workspace member packages"
            )],
            vec![String::from("package probe was not found in the manifest")],
        ]
    );
}
