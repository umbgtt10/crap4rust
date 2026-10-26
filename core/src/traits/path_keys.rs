// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use alloc::string::String;

use crate::analysis::source_path::SourcePath;

pub trait PathKeys {
    fn key_of(&self, path: &SourcePath) -> String;
}
