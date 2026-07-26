// Copyright 2025 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License or Apache License, Version 2.0
// SPDX-License-Identifier: MIT OR Apache-2.0

use std::process::ExitCode;

use serde::Serialize;

use crate::config::Config;
use crate::function_report::FunctionReport;
use crate::verdict::Verdict;

#[derive(Debug, Clone, Serialize)]
pub struct ProjectReport {
    pub scope_name: String,
    pub total_functions: usize,
    pub crappy_functions: usize,
    pub crappy_percent: f64,
    pub verdict: Verdict,
    pub functions: Vec<FunctionReport>,
}

impl ProjectReport {
    #[must_use]
    pub fn exit_code(&self, config: &Config) -> ExitCode {
        if config.warn_only {
            return ExitCode::SUCCESS;
        }
        if config.fails(self.crappy_functions, self.crappy_percent) {
            ExitCode::from(1)
        } else {
            ExitCode::SUCCESS
        }
    }
}
