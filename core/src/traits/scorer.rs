// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::analysis::source_function::SourceFunction;
use crate::coverage_io::coverage_index::CoverageIndex;
use crate::invocation::config::Config;
use crate::reporting::function_report::FunctionReport;
use crate::reporting::project_metrics::ProjectMetrics;

pub trait Scorer {
    fn score_functions(
        &self,
        functions: Vec<SourceFunction>,
        coverage: &CoverageIndex,
        config: &Config,
    ) -> Vec<FunctionReport>;

    fn project_metrics(&self, reports: &[FunctionReport], config: &Config) -> ProjectMetrics;
}
