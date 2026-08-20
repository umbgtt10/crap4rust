// Copyright 2025 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use anyhow::Result;

use crate::invocation::config::Config;
use crate::invocation::package_context::PackageContext;

pub trait PackageResolver {
    fn resolve(&self, config: &Config) -> Result<Vec<PackageContext>>;
}
