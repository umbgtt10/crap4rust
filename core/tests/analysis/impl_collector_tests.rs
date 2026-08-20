// Copyright 2025 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crap4rust::analysis::impl_collector::{
    ImplCollector, end_line, is_test_attrs, qualified_name, start_line,
};
use crap4rust::invocation::package_context::PackageContext;
use std::path::PathBuf;
use syn::parse_file;
use syn::parse_quote;
use syn::spanned::Spanned;

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
fn collect_excludes_test_attributed_methods() {
    // Arrange
    let package = test_package();
    let module_prefix: Vec<String> = vec![];
    let inline_modules: Vec<String> = vec![];
    let item_impl: syn::ItemImpl = parse_quote! {
        impl Foo {
            #[test]
            fn test_only(&self) {}
            pub fn real_method(&self) {}
        }
    };
    let collector = ImplCollector::new(
        &package,
        "src/lib.rs",
        "src/lib.rs",
        &module_prefix,
        &inline_modules,
    );

    // Act
    let functions = collector.collect(&item_impl);

    // Assert
    assert_eq!(functions.len(), 1);
    assert_eq!(functions[0].name, "Foo::real_method");
}

#[test]
fn is_test_attrs_detects_cfg_test_attribute() {
    // Arrange
    let attrs: Vec<syn::Attribute> = vec![parse_quote!(#[cfg(test)])];

    // Act
    let result = is_test_attrs(&attrs);

    // Assert
    assert!(result);
}

#[test]
fn is_test_attrs_detects_test_attribute() {
    // Arrange
    let attrs: Vec<syn::Attribute> = vec![parse_quote!(#[test])];

    // Act
    let result = is_test_attrs(&attrs);

    // Assert
    assert!(result);
}

#[test]
fn is_test_attrs_returns_false_for_empty_attrs() {
    // Arrange
    let attrs: Vec<syn::Attribute> = vec![];

    // Act
    let result = is_test_attrs(&attrs);

    // Assert
    assert!(!result);
}

#[test]
fn is_test_attrs_returns_false_for_regular_attributes() {
    // Arrange
    let attrs: Vec<syn::Attribute> = vec![parse_quote!(#[allow(dead_code)])];

    // Act
    let result = is_test_attrs(&attrs);

    // Assert
    assert!(!result);
}

#[test]
fn qualified_name_with_receiver_and_module_prefix() {
    // Arrange
    let module_prefix = vec![String::from("foo")];
    let inline_modules: Vec<String> = vec![];

    // Act
    let result = qualified_name(&module_prefix, &inline_modules, Some("Bar"), "baz");

    // Assert
    assert_eq!(result, "foo::Bar::baz");
}

#[test]
fn qualified_name_without_receiver() {
    // Arrange
    let module_prefix: Vec<String> = vec![];
    let inline_modules: Vec<String> = vec![];

    // Act
    let result = qualified_name(&module_prefix, &inline_modules, None, "baz");

    // Assert
    assert_eq!(result, "baz");
}

#[test]
fn start_line_and_end_line_reflect_span() {
    // Arrange
    let file = parse_file("fn foo() {\n    let x = 1;\n}").expect("parse source");
    let syn::Item::Fn(item_fn) = &file.items[0] else {
        panic!("expected function")
    };

    // Act
    let start = start_line(item_fn.sig.ident.span());
    let end = end_line(item_fn.span());

    // Assert
    assert_eq!(start, 1);
    assert_eq!(end, 3);
}

#[test]
fn visit_impl_qualifies_method_name_with_receiver() {
    // Arrange
    let package = test_package();
    let module_prefix: Vec<String> = vec![];
    let inline_modules: Vec<String> = vec![];
    let item_impl: syn::ItemImpl = parse_quote! {
        impl Foo {
            pub fn bar(&self) {}
        }
    };

    // Act
    let functions = ImplCollector::visit_impl(
        &package,
        &item_impl,
        "src/lib.rs",
        "src/lib.rs",
        &module_prefix,
        &inline_modules,
    );

    // Assert
    assert_eq!(functions.len(), 1);
    assert_eq!(functions[0].name, "Foo::bar");
}
