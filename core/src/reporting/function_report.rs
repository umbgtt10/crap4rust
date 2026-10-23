// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use serde::Serialize;

use crate::reporting::verdict::Verdict;

#[derive(Debug, Clone, Serialize, Default)]
pub struct FunctionReport {
    package_name: String,
    name: String,
    relative_file: String,
    line: usize,
    complexity: u32,
    coverage: f64,
    crap_score: f64,
    verdict: Verdict,
}

impl FunctionReport {
    pub fn complexity(&self) -> u32 {
        self.complexity
    }

    pub fn coverage(&self) -> f64 {
        self.coverage
    }

    pub fn crap_score(&self) -> f64 {
        self.crap_score
    }

    pub fn line(&self) -> usize {
        self.line
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn package_name(&self) -> &str {
        &self.package_name
    }

    pub fn relative_file(&self) -> &str {
        &self.relative_file
    }

    pub fn verdict(&self) -> Verdict {
        self.verdict
    }

    pub fn with_complexity(self, complexity: u32) -> Self {
        Self { complexity, ..self }
    }

    pub fn with_coverage(self, coverage: f64) -> Self {
        Self { coverage, ..self }
    }

    pub fn with_crap_score(self, crap_score: f64) -> Self {
        Self { crap_score, ..self }
    }

    pub fn with_line(self, line: usize) -> Self {
        Self { line, ..self }
    }

    pub fn with_name(self, name: String) -> Self {
        Self { name, ..self }
    }

    pub fn with_package_name(self, package_name: String) -> Self {
        Self {
            package_name,
            ..self
        }
    }

    pub fn with_relative_file(self, relative_file: String) -> Self {
        Self {
            relative_file,
            ..self
        }
    }

    pub fn with_verdict(self, verdict: Verdict) -> Self {
        Self { verdict, ..self }
    }
}
