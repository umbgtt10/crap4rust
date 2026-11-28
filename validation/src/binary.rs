// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use std::env::consts::EXE_SUFFIX;
use std::path::{Path, PathBuf};

// The built cargo-crap4rust every validation test runs. cargo names a target
// directory to an integration test as `CARGO_TARGET_TMPDIR`, its `tmp`
// directory, and builds the binary into the profile's directory beside it.
// Neither `CARGO_BIN_EXE_<name>`, which cargo sets only for the binary's own
// package, nor where the test executable runs from, which cargo 1.100 moved,
// is read.
pub struct Binary {
    path: PathBuf,
}

impl Binary {
    pub fn new(target_tmpdir: &Path, profile: &str) -> Self {
        let target_dir = target_tmpdir
            .parent()
            .expect("CARGO_TARGET_TMPDIR is a directory inside the target directory");
        Self {
            path: target_dir
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
