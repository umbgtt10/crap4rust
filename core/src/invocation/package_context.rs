// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use std::path::Path;
use std::path::PathBuf;

#[derive(Debug, Clone)]
pub struct PackageContext {
    name: String,
    manifest_dir: PathBuf,
    workspace_root: PathBuf,
    source_roots: Vec<PathBuf>,
    include_test_targets: bool,
    exclude_paths: Vec<String>,
}

impl PackageContext {
    pub fn new(
        name: String,
        manifest_dir: PathBuf,
        workspace_root: PathBuf,
        source_roots: Vec<PathBuf>,
        include_test_targets: bool,
        exclude_paths: Vec<String>,
    ) -> Self {
        Self {
            name,
            manifest_dir,
            workspace_root,
            source_roots,
            include_test_targets,
            exclude_paths,
        }
    }

    pub fn exclude_paths(&self) -> &[String] {
        &self.exclude_paths
    }

    pub fn include_test_targets(&self) -> bool {
        self.include_test_targets
    }

    pub fn manifest_dir(&self) -> &Path {
        &self.manifest_dir
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn source_roots(&self) -> &[PathBuf] {
        &self.source_roots
    }

    pub fn workspace_root(&self) -> &Path {
        &self.workspace_root
    }
}
