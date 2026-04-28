// Copyright 2025 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License or Apache License, Version 2.0
// SPDX-License-Identifier: MIT OR Apache-2.0

use anyhow::Result;

use crate::file_walker::FileWalker;
use crate::model::{PackageContext, SourceFunction};

pub use crate::file_walker::normalize_path;

pub fn discover_functions(package: &PackageContext) -> Result<Vec<SourceFunction>> {
    let walker = FileWalker::new(package);
    let mut functions = Vec::new();
    for source_root in &package.source_roots {
        functions.extend(walker.process_source_root(source_root)?);
    }
    Ok(functions)
}
