// Copyright 2025 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License or Apache License, Version 2.0
// SPDX-License-Identifier: MIT OR Apache-2.0

use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

use anyhow::{Context, Result, bail};
use serde::Deserialize;

use crate::model::{Config, CoverageRecord, PackageContext};
use crate::source::normalize_path;

#[derive(Debug, Deserialize)]
struct Export {
    data: Vec<ExportChunk>,
}

#[derive(Debug, Deserialize)]
struct ExportChunk {
    functions: Vec<ExportFunction>,
}

#[derive(Debug, Deserialize)]
struct ExportFunction {
    filenames: Vec<String>,
    regions: Vec<Vec<u64>>,
}

pub fn ensure_coverage_path(config: &Config, packages: &[PackageContext]) -> Result<PathBuf> {
    if let Some(path) = &config.coverage_path {
        return Ok(path.clone());
    }

    let workspace_root = packages
        .first()
        .map(|package| package.workspace_root.clone())
        .context("no packages were selected for coverage generation")?;

    let output_dir = workspace_root.join("target").join("crap4rust");
    fs::create_dir_all(&output_dir).with_context(|| {
        format!(
            "failed to create coverage output directory {}",
            output_dir.display()
        )
    })?;

    let output_path = output_dir.join(format!(
        "{}-coverage.json",
        packages
            .iter()
            .map(|package| package.name.replace('-', "_"))
            .collect::<Vec<_>>()
            .join("__")
    ));
    let mut command = Command::new("cargo");
    command.arg("llvm-cov");
    command.arg("--json");
    command.arg("--output-path");
    command.arg(&output_path);

    if let Some(manifest_path) = &config.manifest_path {
        command.arg("--manifest-path");
        command.arg(manifest_path);
    }

    for package in packages {
        command.arg("--package");
        command.arg(&package.name);
    }

    let status = command
        .status()
        .context("failed to invoke cargo llvm-cov; ensure cargo-llvm-cov is installed")?;
    if !status.success() {
        bail!("cargo llvm-cov failed with exit code {:?}", status.code());
    }

    Ok(output_path)
}

pub fn load_coverage_records(path: &Path) -> Result<Vec<CoverageRecord>> {
    let contents = fs::read_to_string(path)
        .with_context(|| format!("failed to read coverage file {}", path.display()))?;
    let export: Export =
        serde_json::from_str(&contents).context("failed to parse cargo-llvm-cov JSON")?;

    let mut records = Vec::new();
    for chunk in export.data {
        for function in chunk.functions {
            let Some(filename) = function.filenames.first() else {
                continue;
            };
            let Some(first_region) = function.regions.first() else {
                continue;
            };
            if first_region.len() < 5 {
                continue;
            }

            let total_regions = function.regions.len() as u32;
            let covered_regions = function
                .regions
                .iter()
                .filter(|region| region.get(4).copied().unwrap_or(0) > 0)
                .count() as u32;

            records.push(CoverageRecord {
                path_key: normalize_path(Path::new(filename)),
                line: first_region[0] as usize,
                covered_regions,
                total_regions,
            });
        }
    }

    Ok(records)
}
