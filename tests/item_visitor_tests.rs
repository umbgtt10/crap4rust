// Copyright 2025 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use std::path::PathBuf;

use syn::parse_file;

use crap4rust::item_visitor::ItemVisitor;
use crap4rust::package_context::PackageContext;

fn test_package() -> PackageContext {
    PackageContext {
        name: String::from("test-package"),
        manifest_dir: PathBuf::from("/project"),
        workspace_root: PathBuf::from("/project"),
        source_roots: Vec::new(),
        include_test_targets: false,
        exclude_paths: Vec::new(),
    }
}

#[test]
fn visit_items_records_plain_function() {
    // Arrange
    let package = test_package();
    let file = parse_file("pub fn foo() {}").expect("parse source");
    let visitor = ItemVisitor::new(&package, "src/lib.rs", "src/lib.rs", &[]);

    // Act
    let functions = visitor.visit_items(&file, &[]);

    // Assert
    assert_eq!(functions.len(), 1);
    assert_eq!(functions[0].name, "foo");
}

#[test]
fn visit_items_excludes_test_attributed_function() {
    // Arrange
    let package = test_package();
    let file = parse_file("#[test]\nfn foo() {}").expect("parse source");
    let visitor = ItemVisitor::new(&package, "src/lib.rs", "src/lib.rs", &[]);

    // Act
    let functions = visitor.visit_items(&file, &[]);

    // Assert
    assert!(functions.is_empty());
}

#[test]
fn visit_items_recurses_into_inline_module() {
    // Arrange
    let package = test_package();
    let file = parse_file("mod foo {\n    pub fn bar() {}\n}").expect("parse source");
    let visitor = ItemVisitor::new(&package, "src/lib.rs", "src/lib.rs", &[]);

    // Act
    let functions = visitor.visit_items(&file, &[]);

    // Assert
    assert_eq!(functions.len(), 1);
    assert_eq!(functions[0].name, "foo::bar");
}

#[test]
fn visit_items_excludes_cfg_test_module() {
    // Arrange
    let package = test_package();
    let file = parse_file("#[cfg(test)]\nmod tests {\n    fn foo() {}\n}").expect("parse source");
    let visitor = ItemVisitor::new(&package, "src/lib.rs", "src/lib.rs", &[]);

    // Act
    let functions = visitor.visit_items(&file, &[]);

    // Assert
    assert!(functions.is_empty());
}

#[test]
fn visit_items_delegates_impl_blocks_to_impl_collector() {
    // Arrange
    let package = test_package();
    let file =
        parse_file("struct Foo;\nimpl Foo {\n    pub fn bar(&self) {}\n}").expect("parse source");
    let visitor = ItemVisitor::new(&package, "src/lib.rs", "src/lib.rs", &[]);

    // Act
    let functions = visitor.visit_items(&file, &[]);

    // Assert
    assert_eq!(functions.len(), 1);
    assert_eq!(functions[0].name, "Foo::bar");
}

#[test]
fn visit_items_excludes_struct_and_enum_declarations() {
    // Arrange
    let package = test_package();
    let file = parse_file("pub struct Foo;\npub enum Bar {}").expect("parse source");
    let visitor = ItemVisitor::new(&package, "src/lib.rs", "src/lib.rs", &[]);

    // Act
    let functions = visitor.visit_items(&file, &[]);

    // Assert
    assert!(functions.is_empty());
}
