// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use core::error;
use core::fmt;

use syn::Error;

use crate::analysis::source_path::SourcePath;
use crate::analysis::source_read_failure::SourceReadFailure;

// Why a package's functions could not be discovered: a source file the tree
// could not read, or one that is not Rust syn can parse. A read failure says
// itself and keeps its own cause, so the chain a caller prints is the one the
// tree raised.
#[derive(Debug)]
pub enum DiscoveryFailure {
    Parse { path: SourcePath, cause: Error },
    SourceRead(SourceReadFailure),
}

impl From<SourceReadFailure> for DiscoveryFailure {
    fn from(failure: SourceReadFailure) -> Self {
        Self::SourceRead(failure)
    }
}

impl fmt::Display for DiscoveryFailure {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Parse { path, .. } => write!(formatter, "failed to parse source file {path}"),
            Self::SourceRead(failure) => write!(formatter, "{failure}"),
        }
    }
}

impl error::Error for DiscoveryFailure {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match self {
            Self::Parse { cause, .. } => Some(cause),
            Self::SourceRead(failure) => failure.source(),
        }
    }
}
