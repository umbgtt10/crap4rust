// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::coverage_io::export_function::ExportFunction;

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct ExportChunk {
    pub functions: Vec<ExportFunction>,
}
