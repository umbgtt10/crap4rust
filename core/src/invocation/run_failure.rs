// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use alloc::string::String;
use core::error::Error;
use core::fmt;

use crate::analysis::discovery_failure::DiscoveryFailure;
use crate::coverage_io::coverage_failure::CoverageFailure;
use crate::invocation::workspace_read_failure::WorkspaceReadFailure;

// Why a run produced no report, one variant per way it can fail. A failure the
// run only passes on says itself and keeps its own cause, so the chain a caller
// prints is the one the failure was raised with.
#[derive(Debug)]
pub enum RunFailure {
    Coverage(CoverageFailure),
    Discovery {
        package: String,
        cause: DiscoveryFailure,
    },
    NoCoverageRecords,
    NoFunctions,
    NoMatchedCoverage,
    NoParentDirectory,
    NoWorkspaceMembers,
    PackageNotFound(String),
    Workspace(WorkspaceReadFailure),
}

impl From<CoverageFailure> for RunFailure {
    fn from(failure: CoverageFailure) -> Self {
        Self::Coverage(failure)
    }
}

impl From<WorkspaceReadFailure> for RunFailure {
    fn from(failure: WorkspaceReadFailure) -> Self {
        Self::Workspace(failure)
    }
}

impl fmt::Display for RunFailure {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Coverage(failure) => write!(formatter, "{failure}"),
            Self::Discovery { package, .. } => {
                write!(formatter, "failed to discover functions in package {package}")
            }
            Self::NoCoverageRecords => {
                formatter.write_str("coverage file did not contain any function records")
            }
            Self::NoFunctions => {
                formatter.write_str("no Rust functions were discovered in the selected packages")
            }
            Self::NoMatchedCoverage => formatter.write_str(
                "coverage data could not be matched to any discovered function by file path and line",
            ),
            Self::NoParentDirectory => {
                formatter.write_str("package manifest has no parent directory")
            }
            Self::NoWorkspaceMembers => {
                formatter.write_str("manifest contains no workspace member packages")
            }
            Self::PackageNotFound(name) => {
                write!(formatter, "package {name} was not found in the manifest")
            }
            Self::Workspace(failure) => write!(formatter, "{failure}"),
        }
    }
}

impl Error for RunFailure {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::Coverage(failure) => failure.source(),
            Self::Discovery { cause, .. } => Some(cause),
            Self::Workspace(failure) => failure.source(),
            Self::NoCoverageRecords
            | Self::NoFunctions
            | Self::NoMatchedCoverage
            | Self::NoParentDirectory
            | Self::NoWorkspaceMembers
            | Self::PackageNotFound(_) => None,
        }
    }
}
