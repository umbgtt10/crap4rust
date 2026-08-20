// Copyright 2025 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use std::path::PathBuf;

use crate::invocation::cli::Args;
use crate::reporting::output_format::OutputFormat;

#[derive(Debug, Clone)]
pub struct Config {
    pub coverage_path: Option<PathBuf>,
    pub manifest_path: Option<PathBuf>,
    pub packages: Vec<String>,
    pub features: Option<String>,
    pub all_features: bool,
    pub no_default_features: bool,
    pub include_test_targets: bool,
    pub exclude_paths: Vec<String>,
    pub threshold: f64,
    pub warn_threshold: f64,
    pub project_threshold: f64,
    pub strict: bool,
    pub warn_only: bool,
    pub output_format: OutputFormat,
}

impl Config {
    #[must_use]
    pub fn from_args(args: Args) -> Self {
        Self {
            coverage_path: args.coverage,
            manifest_path: args.manifest_path,
            packages: args.package,
            features: args.features,
            all_features: args.all_features,
            no_default_features: args.no_default_features,
            include_test_targets: args.include_test_targets,
            exclude_paths: args.exclude_path,
            threshold: args.threshold,
            warn_threshold: args.warn_threshold,
            project_threshold: args.project_threshold,
            strict: args.strict,
            warn_only: args.warn_only,
            output_format: args.output_format,
        }
    }

    #[must_use]
    pub fn fails(&self, crappy_functions: usize, crappy_percent: f64) -> bool {
        if self.strict {
            crappy_functions > 0
        } else {
            crappy_percent > self.project_threshold
        }
    }
}
