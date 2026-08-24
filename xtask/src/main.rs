// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use std::env::args;
use std::path::Path;
use std::process::ExitCode;
use xtask::crap::crap_report_parser::CrapReportParser;
use xtask::gates::crap_gate::CrapGate;
use xtask::gates::gate::Gate;
use xtask::gates::iceberg_gate::IcebergGate;
use xtask::gates::stage2::Stage2;
use xtask::gates::stern_gate::SternGate;
use xtask::gates::twin_gate::TwinGate;
use xtask::process::system_command_runner::SystemCommandRunner;

const PACKAGE: &str = "cargo-crap4rust";
const CRAP_THRESHOLD: &str = "15";
const ICEBERG_THRESHOLD: &str = "15.3";

// Reading the real process argv and wiring the concrete runner are the two
// things no test can reach, so they are all this binary does.
fn main() -> ExitCode {
    match args().nth(1).as_deref() {
        Some("stage2") => run_stage2(),
        _ => {
            eprintln!("usage: cargo xtask stage2");
            ExitCode::FAILURE
        }
    }
}

fn run_stage2() -> ExitCode {
    let workspace_manifest = manifest_path(&["Cargo.toml"]);
    // The three measuring gates are pointed at core/ rather than the workspace
    // root, which is what keeps them off the validation crate: those tests drive
    // the built binary end to end and have no source file in core/ to mirror.
    let core_manifest = manifest_path(&["core", "Cargo.toml"]);

    let runner = SystemCommandRunner::new();
    let parser = CrapReportParser::new();
    let packages = vec![String::from(PACKAGE)];

    let stern = SternGate::new(&runner, workspace_manifest, packages.clone());
    let crap = CrapGate::new(
        &runner,
        &parser,
        core_manifest.clone(),
        packages.clone(),
        String::from(CRAP_THRESHOLD),
    );
    let twin = TwinGate::new(&runner, core_manifest.clone(), packages.clone());
    let iceberg = IcebergGate::new(
        &runner,
        core_manifest,
        packages,
        String::from(ICEBERG_THRESHOLD),
    );

    let gates: Vec<&dyn Gate> = vec![&stern, &crap, &twin, &iceberg];

    match Stage2::new(gates).run() {
        Ok(()) => {
            println!("\ncrap4rust Stage 2 passed!");
            ExitCode::SUCCESS
        }
        Err(reason) => {
            eprintln!("\nFailed: {reason}");
            ExitCode::FAILURE
        }
    }
}

fn workspace_root() -> &'static Path {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("xtask lives one directory below the workspace root")
}

fn manifest_path(segments: &[&str]) -> String {
    segments
        .iter()
        .fold(workspace_root().to_path_buf(), |path, segment| {
            path.join(segment)
        })
        .to_string_lossy()
        .into_owned()
}
