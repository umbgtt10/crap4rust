// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::invocation::config::Config;
use crate::reporting::function_report::FunctionReport;
use crate::reporting::project_report::ProjectReport;
use crate::reporting::verdict::Verdict;
use crate::traits::reporter::Reporter;

// The report as a person reads it: a table of every function that is not
// clean, or a line saying there is none, then the summary. Every line ends in
// a newline, the last one included.
#[derive(Debug, Clone, Copy, Default)]
pub struct HumanReporter;

impl HumanReporter {
    #[must_use]
    pub const fn new() -> Self {
        Self
    }

    fn summary_line(report: &ProjectReport, config: &Config) -> String {
        format!(
            "summary: total_functions={} crappy_functions={} crappy_percent={:.1}% threshold={:.1} project_threshold={:.1}% verdict={}",
            report.total_functions(),
            report.crappy_functions(),
            report.crappy_percent(),
            config.threshold(),
            config.project_threshold(),
            report.verdict().as_str(),
        )
    }

    fn table_lines(visible_functions: &[&FunctionReport]) -> Vec<String> {
        let package_width = visible_functions
            .iter()
            .map(|function| function.package_name.len())
            .max()
            .unwrap_or(7)
            .max("package".len());
        let name_width = visible_functions
            .iter()
            .map(|function| function.name.len())
            .max()
            .unwrap_or(4)
            .max("name".len());
        let file_width = visible_functions
            .iter()
            .map(|function| function.relative_file.len())
            .max()
            .unwrap_or(4)
            .max("file".len());

        let mut lines = Vec::new();
        lines.push(format!(
            "{:<package_width$}  {:<name_width$}  {:<file_width$}  {:>4}  {:>10}  {:>10}  {:>10}  {:<7}",
            "package", "name", "file", "line", "complexity", "coverage", "crap", "verdict",
        ));
        lines.push(format!(
            "{}  {}  {}  {}  {}  {}  {}  {}",
            "-".repeat(package_width),
            "-".repeat(name_width),
            "-".repeat(file_width),
            "-".repeat(4),
            "-".repeat(10),
            "-".repeat(10),
            "-".repeat(10),
            "-".repeat(7),
        ));
        for function in visible_functions {
            lines.push(format!(
                "{:<package_width$}  {:<name_width$}  {:<file_width$}  {:>4}  {:>10}  {:>9.1}%  {:>10.1}  {:<7}",
                function.package_name,
                function.name,
                function.relative_file,
                function.line,
                function.complexity,
                function.coverage * 100.0,
                function.crap_score,
                function.verdict.as_str(),
            ));
        }
        lines
    }
}

impl Reporter for HumanReporter {
    fn render(&self, report: &ProjectReport, config: &Config) -> String {
        let visible_functions = report
            .functions()
            .iter()
            .filter(|function| function.verdict != Verdict::Clean)
            .collect::<Vec<_>>();

        let mut lines = Vec::new();
        lines.push(format!("crap4rust report for {}", report.scope_name()));
        lines.push(String::new());
        if visible_functions.is_empty() {
            lines.push(format!(
                "No functions at or above the threshold of {:.1}.",
                config.threshold()
            ));
        } else {
            lines.extend(Self::table_lines(&visible_functions));
        }
        lines.push(String::new());
        lines.push(Self::summary_line(report, config));

        let mut text = lines.join("\n");
        text.push('\n');
        text
    }
}
