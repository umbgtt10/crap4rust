// Copyright 2025 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use assert_cmd::Command;
use predicates::prelude::*;
use predicates::str::{contains, is_empty, starts_with};
use serde_json::json;
use serde_json::to_vec;
use std::fs;
use std::path::{Path, PathBuf};
use tempfile::TempDir;

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

fn fixture_path(segments: &[&str]) -> PathBuf {
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.push("..");
    path.push("fixture");
    for segment in segments {
        path.push(segment);
    }
    path
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
        to_vec(&coverage_json).expect("serialize coverage json"),
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
        to_vec(&coverage_json).expect("serialize empty coverage json"),
    )
    .expect("write empty coverage file");

    coverage_path
}

#[test]
fn all_features_flag_is_accepted_with_precomputed_coverage() {
    // Arrange
    let fixture_dir = fixture_path(&["single_fixture"]);
    let manifest_path = fixture_dir.join("Cargo.toml");
    let source_path = fixture_dir.join("src").join("target.rs");
    let function_line = first_function_line(&source_path);
    let temp_dir = TempDir::new().expect("temp dir");
    let coverage_path = write_coverage_file(temp_dir.path(), &[(source_path, function_line, 0)]);

    let mut command = Command::cargo_bin("cargo-crap4rust").expect("binary");
    command
        .arg("--manifest-path")
        .arg(&manifest_path)
        .arg("--coverage")
        .arg(&coverage_path)
        .arg("--all-features");

    // Act & Assert
    command
        .assert()
        .success()
        .stderr(is_empty())
        .stdout(contains("crap4rust report for single-fixture"))
        .stdout(contains("summary: total_functions=1"));
}

#[test]
fn cargo_subcommand_forwards_arguments_to_crap4rust_binary() {
    // Arrange
    let fixture_dir = fixture_path(&["single_fixture"]);
    let manifest_path = fixture_dir.join("Cargo.toml");
    let source_path = fixture_dir.join("src").join("target.rs");
    let function_line = first_function_line(&source_path);
    let temp_dir = TempDir::new().expect("temp dir");
    let coverage_path = write_coverage_file(temp_dir.path(), &[(source_path, function_line, 0)]);

    let mut command = Command::cargo_bin("cargo-crap4rust").expect("binary");
    command
        .arg("crap4rust")
        .arg("--manifest-path")
        .arg(&manifest_path)
        .arg("--coverage")
        .arg(&coverage_path)
        .arg("--features")
        .arg("demo-feature");

    // Act & Assert
    command
        .assert()
        .success()
        .stderr(is_empty())
        .stdout(contains("crap4rust report for single-fixture"))
        .stdout(contains("summary: total_functions=1"));
}

#[test]
fn cfg_test_file_based_mod_declaration_is_excluded_from_discovery() {
    // Arrange
    let fixture_dir = fixture_path(&["file_based_test_module_fixture"]);
    let manifest_path = fixture_dir.join("Cargo.toml");
    let target_path = fixture_dir.join("src").join("target.rs");
    let tests_path = fixture_dir.join("src").join("tests.rs");
    let shipped_line = first_function_line(&target_path);
    let helper_line = first_function_line(&tests_path);
    let temp_dir = TempDir::new().expect("temp dir");
    let coverage_path = write_coverage_file(
        temp_dir.path(),
        &[(target_path, shipped_line, 0), (tests_path, helper_line, 0)],
    );

    let mut command = Command::cargo_bin("cargo-crap4rust").expect("binary");
    command
        .arg("--manifest-path")
        .arg(&manifest_path)
        .arg("--coverage")
        .arg(&coverage_path);

    // Act & Assert
    command
        .assert()
        .success()
        .stderr(is_empty())
        .stdout(contains(
            "crap4rust report for file-based-test-module-fixture",
        ))
        .stdout(contains("shipped_risky"))
        .stdout(contains("test_only_helper").not())
        .stdout(contains("summary: total_functions=1"));
}

#[test]
fn cfg_test_modules_inside_src_are_excluded_from_discovery() {
    // Arrange
    let fixture_dir = fixture_path(&["inline_test_module_fixture"]);
    let manifest_path = fixture_dir.join("Cargo.toml");
    let source_path = fixture_dir.join("src").join("lib.rs");
    let target_path = fixture_dir.join("src").join("target.rs");
    let shipped_line = named_function_line(&target_path, "shipped_risky");
    let helper_line = named_function_line(&source_path, "test_only_helper");
    let temp_dir = TempDir::new().expect("temp dir");
    let coverage_path = write_coverage_file(
        temp_dir.path(),
        &[
            (target_path, shipped_line, 0),
            (source_path, helper_line, 0),
        ],
    );

    let mut command = Command::cargo_bin("cargo-crap4rust").expect("binary");
    command
        .arg("--manifest-path")
        .arg(&manifest_path)
        .arg("--coverage")
        .arg(&coverage_path);

    // Act & Assert
    command
        .assert()
        .success()
        .stderr(is_empty())
        .stdout(contains("crap4rust report for inline-test-module-fixture"))
        .stdout(contains("shipped_risky"))
        .stdout(contains("test_only_helper").not())
        .stdout(contains("summary: total_functions=1"));
}

#[test]
fn coverage_that_does_not_match_any_function_returns_error() {
    // Arrange
    let fixture_dir = fixture_path(&["single_fixture"]);
    let manifest_path = fixture_dir.join("Cargo.toml");
    let source_path = fixture_dir.join("src").join("target.rs");
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

    // Act & Assert
    command.assert().failure().stdout(contains(
        "coverage data could not be matched to any discovered function by file path and line",
    ));
}

#[test]
fn custom_warn_threshold_appears_in_output_message() {
    // Arrange
    let fixture_dir = fixture_path(&["single_fixture"]);
    let manifest_path = fixture_dir.join("Cargo.toml");
    let source_path = fixture_dir.join("src").join("target.rs");
    let function_line = first_function_line(&source_path);
    let temp_dir = TempDir::new().expect("temp dir");
    let coverage_path = write_coverage_file(temp_dir.path(), &[(source_path, function_line, 1)]);

    let mut command = Command::cargo_bin("cargo-crap4rust").expect("binary");
    command
        .arg("--manifest-path")
        .arg(&manifest_path)
        .arg("--coverage")
        .arg(&coverage_path)
        .arg("--warn-threshold")
        .arg("6.0")
        .arg("--threshold")
        .arg("10")
        .arg("--project-threshold")
        .arg("100.0");

    // Act & Assert
    command
        .assert()
        .success()
        .stdout(contains("No functions at or above the threshold of 10.0."))
        .stdout(contains("verdict=clean"));
}

#[test]
fn duplicate_coverage_entries_discard_zero_ghost_keeping_real_coverage() {
    // Arrange
    let fixture_dir = fixture_path(&["aggregation_fixture"]);
    let manifest_path = fixture_dir.join("Cargo.toml");
    let source_path = fixture_dir.join("src").join("target.rs");
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
        .arg(&coverage_path)
        .arg("--warn-threshold")
        .arg("0");

    // Act & Assert
    command
        .assert()
        .success()
        .stdout(contains("100.0%"))
        .stdout(contains("aggregation_target"));
}

#[test]
fn duplicate_coverage_entries_discard_zero_ghost_regardless_of_arrival_order() {
    // Arrange
    let fixture_dir = fixture_path(&["aggregation_fixture"]);
    let manifest_path = fixture_dir.join("Cargo.toml");
    let source_path = fixture_dir.join("src").join("target.rs");
    let function_line = first_function_line(&source_path);
    let temp_dir = TempDir::new().expect("temp dir");
    let coverage_path = write_coverage_file(
        temp_dir.path(),
        &[
            (source_path.clone(), function_line, 1),
            (source_path, function_line, 0),
        ],
    );

    let mut command = Command::cargo_bin("cargo-crap4rust").expect("binary");
    command
        .arg("--manifest-path")
        .arg(&manifest_path)
        .arg("--coverage")
        .arg(&coverage_path)
        .arg("--warn-threshold")
        .arg("0");

    // Act & Assert
    command
        .assert()
        .success()
        .stdout(contains("100.0%"))
        .stdout(contains("aggregation_target"));
}

#[test]
fn exclude_path_omits_matching_files_from_report() {
    // Arrange
    let fixture_dir = fixture_path(&["workspace_validation_fixture"]);
    let manifest_path = fixture_dir.join("Cargo.toml");
    let source_path = fixture_dir
        .join("app-validation")
        .join("tests")
        .join("validation_support.rs");
    let function_line = first_function_line(&source_path);
    let temp_dir = TempDir::new().expect("temp dir");
    let coverage_path = write_coverage_file(temp_dir.path(), &[(source_path, function_line, 0)]);

    let mut command = Command::cargo_bin("cargo-crap4rust").expect("binary");
    command
        .arg("--manifest-path")
        .arg(&manifest_path)
        .arg("--package")
        .arg("app-validation")
        .arg("--coverage")
        .arg(&coverage_path)
        .arg("--include-test-targets")
        .arg("--exclude-path")
        .arg("tests");

    // Act & Assert
    command.assert().failure().stdout(contains(
        "coverage data could not be matched to any discovered function by file path and line",
    ));
}

#[test]
fn exclude_path_only_omits_matching_prefix_leaving_other_files_intact() {
    // Arrange
    let fixture_dir = fixture_path(&["test_target_fixture"]);
    let manifest_path = fixture_dir.join("Cargo.toml");
    let source_path = fixture_dir.join("src").join("target.rs");
    let source_line = first_function_line(&source_path);
    let temp_dir = TempDir::new().expect("temp dir");
    let coverage_path = write_coverage_file(temp_dir.path(), &[(source_path, source_line, 0)]);

    let mut command = Command::cargo_bin("cargo-crap4rust").expect("binary");
    command
        .arg("--manifest-path")
        .arg(&manifest_path)
        .arg("--coverage")
        .arg(&coverage_path)
        .arg("--exclude-path")
        .arg("tests");

    // Act & Assert
    command
        .assert()
        .success()
        .stdout(contains("shipped_risky"))
        .stdout(contains("summary: total_functions=1"));
}

#[test]
fn explicit_package_in_root_workspace_overrides_all_members_default() {
    // Arrange
    let fixture_dir = fixture_path(&["root_workspace_fixture"]);
    let manifest_path = fixture_dir.join("Cargo.toml");
    let source_path = fixture_dir.join("src").join("target.rs");
    let function_line = first_function_line(&source_path);
    let temp_dir = TempDir::new().expect("temp dir");
    let coverage_path = write_coverage_file(temp_dir.path(), &[(source_path, function_line, 0)]);

    let mut command = Command::cargo_bin("cargo-crap4rust").expect("binary");
    command
        .arg("--manifest-path")
        .arg(&manifest_path)
        .arg("--package")
        .arg("root-app")
        .arg("--coverage")
        .arg(&coverage_path);

    // Act & Assert
    command
        .assert()
        .success()
        .stdout(contains("crap4rust report for root-app"))
        .stdout(contains("summary: total_functions=1"))
        .stdout(contains("helper-member").not());
}

#[test]
fn features_flag_is_accepted_with_precomputed_coverage() {
    // Arrange
    let fixture_dir = fixture_path(&["single_fixture"]);
    let manifest_path = fixture_dir.join("Cargo.toml");
    let source_path = fixture_dir.join("src").join("target.rs");
    let function_line = first_function_line(&source_path);
    let temp_dir = TempDir::new().expect("temp dir");
    let coverage_path = write_coverage_file(temp_dir.path(), &[(source_path, function_line, 0)]);

    let mut command = Command::cargo_bin("cargo-crap4rust").expect("binary");
    command
        .arg("--manifest-path")
        .arg(&manifest_path)
        .arg("--coverage")
        .arg(&coverage_path)
        .arg("--features")
        .arg("demo-feature");

    // Act & Assert
    command
        .assert()
        .success()
        .stderr(is_empty())
        .stdout(contains("crap4rust report for single-fixture"))
        .stdout(contains("summary: total_functions=1"));
}

#[test]
fn full_coverage_keeps_crap_score_below_warning_threshold() {
    // Arrange
    let fixture_dir = fixture_path(&["single_fixture"]);
    let manifest_path = fixture_dir.join("Cargo.toml");
    let source_path = fixture_dir.join("src").join("target.rs");
    let function_line = first_function_line(&source_path);
    let temp_dir = TempDir::new().expect("temp dir");
    let coverage_path = write_coverage_file(temp_dir.path(), &[(source_path, function_line, 1)]);

    let mut command = Command::cargo_bin("cargo-crap4rust").expect("binary");
    command
        .arg("--manifest-path")
        .arg(&manifest_path)
        .arg("--coverage")
        .arg(&coverage_path)
        .arg("--threshold")
        .arg("10")
        .arg("--project-threshold")
        .arg("100.0");

    // Act & Assert
    command
        .assert()
        .success()
        .stdout(contains("No functions at or above the threshold of 10.0."))
        .stdout(contains("verdict=clean"));
}

#[test]
fn json_output_format_produces_valid_json() {
    // Arrange
    let fixture_dir = fixture_path(&["single_fixture"]);
    let manifest_path = fixture_dir.join("Cargo.toml");
    let source_path = fixture_dir.join("src").join("target.rs");
    let function_line = first_function_line(&source_path);
    let temp_dir = TempDir::new().expect("temp dir");
    let coverage_path = write_coverage_file(temp_dir.path(), &[(source_path, function_line, 0)]);

    let mut command = Command::cargo_bin("cargo-crap4rust").expect("binary");
    command
        .arg("--manifest-path")
        .arg(&manifest_path)
        .arg("--coverage")
        .arg(&coverage_path)
        .arg("--output-format")
        .arg("json");

    // Act & Assert
    command
        .assert()
        .success()
        .stdout(starts_with("{"))
        .stdout(contains(r#""scope_name": "single-fixture""#))
        .stdout(contains(r#""verdict": "Warn""#));
}

#[test]
fn multiple_packages_produce_single_aggregate_report() {
    // Arrange
    let fixture_dir = fixture_path(&["workspace_fixture"]);
    let manifest_path = fixture_dir.join("Cargo.toml");
    let core_source = fixture_dir.join("app-core").join("src").join("target.rs");
    let validation_source = fixture_dir
        .join("app-validation")
        .join("src")
        .join("target.rs");
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

    // Act & Assert
    command
        .assert()
        .success()
        .stdout(contains("crap4rust report for app-core, app-validation"))
        .stdout(contains("package"))
        .stdout(contains("app-core"))
        .stdout(contains("app-validation"))
        .stdout(contains("summary: total_functions=2"));
}

#[test]
fn multiple_packages_without_coverage_generate_aggregate_coverage_automatically() {
    // Arrange
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

    // Act & Assert
    command
        .assert()
        .success()
        .stdout(contains("crap4rust report for app-core, app-validation"))
        .stdout(contains("summary: total_functions=2"));

    assert!(
        generated_coverage_path.exists(),
        "automatic aggregate coverage file was not generated"
    );
}

#[test]
fn no_default_features_flag_is_accepted_with_precomputed_coverage() {
    // Arrange
    let fixture_dir = fixture_path(&["single_fixture"]);
    let manifest_path = fixture_dir.join("Cargo.toml");
    let source_path = fixture_dir.join("src").join("target.rs");
    let function_line = first_function_line(&source_path);
    let temp_dir = TempDir::new().expect("temp dir");
    let coverage_path = write_coverage_file(temp_dir.path(), &[(source_path, function_line, 0)]);

    let mut command = Command::cargo_bin("cargo-crap4rust").expect("binary");
    command
        .arg("--manifest-path")
        .arg(&manifest_path)
        .arg("--coverage")
        .arg(&coverage_path)
        .arg("--no-default-features");

    // Act & Assert
    command
        .assert()
        .success()
        .stderr(is_empty())
        .stdout(contains("crap4rust report for single-fixture"))
        .stdout(contains("summary: total_functions=1"));
}

#[test]
fn package_without_functions_returns_error() {
    // Arrange
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

    // Act & Assert
    command.assert().failure().stdout(contains(
        "no Rust functions were discovered in the selected packages",
    ));
}

#[test]
fn root_workspace_defaults_to_all_workspace_members_when_no_package_is_provided() {
    // Arrange
    let fixture_dir = fixture_path(&["root_workspace_fixture"]);
    let manifest_path = fixture_dir.join("Cargo.toml");
    let root_source_path = fixture_dir.join("src").join("target.rs");
    let helper_source_path = fixture_dir
        .join("helper-member")
        .join("src")
        .join("target.rs");
    let root_function_line = first_function_line(&root_source_path);
    let helper_function_line = first_function_line(&helper_source_path);
    let temp_dir = TempDir::new().expect("temp dir");
    let coverage_path = write_coverage_file(
        temp_dir.path(),
        &[
            (root_source_path, root_function_line, 0),
            (helper_source_path, helper_function_line, 0),
        ],
    );

    let mut command = Command::cargo_bin("cargo-crap4rust").expect("binary");
    command
        .arg("--manifest-path")
        .arg(&manifest_path)
        .arg("--coverage")
        .arg(&coverage_path);

    // Act & Assert
    command
        .assert()
        .success()
        .stdout(
            contains("crap4rust report for ")
                .and(contains("root-app"))
                .and(contains("helper-member")),
        )
        .stdout(contains("summary: total_functions=2"));
}

#[test]
fn root_workspace_without_coverage_generates_coverage_for_all_workspace_members() {
    // Arrange
    let fixture_dir = fixture_path(&["root_workspace_fixture"]);
    let manifest_path = fixture_dir.join("Cargo.toml");
    let generated_coverage_path_a = fixture_dir
        .join("target")
        .join("crap4rust")
        .join("root_app__helper_member-coverage.json");
    let generated_coverage_path_b = fixture_dir
        .join("target")
        .join("crap4rust")
        .join("helper_member__root_app-coverage.json");
    if generated_coverage_path_a.exists() {
        fs::remove_file(&generated_coverage_path_a)
            .expect("remove stale workspace coverage file (a)");
    }
    if generated_coverage_path_b.exists() {
        fs::remove_file(&generated_coverage_path_b)
            .expect("remove stale workspace coverage file (b)");
    }

    let mut command = Command::cargo_bin("cargo-crap4rust").expect("binary");
    command.arg("--manifest-path").arg(&manifest_path);

    // Act & Assert
    command
        .assert()
        .failure()
        .stdout(contains("cargo llvm-cov failed"));

    assert!(
        !generated_coverage_path_a.exists() && !generated_coverage_path_b.exists(),
        "automatic workspace-member coverage file should not be generated when a workspace member test fails"
    );
}

#[test]
fn single_package_with_precomputed_coverage_prints_report() {
    // Arrange
    let fixture_dir = fixture_path(&["single_fixture"]);
    let manifest_path = fixture_dir.join("Cargo.toml");
    let source_path = fixture_dir.join("src").join("target.rs");
    let function_line = first_function_line(&source_path);
    let temp_dir = TempDir::new().expect("temp dir");
    let coverage_path = write_coverage_file(temp_dir.path(), &[(source_path, function_line, 0)]);

    let mut command = Command::cargo_bin("cargo-crap4rust").expect("binary");
    command
        .arg("--manifest-path")
        .arg(&manifest_path)
        .arg("--coverage")
        .arg(&coverage_path);

    // Act & Assert
    command
        .assert()
        .success()
        .stderr(is_empty())
        .stdout(contains("crap4rust report for single-fixture"))
        .stdout(contains("risky"))
        .stdout(contains("summary: total_functions=1"));
}

#[test]
fn single_package_without_coverage_generates_coverage_automatically() {
    // Arrange
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

    // Act & Assert
    command
        .assert()
        .success()
        .stdout(contains("crap4rust report for single-fixture"))
        .stdout(contains("summary: total_functions=1"));

    assert!(
        generated_coverage_path.exists(),
        "automatic coverage file was not generated"
    );
}

#[test]
fn strict_mode_fails_when_project_threshold_would_otherwise_pass() {
    // Arrange
    let fixture_dir = fixture_path(&["single_fixture"]);
    let manifest_path = fixture_dir.join("Cargo.toml");
    let source_path = fixture_dir.join("src").join("target.rs");
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

    // Act & Assert
    command
        .assert()
        .failure()
        .stdout(contains("verdict=crappy"));
}

#[test]
fn test_targets_are_excluded_from_discovery_by_default() {
    // Arrange
    let fixture_dir = fixture_path(&["test_target_fixture"]);
    let manifest_path = fixture_dir.join("Cargo.toml");
    let source_path = fixture_dir.join("src").join("target.rs");
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

    // Act & Assert
    command
        .assert()
        .success()
        .stdout(contains("crap4rust report for test-target-fixture"))
        .stdout(contains("shipped_risky"))
        .stdout(contains("test_support_risky").not())
        .stdout(contains("summary: total_functions=1"));
}

#[test]
fn threshold_boundary_at_thirty_is_warn_not_crappy() {
    // Arrange
    let fixture_dir = fixture_path(&["single_fixture"]);
    let manifest_path = fixture_dir.join("Cargo.toml");
    let source_path = fixture_dir.join("src").join("target.rs");
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

    // Act & Assert
    command
        .assert()
        .success()
        .stdout(contains("30.0  warn"))
        .stdout(contains("verdict=warn"));
}

#[test]
fn unknown_package_returns_error() {
    // Arrange
    let fixture_dir = fixture_path(&["workspace_fixture"]);
    let manifest_path = fixture_dir.join("Cargo.toml");

    let mut command = Command::cargo_bin("cargo-crap4rust").expect("binary");
    command
        .arg("--manifest-path")
        .arg(&manifest_path)
        .arg("--package")
        .arg("does-not-exist");

    // Act & Assert
    command.assert().failure().stdout(contains(
        "package does-not-exist was not found in the manifest",
    ));
}

#[test]
fn validation_only_package_with_optional_test_target_discovery_prints_report() {
    // Arrange
    let fixture_dir = fixture_path(&["workspace_validation_fixture"]);
    let manifest_path = fixture_dir.join("Cargo.toml");
    let source_path = fixture_dir
        .join("app-validation")
        .join("tests")
        .join("validation_support.rs");
    let function_line = first_function_line(&source_path);
    let temp_dir = TempDir::new().expect("temp dir");
    let coverage_path = write_coverage_file(temp_dir.path(), &[(source_path, function_line, 0)]);

    let mut command = Command::cargo_bin("cargo-crap4rust").expect("binary");
    command
        .arg("--manifest-path")
        .arg(&manifest_path)
        .arg("--package")
        .arg("app-validation")
        .arg("--coverage")
        .arg(&coverage_path)
        .arg("--include-test-targets");

    // Act & Assert
    command
        .assert()
        .success()
        .stderr(is_empty())
        .stdout(contains("crap4rust report for app-validation"))
        .stdout(contains("validation_only_risky"))
        .stdout(contains("summary: total_functions=2"));
}

#[test]
fn warn_only_succeeds_even_when_thresholds_are_exceeded() {
    // Arrange
    let fixture_dir = fixture_path(&["single_fixture"]);
    let manifest_path = fixture_dir.join("Cargo.toml");
    let source_path = fixture_dir.join("src").join("target.rs");
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

    // Act & Assert
    command
        .assert()
        .success()
        .stdout(contains("verdict=crappy"));
}

#[test]
fn workspace_without_selected_package_selects_all_workspace_members() {
    // Arrange
    let fixture_dir = fixture_path(&["workspace_fixture"]);
    let manifest_path = fixture_dir.join("Cargo.toml");
    let core_source = fixture_dir.join("app-core").join("src").join("target.rs");
    let validation_source = fixture_dir
        .join("app-validation")
        .join("src")
        .join("target.rs");
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
        .arg("--coverage")
        .arg(&coverage_path);

    // Act & Assert
    command
        .assert()
        .success()
        .stdout(
            contains("crap4rust report for ")
                .and(contains("app-core"))
                .and(contains("app-validation")),
        )
        .stdout(contains("summary: total_functions=2"));
}

#[test]
fn zero_coverage_produces_fixture_expected_crap_score() {
    // Arrange
    let fixture_dir = fixture_path(&["single_fixture"]);
    let manifest_path = fixture_dir.join("Cargo.toml");
    let source_path = fixture_dir.join("src").join("target.rs");
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
        .arg("200")
        .arg("--project-threshold")
        .arg("100.0");

    // Act & Assert
    command
        .assert()
        .success()
        .stdout(contains("30.0  warn"))
        .stdout(contains("verdict=warn"));
}
