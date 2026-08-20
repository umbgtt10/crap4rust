// Copyright 2025 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::verdict::Verdict;

#[derive(Debug, Clone, Copy, Default)]
pub struct CrapFormula;

impl CrapFormula {
    #[must_use]
    pub const fn new() -> Self {
        Self
    }

    #[must_use]
    pub fn score(&self, complexity: u32, coverage: f64) -> f64 {
        let complexity: f64 = complexity.into();
        complexity.powi(2) * (1.0 - coverage).powi(3) + complexity
    }

    #[must_use]
    pub fn classify(&self, score: f64, threshold: f64, warn_threshold: f64) -> Verdict {
        if score > threshold {
            Verdict::Crappy
        } else if score >= warn_threshold {
            Verdict::Warn
        } else {
            Verdict::Clean
        }
    }
}
