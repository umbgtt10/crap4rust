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
    let block = parse_fn_block("");
    let score = cognitive_complexity(&block);
    assert_eq!(score, 0);
}

#[test]
fn single_if_scores_one() {
    let block = parse_fn_block("if x { }");
    let score = cognitive_complexity(&block);
    assert_eq!(score, 1);
}

#[test]
fn nested_if_scores_three() {
    let block = parse_fn_block("if x { if y { } }");
    let score = cognitive_complexity(&block);
    assert_eq!(score, 3);
}

#[test]
fn match_three_arms_scores_one() {
    let block = parse_fn_block("match x { 1 => {}, 2 => {}, 3 => {} }");
    let score = cognitive_complexity(&block);
    assert_eq!(score, 1);
}

#[test]
fn for_loop_scores_one() {
    let block = parse_fn_block("for _ in 0..10 { }");
    let score = cognitive_complexity(&block);
    assert_eq!(score, 1);
}

#[test]
fn while_loop_scores_one() {
    let block = parse_fn_block("while true { }");
    let score = cognitive_complexity(&block);
    assert_eq!(score, 1);
}

#[test]
fn logical_and_scores_one() {
    let block = parse_fn_block("if a && b { }");
    let score = cognitive_complexity(&block);
    assert_eq!(score, 2);
}

#[test]
fn logical_and_or_scores_two() {
    let block = parse_fn_block("if a && b || c { }");
    let score = cognitive_complexity(&block);
    assert_eq!(score, 3);
}

#[test]
fn try_block_scores_one() {
    let block = parse_fn_block("let _ = try { 1 };");
    let score = cognitive_complexity(&block);
    assert_eq!(score, 1);
}

#[test]
fn loop_scores_one() {
    let block = parse_fn_block("loop { break; }");
    let score = cognitive_complexity(&block);
    assert_eq!(score, 1);
}
