// Copyright 2025 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Export {
    pub data: Vec<ExportChunk>,
}

#[derive(Debug, Deserialize)]
pub struct ExportChunk {
    pub functions: Vec<ExportFunction>,
}

#[derive(Debug, Deserialize)]
pub struct ExportFunction {
    pub filenames: Vec<String>,
    pub regions: Vec<Vec<u64>>,
}
