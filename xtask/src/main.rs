// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use std::env::args;
use std::path::Path;
use std::process::ExitCode;
use xtask::crap::crap_report_parser::CrapReportParser;
use xtask::gates::crap_gate::CrapGate;
use xtask::gates::dry_gate::DryGate;
use xtask::gates::gate::Gate;
use xtask::gates::iceberg_gate::IcebergGate;
use xtask::gates::shear_gate::ShearGate;
use xtask::gates::stage2::Stage2;
use xtask::gates::stern_gate::SternGate;
use xtask::gates::twin_gate::TwinGate;
use xtask::process::system_command_runner::SystemCommandRunner;

const PACKAGE: &str = "cargo-crap4rust";
const VALIDATION_PACKAGE: &str = "validation";
const XTASK_PACKAGE: &str = "xtask";
const CRAP_THRESHOLD: &str = "15";
const ICEBERG_THRESHOLD: &str = "15.3";
const DRY_PATH: &str = "core/src";
const DRY_BASELINE: &str = "dry4rust-baseline.json";
const DRY_MIN_NODES: &str = "25";

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

    // The house rules reach every member, which is what a bare hand-run of
    // `cargo stern4rust` at the root scans too: xtask, so the crate that runs
    // the gates is held to them, and validation, whose tests are each named for
    // the apparatus in its src/ they are built on. The measuring gates below
    // cannot follow: they are scoped to core/ to stay off validation, and xtask
    // lives outside that manifest.
    let stern = SternGate::new(
        &runner,
        workspace_manifest.clone(),
        vec![
            String::from(PACKAGE),
            String::from(VALIDATION_PACKAGE),
            String::from(XTASK_PACKAGE),
        ],
    );

    // Second: every declared dependency used, in the section that uses it. Its
    // corrections edit manifests only, so they move nothing the gates behind it
    // measure, and it costs a second.
    let shear = ShearGate::new(&runner, workspace_manifest);

    // Third, for the same reason stern4rust is first: removing a duplicate
    // moves code between files, which changes what the three behind it measure.
    // The published crate's source only -- tests repeat their arrangement by
    // design -- against a baseline of what was already duplicated when the gate
    // arrived, so it fails on what a change adds rather than on what it
    // inherited. The floor is the one the family gates at, and the one the
    // baseline was recorded at: a baseline matches only at its own floor.
    let root = workspace_root();
    let dry = DryGate::new(
        &runner,
        root.join(DRY_PATH).to_string_lossy().into_owned(),
        root.join(DRY_BASELINE).to_string_lossy().into_owned(),
        String::from(DRY_MIN_NODES),
    );

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

    let gates: Vec<&dyn Gate> = vec![&stern, &shear, &dry, &crap, &twin, &iceberg];

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
