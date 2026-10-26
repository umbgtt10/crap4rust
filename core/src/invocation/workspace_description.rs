// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use alloc::string::String;
use alloc::vec::Vec;

use crate::analysis::source_path::SourcePath;
use crate::invocation::package_description::PackageDescription;

// The workspace's members, as `cargo metadata --no-deps` lists them: it lists
// nothing else.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WorkspaceDescription {
    pub workspace_root: SourcePath,
    pub packages: Vec<PackageDescription>,
    pub root_package: Option<String>,
}
