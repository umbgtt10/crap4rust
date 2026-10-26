// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use alloc::string::String;
use alloc::vec;
use alloc::vec::Vec;

use crate::analysis::source_path::SourcePath;
use crate::analysis::source_root_collector::SourceRootCollector;
use crate::invocation::config::Config;
use crate::invocation::package_context::PackageContext;
use crate::invocation::package_description::PackageDescription;
use crate::invocation::run_failure::RunFailure;
use crate::invocation::workspace_description::WorkspaceDescription;
use crate::traits::package_resolver::PackageResolver;
use crate::traits::workspace_reader::WorkspaceReader;

// Cargo's rules for which packages a run covers, applied to the workspace the
// reader describes: the ones asked for, in the order asked; else the root
// package of a workspace of one; else every member.
pub struct CargoPackageResolver<'a> {
    workspace_reader: &'a dyn WorkspaceReader,
}

impl<'a> CargoPackageResolver<'a> {
    #[must_use]
    pub fn new(workspace_reader: &'a dyn WorkspaceReader) -> Self {
        Self { workspace_reader }
    }

    fn build_package_context(
        package: &PackageDescription,
        workspace_root: &SourcePath,
        include_test_targets: bool,
        exclude_paths: Vec<String>,
    ) -> Result<PackageContext, RunFailure> {
        let manifest_dir = package
            .manifest_path
            .parent()
            .ok_or(RunFailure::NoParentDirectory)?;

        let mut collector = SourceRootCollector::new(include_test_targets, &manifest_dir);
        collector.collect(&package.targets);
        let source_roots = collector.finalize();

        Ok(PackageContext {
            name: package.name.clone(),
            manifest_dir,
            workspace_root: workspace_root.clone(),
            source_roots,
            include_test_targets,
            exclude_paths,
        })
    }

    fn select_packages<'w>(
        workspace: &'w WorkspaceDescription,
        requested: &[String],
    ) -> Result<Vec<&'w PackageDescription>, RunFailure> {
        if !requested.is_empty() {
            let mut selected = Vec::new();
            for package_name in requested {
                let package = workspace
                    .packages
                    .iter()
                    .find(|package| &package.name == package_name)
                    .ok_or_else(|| RunFailure::PackageNotFound(package_name.clone()))?;
                selected.push(package);
            }
            return Ok(selected);
        }

        if let Some(root) = workspace.root_package.as_ref().and_then(|root| {
            workspace
                .packages
                .iter()
                .find(|package| &package.name == root)
        }) && workspace.packages.len() <= 1
        {
            return Ok(vec![root]);
        }

        let selected = workspace.packages.iter().collect::<Vec<_>>();
        if selected.is_empty() {
            return Err(RunFailure::NoWorkspaceMembers);
        }

        Ok(selected)
    }
}

impl PackageResolver for CargoPackageResolver<'_> {
    fn resolve(&self, config: &Config) -> Result<Vec<PackageContext>, RunFailure> {
        let workspace = self.workspace_reader.describe()?;
        let packages = Self::select_packages(&workspace, config.packages())?;

        packages
            .into_iter()
            .map(|package| {
                Self::build_package_context(
                    package,
                    &workspace.workspace_root,
                    config.include_test_targets(),
                    config.exclude_paths().to_vec(),
                )
            })
            .collect()
    }
}
