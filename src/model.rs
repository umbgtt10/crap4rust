// Copyright 2025 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License or Apache License, Version 2.0
// SPDX-License-Identifier: MIT OR Apache-2.0

use std::path::PathBuf;

#[derive(Debug, Clone)]
pub struct Config {
    pub coverage_path: Option<PathBuf>,
    pub manifest_path: Option<PathBuf>,
    pub packages: Vec<String>,
    pub threshold: f64,
    pub warn_threshold: f64,
    pub project_threshold: f64,
    pub strict: bool,
    pub warn_only: bool,
}

#[derive(Debug, Clone)]
pub struct PackageContext {
    pub name: String,
    pub manifest_dir: PathBuf,
    pub workspace_root: PathBuf,
    pub source_roots: Vec<PathBuf>,
}

#[derive(Debug, Clone)]
pub struct SourceFunction {
    pub package_name: String,
    pub name: String,
    pub path_key: String,
    pub relative_file: String,
    pub line: usize,
    pub end_line: usize,
    pub complexity: u32,
}

#[derive(Debug, Clone)]
pub struct CoverageRecord {
    pub path_key: String,
    pub line: usize,
    pub covered_regions: u32,
    pub total_regions: u32,
}

impl CoverageRecord {
    pub fn coverage_ratio(&self) -> f64 {
        if self.total_regions == 0 {
            0.0
        } else {
            f64::from(self.covered_regions) / f64::from(self.total_regions)
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Verdict {
    Clean,
    Warn,
    Crappy,
}

impl Verdict {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Clean => "clean",
            Self::Warn => "warn",
            Self::Crappy => "crappy",
        }
    }
}

#[derive(Debug, Clone)]
pub struct FunctionReport {
    pub package_name: String,
    pub name: String,
    pub relative_file: String,
    pub line: usize,
    pub complexity: u32,
    pub coverage: f64,
    pub crap_score: f64,
    pub verdict: Verdict,
}

#[derive(Debug, Clone)]
pub struct ProjectReport {
    pub scope_name: String,
    pub total_functions: usize,
    pub crappy_functions: usize,
    pub crappy_percent: f64,
    pub verdict: Verdict,
    pub functions: Vec<FunctionReport>,
}
