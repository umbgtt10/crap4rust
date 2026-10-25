// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use anyhow::Result;

use crate::analysis::file_walker::FileWalker;
use crate::analysis::source_function::SourceFunction;
use crate::invocation::package_context::PackageContext;
use crate::traits::function_discovery::FunctionDiscovery;

#[derive(Debug, Clone, Copy, Default)]
pub struct SourceFunctionDiscovery;

impl SourceFunctionDiscovery {
    #[must_use]
    pub const fn new() -> Self {
        Self
    }

    fn discover_functions(package: &PackageContext) -> Result<Vec<SourceFunction>> {
        let walker = FileWalker::new(package);
        let mut functions = Vec::new();
        for source_root in &package.source_roots {
            functions.extend(walker.process_source_root(source_root)?);
        }
        Ok(functions)
    }
}

impl FunctionDiscovery for SourceFunctionDiscovery {
    fn discover(&self, package: &PackageContext) -> Result<Vec<SourceFunction>> {
        Self::discover_functions(package)
    }
}
