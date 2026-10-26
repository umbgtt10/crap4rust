// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::support::in_memory_workspace_reader::InMemoryWorkspaceReader;
use crate::support::in_memory_workspace_reader::library_package;
use crap4rust::analysis::source_path::SourcePath;
use crap4rust::invocation::cargo_package_resolver::CargoPackageResolver;
use crap4rust::invocation::config::Config;
use crap4rust::invocation::package_description::PackageDescription;
use crap4rust::invocation::target_description::TargetDescription;
use crap4rust::invocation::workspace_description::WorkspaceDescription;
use crap4rust::traits::package_resolver::PackageResolver;

fn names(config: &Config, workspace: WorkspaceDescription) -> Vec<String> {
    let reader = InMemoryWorkspaceReader::new(workspace);
    CargoPackageResolver::new(&reader)
        .resolve(config)
        .expect("resolve packages")
        .into_iter()
        .map(|package| package.name)
        .collect()
}

fn path(text: &str) -> SourcePath {
    SourcePath::new(String::from(text))
}

fn workspace(
    packages: Vec<PackageDescription>,
    root_package: Option<&str>,
) -> WorkspaceDescription {
    WorkspaceDescription {
        workspace_root: path("/ws"),
        packages,
        root_package: root_package.map(String::from),
    }
}

#[test]
fn resolve_a_package_carries_its_directory_the_workspace_root_and_the_run_options() {
    // Arrange
    let reader =
        InMemoryWorkspaceReader::new(workspace(vec![library_package("solo", "/ws/solo")], None));
    let config = Config::default()
        .with_include_test_targets(true)
        .with_exclude_paths(vec![String::from("src/generated")]);

    // Act
    let packages = CargoPackageResolver::new(&reader)
        .resolve(&config)
        .expect("resolve packages");

    // Assert
    assert_eq!(packages[0].manifest_dir, path("/ws/solo"));
    assert_eq!(packages[0].workspace_root, path("/ws"));
    assert_eq!(packages[0].source_roots, vec![path("/ws/solo/src")]);
    assert!(packages[0].include_test_targets);
    assert_eq!(
        packages[0].exclude_paths,
        vec![String::from("src/generated")]
    );
}

#[test]
fn resolve_a_package_whose_manifest_has_no_parent_returns_an_error() {
    // Arrange
    let package = PackageDescription {
        name: String::from("rootless"),
        manifest_path: path(""),
        targets: vec![],
    };
    let reader = InMemoryWorkspaceReader::new(workspace(vec![package], None));

    // Act
    let result = CargoPackageResolver::new(&reader).resolve(&Config::default());

    // Assert
    let error = result.expect_err("a manifest with no parent must not resolve");
    assert_eq!(
        error.to_string(),
        "package manifest has no parent directory"
    );
}

#[test]
fn resolve_a_package_with_no_production_target_falls_back_to_its_src() {
    // Arrange
    let package = PackageDescription {
        name: String::from("bare"),
        manifest_path: path("/ws/bare/Cargo.toml"),
        targets: vec![TargetDescription {
            kinds: vec![String::from("bench")],
            src_path: path("/ws/bare/benches/b.rs"),
        }],
    };
    let reader = InMemoryWorkspaceReader::new(workspace(vec![package], None));

    // Act
    let packages = CargoPackageResolver::new(&reader)
        .resolve(&Config::default())
        .expect("resolve packages");

    // Assert
    assert_eq!(packages[0].source_roots, vec![path("/ws/bare/src")]);
}

#[test]
fn resolve_with_a_workspace_that_cannot_be_read_returns_the_error() {
    // Arrange
    let reader = InMemoryWorkspaceReader::failing();

    // Act
    let result = CargoPackageResolver::new(&reader).resolve(&Config::default());

    // Assert
    let error = result.expect_err("an unreadable workspace must not resolve");
    assert_eq!(error.to_string(), "failed to read Cargo metadata");
}

#[test]
fn resolve_with_an_unknown_package_names_it() {
    // Arrange
    let reader =
        InMemoryWorkspaceReader::new(workspace(vec![library_package("solo", "/ws/solo")], None));
    let config = Config::default().with_packages(vec![String::from("no-such-package")]);

    // Act
    let result = CargoPackageResolver::new(&reader).resolve(&config);

    // Assert
    let error = result.expect_err("unknown package must not resolve");
    assert_eq!(
        error.to_string(),
        "package no-such-package was not found in the manifest"
    );
}

#[test]
fn resolve_with_no_members_returns_an_error() {
    // Arrange
    let reader = InMemoryWorkspaceReader::new(workspace(vec![], None));

    // Act
    let result = CargoPackageResolver::new(&reader).resolve(&Config::default());

    // Assert
    let error = result.expect_err("an empty workspace must not resolve");
    assert_eq!(
        error.to_string(),
        "manifest contains no workspace member packages"
    );
}

#[test]
fn resolve_with_requested_packages_returns_them_in_request_order() {
    // Arrange
    let config = Config::default().with_packages(vec![String::from("b"), String::from("a")]);
    let workspace = workspace(
        vec![library_package("a", "/ws/a"), library_package("b", "/ws/b")],
        None,
    );

    // Act
    let names = names(&config, workspace);

    // Assert
    assert_eq!(names, vec![String::from("b"), String::from("a")]);
}

#[test]
fn resolve_without_packages_in_a_workspace_of_one_selects_its_root() {
    // Arrange
    let workspace = workspace(vec![library_package("solo", "/ws")], Some("solo"));

    // Act
    let names = names(&Config::default(), workspace);

    // Assert
    assert_eq!(names, vec![String::from("solo")]);
}

#[test]
fn resolve_without_packages_in_a_workspace_with_a_root_and_members_selects_every_member() {
    // Arrange
    let workspace = workspace(
        vec![
            library_package("root", "/ws"),
            library_package("member", "/ws/member"),
        ],
        Some("root"),
    );

    // Act
    let names = names(&Config::default(), workspace);

    // Assert
    assert_eq!(names, vec![String::from("root"), String::from("member")]);
}
