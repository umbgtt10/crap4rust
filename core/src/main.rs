// Copyright 2025 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// SPDX-License-Identifier: MIT

use crap4rust::entry_point::EntryPoint;
use std::env::args;
use std::process::ExitCode;

fn main() -> ExitCode {
    EntryPoint::run(args().collect())
}
