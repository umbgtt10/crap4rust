// Copyright 2025 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use anyhow::Result;

use crate::package_context::PackageContext;
use crate::source_function::SourceFunction;

pub trait FunctionDiscovery {
    fn discover(&self, package: &PackageContext) -> Result<Vec<SourceFunction>>;
}
