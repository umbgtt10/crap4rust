// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crap4rust::invocation::entry_point::EntryPoint;
use std::env::args;
use std::process::ExitCode;

fn main() -> ExitCode {
    EntryPoint::run(args().collect())
}
