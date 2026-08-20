// Copyright 2025 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::invocation::app::run_from_args;
use std::process::ExitCode;

// Everything `main` used to do, moved somewhere a test can reach it.
//
// A binary entry point is not reachable from an integration test, so logic
// living in `main.rs` is logic nothing can check. What was sitting there is not
// glue: it is the argv fixup every cargo subcommand needs, and it has two
// properties that are easy to get wrong and impossible to notice.
//
// Cargo runs `cargo crap4rust ...` as `cargo-crap4rust crap4rust ...`, so the
// name arrives twice. Running the binary directly does not repeat it, which is
// why the strip is **conditional**. And it is **positional** -- only argv[1] is
// dropped -- because removing every occurrence would swallow
// `--package crap4rust`.
pub struct EntryPoint;

impl EntryPoint {
    pub const SUBCOMMAND: &'static str = "crap4rust";

    // Conditional and positional: only a repeated name in argv[1] goes.
    pub fn without_cargo_subcommand(args: Vec<String>) -> Vec<String> {
        if args.get(1).map(String::as_str) != Some(Self::SUBCOMMAND) {
            return args;
        }
        let mut forwarded = Vec::with_capacity(args.len().saturating_sub(1));
        let mut rest = args.into_iter();
        if let Some(binary) = rest.next() {
            forwarded.push(binary);
        }
        forwarded.extend(rest.skip(1));
        forwarded
    }

    // An error is exit 2, the same code a broken rule uses, because a run that
    // could not finish must never read as a clean project.
    pub fn run(args: Vec<String>) -> ExitCode {
        match run_from_args(Self::without_cargo_subcommand(args)) {
            Ok(code) => code,
            Err(error) => {
                println!("error: {error:#}");
                ExitCode::from(2)
            }
        }
    }
}
