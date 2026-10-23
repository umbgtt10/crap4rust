// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct ExportFunction {
    filenames: Vec<String>,
    regions: Vec<Vec<u64>>,
}

impl ExportFunction {
    pub fn filenames(&self) -> &[String] {
        &self.filenames
    }

    pub fn regions(&self) -> &[Vec<u64>] {
        &self.regions
    }
}
