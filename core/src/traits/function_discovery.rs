// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use alloc::vec::Vec;

use crate::analysis::discovery_failure::DiscoveryFailure;
use crate::analysis::source_function::SourceFunction;
use crate::invocation::package_context::PackageContext;

pub trait FunctionDiscovery {
    fn discover(&self, package: &PackageContext) -> Result<Vec<SourceFunction>, DiscoveryFailure>;
}
