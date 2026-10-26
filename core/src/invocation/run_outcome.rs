// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use alloc::string::String;

// What a run hands back: the report to print, and the code to exit with -- 0
// for a project that passes, 1 for one that fails. Printing it is the caller's.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RunOutcome {
    pub output: String,
    pub exit_code: u8,
}
