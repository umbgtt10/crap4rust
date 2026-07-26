// Copyright 2025 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License or Apache License, Version 2.0
// SPDX-License-Identifier: MIT OR Apache-2.0

use crate::verdict::Verdict;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ProjectMetrics {
    pub verdict: Verdict,
    pub crappy_functions: usize,
    pub total_functions: usize,
    pub crappy_percent: f64,
}
