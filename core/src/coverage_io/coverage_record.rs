// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

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
            let covered: f64 = self.covered_regions.into();
            let total: f64 = self.total_regions.into();
            covered / total
        }
    }
}
