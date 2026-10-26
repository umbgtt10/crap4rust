// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use alloc::boxed::Box;
use core::error::Error;
use core::fmt;

// Coverage the source could not provide, in the source's own words: it says
// what its cause says and hands on that cause's own cause, so the chain a
// caller prints is the one the source raised.
#[derive(Debug)]
pub struct CoverageFailure {
    pub cause: Box<dyn Error + Send + Sync>,
}

impl fmt::Display for CoverageFailure {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{}", self.cause)
    }
}

impl Error for CoverageFailure {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        self.cause.source()
    }
}
