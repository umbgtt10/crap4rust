// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use std::path::PathBuf;

// One of the crates under fixture/, the analysis input every validation test
// points the built binary at. Located from this package's own manifest
// directory, so the tests run from any working directory.
pub struct Fixture {
    directory: PathBuf,
}

impl Fixture {
    pub fn new(segments: &[&str]) -> Self {
        let mut directory = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        directory.push("..");
        directory.push("fixture");
        for segment in segments {
            directory.push(segment);
        }
        Self { directory }
    }

    pub fn directory(&self) -> PathBuf {
        self.directory.clone()
    }
}
