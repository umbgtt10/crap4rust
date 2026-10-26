// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use std::path::PathBuf;

// What a run hands to cargo rather than to the analysis: the manifest that
// `cargo metadata` and `cargo llvm-cov` read, the features the coverage build
// turns on, and a coverage file that makes generating one unnecessary. Paths
// stay `PathBuf`s, so one that is not UTF-8 reaches cargo unchanged.
#[derive(Debug, Clone, Default)]
pub struct CargoConfig {
    pub coverage_path: Option<PathBuf>,
    pub manifest_path: Option<PathBuf>,
    pub features: Option<String>,
    pub all_features: bool,
    pub no_default_features: bool,
}
