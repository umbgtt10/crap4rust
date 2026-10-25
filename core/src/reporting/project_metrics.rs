// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::reporting::verdict::Verdict;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ProjectMetrics {
    pub verdict: Verdict,
    pub crappy_functions: usize,
    pub total_functions: usize,
    pub crappy_percent: f64,
}
