// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crap4rust_cli::invocation::entry_point::EntryPoint;
use serde_json::Value;
use serde_json::from_slice;
use serde_json::json;
use serde_json::to_string;
use std::fs;
use std::process::Command;
use std::process::ExitCode;
use tempfile::TempDir;

#[test]
fn run_against_a_manifest_that_does_not_exist_returns_the_error_code() {
    // Arrange
    let args = [
        "cargo-crap4rust",
        "crap4rust",
        "--manifest-path",
        "no/such/directory/Cargo.toml",
    ]
    .map(String::from)
    .to_vec();

    // Act
    let code = EntryPoint::run(args);

    // Assert
    assert_eq!(format!("{code:?}"), format!("{:?}", ExitCode::from(2)));
}

#[test]
fn run_from_args_against_a_manifest_that_does_not_exist_is_an_error() {
    // Arrange
    let args = [
        "cargo-crap4rust",
        "--manifest-path",
        "no/such/directory/Cargo.toml",
    ];

    // Act
    let outcome = EntryPoint::run_from_args(args);

    // Assert
    assert!(
        outcome.is_err(),
        "a manifest that cannot be read must fail the run rather than report a clean project"
    );
}

// The output format picked on the command line is the one that renders the
// report: with --output-format json, what the binary cargo runs prints is the
// JSON document.
#[test]
fn run_with_the_json_output_format_renders_the_json_report() {
    // Arrange
    let package = TempDir::new().expect("temp dir");
    let source_dir = package.path().join("src");
    fs::create_dir_all(&source_dir).expect("create src dir");
    fs::write(
        package.path().join("Cargo.toml"),
        "[package]\nname = \"probe\"\nversion = \"0.1.0\"\nedition = \"2021\"\n",
    )
    .expect("write manifest");
    fs::write(
        source_dir.join("lib.rs"),
        "pub fn probe() -> u32 {\n    1\n}\n",
    )
    .expect("write lib.rs");
    let source = source_dir
        .join("lib.rs")
        .canonicalize()
        .expect("canonical source path");
    let coverage = package.path().join("coverage.json");
    let export = json!({
        "data": [{
            "functions": [{
                "filenames": [source.to_string_lossy()],
                "regions": [[1, 1, 3, 2, 1, 0, 0, 0]],
            }],
        }],
    });
    fs::write(&coverage, to_string(&export).expect("serialize export"))
        .expect("write coverage file");

    // Act
    let output = Command::new(env!("CARGO_BIN_EXE_cargo-crap4rust"))
        .arg("crap4rust")
        .arg("--manifest-path")
        .arg(package.path().join("Cargo.toml"))
        .arg("--coverage")
        .arg(&coverage)
        .arg("--output-format")
        .arg("json")
        .output()
        .expect("run the binary");

    // Assert
    let report: Value = from_slice(&output.stdout).expect("stdout is the JSON report");
    assert_eq!(report["scope_name"], "probe");
}

#[test]
fn without_cargo_subcommand_drops_only_the_first_repeat() {
    // Arrange
    let args = ["cargo-crap4rust", "crap4rust", "--package", "crap4rust"]
        .map(String::from)
        .to_vec();

    // Act
    let forwarded = EntryPoint::without_cargo_subcommand(args);

    // Assert
    assert_eq!(forwarded, vec!["cargo-crap4rust", "--package", "crap4rust"]);
}

#[test]
fn without_cargo_subcommand_drops_the_name_cargo_repeats() {
    // Arrange
    let args = ["cargo-crap4rust", "crap4rust", "--threshold", "15"]
        .map(String::from)
        .to_vec();

    // Act
    let forwarded = EntryPoint::without_cargo_subcommand(args);

    // Assert
    assert_eq!(forwarded, vec!["cargo-crap4rust", "--threshold", "15"]);
}

#[test]
fn without_cargo_subcommand_keeps_a_package_named_after_the_tool() {
    // Arrange
    let args = ["cargo-crap4rust", "--package", "crap4rust"]
        .map(String::from)
        .to_vec();

    // Act
    let forwarded = EntryPoint::without_cargo_subcommand(args);

    // Assert
    assert_eq!(forwarded, vec!["cargo-crap4rust", "--package", "crap4rust"]);
}

#[test]
fn without_cargo_subcommand_leaves_a_direct_invocation_untouched() {
    // Arrange
    let args = ["cargo-crap4rust", "--threshold", "15"]
        .map(String::from)
        .to_vec();

    // Act
    let forwarded = EntryPoint::without_cargo_subcommand(args);

    // Assert
    assert_eq!(forwarded, vec!["cargo-crap4rust", "--threshold", "15"]);
}
