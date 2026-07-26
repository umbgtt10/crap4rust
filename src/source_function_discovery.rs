// Copyright 2025 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use anyhow::Result;

use crate::file_walker::FileWalker;
use crate::package_context::PackageContext;
use crate::source_function::SourceFunction;
use crate::traits::function_discovery::FunctionDiscovery;

#[derive(Debug, Clone, Copy, Default)]
pub struct SourceFunctionDiscovery;

impl SourceFunctionDiscovery {
    #[must_use]
    pub const fn new() -> Self {
        Self
    }
}

impl FunctionDiscovery for SourceFunctionDiscovery {
    fn discover(&self, package: &PackageContext) -> Result<Vec<SourceFunction>> {
        discover_functions(package)
    }
}

pub fn discover_functions(package: &PackageContext) -> Result<Vec<SourceFunction>> {
    let walker = FileWalker::new(package);
    let mut functions = Vec::new();
    for source_root in &package.source_roots {
        functions.extend(walker.process_source_root(source_root)?);
    }
    Ok(functions)
}
