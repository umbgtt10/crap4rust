// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use alloc::vec::Vec;

use crate::invocation::config::Config;
use crate::invocation::package_context::PackageContext;
use crate::invocation::run_failure::RunFailure;

pub trait PackageResolver {
    fn resolve(&self, config: &Config) -> Result<Vec<PackageContext>, RunFailure>;
}
