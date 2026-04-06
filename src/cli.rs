// Copyright 2025 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License or Apache License, Version 2.0
// SPDX-License-Identifier: MIT OR Apache-2.0

use std::path::PathBuf;

use clap::Parser;

#[derive(Debug, Clone, Parser)]
#[command(name = "crap4rust")]
#[command(about = "Compute CRAP scores for Rust functions")]
pub struct Args {
    #[arg(long)]
    pub coverage: Option<PathBuf>,
    #[arg(long)]
    pub manifest_path: Option<PathBuf>,
    #[arg(long)]
    pub package: Vec<String>,
    #[arg(long)]
    pub features: Option<String>,
    #[arg(long, default_value_t = false)]
    pub all_features: bool,
    #[arg(long, default_value_t = false)]
    pub no_default_features: bool,
    #[arg(long, default_value_t = false)]
    pub include_test_targets: bool,
    #[arg(long)]
    pub exclude_path: Vec<String>,
    #[arg(long, default_value_t = 30.0)]
    pub threshold: f64,
    #[arg(long, default_value_t = 5.0)]
    pub project_threshold: f64,
    #[arg(long, default_value_t = false)]
    pub strict: bool,
    #[arg(long, default_value_t = false)]
    pub warn_only: bool,
}

impl Args {
    pub fn parse_args() -> Self {
        Self::parse()
    }

    pub fn parse_from_args<I, T>(args: I) -> Self
    where
        I: IntoIterator<Item = T>,
        T: Into<std::ffi::OsString> + Clone,
    {
        Self::parse_from(args)
    }
}
