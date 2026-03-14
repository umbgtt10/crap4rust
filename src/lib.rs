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
