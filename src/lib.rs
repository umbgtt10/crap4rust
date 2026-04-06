// Copyright 2025 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License or Apache License, Version 2.0
// SPDX-License-Identifier: MIT OR Apache-2.0

pub mod app;
pub mod cli;
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

pub fn run_from_args<I, T>(args: I) -> Result<ExitCode>
where
    I: IntoIterator<Item = T>,
    T: Into<std::ffi::OsString> + Clone,
{
    let args = <cli::Args as clap::Parser>::parse_from(args);
    app::run(args)
}
