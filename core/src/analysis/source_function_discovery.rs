// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use alloc::vec::Vec;

use crate::analysis::discovery_failure::DiscoveryFailure;
use crate::analysis::file_walker::FileWalker;
use crate::analysis::source_function::SourceFunction;
use crate::invocation::package_context::PackageContext;
use crate::traits::function_discovery::FunctionDiscovery;
use crate::traits::path_keys::PathKeys;
use crate::traits::source_tree::SourceTree;

pub struct SourceFunctionDiscovery<'a> {
    source_tree: &'a dyn SourceTree,
    path_keys: &'a dyn PathKeys,
}

impl<'a> SourceFunctionDiscovery<'a> {
    #[must_use]
    pub fn new(source_tree: &'a dyn SourceTree, path_keys: &'a dyn PathKeys) -> Self {
        Self {
            source_tree,
            path_keys,
        }
    }
}

impl FunctionDiscovery for SourceFunctionDiscovery<'_> {
    fn discover(&self, package: &PackageContext) -> Result<Vec<SourceFunction>, DiscoveryFailure> {
        let walker = FileWalker::new(package, self.source_tree, self.path_keys);
        let mut functions = Vec::new();
        for source_root in &package.source_roots {
            functions.extend(walker.process_source_root(source_root)?);
        }
        Ok(functions)
    }
}
