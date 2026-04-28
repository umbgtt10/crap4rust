// Copyright 2025 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License or Apache License, Version 2.0
// SPDX-License-Identifier: MIT OR Apache-2.0

use syn::parse_quote;

use crap4rust::impl_collector::is_test_attrs;

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
fn is_test_attrs_detects_cfg_test_attribute() {
    // Arrange
    let attrs: Vec<syn::Attribute> = vec![parse_quote!(#[cfg(test)])];

    // Act
    let result = is_test_attrs(&attrs);

    // Assert
    assert!(result);
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
fn is_test_attrs_returns_false_for_empty_attrs() {
    // Arrange
    let attrs: Vec<syn::Attribute> = vec![];

    // Act
    let result = is_test_attrs(&attrs);

    // Assert
    assert!(!result);
}
