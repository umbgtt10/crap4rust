// Copyright 2025 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License or Apache License, Version 2.0
// SPDX-License-Identifier: MIT OR Apache-2.0

use crate::config::Config;
use crate::output_format::OutputFormat;
use crate::project_report::ProjectReport;
use crate::traits::reporter::Reporter;
use crate::verdict::Verdict;

#[derive(Debug, Clone, Copy, Default)]
pub struct StdoutReporter;

impl StdoutReporter {
    #[must_use]
    pub const fn new() -> Self {
        Self
    }

    fn print_human_report(&self, report: &ProjectReport, config: &Config) {
        println!("crap4rust report for {}", report.scope_name);
        println!();

        let visible_functions = report
            .functions
            .iter()
            .filter(|function| function.verdict != Verdict::Clean)
            .collect::<Vec<_>>();

        if visible_functions.is_empty() {
            println!(
                "No functions at or above the threshold of {:.1}.",
                config.threshold
            );
        } else {
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

            println!(
                "{:<package_width$}  {:<name_width$}  {:<file_width$}  {:>4}  {:>10}  {:>10}  {:>10}  {:<7}",
                "package", "name", "file", "line", "complexity", "coverage", "crap", "verdict",
            );
            println!(
                "{}  {}  {}  {}  {}  {}  {}  {}",
                "-".repeat(package_width),
                "-".repeat(name_width),
                "-".repeat(file_width),
                "-".repeat(4),
                "-".repeat(10),
                "-".repeat(10),
                "-".repeat(10),
                "-".repeat(7),
            );

            for function in visible_functions {
                println!(
                    "{:<package_width$}  {:<name_width$}  {:<file_width$}  {:>4}  {:>10}  {:>9.1}%  {:>10.1}  {:<7}",
                    function.package_name,
                    function.name,
                    function.relative_file,
                    function.line,
                    function.complexity,
                    function.coverage * 100.0,
                    function.crap_score,
                    function.verdict.as_str(),
                );
            }
        }

        println!();
        println!(
            "summary: total_functions={} crappy_functions={} crappy_percent={:.1}% threshold={:.1} project_threshold={:.1}% verdict={}",
            report.total_functions,
            report.crappy_functions,
            report.crappy_percent,
            config.threshold,
            config.project_threshold,
            report.verdict.as_str(),
        );
    }

    fn print_json_report(&self, report: &ProjectReport) {
        let json = serde_json::to_string_pretty(report)
            .expect("serialization of report to JSON should never fail");
        println!("{json}");
    }
}

impl Reporter for StdoutReporter {
    fn render(&self, report: &ProjectReport, config: &Config) {
        match config.output_format {
            OutputFormat::Json => self.print_json_report(report),
            OutputFormat::Human => self.print_human_report(report, config),
        }
    }
}
