// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use std::fs;
use std::path::{Path, PathBuf};

use anyhow::{Context, Result};
use crap4rust::coverage_io::coverage_failure::CoverageFailure;
use crap4rust::coverage_io::export::Export;
use crap4rust::coverage_io::export_chunk::ExportChunk;
use crap4rust::coverage_io::export_function::ExportFunction;
use crap4rust::invocation::package_context::PackageContext;
use crap4rust::traits::coverage_source::CoverageSource;
use serde::Deserialize;
use serde_json::from_str;

use crate::invocation::cargo_config::CargoConfig;
use crate::system::llvm_cov_builder::LlvmCovBuilder;
use crate::system::portable_path::PortablePath;

// cargo-llvm-cov's JSON export: the file `--coverage` names, or else one
// generated for the selected packages under the workspace's
// target/crap4rust/. Read and parsed here, handed to the core as its own
// records with every file name in the core's `/` form.
pub struct LlvmCovSource {
    config: CargoConfig,
}

impl LlvmCovSource {
    #[must_use]
    pub fn new(config: CargoConfig) -> Self {
        Self { config }
    }

    fn chunk(chunk: JsonExportChunk) -> ExportChunk {
        ExportChunk {
            functions: chunk.functions.into_iter().map(Self::function).collect(),
        }
    }

    fn ensure_coverage_path(&self, packages: &[PackageContext]) -> Result<PathBuf> {
        if let Some(path) = &self.config.coverage_path {
            return Ok(path.clone());
        }

        let workspace_root = packages
            .first()
            .map(|package| PathBuf::from(package.workspace_root.as_str()))
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
            .apply_config(&self.config)
            .add_packages(packages)
            .execute()?;

        Ok(output_path)
    }

    fn function(function: JsonExportFunction) -> ExportFunction {
        ExportFunction {
            filenames: function
                .filenames
                .iter()
                .map(|filename| String::from(PortablePath::of_text(filename).as_str()))
                .collect(),
            regions: function.regions,
        }
    }

    fn load(path: &Path) -> Result<Export> {
        let contents = fs::read_to_string(path)
            .with_context(|| format!("failed to read coverage file {}", path.display()))?;
        let export: JsonExport =
            from_str(&contents).context("failed to parse cargo-llvm-cov JSON")?;
        Ok(Export {
            data: export.data.into_iter().map(Self::chunk).collect(),
        })
    }
}

impl CoverageSource for LlvmCovSource {
    fn export(&self, packages: &[PackageContext]) -> Result<Export, CoverageFailure> {
        self.ensure_coverage_path(packages)
            .and_then(|path| Self::load(&path))
            .map_err(|error| CoverageFailure {
                cause: error.into(),
            })
    }
}

#[derive(Deserialize)]
struct JsonExport {
    pub data: Vec<JsonExportChunk>,
}

#[derive(Deserialize)]
struct JsonExportChunk {
    pub functions: Vec<JsonExportFunction>,
}

#[derive(Deserialize)]
struct JsonExportFunction {
    pub filenames: Vec<String>,
    pub regions: Vec<Vec<u64>>,
}
