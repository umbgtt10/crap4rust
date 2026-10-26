// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use std::path::Path;
use std::path::PathBuf;

use crate::reporting::output_format::OutputFormat;

#[derive(Debug, Clone, Default)]
pub struct Config {
    coverage_path: Option<PathBuf>,
    manifest_path: Option<PathBuf>,
    packages: Vec<String>,
    features: Option<String>,
    all_features: bool,
    no_default_features: bool,
    include_test_targets: bool,
    exclude_paths: Vec<String>,
    threshold: f64,
    warn_threshold: f64,
    project_threshold: f64,
    strict: bool,
    warn_only: bool,
    output_format: OutputFormat,
}

impl Config {
    pub fn all_features(&self) -> bool {
        self.all_features
    }

    pub fn coverage_path(&self) -> Option<&Path> {
        self.coverage_path.as_deref()
    }

    pub fn exclude_paths(&self) -> &[String] {
        &self.exclude_paths
    }

    #[must_use]
    pub fn fails(&self, crappy_functions: usize, crappy_percent: f64) -> bool {
        if self.strict {
            crappy_functions > 0
        } else {
            crappy_percent > self.project_threshold
        }
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

    pub fn packages(&self) -> &[String] {
        &self.packages
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

    pub fn with_all_features(self, all_features: bool) -> Self {
        Self {
            all_features,
            ..self
        }
    }

    pub fn with_coverage_path(self, coverage_path: Option<PathBuf>) -> Self {
        Self {
            coverage_path,
            ..self
        }
    }

    pub fn with_exclude_paths(self, exclude_paths: Vec<String>) -> Self {
        Self {
            exclude_paths,
            ..self
        }
    }

    pub fn with_features(self, features: Option<String>) -> Self {
        Self { features, ..self }
    }

    pub fn with_include_test_targets(self, include_test_targets: bool) -> Self {
        Self {
            include_test_targets,
            ..self
        }
    }

    pub fn with_manifest_path(self, manifest_path: Option<PathBuf>) -> Self {
        Self {
            manifest_path,
            ..self
        }
    }

    pub fn with_no_default_features(self, no_default_features: bool) -> Self {
        Self {
            no_default_features,
            ..self
        }
    }

    pub fn with_output_format(self, output_format: OutputFormat) -> Self {
        Self {
            output_format,
            ..self
        }
    }

    pub fn with_packages(self, packages: Vec<String>) -> Self {
        Self { packages, ..self }
    }

    pub fn with_project_threshold(self, project_threshold: f64) -> Self {
        Self {
            project_threshold,
            ..self
        }
    }

    pub fn with_strict(self, strict: bool) -> Self {
        Self { strict, ..self }
    }

    pub fn with_threshold(self, threshold: f64) -> Self {
        Self { threshold, ..self }
    }

    pub fn with_warn_only(self, warn_only: bool) -> Self {
        Self { warn_only, ..self }
    }

    pub fn with_warn_threshold(self, warn_threshold: f64) -> Self {
        Self {
            warn_threshold,
            ..self
        }
    }
}
