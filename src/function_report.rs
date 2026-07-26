// Copyright 2025 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use serde::Serialize;

use crate::verdict::Verdict;

#[derive(Debug, Clone, Serialize)]
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
