// Copyright 2025 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License or Apache License, Version 2.0
// SPDX-License-Identifier: MIT OR Apache-2.0

use std::fs;
use std::path::{Path, PathBuf};

use assert_cmd::Command;
use predicates::prelude::*;
use serde_json::json;
use tempfile::TempDir;

#[test]
fn single_package_with_precomputed_coverage_prints_report() {
    let fixture_dir = fixture_path(&["single_fixture"]);
    let manifest_path = fixture_dir.join("Cargo.toml");
    let source_path = fixture_dir.join("src").join("lib.rs");
    let function_line = first_function_line(&source_path);
    let temp_dir = TempDir::new().expect("temp dir");
    let coverage_path = write_coverage_file(temp_dir.path(), &[(source_path, function_line, 0)]);

    let mut command = Command::cargo_bin("cargo-crap4rust").expect("binary");
    command
        .arg("--manifest-path")
        .arg(&manifest_path)
        .arg("--coverage")
        .arg(&coverage_path);

    command
        .assert()
        .success()
        .stderr(predicate::str::is_empty())
        .stdout(predicate::str::contains(
            "crap4rust report for single-fixture",
        ))
        .stdout(predicate::str::contains("risky"))
        .stdout(predicate::str::contains("summary: total_functions=1"));
}

#[test]
fn multiple_packages_produce_single_aggregate_report() {
    let fixture_dir = fixture_path(&["workspace_fixture"]);
    let manifest_path = fixture_dir.join("Cargo.toml");
    let core_source = fixture_dir.join("app-core").join("src").join("lib.rs");
    let validation_source = fixture_dir
        .join("app-validation")
        .join("src")
        .join("lib.rs");
    let core_function_line = first_function_line(&core_source);
    let validation_function_line = first_function_line(&validation_source);
    let temp_dir = TempDir::new().expect("temp dir");
    let coverage_path = write_coverage_file(
        temp_dir.path(),
        &[
            (core_source, core_function_line, 0),
            (validation_source, validation_function_line, 0),
        ],
    );

    let mut command = Command::cargo_bin("cargo-crap4rust").expect("binary");
    command
        .arg("--manifest-path")
        .arg(&manifest_path)
        .arg("--package")
        .arg("app-core")
        .arg("--package")
        .arg("app-validation")
        .arg("--coverage")
        .arg(&coverage_path);

    command
        .assert()
        .success()
        .stdout(predicate::str::contains(
            "crap4rust report for app-core, app-validation",
        ))
        .stdout(predicate::str::contains("package"))
        .stdout(predicate::str::contains("app-core"))
        .stdout(predicate::str::contains("app-validation"))
        .stdout(predicate::str::contains("summary: total_functions=2"));
}

#[test]
fn duplicate_coverage_entries_are_aggregated() {
    let fixture_dir = fixture_path(&["aggregation_fixture"]);
    let manifest_path = fixture_dir.join("Cargo.toml");
    let source_path = fixture_dir.join("src").join("lib.rs");
    let function_line = first_function_line(&source_path);
    let temp_dir = TempDir::new().expect("temp dir");
    let coverage_path = write_coverage_file(
        temp_dir.path(),
        &[
            (source_path.clone(), function_line, 0),
            (source_path, function_line, 1),
        ],
    );

    let mut command = Command::cargo_bin("cargo-crap4rust").expect("binary");
    command
        .arg("--manifest-path")
        .arg(&manifest_path)
        .arg("--coverage")
        .arg(&coverage_path);

    command
        .assert()
        .success()
        .stdout(predicate::str::contains("50.0%"))
        .stdout(predicate::str::contains("aggregation_target"));
}

#[test]
fn workspace_without_selected_package_returns_error() {
    let fixture_dir = fixture_path(&["workspace_fixture"]);
    let manifest_path = fixture_dir.join("Cargo.toml");

    let mut command = Command::cargo_bin("cargo-crap4rust").expect("binary");
    command.arg("--manifest-path").arg(&manifest_path);

    command.assert().failure().stderr(predicate::str::contains(
        "manifest contains multiple packages; pass --package <name>",
    ));
}

#[test]
fn single_package_without_coverage_generates_coverage_automatically() {
    let fixture_dir = fixture_path(&["single_fixture"]);
    let manifest_path = fixture_dir.join("Cargo.toml");
    let generated_coverage_path = fixture_dir
        .join("target")
        .join("crap4rust")
        .join("single_fixture-coverage.json");
    if generated_coverage_path.exists() {
        fs::remove_file(&generated_coverage_path).expect("remove stale coverage file");
    }

    let mut command = Command::cargo_bin("cargo-crap4rust").expect("binary");
    command.arg("--manifest-path").arg(&manifest_path);

    command
        .assert()
        .success()
        .stdout(predicate::str::contains(
            "crap4rust report for single-fixture",
        ))
        .stdout(predicate::str::contains("summary: total_functions=1"));

    assert!(
        generated_coverage_path.exists(),
        "automatic coverage file was not generated"
    );
}

#[test]
fn multiple_packages_without_coverage_generate_aggregate_coverage_automatically() {
    let fixture_dir = fixture_path(&["workspace_fixture"]);
    let manifest_path = fixture_dir.join("Cargo.toml");
    let generated_coverage_path = fixture_dir
        .join("target")
        .join("crap4rust")
        .join("app_core__app_validation-coverage.json");
    if generated_coverage_path.exists() {
        fs::remove_file(&generated_coverage_path).expect("remove stale aggregate coverage file");
    }

    let mut command = Command::cargo_bin("cargo-crap4rust").expect("binary");
    command
        .arg("--manifest-path")
        .arg(&manifest_path)
        .arg("--package")
        .arg("app-core")
        .arg("--package")
        .arg("app-validation");

    command
        .assert()
        .success()
        .stdout(predicate::str::contains(
            "crap4rust report for app-core, app-validation",
        ))
        .stdout(predicate::str::contains("summary: total_functions=2"));

    assert!(
        generated_coverage_path.exists(),
        "automatic aggregate coverage file was not generated"
    );
}

#[test]
fn root_package_without_coverage_generates_coverage_for_root_only() {
    let fixture_dir = fixture_path(&["root_workspace_fixture"]);
    let manifest_path = fixture_dir.join("Cargo.toml");
    let generated_coverage_path = fixture_dir
        .join("target")
        .join("crap4rust")
        .join("root_app-coverage.json");
    if generated_coverage_path.exists() {
        fs::remove_file(&generated_coverage_path).expect("remove stale root coverage file");
    }

    let mut command = Command::cargo_bin("cargo-crap4rust").expect("binary");
    command.arg("--manifest-path").arg(&manifest_path);

    command
        .assert()
        .success()
        .stdout(predicate::str::contains("crap4rust report for root-app"))
        .stdout(predicate::str::contains("helper-member").not());

    assert!(
        generated_coverage_path.exists(),
        "automatic root-package coverage file was not generated"
    );
}

#[test]
fn test_targets_are_excluded_from_discovery_by_default() {
    let fixture_dir = fixture_path(&["test_target_fixture"]);
    let manifest_path = fixture_dir.join("Cargo.toml");
    let source_path = fixture_dir.join("src").join("lib.rs");
    let test_support_path = fixture_dir.join("tests").join("support.rs");
    let source_line = first_function_line(&source_path);
    let test_support_line = first_function_line(&test_support_path);
    let temp_dir = TempDir::new().expect("temp dir");
    let coverage_path = write_coverage_file(
        temp_dir.path(),
        &[
            (source_path, source_line, 0),
            (test_support_path, test_support_line, 0),
        ],
    );

    let mut command = Command::cargo_bin("cargo-crap4rust").expect("binary");
    command
        .arg("--manifest-path")
        .arg(&manifest_path)
        .arg("--coverage")
        .arg(&coverage_path);

    command
        .assert()
        .success()
        .stdout(predicate::str::contains(
            "crap4rust report for test-target-fixture",
        ))
        .stdout(predicate::str::contains("shipped_risky"))
        .stdout(predicate::str::contains("test_support_risky").not())
        .stdout(predicate::str::contains("summary: total_functions=1"));
}

#[test]
fn cfg_test_modules_inside_src_are_excluded_from_discovery() {
    let fixture_dir = fixture_path(&["inline_test_module_fixture"]);
    let manifest_path = fixture_dir.join("Cargo.toml");
    let source_path = fixture_dir.join("src").join("lib.rs");
    let shipped_line = named_function_line(&source_path, "shipped_risky");
    let helper_line = named_function_line(&source_path, "test_only_helper");
    let temp_dir = TempDir::new().expect("temp dir");
    let coverage_path = write_coverage_file(
        temp_dir.path(),
        &[
            (source_path.clone(), shipped_line, 0),
            (source_path, helper_line, 0),
        ],
    );

    let mut command = Command::cargo_bin("cargo-crap4rust").expect("binary");
    command
        .arg("--manifest-path")
        .arg(&manifest_path)
        .arg("--coverage")
        .arg(&coverage_path);

    command
        .assert()
        .success()
        .stderr(predicate::str::is_empty())
        .stdout(predicate::str::contains(
            "crap4rust report for inline-test-module-fixture",
        ))
        .stdout(predicate::str::contains("shipped_risky"))
        .stdout(predicate::str::contains("test_only_helper").not())
        .stdout(predicate::str::contains("summary: total_functions=1"));
}

#[test]
fn coverage_that_does_not_match_any_function_returns_error() {
    let fixture_dir = fixture_path(&["single_fixture"]);
    let manifest_path = fixture_dir.join("Cargo.toml");
    let source_path = fixture_dir.join("src").join("lib.rs");
    let function_line = first_function_line(&source_path);
    let temp_dir = TempDir::new().expect("temp dir");
    let coverage_path =
        write_coverage_file(temp_dir.path(), &[(source_path, function_line + 100, 1)]);

    let mut command = Command::cargo_bin("cargo-crap4rust").expect("binary");
    command
        .arg("--manifest-path")
        .arg(&manifest_path)
        .arg("--coverage")
        .arg(&coverage_path);

    command.assert().failure().stderr(predicate::str::contains(
        "coverage data could not be matched to any discovered function by file path and line",
    ));
}

#[test]
fn unknown_package_returns_error() {
    let fixture_dir = fixture_path(&["workspace_fixture"]);
    let manifest_path = fixture_dir.join("Cargo.toml");

    let mut command = Command::cargo_bin("cargo-crap4rust").expect("binary");
    command
        .arg("--manifest-path")
        .arg(&manifest_path)
        .arg("--package")
        .arg("does-not-exist");

    command.assert().failure().stderr(predicate::str::contains(
        "package does-not-exist was not found in the manifest",
    ));
}

#[test]
fn strict_mode_fails_when_project_threshold_would_otherwise_pass() {
    let fixture_dir = fixture_path(&["single_fixture"]);
    let manifest_path = fixture_dir.join("Cargo.toml");
    let source_path = fixture_dir.join("src").join("lib.rs");
    let function_line = first_function_line(&source_path);
    let temp_dir = TempDir::new().expect("temp dir");
    let coverage_path = write_coverage_file(temp_dir.path(), &[(source_path, function_line, 0)]);

    let mut command = Command::cargo_bin("cargo-crap4rust").expect("binary");
    command
        .arg("--manifest-path")
        .arg(&manifest_path)
        .arg("--coverage")
        .arg(&coverage_path)
        .arg("--threshold")
        .arg("29")
        .arg("--project-threshold")
        .arg("100.0")
        .arg("--strict");

    command
        .assert()
        .failure()
        .stdout(predicate::str::contains("verdict=crappy"));
}

#[test]
fn warn_only_succeeds_even_when_thresholds_are_exceeded() {
    let fixture_dir = fixture_path(&["single_fixture"]);
    let manifest_path = fixture_dir.join("Cargo.toml");
    let source_path = fixture_dir.join("src").join("lib.rs");
    let function_line = first_function_line(&source_path);
    let temp_dir = TempDir::new().expect("temp dir");
    let coverage_path = write_coverage_file(temp_dir.path(), &[(source_path, function_line, 0)]);

    let mut command = Command::cargo_bin("cargo-crap4rust").expect("binary");
    command
        .arg("--manifest-path")
        .arg(&manifest_path)
        .arg("--coverage")
        .arg(&coverage_path)
        .arg("--threshold")
        .arg("29")
        .arg("--project-threshold")
        .arg("0.0")
        .arg("--warn-only");

    command
        .assert()
        .success()
        .stdout(predicate::str::contains("verdict=crappy"));
}

#[test]
fn threshold_boundary_at_thirty_is_warn_not_crappy() {
    let fixture_dir = fixture_path(&["single_fixture"]);
    let manifest_path = fixture_dir.join("Cargo.toml");
    let source_path = fixture_dir.join("src").join("lib.rs");
    let function_line = first_function_line(&source_path);
    let temp_dir = TempDir::new().expect("temp dir");
    let coverage_path = write_coverage_file(temp_dir.path(), &[(source_path, function_line, 0)]);

    let mut command = Command::cargo_bin("cargo-crap4rust").expect("binary");
    command
        .arg("--manifest-path")
        .arg(&manifest_path)
        .arg("--coverage")
        .arg(&coverage_path)
        .arg("--threshold")
        .arg("30")
        .arg("--project-threshold")
        .arg("100.0");

    command
        .assert()
        .success()
        .stdout(predicate::str::contains("30.0  warn"))
        .stdout(predicate::str::contains("verdict=warn"));
}

#[test]
fn root_package_is_selected_by_default_when_present() {
    let fixture_dir = fixture_path(&["root_workspace_fixture"]);
    let manifest_path = fixture_dir.join("Cargo.toml");
    let source_path = fixture_dir.join("src").join("lib.rs");
    let function_line = first_function_line(&source_path);
    let temp_dir = TempDir::new().expect("temp dir");
    let coverage_path = write_coverage_file(temp_dir.path(), &[(source_path, function_line, 0)]);

    let mut command = Command::cargo_bin("cargo-crap4rust").expect("binary");
    command
        .arg("--manifest-path")
        .arg(&manifest_path)
        .arg("--coverage")
        .arg(&coverage_path);

    command
        .assert()
        .success()
        .stdout(predicate::str::contains("crap4rust report for root-app"))
        .stdout(predicate::str::contains("root-app"))
        .stdout(predicate::str::contains("summary: total_functions=1"))
        .stdout(predicate::str::contains("helper-member").not());
}

#[test]
fn package_without_functions_returns_error() {
    let fixture_dir = fixture_path(&["no_function_fixture"]);
    let manifest_path = fixture_dir.join("Cargo.toml");
    let temp_dir = TempDir::new().expect("temp dir");
    let coverage_path = write_empty_coverage_file(temp_dir.path());

    let mut command = Command::cargo_bin("cargo-crap4rust").expect("binary");
    command
        .arg("--manifest-path")
        .arg(&manifest_path)
        .arg("--coverage")
        .arg(&coverage_path);

    command.assert().failure().stderr(predicate::str::contains(
        "no Rust functions were discovered in the selected packages",
    ));
}

fn fixture_path(segments: &[&str]) -> PathBuf {
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.push("tests");
    path.push("fixtures");
    for segment in segments {
        path.push(segment);
    }
    path
}

fn first_function_line(path: &Path) -> usize {
    fs::read_to_string(path)
        .expect("read fixture source")
        .lines()
        .enumerate()
        .find_map(|(index, line)| {
            line.trim_start()
                .starts_with("pub fn ")
                .then_some(index + 1)
        })
        .expect("fixture source contains a public function")
}

fn named_function_line(path: &Path, function_name: &str) -> usize {
    let needle = format!("pub fn {function_name}");

    fs::read_to_string(path)
        .expect("read fixture source")
        .lines()
        .enumerate()
        .find_map(|(index, line)| line.trim_start().starts_with(&needle).then_some(index + 1))
        .expect("fixture source contains the named public function")
}

fn write_coverage_file(temp_dir: &Path, entries: &[(PathBuf, usize, u64)]) -> PathBuf {
    let coverage_path = temp_dir.join("coverage.json");
    let functions = entries
        .iter()
        .map(|(path, line, count)| {
            json!({
                "filenames": [path.canonicalize().expect("canonical source path").to_string_lossy().to_string()],
                "regions": [[*line, 1, *line + 6, 2, *count, 0, 0, 0]],
            })
        })
        .collect::<Vec<_>>();
    let coverage_json = json!({
        "data": [
            {
                "functions": functions,
            }
        ]
    });

    fs::write(
        &coverage_path,
        serde_json::to_vec(&coverage_json).expect("serialize coverage json"),
    )
    .expect("write coverage file");

    coverage_path
}

fn write_empty_coverage_file(temp_dir: &Path) -> PathBuf {
    let coverage_path = temp_dir.join("coverage.json");
    let coverage_json = json!({
        "data": [
            {
                "functions": [],
            }
        ]
    });

    fs::write(
        &coverage_path,
        serde_json::to_vec(&coverage_json).expect("serialize empty coverage json"),
    )
    .expect("write empty coverage file");

    coverage_path
}
