// Copyright 2025 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License or Apache License, Version 2.0
// SPDX-License-Identifier: MIT OR Apache-2.0

use std::collections::HashMap;
use std::process::ExitCode;

use anyhow::{Context, Result, bail};

use crate::cli::Args;
use crate::coverage;
use crate::manifest;
use crate::model::{Config, FunctionReport, ProjectReport, Verdict};
use crate::report;
use crate::source;

pub fn run(args: Args) -> Result<ExitCode> {
    let config = Config {
        coverage_path: args.coverage,
        manifest_path: args.manifest_path,
        packages: args.package,
        threshold: args.threshold,
        warn_threshold: 20.0,
        project_threshold: args.project_threshold,
        strict: args.strict,
        warn_only: args.warn_only,
    };

    let packages = manifest::resolve_packages(&config)?;
    let coverage_path = coverage::ensure_coverage_path(&config, &packages)?;
    let mut functions = Vec::new();
    for package in &packages {
        let mut package_functions = source::discover_functions(package)
            .with_context(|| format!("failed to discover functions in package {}", package.name))?;
        functions.append(&mut package_functions);
    }
    if functions.is_empty() {
        bail!("no Rust functions were discovered in the selected packages");
    }

    let coverage_records = coverage::load_coverage_records(&coverage_path)?;
    if coverage_records.is_empty() {
        bail!("coverage file did not contain any function records");
    }

    let mut coverage_index: HashMap<(String, usize), crate::model::CoverageRecord> = HashMap::new();
    for record in coverage_records {
        let key = (record.path_key.clone(), record.line);
        coverage_index
            .entry(key)
            .and_modify(|existing| {
                existing.covered_regions += record.covered_regions;
                existing.total_regions += record.total_regions;
            })
            .or_insert(record);
    }

    let matched_count = functions
        .iter()
        .filter(|function| match_function_coverage(function, &coverage_index).is_some())
        .count();
    if matched_count == 0 {
        bail!(
            "coverage data could not be matched to any discovered function by file path and line"
        );
    }

    let mut reports = functions
        .into_iter()
        .map(|function| {
            let coverage = match_function_coverage(&function, &coverage_index)
                .map_or(0.0, |record| record.coverage_ratio());
            let crap_score = compute_crap_score(function.complexity, coverage);
            let verdict = classify(crap_score, config.threshold, config.warn_threshold);

            FunctionReport {
                package_name: function.package_name,
                name: function.name,
                relative_file: function.relative_file,
                line: function.line,
                complexity: function.complexity,
                coverage,
                crap_score,
                verdict,
            }
        })
        .collect::<Vec<_>>();

    reports.sort_by(|left, right| {
        right
            .crap_score
            .partial_cmp(&left.crap_score)
            .unwrap_or(std::cmp::Ordering::Equal)
            .then_with(|| left.name.cmp(&right.name))
    });

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
    let project_verdict = if project_fails(crappy_functions, crappy_percent, &config) {
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

    let report_data = ProjectReport {
        scope_name: packages
            .iter()
            .map(|package| package.name.clone())
            .collect::<Vec<_>>()
            .join(", "),
        total_functions,
        crappy_functions,
        crappy_percent,
        verdict: project_verdict,
        functions: reports,
    };

    report::print_report(&report_data, &config);

    Ok(determine_exit_code(&report_data, &config))
}

fn classify(score: f64, threshold: f64, warn_threshold: f64) -> Verdict {
    if score > threshold {
        Verdict::Crappy
    } else if score >= warn_threshold {
        Verdict::Warn
    } else {
        Verdict::Clean
    }
}

fn match_function_coverage(
    function: &crate::model::SourceFunction,
    coverage_index: &HashMap<(String, usize), crate::model::CoverageRecord>,
) -> Option<crate::model::CoverageRecord> {
    if let Some(record) = coverage_index.get(&(function.path_key.clone(), function.line)) {
        return Some(record.clone());
    }

    coverage_index
        .iter()
        .filter(|((path_key, line), _)| {
            path_key == &function.path_key && *line >= function.line && *line <= function.end_line
        })
        .min_by_key(|((_, line), _)| *line - function.line)
        .map(|(_, record)| record.clone())
}

fn compute_crap_score(complexity: u32, coverage: f64) -> f64 {
    let complexity = f64::from(complexity);
    complexity.powi(2) * (1.0 - coverage).powi(3) + complexity
}

fn determine_exit_code(report: &ProjectReport, config: &Config) -> ExitCode {
    if config.warn_only {
        return ExitCode::SUCCESS;
    }

    if project_fails(report.crappy_functions, report.crappy_percent, config) {
        ExitCode::from(1)
    } else {
        ExitCode::SUCCESS
    }
}

fn project_fails(crappy_functions: usize, crappy_percent: f64, config: &Config) -> bool {
    if config.strict {
        crappy_functions > 0
    } else {
        crappy_percent > config.project_threshold
    }
}

#[cfg(test)]
mod tests {
    use super::compute_crap_score;

    #[test]
    fn compute_crap_score_full_coverage_returns_complexity() {
        let score = compute_crap_score(10, 1.0);

        assert!((score - 10.0).abs() < f64::EPSILON);
    }

    #[test]
    fn compute_crap_score_zero_coverage_returns_expected_value() {
        let score = compute_crap_score(10, 0.0);

        assert!((score - 110.0).abs() < f64::EPSILON);
    }
}
