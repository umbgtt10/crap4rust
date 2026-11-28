// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use std::env::consts::EXE_SUFFIX;
use std::path::{Path, PathBuf};

// The built cargo-crap4rust every validation test runs: the profile
// directory's, in the target directory cargo puts final binaries in.
pub struct Binary {
    path: PathBuf,
}

impl Binary {
    pub fn new(target_directory: &Path, profile: &str) -> Self {
        Self {
            path: target_directory
                .join(profile)
                .join(format!("cargo-crap4rust{EXE_SUFFIX}")),
        }
    }

    pub fn path(&self) -> PathBuf {
        assert!(
            self.path.is_file(),
            "{} is not built: `cargo build -p cargo-crap4rust` builds it, as `cargo test --workspace` does",
            self.path.display()
        );
        self.path.clone()
    }
}
