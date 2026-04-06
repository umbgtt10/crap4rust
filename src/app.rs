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
        features: args.features,
        all_features: args.all_features,
        no_default_features: args.no_default_features,
        include_test_targets: args.include_test_targets,
        exclude_paths: args.exclude_path,
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
    use super::*;

    #[test]
    fn compute_crap_score_zero_coverage_returns_complexity_squared_plus_complexity() {
        let score = compute_crap_score(5, 0.0);
        assert!((score - 30.0).abs() < 0.001); // 5² × 1³ + 5 = 30
    }

    #[test]
    fn compute_crap_score_full_coverage_returns_complexity_only() {
        let score = compute_crap_score(10, 1.0);
        assert!((score - 10.0).abs() < 0.001); // 10² × 0³ + 10 = 10
    }

    #[test]
    fn compute_crap_score_half_coverage_returns_expected_value() {
        let score = compute_crap_score(4, 0.5);
        let expected = 16.0 * 0.125 + 4.0; // 4² × 0.5³ + 4 = 6.0
        assert!((score - expected).abs() < 0.001);
    }

    #[test]
    fn compute_crap_score_zero_complexity_returns_zero() {
        let score = compute_crap_score(0, 0.0);
        assert!((score - 0.0).abs() < 0.001);
    }

    #[test]
    fn compute_crap_score_complexity_one_full_coverage_returns_one() {
        let score = compute_crap_score(1, 1.0);
        assert!((score - 1.0).abs() < 0.001);
    }

    #[test]
    fn classify_above_threshold_returns_crappy() {
        assert_eq!(classify(31.0, 30.0, 20.0), Verdict::Crappy);
    }

    #[test]
    fn classify_at_threshold_returns_warn() {
        assert_eq!(classify(30.0, 30.0, 20.0), Verdict::Warn);
    }

    #[test]
    fn classify_between_warn_and_threshold_returns_warn() {
        assert_eq!(classify(25.0, 30.0, 20.0), Verdict::Warn);
    }

    #[test]
    fn classify_at_warn_threshold_returns_warn() {
        assert_eq!(classify(20.0, 30.0, 20.0), Verdict::Warn);
    }

    #[test]
    fn classify_below_warn_threshold_returns_clean() {
        assert_eq!(classify(19.9, 30.0, 20.0), Verdict::Clean);
    }

    #[test]
    fn classify_zero_score_returns_clean() {
        assert_eq!(classify(0.0, 30.0, 20.0), Verdict::Clean);
    }

    #[test]
    fn coverage_ratio_zero_total_regions_returns_zero() {
        let record = crate::model::CoverageRecord {
            path_key: String::new(),
            line: 0,
            covered_regions: 0,
            total_regions: 0,
        };
        assert!((record.coverage_ratio() - 0.0).abs() < 0.001);
    }

    #[test]
    fn coverage_ratio_half_covered_returns_half() {
        let record = crate::model::CoverageRecord {
            path_key: String::new(),
            line: 0,
            covered_regions: 5,
            total_regions: 10,
        };
        assert!((record.coverage_ratio() - 0.5).abs() < 0.001);
    }

    #[test]
    fn coverage_ratio_fully_covered_returns_one() {
        let record = crate::model::CoverageRecord {
            path_key: String::new(),
            line: 0,
            covered_regions: 10,
            total_regions: 10,
        };
        assert!((record.coverage_ratio() - 1.0).abs() < 0.001);
    }

    #[test]
    fn match_function_coverage_exact_match_returns_record() {
        let mut index = HashMap::new();
        index.insert(
            (String::from("src/lib.rs"), 10),
            crate::model::CoverageRecord {
                path_key: String::from("src/lib.rs"),
                line: 10,
                covered_regions: 3,
                total_regions: 5,
            },
        );
        let function = crate::model::SourceFunction {
            package_name: String::from("test"),
            name: String::from("foo"),
            path_key: String::from("src/lib.rs"),
            relative_file: String::from("src/lib.rs"),
            line: 10,
            end_line: 20,
            complexity: 1,
        };
        let result = match_function_coverage(&function, &index);
        assert!(result.is_some());
        assert_eq!(result.unwrap().covered_regions, 3);
    }

    #[test]
    fn match_function_coverage_fuzzy_match_within_span_returns_nearest() {
        let mut index = HashMap::new();
        index.insert(
            (String::from("src/lib.rs"), 12),
            crate::model::CoverageRecord {
                path_key: String::from("src/lib.rs"),
                line: 12,
                covered_regions: 7,
                total_regions: 10,
            },
        );
        let function = crate::model::SourceFunction {
            package_name: String::from("test"),
            name: String::from("foo"),
            path_key: String::from("src/lib.rs"),
            relative_file: String::from("src/lib.rs"),
            line: 10,
            end_line: 20,
            complexity: 1,
        };
        let result = match_function_coverage(&function, &index);
        assert!(result.is_some());
        assert_eq!(result.unwrap().covered_regions, 7);
    }

    #[test]
    fn match_function_coverage_no_match_returns_none() {
        let mut index = HashMap::new();
        index.insert(
            (String::from("src/other.rs"), 10),
            crate::model::CoverageRecord {
                path_key: String::from("src/other.rs"),
                line: 10,
                covered_regions: 1,
                total_regions: 1,
            },
        );
        let function = crate::model::SourceFunction {
            package_name: String::from("test"),
            name: String::from("foo"),
            path_key: String::from("src/lib.rs"),
            relative_file: String::from("src/lib.rs"),
            line: 10,
            end_line: 20,
            complexity: 1,
        };
        assert!(match_function_coverage(&function, &index).is_none());
    }

    #[test]
    fn project_fails_strict_with_one_crappy_returns_true() {
        let config = Config {
            coverage_path: None,
            manifest_path: None,
            packages: vec![],
            features: None,
            all_features: false,
            no_default_features: false,
            include_test_targets: false,
            exclude_paths: vec![],
            threshold: 30.0,
            warn_threshold: 20.0,
            project_threshold: 5.0,
            strict: true,
            warn_only: false,
        };
        assert!(project_fails(1, 0.5, &config));
    }

    #[test]
    fn project_fails_non_strict_below_threshold_returns_false() {
        let config = Config {
            coverage_path: None,
            manifest_path: None,
            packages: vec![],
            features: None,
            all_features: false,
            no_default_features: false,
            include_test_targets: false,
            exclude_paths: vec![],
            threshold: 30.0,
            warn_threshold: 20.0,
            project_threshold: 5.0,
            strict: false,
            warn_only: false,
        };
        assert!(!project_fails(1, 4.9, &config));
    }

    #[test]
    fn project_fails_non_strict_above_threshold_returns_true() {
        let config = Config {
            coverage_path: None,
            manifest_path: None,
            packages: vec![],
            features: None,
            all_features: false,
            no_default_features: false,
            include_test_targets: false,
            exclude_paths: vec![],
            threshold: 30.0,
            warn_threshold: 20.0,
            project_threshold: 5.0,
            strict: false,
            warn_only: false,
        };
        assert!(project_fails(2, 5.1, &config));
    }
}
