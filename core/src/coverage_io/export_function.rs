// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct ExportFunction {
    pub filenames: Vec<String>,
    pub regions: Vec<Vec<u64>>,
}
