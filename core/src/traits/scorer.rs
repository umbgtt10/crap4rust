// Copyright 2025 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::config::Config;
use crate::coverage_index::CoverageIndex;
use crate::function_report::FunctionReport;
use crate::project_metrics::ProjectMetrics;
use crate::source_function::SourceFunction;

pub trait Scorer {
    fn score_functions(
        &self,
        functions: Vec<SourceFunction>,
        coverage: &CoverageIndex,
        config: &Config,
    ) -> Vec<FunctionReport>;

    fn project_metrics(&self, reports: &[FunctionReport], config: &Config) -> ProjectMetrics;
}
