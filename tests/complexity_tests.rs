// Copyright 2025 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License or Apache License, Version 2.0
// SPDX-License-Identifier: MIT OR Apache-2.0

use crap4rust::complexity::cognitive_complexity;
use syn::parse_file;

fn parse_fn_block(source: &str) -> syn::Block {
    let source = format!("fn f() {{ {} }}", source);
    let file = parse_file(&source).expect("parse source");
    let item_fn = match &file.items[0] {
        syn::Item::Fn(f) => f,
        _ => panic!("expected function"),
    };
    (*item_fn.block).clone()
}

#[test]
fn empty_function_returns_zero() {
    // Arrange
    let block = parse_fn_block("");

    // Act
    let score = cognitive_complexity(&block);

    // Assert
    assert_eq!(score, 0);
}

#[test]
fn single_if_scores_one() {
    // Arrange
    let block = parse_fn_block("if x { }");

    // Act
    let score = cognitive_complexity(&block);

    // Assert
    assert_eq!(score, 1);
}

#[test]
fn nested_if_scores_three() {
    // Arrange
    let block = parse_fn_block("if x { if y { } }");

    // Act
    let score = cognitive_complexity(&block);

    // Assert
    assert_eq!(score, 3);
}

#[test]
fn match_three_arms_scores_one() {
    // Arrange
    let block = parse_fn_block("match x { 1 => {}, 2 => {}, 3 => {} }");

    // Act
    let score = cognitive_complexity(&block);

    // Assert
    assert_eq!(score, 1);
}

#[test]
fn for_loop_scores_one() {
    // Arrange
    let block = parse_fn_block("for _ in 0..10 { }");

    // Act
    let score = cognitive_complexity(&block);

    // Assert
    assert_eq!(score, 1);
}

#[test]
fn while_loop_scores_one() {
    // Arrange
    let block = parse_fn_block("while true { }");

    // Act
    let score = cognitive_complexity(&block);

    // Assert
    assert_eq!(score, 1);
}

#[test]
fn if_with_logical_and_condition_scores_two() {
    // Arrange
    let block = parse_fn_block("if a && b { }");

    // Act
    let score = cognitive_complexity(&block);

    // Assert
    assert_eq!(score, 2);
}

#[test]
fn if_with_logical_and_or_condition_scores_three() {
    // Arrange
    let block = parse_fn_block("if a && b || c { }");

    // Act
    let score = cognitive_complexity(&block);

    // Assert
    assert_eq!(score, 3);
}

#[test]
fn try_block_scores_one() {
    // Arrange
    let block = parse_fn_block("let _ = try { 1 };");

    // Act
    let score = cognitive_complexity(&block);

    // Assert
    assert_eq!(score, 1);
}

#[test]
fn loop_scores_one() {
    // Arrange
    let block = parse_fn_block("loop { break; }");

    // Act
    let score = cognitive_complexity(&block);

    // Assert
    assert_eq!(score, 1);
}

#[test]
fn while_with_logical_and_condition_scores_two() {
    // Arrange
    let block = parse_fn_block("while a && b { }");

    // Act
    let score = cognitive_complexity(&block);

    // Assert
    assert_eq!(score, 2);
}

#[test]
fn while_with_bare_condition_matches_logical_condition_base_cost() {
    // Arrange
    let bare = parse_fn_block("while a { }");
    let logical = parse_fn_block("while a && b { }");

    // Act
    let bare_score = cognitive_complexity(&bare);
    let logical_score = cognitive_complexity(&logical);

    // Assert
    assert_eq!(logical_score - bare_score, 1);
}

#[test]
fn match_guard_with_logical_and_condition_scores_two() {
    // Arrange
    let block = parse_fn_block("match x { n if a && b => 1, _ => 0 };");

    // Act
    let score = cognitive_complexity(&block);

    // Assert
    assert_eq!(score, 2);
}
