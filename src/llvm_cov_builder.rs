// Copyright 2025 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License or Apache License, Version 2.0
// SPDX-License-Identifier: MIT OR Apache-2.0

use std::path::Path;
use std::process::{Command, Stdio};

use anyhow::{Context, Result, bail};

use crate::model::{Config, PackageContext};

pub struct LlvmCovBuilder {
    command: Command,
}

impl LlvmCovBuilder {
    pub fn new(output_path: &Path) -> Self {
        let mut command = Command::new("cargo");
        command.arg("llvm-cov");
        command.arg("--json");
        command.arg("--output-path");
        command.arg(output_path);
        Self { command }
    }

    pub(crate) fn apply_config(mut self, config: &Config) -> Self {
        if let Some(manifest_path) = &config.manifest_path {
            self.command.arg("--manifest-path");
            self.command.arg(manifest_path);
        }
        if let Some(features) = &config.features {
            self.command.arg("--features");
            self.command.arg(features);
        }
        if config.all_features {
            self.command.arg("--all-features");
        }
        if config.no_default_features {
            self.command.arg("--no-default-features");
        }
        self
    }

    pub(crate) fn add_packages(mut self, packages: &[PackageContext]) -> Self {
        for package in packages {
            self.command.arg("--package");
            self.command.arg(&package.name);
        }
        self
    }

    pub(crate) fn execute(mut self) -> Result<()> {
        self.command.stderr(Stdio::null());
        let status = self
            .command
            .status()
            .context("failed to invoke cargo llvm-cov; ensure cargo-llvm-cov is installed")?;
        if !status.success() {
            bail!("cargo llvm-cov failed with exit code {:?}", status.code());
        }
        Ok(())
    }
}
