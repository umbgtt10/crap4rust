// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crap4rust::invocation::cli::Args;
use crap4rust::invocation::config::Config;

fn test_config(strict: bool, project_threshold: f64) -> Config {
    let mut args = vec!["crap4rust".to_string()];
    if strict {
        args.push("--strict".to_string());
    }
    args.push("--project-threshold".to_string());
    args.push(project_threshold.to_string());
    Config::from_args(Args::parse_from_args(args))
}

#[test]
fn fails_non_strict_above_threshold_returns_true() {
    // Arrange
    let config = test_config(false, 5.0);

    // Act
    let result = config.fails(2, 5.1);

    // Assert
    assert!(result);
}

#[test]
fn fails_non_strict_below_threshold_returns_false() {
    // Arrange
    let config = test_config(false, 5.0);

    // Act
    let result = config.fails(1, 4.9);

    // Assert
    assert!(!result);
}

#[test]
fn fails_strict_with_one_crappy_returns_true() {
    // Arrange
    let config = test_config(true, 5.0);

    // Act
    let result = config.fails(1, 0.5);

    // Assert
    assert!(result);
}

#[test]
fn fails_strict_with_zero_crappy_returns_false() {
    // Arrange
    let config = test_config(true, 5.0);

    // Act
    let result = config.fails(0, 100.0);

    // Assert
    assert!(!result);
}

#[test]
fn from_args_preserves_packages() {
    // Arrange
    let args = Args::parse_from_args(["crap4rust", "--package", "foo", "--package", "bar"]);

    // Act
    let config = Config::from_args(args);

    // Assert
    assert_eq!(config.packages, vec!["foo", "bar"]);
}

#[test]
fn from_args_preserves_strict_and_warn_only() {
    // Arrange
    let args = Args::parse_from_args(["crap4rust", "--strict", "--warn-only"]);

    // Act
    let config = Config::from_args(args);

    // Assert
    assert!(config.strict);
    assert!(config.warn_only);
}

#[test]
fn from_args_preserves_threshold_fields() {
    // Arrange
    let args = Args::parse_from_args(["crap4rust", "--threshold", "42", "--warn-threshold", "7"]);

    // Act
    let config = Config::from_args(args);

    // Assert
    assert_eq!(config.threshold, 42.0);
    assert_eq!(config.warn_threshold, 7.0);
}
