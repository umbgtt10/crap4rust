// Copyright 2025 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License or Apache License, Version 2.0
// SPDX-License-Identifier: MIT OR Apache-2.0

use anyhow::Result;

use crate::package_context::PackageContext;
use crate::source_function::SourceFunction;

pub trait FunctionDiscovery {
    fn discover(&self, package: &PackageContext) -> Result<Vec<SourceFunction>>;
}
