// Copyright 2025 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use std::cmp::Ordering;

use crate::config::Config;
use crate::coverage_index::CoverageIndex;
use crate::crap_formula::CrapFormula;
use crate::function_report::FunctionReport;
use crate::project_metrics::ProjectMetrics;
use crate::source_function::SourceFunction;
use crate::traits::scorer::Scorer;
use crate::verdict::Verdict;

#[derive(Debug, Clone, Copy, Default)]
pub struct DefaultScorer {
    formula: CrapFormula,
}

impl DefaultScorer {
    #[must_use]
    pub const fn new() -> Self {
        Self {
            formula: CrapFormula::new(),
        }
    }

    fn score_function(
        &self,
        function: SourceFunction,
        coverage: &CoverageIndex,
        config: &Config,
    ) -> FunctionReport {
        let coverage_ratio = coverage.match_function(&function).unwrap_or(0.0);
        let crap_score = self.formula.score(function.complexity, coverage_ratio);
        let verdict = self
            .formula
            .classify(crap_score, config.threshold, config.warn_threshold);

        FunctionReport {
            package_name: function.package_name,
            name: function.name,
            relative_file: function.relative_file,
            line: function.line,
            complexity: function.complexity,
            coverage: coverage_ratio,
            crap_score,
            verdict,
        }
    }
}

impl Scorer for DefaultScorer {
    fn score_functions(
        &self,
        functions: Vec<SourceFunction>,
        coverage: &CoverageIndex,
        config: &Config,
    ) -> Vec<FunctionReport> {
        let mut reports = functions
            .into_iter()
            .map(|function| self.score_function(function, coverage, config))
            .collect::<Vec<_>>();

        reports.sort_by(|left, right| {
            right
                .crap_score
                .partial_cmp(&left.crap_score)
                .unwrap_or(Ordering::Equal)
                .then_with(|| left.name.cmp(&right.name))
        });

        reports
    }

    fn project_metrics(&self, reports: &[FunctionReport], config: &Config) -> ProjectMetrics {
        let crappy_functions = reports
            .iter()
            .filter(|function| function.verdict == Verdict::Crappy)
            .count();
        let total_functions = reports.len();
        let crappy_percent = if total_functions == 0 {
            0.0
        } else {
            (crappy_functions as f64 / total_functions as f64) * 100.0
        };
        let verdict = if config.fails(crappy_functions, crappy_percent) {
            Verdict::Crappy
        } else if crappy_functions > 0
            || reports
                .iter()
                .any(|function| function.verdict == Verdict::Warn)
        {
            Verdict::Warn
        } else {
            Verdict::Clean
        };

        ProjectMetrics {
            verdict,
            crappy_functions,
            total_functions,
            crappy_percent,
        }
    }
}
