// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crap4rust::invocation::config::Config;

fn test_config(strict: bool, project_threshold: f64) -> Config {
    Config::default()
        .with_threshold(30.0)
        .with_warn_threshold(20.0)
        .with_strict(strict)
        .with_project_threshold(project_threshold)
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
fn with_exclude_paths_keeps_every_path() {
    // Arrange
    let config = Config::default();

    // Act
    let config = config.with_exclude_paths(vec![String::from("benches"), String::from("examples")]);

    // Assert
    assert_eq!(config.exclude_paths(), ["benches", "examples"]);
}

#[test]
fn with_include_test_targets_keeps_the_flag() {
    // Arrange
    let config = Config::default();

    // Act
    let config = config.with_include_test_targets(true);

    // Assert
    assert!(config.include_test_targets());
}

#[test]
fn with_packages_keeps_every_package_in_order() {
    // Arrange
    let config = Config::default();

    // Act
    let config = config.with_packages(vec![String::from("foo"), String::from("bar")]);

    // Assert
    assert_eq!(config.packages(), ["foo", "bar"]);
}

#[test]
fn with_project_threshold_keeps_the_threshold() {
    // Arrange
    let config = Config::default();

    // Act
    let config = config.with_project_threshold(7.5);

    // Assert
    assert_eq!(config.project_threshold(), 7.5);
}

#[test]
fn with_strict_keeps_the_flag() {
    // Arrange
    let config = Config::default();

    // Act
    let config = config.with_strict(true);

    // Assert
    assert!(config.strict());
}

#[test]
fn with_threshold_keeps_the_threshold() {
    // Arrange
    let config = Config::default();

    // Act
    let config = config.with_threshold(42.0);

    // Assert
    assert_eq!(config.threshold(), 42.0);
}

#[test]
fn with_warn_only_keeps_the_flag() {
    // Arrange
    let config = Config::default();

    // Act
    let config = config.with_warn_only(true);

    // Assert
    assert!(config.warn_only());
}

#[test]
fn with_warn_threshold_keeps_the_threshold() {
    // Arrange
    let config = Config::default();

    // Act
    let config = config.with_warn_threshold(7.0);

    // Assert
    assert_eq!(config.warn_threshold(), 7.0);
}
