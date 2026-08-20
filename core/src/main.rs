// Copyright 2025 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crap4rust::app::run_from_args;
use std::env::args;
use std::process::ExitCode;

fn main() -> ExitCode {
    let args = args().collect::<Vec<_>>();
    let forwarded_args = if args.get(1).map(String::as_str) == Some("crap4rust") {
        let mut forwarded = Vec::with_capacity(args.len().saturating_sub(1));
        if let Some(binary) = args.first() {
            forwarded.push(binary.clone());
        }
        forwarded.extend(args.into_iter().skip(2));
        forwarded
    } else {
        args
    };

    match run_from_args(forwarded_args) {
        Ok(code) => code,
        Err(error) => {
            println!("error: {error:#}");
            ExitCode::from(2)
        }
    }
}
