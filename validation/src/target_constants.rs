// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use std::env::var_os;
use std::path::{Path, PathBuf};
use std::sync::LazyLock;

use crate::target_directory::TargetDirectory;

// The target directory this workspace builds into, where cargo puts the final
// binaries, asked of the cargo running the tests -- `$CARGO`, else `cargo` --
// once per test process.
pub static TARGET_DIRECTORY: LazyLock<PathBuf> = LazyLock::new(|| {
    TargetDirectory::new(
        var_os("CARGO").map_or_else(|| PathBuf::from("cargo"), PathBuf::from),
        Path::new(env!("CARGO_MANIFEST_DIR")).join("Cargo.toml"),
    )
    .locate()
});
