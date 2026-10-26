// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crap4rust::invocation::config::Config;
use crap4rust::reporting::function_report::FunctionReport;
use crap4rust::reporting::project_report::ProjectReport;
use crap4rust::reporting::verdict::Verdict;
use crap4rust::traits::reporter::Reporter;
use serde::Serialize;
use serde_json::to_string_pretty;

// The report as a document, for a gate script: every function the report
// covers, clean ones included, each verdict by its variant name, and a newline
// after the closing brace. The records below are the document's shape, field
// for field in the order it has always had.
#[derive(Debug, Clone, Copy, Default)]
pub struct JsonReporter;

impl JsonReporter {
    #[must_use]
    pub const fn new() -> Self {
        Self
    }

    fn document(report: &ProjectReport) -> JsonProjectReport<'_> {
        JsonProjectReport {
            scope_name: report.scope_name(),
            total_functions: report.total_functions(),
            crappy_functions: report.crappy_functions(),
            crappy_percent: report.crappy_percent(),
            verdict: Self::verdict(report.verdict()),
            functions: report.functions().iter().map(Self::function).collect(),
        }
    }

    fn function(function: &FunctionReport) -> JsonFunctionReport<'_> {
        JsonFunctionReport {
            package_name: &function.package_name,
            name: &function.name,
            relative_file: &function.relative_file,
            line: function.line,
            complexity: function.complexity,
            coverage: function.coverage,
            crap_score: function.crap_score,
            verdict: Self::verdict(function.verdict),
        }
    }

    fn verdict(verdict: Verdict) -> &'static str {
        match verdict {
            Verdict::Clean => "Clean",
            Verdict::Warn => "Warn",
            Verdict::Crappy => "Crappy",
        }
    }
}

impl Reporter for JsonReporter {
    fn render(&self, report: &ProjectReport, _config: &Config) -> String {
        let json = to_string_pretty(&Self::document(report))
            .expect("serialization of report to JSON should never fail");
        format!("{json}\n")
    }
}

#[derive(Serialize)]
struct JsonProjectReport<'a> {
    pub scope_name: &'a str,
    pub total_functions: usize,
    pub crappy_functions: usize,
    pub crappy_percent: f64,
    pub verdict: &'static str,
    pub functions: Vec<JsonFunctionReport<'a>>,
}

#[derive(Serialize)]
struct JsonFunctionReport<'a> {
    pub package_name: &'a str,
    pub name: &'a str,
    pub relative_file: &'a str,
    pub line: usize,
    pub complexity: u32,
    pub coverage: f64,
    pub crap_score: f64,
    pub verdict: &'static str,
}
