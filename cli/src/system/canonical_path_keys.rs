// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use std::path::Path;

use crap4rust::analysis::source_path::SourcePath;
use crap4rust::traits::path_keys::PathKeys;

// A path as the file system knows it, so that the file a source walk found and
// the file a coverage export names compare equal however each was spelled: the
// canonical path where one exists, with `/` for every separator, and folded to
// lower case on Windows, whose file names ignore case.
#[derive(Debug, Clone, Copy, Default)]
pub struct CanonicalPathKeys;

impl CanonicalPathKeys {
    #[must_use]
    pub const fn new() -> Self {
        Self
    }
}

impl PathKeys for CanonicalPathKeys {
    fn key_of(&self, path: &SourcePath) -> String {
        let path = Path::new(path.as_str());
        let normalized = path
            .canonicalize()
            .unwrap_or_else(|_| path.to_path_buf())
            .to_string_lossy()
            .replace('\\', "/");
        if cfg!(windows) {
            normalized.to_lowercase()
        } else {
            normalized
        }
    }
}
