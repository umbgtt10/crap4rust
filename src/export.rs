// Copyright 2025 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

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
