// Copyright 2025 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use std::path::PathBuf;

#[derive(Debug, Clone)]
pub struct PackageContext {
    pub name: String,
    pub manifest_dir: PathBuf,
    pub workspace_root: PathBuf,
    pub source_roots: Vec<PathBuf>,
    pub include_test_targets: bool,
    pub exclude_paths: Vec<String>,
}
