// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::reporting::verdict::Verdict;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ProjectMetrics {
    verdict: Verdict,
    crappy_functions: usize,
    total_functions: usize,
    crappy_percent: f64,
}

impl ProjectMetrics {
    pub fn new(
        verdict: Verdict,
        crappy_functions: usize,
        total_functions: usize,
        crappy_percent: f64,
    ) -> Self {
        Self {
            verdict,
            crappy_functions,
            total_functions,
            crappy_percent,
        }
    }

    pub fn crappy_functions(&self) -> usize {
        self.crappy_functions
    }

    pub fn crappy_percent(&self) -> f64 {
        self.crappy_percent
    }

    pub fn total_functions(&self) -> usize {
        self.total_functions
    }

    pub fn verdict(&self) -> Verdict {
        self.verdict
    }
}
