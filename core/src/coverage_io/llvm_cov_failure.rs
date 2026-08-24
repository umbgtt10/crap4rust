// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

const MISSING_SUBCOMMAND_MARKER: &str = "no such command";

pub struct LlvmCovFailure {
    exit_code: Option<i32>,
    stderr: String,
}

impl LlvmCovFailure {
    pub fn new(exit_code: Option<i32>, stderr: String) -> Self {
        Self { exit_code, stderr }
    }

    // cargo runs and exits non-zero when the subcommand is absent, so a missing
    // cargo-llvm-cov never reaches the spawn error that used to carry the
    // install hint. What distinguishes it is cargo's own message.
    pub fn is_missing_subcommand(&self) -> bool {
        self.stderr.contains(MISSING_SUBCOMMAND_MARKER)
    }

    pub fn describe(&self) -> String {
        if self.is_missing_subcommand() {
            return String::from(
                "cargo-llvm-cov is not installed, and crap4rust needs it to measure coverage. \
                 Install it with: cargo install cargo-llvm-cov",
            );
        }

        let trimmed = self.stderr.trim();
        if trimmed.is_empty() {
            return format!("cargo llvm-cov failed with exit code {:?}", self.exit_code);
        }

        format!(
            "cargo llvm-cov failed with exit code {:?}: {trimmed}",
            self.exit_code
        )
    }
}
