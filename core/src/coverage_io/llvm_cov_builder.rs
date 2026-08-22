// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use std::path::Path;
use std::process::{Command, Stdio};

use anyhow::{Context, Result, bail};

use crate::invocation::config::Config;
use crate::invocation::package_context::PackageContext;

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

    // The arguments built so far, in the order cargo will receive them. The
    // builder wraps a Command, whose arguments are otherwise write-only, so
    // without this there is no way to assert on what a configuration produced
    // short of running cargo llvm-cov for real.
    #[must_use]
    pub fn arguments(&self) -> Vec<String> {
        self.command
            .get_args()
            .map(|argument| argument.to_string_lossy().into_owned())
            .collect()
    }

    pub fn apply_config(mut self, config: &Config) -> Self {
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
