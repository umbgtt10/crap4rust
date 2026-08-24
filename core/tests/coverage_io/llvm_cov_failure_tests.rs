// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crap4rust::coverage_io::llvm_cov_failure::LlvmCovFailure;

// What cargo actually prints when the subcommand is absent. The install is
// missing, not cargo, so this arrives as a normal non-zero exit rather than a
// spawn error.
const MISSING_SUBCOMMAND_STDERR: &str =
    "error: no such command: `llvm-cov`\n\nhelp: view all installed commands with `cargo --list`\n";

// Pinned whole rather than by substring. The message is written across two
// source lines with a `\` continuation, which strips the newline and the
// indentation that follows it -- so the wording is one line with single spaces,
// and a reader of the source cannot tell that from the layout alone.
#[test]
fn describe_a_missing_subcommand_names_the_install_command() {
    // Arrange
    let failure = LlvmCovFailure::new(Some(101), String::from(MISSING_SUBCOMMAND_STDERR));

    // Act
    let described = failure.describe();

    // Assert
    assert_eq!(
        described,
        "cargo-llvm-cov is not installed, and crap4rust needs it to measure coverage. \
         Install it with: cargo install cargo-llvm-cov"
    );
}

#[test]
fn describe_a_missing_subcommand_runs_the_wording_together_on_one_line() {
    // Arrange
    let failure = LlvmCovFailure::new(Some(101), String::from(MISSING_SUBCOMMAND_STDERR));

    // Act
    let described = failure.describe();

    // Assert
    assert!(!described.contains('\n'), "the message is a single line");
    assert!(
        !described.contains("  "),
        "the source indentation after the continuation must not survive into the message"
    );
}

// The exit code alone was the whole message before, and it explained nothing.
#[test]
fn describe_a_missing_subcommand_says_it_is_not_installed_rather_than_an_exit_code() {
    // Arrange
    let failure = LlvmCovFailure::new(Some(101), String::from(MISSING_SUBCOMMAND_STDERR));

    // Act
    let described = failure.describe();

    // Assert
    assert!(described.contains("not installed"));
    assert!(
        !described.contains("101"),
        "a missing install is not usefully described by an exit code"
    );
}

#[test]
fn describe_a_real_failure_carries_the_tools_own_stderr() {
    // Arrange
    let failure = LlvmCovFailure::new(Some(1), String::from("error: could not compile `app`"));

    // Act
    let described = failure.describe();

    // Assert
    assert!(described.contains("could not compile `app`"));
}

#[test]
fn describe_a_real_failure_reports_the_exit_code() {
    // Arrange
    let failure = LlvmCovFailure::new(Some(1), String::from("error: could not compile `app`"));

    // Act
    let described = failure.describe();

    // Assert
    assert!(described.contains("1"));
}

#[test]
fn describe_with_empty_stderr_falls_back_to_the_exit_code() {
    // Arrange
    let failure = LlvmCovFailure::new(Some(2), String::new());

    // Act
    let described = failure.describe();

    // Assert
    assert_eq!(described, "cargo llvm-cov failed with exit code Some(2)");
}

#[test]
fn describe_with_only_whitespace_on_stderr_falls_back_to_the_exit_code() {
    // Arrange
    let failure = LlvmCovFailure::new(Some(2), String::from("   \n  "));

    // Act
    let described = failure.describe();

    // Assert
    assert_eq!(described, "cargo llvm-cov failed with exit code Some(2)");
}

#[test]
fn is_missing_subcommand_for_a_compile_failure_returns_false() {
    // Arrange
    let failure = LlvmCovFailure::new(Some(1), String::from("error: could not compile `app`"));

    // Act & Assert
    assert!(!failure.is_missing_subcommand());
}

#[test]
fn is_missing_subcommand_for_cargos_own_message_returns_true() {
    // Arrange
    let failure = LlvmCovFailure::new(Some(101), String::from(MISSING_SUBCOMMAND_STDERR));

    // Act & Assert
    assert!(failure.is_missing_subcommand());
}
