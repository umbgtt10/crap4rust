// Copyright 2025 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use std::collections::BTreeSet;
use std::path::{Path, PathBuf};

use anyhow::{Context, Result, bail};
use cargo_metadata::{Metadata, MetadataCommand, Package};

use crate::config::Config;
use crate::package_context::PackageContext;
use crate::source_root_collector::SourceRootCollector;
use crate::traits::package_resolver::PackageResolver;

#[derive(Debug, Clone, Copy, Default)]
pub struct CargoPackageResolver;

impl CargoPackageResolver {
    #[must_use]
    pub const fn new() -> Self {
        Self
    }

    fn build_package_context(
        package: &Package,
        workspace_root: &Path,
        include_test_targets: bool,
        exclude_paths: Vec<String>,
    ) -> Result<PackageContext> {
        let manifest_dir = package
            .manifest_path
            .clone()
            .into_std_path_buf()
            .parent()
            .map(PathBuf::from)
            .context("package manifest has no parent directory")?;

        let mut collector = SourceRootCollector::new(include_test_targets, &manifest_dir);
        collector.collect(&package.targets);
        let source_roots = collector.finalize();

        Ok(PackageContext {
            name: package.name.to_string(),
            manifest_dir,
            workspace_root: workspace_root.to_path_buf(),
            source_roots,
            include_test_targets,
            exclude_paths,
        })
    }

    fn select_packages<'a>(
        metadata: &'a Metadata,
        requested: &[String],
    ) -> Result<Vec<&'a Package>> {
        if !requested.is_empty() {
            let mut selected = Vec::new();
            for package_name in requested {
                let package = metadata
                    .packages
                    .iter()
                    .find(|package| package.name == package_name)
                    .with_context(|| {
                        format!("package {package_name} was not found in the manifest")
                    })?;
                selected.push(package);
            }
            return Ok(selected);
        }

        if let Some(root) = metadata.root_package()
            && metadata.workspace_members.len() <= 1
        {
            return Ok(vec![root]);
        }

        let workspace_member_ids = metadata
            .workspace_members
            .iter()
            .cloned()
            .collect::<BTreeSet<_>>();
        let selected = metadata
            .packages
            .iter()
            .filter(|package| workspace_member_ids.contains(&package.id))
            .collect::<Vec<_>>();
        if selected.is_empty() {
            bail!("manifest contains no workspace member packages");
        }

        Ok(selected)
    }
}

impl PackageResolver for CargoPackageResolver {
    fn resolve(&self, config: &Config) -> Result<Vec<PackageContext>> {
        let mut command = MetadataCommand::new();
        command.no_deps();
        if let Some(manifest_path) = &config.manifest_path {
            command.manifest_path(manifest_path);
        }

        let metadata = command.exec().context("failed to read Cargo metadata")?;
        let workspace_root = metadata.workspace_root.clone().into_std_path_buf();
        let packages = Self::select_packages(&metadata, &config.packages)?;

        packages
            .into_iter()
            .map(|package| {
                Self::build_package_context(
                    package,
                    &workspace_root,
                    config.include_test_targets,
                    config.exclude_paths.clone(),
                )
            })
            .collect()
    }
}
