// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::coverage_io::export_chunk::ExportChunk;

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Export {
    pub data: Vec<ExportChunk>,
}
