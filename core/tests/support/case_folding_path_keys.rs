// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crap4rust::analysis::source_path::SourcePath;
use crap4rust::traits::path_keys::PathKeys;

// Every path keyed in lower case, as on a file system whose names ignore case:
// two spellings of one file share its key.
pub struct CaseFoldingPathKeys;

impl PathKeys for CaseFoldingPathKeys {
    fn key_of(&self, path: &SourcePath) -> String {
        path.as_str().to_lowercase()
    }
}
