// Copyright 2025 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use std::fs;
use std::path::{Path, PathBuf};

use anyhow::{Context, Result};

use crate::config::Config;
use crate::coverage_record::CoverageRecord;
use crate::export::Export;
use crate::llvm_cov_builder::LlvmCovBuilder;
use crate::normalize_path::normalize_path;
use crate::package_context::PackageContext;

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
    LlvmCovBuilder::new(&output_path)
        .apply_config(config)
        .add_packages(packages)
        .execute()?;

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
