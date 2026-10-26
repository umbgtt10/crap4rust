// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::analysis::source_path::SourcePath;

#[derive(Debug, Clone)]
pub struct PackageContext {
    pub name: String,
    pub manifest_dir: SourcePath,
    pub workspace_root: SourcePath,
    pub source_roots: Vec<SourcePath>,
    pub include_test_targets: bool,
    pub exclude_paths: Vec<String>,
}
