// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use std::path::PathBuf;
use std::process::Command;

use serde_json::Value;
use serde_json::from_slice;

// The target directory cargo builds a package into, as `cargo metadata` names
// it: where the final binaries go, wherever `build.build-dir` moves the rest
// of the build, and wherever `CARGO_TARGET_DIR` or `build.target-dir` puts
// it. Asked from the manifest's directory, so its `.cargo/config.toml` counts.
// Neither `CARGO_BIN_EXE_<name>`, set only for the binary's own package,
// `CARGO_TARGET_TMPDIR`, which is in the build directory, nor where the test
// executable runs from is read.
pub struct TargetDirectory {
    cargo: PathBuf,
    manifest_path: PathBuf,
}

impl TargetDirectory {
    pub fn new(cargo: PathBuf, manifest_path: PathBuf) -> Self {
        Self {
            cargo,
            manifest_path,
        }
    }

    pub fn locate(&self) -> PathBuf {
        let mut command = Command::new(&self.cargo);
        command
            .args([
                "metadata",
                "--format-version",
                "1",
                "--no-deps",
                "--manifest-path",
            ])
            .arg(&self.manifest_path);
        if let Some(directory) = self.manifest_path.parent() {
            command.current_dir(directory);
        }
        let output = command
            .output()
            .unwrap_or_else(|cause| panic!("{} could not run: {cause}", self.cargo.display()));
        assert!(
            output.status.success(),
            "cargo metadata failed for {}: {}",
            self.manifest_path.display(),
            String::from_utf8_lossy(&output.stderr)
        );
        let metadata: Value = from_slice(&output.stdout).expect("cargo metadata prints JSON");
        PathBuf::from(
            metadata["target_directory"]
                .as_str()
                .expect("cargo metadata names the target directory"),
        )
    }
}
