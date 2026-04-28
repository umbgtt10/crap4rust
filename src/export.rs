// Copyright 2025 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License or Apache License, Version 2.0
// SPDX-License-Identifier: MIT OR Apache-2.0

use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub(crate) struct Export {
    pub(crate) data: Vec<ExportChunk>,
}

#[derive(Debug, Deserialize)]
pub(crate) struct ExportChunk {
    pub(crate) functions: Vec<ExportFunction>,
}

#[derive(Debug, Deserialize)]
pub(crate) struct ExportFunction {
    pub(crate) filenames: Vec<String>,
    pub(crate) regions: Vec<Vec<u64>>,
}
