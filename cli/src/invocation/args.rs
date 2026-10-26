// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use std::ffi::OsString;
use std::path::Path;
use std::path::PathBuf;

use clap::Parser;
use crap4rust::invocation::config::Config;

use crate::invocation::cargo_config::CargoConfig;
use crate::invocation::output_format::OutputFormat;

#[derive(Debug, Clone, Parser)]
#[command(name = "crap4rust")]
#[command(about = "Compute CRAP scores for Rust functions")]
pub struct Args {
    #[arg(long)]
    coverage: Option<PathBuf>,
    #[arg(long)]
    manifest_path: Option<PathBuf>,
    #[arg(long)]
    package: Vec<String>,
    #[arg(long)]
    features: Option<String>,
    #[arg(long, default_value_t = false)]
    all_features: bool,
    #[arg(long, default_value_t = false)]
    no_default_features: bool,
    #[arg(long, default_value_t = false)]
    include_test_targets: bool,
    #[arg(long)]
    exclude_path: Vec<String>,
    #[arg(long, default_value_t = 30.0)]
    threshold: f64,
    #[arg(long, default_value_t = 5.0)]
    project_threshold: f64,
    #[arg(long, default_value_t = false)]
    strict: bool,
    #[arg(long, default_value_t = false)]
    warn_only: bool,
    #[arg(long, default_value_t = 20.0)]
    warn_threshold: f64,
    #[arg(long, default_value_t = OutputFormat::Human, value_enum)]
    output_format: OutputFormat,
}

impl Args {
    pub fn parse_from_args<I, T>(args: I) -> Self
    where
        I: IntoIterator<Item = T>,
        T: Into<OsString> + Clone,
    {
        Self::parse_from(args)
    }

    pub fn all_features(&self) -> bool {
        self.all_features
    }

    pub fn cargo_config(&self) -> CargoConfig {
        CargoConfig {
            coverage_path: self.coverage.clone(),
            manifest_path: self.manifest_path.clone(),
            features: self.features.clone(),
            all_features: self.all_features,
            no_default_features: self.no_default_features,
        }
    }

    pub fn config(&self) -> Config {
        Config::default()
            .with_packages(self.package.clone())
            .with_include_test_targets(self.include_test_targets)
            .with_exclude_paths(self.exclude_path.clone())
            .with_threshold(self.threshold)
            .with_warn_threshold(self.warn_threshold)
            .with_project_threshold(self.project_threshold)
            .with_strict(self.strict)
            .with_warn_only(self.warn_only)
    }

    pub fn coverage(&self) -> Option<&Path> {
        self.coverage.as_deref()
    }

    pub fn exclude_path(&self) -> &[String] {
        &self.exclude_path
    }

    pub fn features(&self) -> Option<&str> {
        self.features.as_deref()
    }

    pub fn include_test_targets(&self) -> bool {
        self.include_test_targets
    }

    pub fn manifest_path(&self) -> Option<&Path> {
        self.manifest_path.as_deref()
    }

    pub fn no_default_features(&self) -> bool {
        self.no_default_features
    }

    pub fn output_format(&self) -> OutputFormat {
        self.output_format
    }

    pub fn package(&self) -> &[String] {
        &self.package
    }

    pub fn project_threshold(&self) -> f64 {
        self.project_threshold
    }

    pub fn strict(&self) -> bool {
        self.strict
    }

    pub fn threshold(&self) -> f64 {
        self.threshold
    }

    pub fn warn_only(&self) -> bool {
        self.warn_only
    }

    pub fn warn_threshold(&self) -> f64 {
        self.warn_threshold
    }
}
