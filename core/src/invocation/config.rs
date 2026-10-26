// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use alloc::string::String;
use alloc::vec::Vec;

#[derive(Debug, Clone, Default)]
pub struct Config {
    packages: Vec<String>,
    include_test_targets: bool,
    exclude_paths: Vec<String>,
    threshold: f64,
    warn_threshold: f64,
    project_threshold: f64,
    strict: bool,
    warn_only: bool,
}

impl Config {
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

    pub fn include_test_targets(&self) -> bool {
        self.include_test_targets
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

    pub fn with_exclude_paths(self, exclude_paths: Vec<String>) -> Self {
        Self {
            exclude_paths,
            ..self
        }
    }

    pub fn with_include_test_targets(self, include_test_targets: bool) -> Self {
        Self {
            include_test_targets,
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
