// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use alloc::string::String;

#[derive(Debug, Clone)]
pub struct CoverageRecord {
    path_key: String,
    line: usize,
    covered_regions: u32,
    total_regions: u32,
}

impl CoverageRecord {
    pub fn new(path_key: String, line: usize, covered_regions: u32, total_regions: u32) -> Self {
        Self {
            path_key,
            line,
            covered_regions,
            total_regions,
        }
    }

    pub fn coverage_ratio(&self) -> f64 {
        if self.total_regions == 0 {
            0.0
        } else {
            let covered: f64 = self.covered_regions.into();
            let total: f64 = self.total_regions.into();
            covered / total
        }
    }

    pub fn covered_regions(&self) -> u32 {
        self.covered_regions
    }

    pub fn line(&self) -> usize {
        self.line
    }

    pub fn path_key(&self) -> &str {
        &self.path_key
    }

    pub fn total_regions(&self) -> u32 {
        self.total_regions
    }
}
