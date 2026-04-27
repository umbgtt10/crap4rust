// Copyright 2025 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License or Apache License, Version 2.0
// SPDX-License-Identifier: MIT OR Apache-2.0

use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};

use assert_cmd::Command;
use crap4rust::app::{classify, compute_crap_score, match_function_coverage, project_fails};
use crap4rust::cli::OutputFormat;
use crap4rust::model::{Config, CoverageRecord, SourceFunction, Verdict};
use predicates::prelude::*;
use serde_json::json;
use tempfile::TempDir;

#[test]
fn validation_only_package_with_optional_test_target_discovery_prints_report() {
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

    command
        .assert()
        .success()
        .stderr(predicate::str::is_empty())
        .stdout(predicate::str::contains(
            "crap4rust report for app-validation",
        ))
        .stdout(predicate::str::contains("validation_only_risky"))
        .stdout(predicate::str::contains("summary: total_functions=2"));
}

#[test]
fn exclude_path_omits_matching_files_from_report() {
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

    command.assert().failure().stderr(predicate::str::contains(
        "coverage data could not be matched to any discovered function by file path and line",
    ));
}

#[test]
fn exclude_path_only_omits_matching_prefix_leaving_other_files_intact() {
    let fixture_dir = fixture_path(&["test_target_fixture"]);
    let manifest_path = fixture_dir.join("Cargo.toml");
    let source_path = fixture_dir.join("src").join("lib.rs");
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

    command
        .assert()
        .success()
        .stdout(predicate::str::contains("shipped_risky"))
        .stdout(predicate::str::contains("summary: total_functions=1"));
}

#[test]
fn cargo_subcommand_forwards_arguments_to_crap4rust_binary() {
    let fixture_dir = fixture_path(&["single_fixture"]);
    let manifest_path = fixture_dir.join("Cargo.toml");
    let source_path = fixture_dir.join("src").join("lib.rs");
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

    command
        .assert()
        .success()
        .stderr(predicate::str::is_empty())
        .stdout(predicate::str::contains(
            "crap4rust report for single-fixture",
        ))
        .stdout(predicate::str::contains("summary: total_functions=1"));
}

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
fn workspace_without_selected_package_selects_all_workspace_members() {
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
        .arg("--coverage")
        .arg(&coverage_path);

    command
        .assert()
        .success()
        .stdout(
            predicate::str::contains("crap4rust report for ")
                .and(predicate::str::contains("app-core"))
                .and(predicate::str::contains("app-validation")),
        )
        .stdout(predicate::str::contains("summary: total_functions=2"));
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
fn root_workspace_without_coverage_generates_coverage_for_all_workspace_members() {
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

    command
        .assert()
        .failure()
        .stderr(predicate::str::contains("cargo llvm-cov failed"))
        .stderr(predicate::str::contains("helper-member"));

    assert!(
        !generated_coverage_path_a.exists() && !generated_coverage_path_b.exists(),
        "automatic workspace-member coverage file should not be generated when a workspace member test fails"
    );
}

#[test]
fn features_flag_is_accepted_with_precomputed_coverage() {
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
        .arg("--features")
        .arg("demo-feature");

    command
        .assert()
        .success()
        .stderr(predicate::str::is_empty())
        .stdout(predicate::str::contains(
            "crap4rust report for single-fixture",
        ))
        .stdout(predicate::str::contains("summary: total_functions=1"));
}

#[test]
fn all_features_flag_is_accepted_with_precomputed_coverage() {
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
        .arg("--all-features");

    command
        .assert()
        .success()
        .stderr(predicate::str::is_empty())
        .stdout(predicate::str::contains(
            "crap4rust report for single-fixture",
        ))
        .stdout(predicate::str::contains("summary: total_functions=1"));
}

#[test]
fn no_default_features_flag_is_accepted_with_precomputed_coverage() {
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
        .arg("--no-default-features");

    command
        .assert()
        .success()
        .stderr(predicate::str::is_empty())
        .stdout(predicate::str::contains(
            "crap4rust report for single-fixture",
        ))
        .stdout(predicate::str::contains("summary: total_functions=1"));
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
fn full_coverage_keeps_crap_score_below_warning_threshold() {
    let fixture_dir = fixture_path(&["single_fixture"]);
    let manifest_path = fixture_dir.join("Cargo.toml");
    let source_path = fixture_dir.join("src").join("lib.rs");
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

    command
        .assert()
        .success()
        .stdout(predicate::str::contains(
            "No functions at or above the warning threshold of 20.0.",
        ))
        .stdout(predicate::str::contains("verdict=clean"));
}

#[test]
fn custom_warn_threshold_appears_in_output_message() {
    let fixture_dir = fixture_path(&["single_fixture"]);
    let manifest_path = fixture_dir.join("Cargo.toml");
    let source_path = fixture_dir.join("src").join("lib.rs");
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

    command
        .assert()
        .success()
        .stdout(predicate::str::contains(
            "No functions at or above the warning threshold of 6.0.",
        ))
        .stdout(predicate::str::contains("verdict=clean"));
}

#[test]
fn zero_coverage_produces_fixture_expected_crap_score() {
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
        .arg("200")
        .arg("--project-threshold")
        .arg("100.0");

    command
        .assert()
        .success()
        .stdout(predicate::str::contains("30.0  warn"))
        .stdout(predicate::str::contains("verdict=warn"));
}

#[test]
fn root_workspace_defaults_to_all_workspace_members_when_no_package_is_provided() {
    let fixture_dir = fixture_path(&["root_workspace_fixture"]);
    let manifest_path = fixture_dir.join("Cargo.toml");
    let root_source_path = fixture_dir.join("src").join("lib.rs");
    let helper_source_path = fixture_dir.join("helper-member").join("src").join("lib.rs");
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

    command
        .assert()
        .success()
        .stdout(
            predicate::str::contains("crap4rust report for ")
                .and(predicate::str::contains("root-app"))
                .and(predicate::str::contains("helper-member")),
        )
        .stdout(predicate::str::contains("summary: total_functions=2"));
}

#[test]
fn explicit_package_in_root_workspace_overrides_all_members_default() {
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
        .arg("--package")
        .arg("root-app")
        .arg("--coverage")
        .arg(&coverage_path);

    command
        .assert()
        .success()
        .stdout(predicate::str::contains("crap4rust report for root-app"))
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

#[test]
fn json_output_format_produces_valid_json() {
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
        .arg("--output-format")
        .arg("json");

    command
        .assert()
        .success()
        .stdout(predicate::str::starts_with("{"))
        .stdout(predicate::str::contains(
            r#""scope_name": "single-fixture""#,
        ))
        .stdout(predicate::str::contains(r#""verdict": "Warn""#));
}

#[test]
fn compute_crap_score_zero_coverage_returns_complexity_squared_plus_complexity() {
    // Arrange
    let complexity = 5;
    let coverage = 0.0;

    // Act
    let score = compute_crap_score(complexity, coverage);

    // Assert
    assert!((score - 30.0).abs() < 0.001);
}

#[test]
fn compute_crap_score_full_coverage_returns_complexity_only() {
    // Arrange
    let complexity = 10;
    let coverage = 1.0;

    // Act
    let score = compute_crap_score(complexity, coverage);

    // Assert
    assert!((score - 10.0).abs() < 0.001);
}

#[test]
fn compute_crap_score_half_coverage_returns_expected_value() {
    // Arrange
    let complexity = 4;
    let coverage = 0.5;

    // Act
    let score = compute_crap_score(complexity, coverage);

    // Assert
    let expected = 16.0 * 0.125 + 4.0;
    assert!((score - expected).abs() < 0.001);
}

#[test]
fn compute_crap_score_zero_complexity_returns_zero() {
    // Arrange
    let complexity = 0;
    let coverage = 0.0;

    // Act
    let score = compute_crap_score(complexity, coverage);

    // Assert
    assert!((score - 0.0).abs() < 0.001);
}

#[test]
fn compute_crap_score_complexity_one_full_coverage_returns_one() {
    // Arrange
    let complexity = 1;
    let coverage = 1.0;

    // Act
    let score = compute_crap_score(complexity, coverage);

    // Assert
    assert!((score - 1.0).abs() < 0.001);
}

#[test]
fn classify_above_threshold_returns_crappy() {
    // Arrange & Act
    let verdict = classify(31.0, 30.0, 20.0);

    // Assert
    assert_eq!(verdict, Verdict::Crappy);
}

#[test]
fn classify_at_threshold_returns_warn() {
    // Arrange & Act
    let verdict = classify(30.0, 30.0, 20.0);

    // Assert
    assert_eq!(verdict, Verdict::Warn);
}

#[test]
fn classify_between_warn_and_threshold_returns_warn() {
    // Arrange & Act
    let verdict = classify(25.0, 30.0, 20.0);

    // Assert
    assert_eq!(verdict, Verdict::Warn);
}

#[test]
fn classify_at_warn_threshold_returns_warn() {
    // Arrange & Act
    let verdict = classify(20.0, 30.0, 20.0);

    // Assert
    assert_eq!(verdict, Verdict::Warn);
}

#[test]
fn classify_below_warn_threshold_returns_clean() {
    // Arrange & Act
    let verdict = classify(19.9, 30.0, 20.0);

    // Assert
    assert_eq!(verdict, Verdict::Clean);
}

#[test]
fn classify_zero_score_returns_clean() {
    // Arrange & Act
    let verdict = classify(0.0, 30.0, 20.0);

    // Assert
    assert_eq!(verdict, Verdict::Clean);
}

#[test]
fn coverage_ratio_zero_total_regions_returns_zero() {
    // Arrange
    let record = CoverageRecord {
        path_key: String::new(),
        line: 0,
        covered_regions: 0,
        total_regions: 0,
    };

    // Act
    let ratio = record.coverage_ratio();

    // Assert
    assert!((ratio - 0.0).abs() < 0.001);
}

#[test]
fn coverage_ratio_half_covered_returns_half() {
    // Arrange
    let record = CoverageRecord {
        path_key: String::new(),
        line: 0,
        covered_regions: 5,
        total_regions: 10,
    };

    // Act
    let ratio = record.coverage_ratio();

    // Assert
    assert!((ratio - 0.5).abs() < 0.001);
}

#[test]
fn coverage_ratio_fully_covered_returns_one() {
    // Arrange
    let record = CoverageRecord {
        path_key: String::new(),
        line: 0,
        covered_regions: 10,
        total_regions: 10,
    };

    // Act
    let ratio = record.coverage_ratio();

    // Assert
    assert!((ratio - 1.0).abs() < 0.001);
}

#[test]
fn match_function_coverage_exact_match_returns_record() {
    // Arrange
    let mut index = HashMap::new();
    index.insert(
        (String::from("src/lib.rs"), 10),
        CoverageRecord {
            path_key: String::from("src/lib.rs"),
            line: 10,
            covered_regions: 3,
            total_regions: 5,
        },
    );
    let function = SourceFunction {
        package_name: String::from("test"),
        name: String::from("foo"),
        path_key: String::from("src/lib.rs"),
        relative_file: String::from("src/lib.rs"),
        line: 10,
        end_line: 20,
        complexity: 1,
    };

    // Act
    let result = match_function_coverage(&function, &index);

    // Assert
    assert!(result.is_some());
    assert_eq!(result.unwrap().covered_regions, 3);
}

#[test]
fn match_function_coverage_fuzzy_match_within_span_returns_nearest() {
    // Arrange
    let mut index = HashMap::new();
    index.insert(
        (String::from("src/lib.rs"), 12),
        CoverageRecord {
            path_key: String::from("src/lib.rs"),
            line: 12,
            covered_regions: 7,
            total_regions: 10,
        },
    );
    let function = SourceFunction {
        package_name: String::from("test"),
        name: String::from("foo"),
        path_key: String::from("src/lib.rs"),
        relative_file: String::from("src/lib.rs"),
        line: 10,
        end_line: 20,
        complexity: 1,
    };

    // Act
    let result = match_function_coverage(&function, &index);

    // Assert
    assert!(result.is_some());
    assert_eq!(result.unwrap().covered_regions, 7);
}

#[test]
fn match_function_coverage_no_match_returns_none() {
    // Arrange
    let mut index = HashMap::new();
    index.insert(
        (String::from("src/other.rs"), 10),
        CoverageRecord {
            path_key: String::from("src/other.rs"),
            line: 10,
            covered_regions: 1,
            total_regions: 1,
        },
    );
    let function = SourceFunction {
        package_name: String::from("test"),
        name: String::from("foo"),
        path_key: String::from("src/lib.rs"),
        relative_file: String::from("src/lib.rs"),
        line: 10,
        end_line: 20,
        complexity: 1,
    };

    // Act
    let result = match_function_coverage(&function, &index);

    // Assert
    assert!(result.is_none());
}

fn test_config() -> Config {
    Config {
        coverage_path: None,
        manifest_path: None,
        packages: vec![],
        features: None,
        all_features: false,
        no_default_features: false,
        include_test_targets: false,
        exclude_paths: vec![],
        threshold: 30.0,
        warn_threshold: 20.0,
        project_threshold: 5.0,
        strict: true,
        warn_only: false,
        output_format: OutputFormat::Human,
    }
}

#[test]
fn project_fails_strict_with_one_crappy_returns_true() {
    // Arrange
    let config = test_config();

    // Act
    let result = project_fails(1, 0.5, &config);

    // Assert
    assert!(result);
}

#[test]
fn project_fails_non_strict_below_threshold_returns_false() {
    // Arrange
    let mut config = test_config();
    config.strict = false;
    config.project_threshold = 5.0;

    // Act
    let result = project_fails(1, 4.9, &config);

    // Assert
    assert!(!result);
}

#[test]
fn project_fails_non_strict_above_threshold_returns_true() {
    // Arrange
    let mut config = test_config();
    config.strict = false;

    // Act
    let result = project_fails(2, 5.1, &config);

    // Assert
    assert!(result);
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
