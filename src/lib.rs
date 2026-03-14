// Copyright 2025 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License or Apache License, Version 2.0
// SPDX-License-Identifier: MIT OR Apache-2.0

mod app;
mod cli;
mod coverage;
mod manifest;
mod model;
mod report;
mod source;

use std::process::ExitCode;

use anyhow::Result;

pub fn run() -> Result<ExitCode> {
    let args = cli::Args::parse_args();
    app::run(args)
}
