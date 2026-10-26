// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct ExportFunction {
    pub filenames: Vec<String>,
    pub regions: Vec<Vec<u64>>,
}
