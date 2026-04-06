// Copyright 2025 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License or Apache License, Version 2.0
// SPDX-License-Identifier: MIT OR Apache-2.0

use std::collections::BTreeSet;
use std::path::{Path, PathBuf};

use anyhow::{Context, Result, bail};
use cargo_metadata::{Metadata, MetadataCommand, Package};

use crate::model::{Config, PackageContext};

pub fn resolve_packages(config: &Config) -> Result<Vec<PackageContext>> {
    let mut command = MetadataCommand::new();
    command.no_deps();
    if let Some(manifest_path) = &config.manifest_path {
        command.manifest_path(manifest_path);
    }

    let metadata = command.exec().context("failed to read Cargo metadata")?;
    let workspace_root = metadata.workspace_root.clone().into_std_path_buf();
    let packages = select_packages(&metadata, &config.packages)?;

    packages
        .into_iter()
        .map(|package| {
            build_package_context(
                package,
                &workspace_root,
                config.include_test_targets,
                config.exclude_paths.clone(),
            )
        })
        .collect()
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

    let mut source_roots = BTreeSet::new();
    for target in &package.targets {
        if !is_selected_target(target, include_test_targets) {
            continue;
        }

        if target
            .src_path
            .extension()
            .is_some_and(|extension| extension == "rs")
        {
            let path = target.src_path.clone().into_std_path_buf();
            if let Some(parent) = path.parent() {
                source_roots.insert(parent.to_path_buf());
            }
        }
    }

    if source_roots.is_empty() {
        source_roots.insert(manifest_dir.join("src"));
    }

    Ok(PackageContext {
        name: package.name.to_string(),
        manifest_dir,
        workspace_root: workspace_root.to_path_buf(),
        source_roots: source_roots.into_iter().collect(),
        include_test_targets,
        exclude_paths,
    })
}

fn is_selected_target(target: &cargo_metadata::Target, include_test_targets: bool) -> bool {
    let kinds = target
        .kind
        .iter()
        .map(|kind| kind.to_string())
        .collect::<Vec<_>>();

    if kinds.iter().any(|kind| kind == "custom-build") {
        return false;
    }

    if include_test_targets {
        return kinds.iter().any(|kind| {
            matches!(
                kind.as_str(),
                "lib" | "bin" | "proc-macro" | "rlib" | "dylib" | "cdylib" | "staticlib" | "test"
            )
        });
    }

    if kinds
        .iter()
        .any(|kind| matches!(kind.as_str(), "test" | "bench" | "example"))
    {
        return false;
    }

    kinds.iter().any(|kind| {
        matches!(
            kind.as_str(),
            "lib" | "bin" | "proc-macro" | "rlib" | "dylib" | "cdylib" | "staticlib"
        )
    })
}

fn select_packages<'a>(metadata: &'a Metadata, requested: &[String]) -> Result<Vec<&'a Package>> {
    if !requested.is_empty() {
        let mut selected = Vec::new();
        for package_name in requested {
            let package = metadata
                .packages
                .iter()
                .find(|package| package.name == package_name)
                .with_context(|| format!("package {package_name} was not found in the manifest"))?;
            selected.push(package);
        }
        return Ok(selected);
    }

    if let Some(root) = metadata.root_package() {
        return Ok(vec![root]);
    }

    bail!("manifest contains multiple packages; pass --package <name>")
}
