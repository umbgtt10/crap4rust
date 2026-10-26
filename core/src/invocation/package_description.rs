// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use alloc::string::String;
use alloc::vec::Vec;

use crate::analysis::source_path::SourcePath;
use crate::invocation::target_description::TargetDescription;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PackageDescription {
    pub name: String,
    pub manifest_path: SourcePath,
    pub targets: Vec<TargetDescription>,
}
