// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::invocation::config::Config;
use crate::reporting::function_report::FunctionReport;
use crate::reporting::verdict::Verdict;

#[derive(Debug, Clone)]
pub struct ProjectReport {
    scope_name: String,
    total_functions: usize,
    crappy_functions: usize,
    crappy_percent: f64,
    verdict: Verdict,
    functions: Vec<FunctionReport>,
}

impl ProjectReport {
    pub fn new(
        scope_name: String,
        total_functions: usize,
        crappy_functions: usize,
        crappy_percent: f64,
        verdict: Verdict,
        functions: Vec<FunctionReport>,
    ) -> Self {
        Self {
            scope_name,
            total_functions,
            crappy_functions,
            crappy_percent,
            verdict,
            functions,
        }
    }

    pub fn crappy_functions(&self) -> usize {
        self.crappy_functions
    }

    pub fn crappy_percent(&self) -> f64 {
        self.crappy_percent
    }

    #[must_use]
    pub fn exit_code(&self, config: &Config) -> u8 {
        if config.warn_only() {
            return 0;
        }
        if config.fails(self.crappy_functions, self.crappy_percent) {
            1
        } else {
            0
        }
    }

    pub fn functions(&self) -> &[FunctionReport] {
        &self.functions
    }

    pub fn scope_name(&self) -> &str {
        &self.scope_name
    }

    pub fn total_functions(&self) -> usize {
        self.total_functions
    }

    pub fn verdict(&self) -> Verdict {
        self.verdict
    }
}
