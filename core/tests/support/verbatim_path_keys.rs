// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crap4rust::analysis::source_path::SourcePath;
use crap4rust::traits::path_keys::PathKeys;

// Every path is its own key, as on a file system with no links and one
// spelling per file.
pub struct VerbatimPathKeys;

impl PathKeys for VerbatimPathKeys {
    fn key_of(&self, path: &SourcePath) -> String {
        String::from(path.as_str())
    }
}
