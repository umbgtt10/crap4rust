// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crap4rust::analysis::complexity::cognitive_complexity;
use syn::Block;
use syn::Item;
use syn::parse_file;

fn parse_fn_block(source: &str) -> Block {
    let source = format!("fn f() {{ {} }}", source);
    let file = parse_file(&source).expect("parse source");
    let item_fn = match &file.items[0] {
        Item::Fn(f) => f,
        _ => panic!("expected function"),
    };
    (*item_fn.block).clone()
}

#[test]
fn cognitive_complexity_of_a_block_expression_returns_the_score_of_its_contents() {
    // Arrange
    let block = parse_fn_block("let _ = { a && b };");

    // Act
    let score = cognitive_complexity(&block);

    // Assert
    assert_eq!(score, 1);
}

#[test]
fn cognitive_complexity_of_a_break_returns_the_score_of_its_value() {
    // Arrange
    let block = parse_fn_block("loop { break a && b; }");

    // Act
    let score = cognitive_complexity(&block);

    // Assert
    assert_eq!(score, 2);
}

#[test]
fn cognitive_complexity_of_a_call_returns_the_score_of_its_arguments() {
    // Arrange
    let block = parse_fn_block("f(a && b);");

    // Act
    let score = cognitive_complexity(&block);

    // Assert
    assert_eq!(score, 1);
}

#[test]
fn cognitive_complexity_of_a_cast_returns_the_score_of_its_operand() {
    // Arrange
    let block = parse_fn_block("let _ = f(a && b) as u8;");

    // Act
    let score = cognitive_complexity(&block);

    // Assert
    assert_eq!(score, 1);
}

#[test]
fn cognitive_complexity_of_a_closure_returns_the_score_of_its_body() {
    // Arrange
    let block = parse_fn_block("let _ = |x| a && b;");

    // Act
    let score = cognitive_complexity(&block);

    // Assert
    assert_eq!(score, 1);
}

#[test]
fn cognitive_complexity_of_a_comparison_returns_the_scores_of_its_operands_alone() {
    // Arrange
    let block = parse_fn_block("let _ = f(a && b) == g(c && d);");

    // Act
    let score = cognitive_complexity(&block);

    // Assert
    assert_eq!(score, 2);
}

#[test]
fn cognitive_complexity_of_a_condition_with_an_and_right_of_an_or_returns_both_operators() {
    // Arrange
    let block = parse_fn_block("if a || b && c { }");

    // Act
    let score = cognitive_complexity(&block);

    // Assert
    assert_eq!(score, 3);
}

#[test]
fn cognitive_complexity_of_a_const_block_returns_the_score_of_its_body() {
    // Arrange
    let block = parse_fn_block("let _ = const { A && B };");

    // Act
    let score = cognitive_complexity(&block);

    // Assert
    assert_eq!(score, 1);
}

#[test]
fn cognitive_complexity_of_a_field_access_returns_the_score_of_its_base() {
    // Arrange
    let block = parse_fn_block("let _ = f(a && b).field;");

    // Act
    let score = cognitive_complexity(&block);

    // Assert
    assert_eq!(score, 1);
}

#[test]
fn cognitive_complexity_of_a_for_loop_body_returns_its_score_one_level_deeper() {
    // Arrange
    let block = parse_fn_block("for _ in xs { if a { } }");

    // Act
    let score = cognitive_complexity(&block);

    // Assert
    assert_eq!(score, 3);
}

#[test]
fn cognitive_complexity_of_a_for_loop_inside_an_if_returns_its_nesting() {
    // Arrange
    let block = parse_fn_block("if x { for _ in xs { } }");

    // Act
    let score = cognitive_complexity(&block);

    // Assert
    assert_eq!(score, 3);
}

#[test]
fn cognitive_complexity_of_a_for_loop_over_an_or_pattern_returns_one_per_extra_case() {
    // Arrange
    let block = parse_fn_block("for Ok(x) | Err(x) in xs { }");

    // Act
    let score = cognitive_complexity(&block);

    // Assert
    assert_eq!(score, 2);
}

#[test]
fn cognitive_complexity_of_a_for_loop_returns_the_score_of_its_iterator() {
    // Arrange
    let block = parse_fn_block("for _ in f(a && b) { }");

    // Act
    let score = cognitive_complexity(&block);

    // Assert
    assert_eq!(score, 2);
}

#[test]
fn cognitive_complexity_of_a_function_in_a_nested_module_returns_the_score_of_its_body() {
    // Arrange
    let block = parse_fn_block("mod m { fn inner() { if a { if b { } } } }");

    // Act
    let score = cognitive_complexity(&block);

    // Assert
    assert_eq!(score, 3);
}

#[test]
fn cognitive_complexity_of_a_let_else_returns_the_score_of_its_else_block() {
    // Arrange
    let block = parse_fn_block("let Some(x) = y else { return a && b; };");

    // Act
    let score = cognitive_complexity(&block);

    // Assert
    assert_eq!(score, 1);
}

#[test]
fn cognitive_complexity_of_a_let_returns_the_score_of_its_initialiser() {
    // Arrange
    let block = parse_fn_block("let _ = a && b && c;");

    // Act
    let score = cognitive_complexity(&block);

    // Assert
    assert_eq!(score, 2);
}

#[test]
fn cognitive_complexity_of_a_loop_body_returns_its_score_one_level_deeper() {
    // Arrange
    let block = parse_fn_block("loop { if a { break; } }");

    // Act
    let score = cognitive_complexity(&block);

    // Assert
    assert_eq!(score, 3);
}

#[test]
fn cognitive_complexity_of_a_loop_inside_an_if_returns_its_nesting() {
    // Arrange
    let block = parse_fn_block("if x { loop { } }");

    // Act
    let score = cognitive_complexity(&block);

    // Assert
    assert_eq!(score, 3);
}

#[test]
fn cognitive_complexity_of_a_match_arm_returns_its_score_one_level_deeper() {
    // Arrange
    let block = parse_fn_block("match x { _ => if a { } }");

    // Act
    let score = cognitive_complexity(&block);

    // Assert
    assert_eq!(score, 3);
}

#[test]
fn cognitive_complexity_of_a_match_guard_returns_the_score_of_an_if_in_it_one_level_deeper() {
    // Arrange
    let block = parse_fn_block("match x { n if f(if a { 1 } else { 2 }) => 1, _ => 0 };");

    // Act
    let score = cognitive_complexity(&block);

    // Assert
    assert_eq!(score, 3);
}

#[test]
fn cognitive_complexity_of_a_match_inside_an_if_returns_its_nesting() {
    // Arrange
    let block = parse_fn_block("if x { match y { _ => {} } }");

    // Act
    let score = cognitive_complexity(&block);

    // Assert
    assert_eq!(score, 3);
}

#[test]
fn cognitive_complexity_of_a_match_returns_the_score_of_its_scrutinee() {
    // Arrange
    let block = parse_fn_block("match f(a && b) { _ => {} }");

    // Act
    let score = cognitive_complexity(&block);

    // Assert
    assert_eq!(score, 2);
}

#[test]
fn cognitive_complexity_of_a_method_call_returns_the_score_of_its_arguments() {
    // Arrange
    let block = parse_fn_block("x.f(a && b);");

    // Act
    let score = cognitive_complexity(&block);

    // Assert
    assert_eq!(score, 1);
}

#[test]
fn cognitive_complexity_of_a_method_in_a_nested_module_returns_the_score_of_its_body() {
    // Arrange
    let block = parse_fn_block("mod m { impl S { fn inner() { if a { } } } }");

    // Act
    let score = cognitive_complexity(&block);

    // Assert
    assert_eq!(score, 1);
}

#[test]
fn cognitive_complexity_of_a_method_in_an_impl_in_the_body_returns_the_score_of_its_body() {
    // Arrange
    let block = parse_fn_block("impl S { fn inner() { if a { } } }");

    // Act
    let score = cognitive_complexity(&block);

    // Assert
    assert_eq!(score, 1);
}

#[test]
fn cognitive_complexity_of_a_module_in_a_nested_module_returns_the_score_of_its_functions() {
    // Arrange
    let block = parse_fn_block("mod m { mod n { fn inner() { if a { } } } }");

    // Act
    let score = cognitive_complexity(&block);

    // Assert
    assert_eq!(score, 1);
}

#[test]
fn cognitive_complexity_of_a_negation_returns_the_score_of_its_operand() {
    // Arrange
    let block = parse_fn_block("let _ = !f(a && b);");

    // Act
    let score = cognitive_complexity(&block);

    // Assert
    assert_eq!(score, 1);
}

#[test]
fn cognitive_complexity_of_a_nested_function_returns_the_score_of_its_body() {
    // Arrange
    let block = parse_fn_block("fn inner() { if a { } }");

    // Act
    let score = cognitive_complexity(&block);

    // Assert
    assert_eq!(score, 1);
}

#[test]
fn cognitive_complexity_of_a_parenthesised_condition_returns_its_operators() {
    // Arrange
    let block = parse_fn_block("if (a && b) || c { }");

    // Act
    let score = cognitive_complexity(&block);

    // Assert
    assert_eq!(score, 3);
}

#[test]
fn cognitive_complexity_of_a_parenthesised_expression_returns_the_score_of_its_contents() {
    // Arrange
    let block = parse_fn_block("let _ = (a && b);");

    // Act
    let score = cognitive_complexity(&block);

    // Assert
    assert_eq!(score, 1);
}

#[test]
fn cognitive_complexity_of_a_question_mark_returns_the_score_of_its_operand() {
    // Arrange
    let block = parse_fn_block("f(a && b)?;");

    // Act
    let score = cognitive_complexity(&block);

    // Assert
    assert_eq!(score, 1);
}

#[test]
fn cognitive_complexity_of_a_range_returns_the_score_of_its_end() {
    // Arrange
    let block = parse_fn_block("let _ = 0..f(a && b);");

    // Act
    let score = cognitive_complexity(&block);

    // Assert
    assert_eq!(score, 1);
}

#[test]
fn cognitive_complexity_of_a_raw_address_returns_the_score_of_its_place() {
    // Arrange
    let block = parse_fn_block("let _ = &raw const x[f(a && b)];");

    // Act
    let score = cognitive_complexity(&block);

    // Assert
    assert_eq!(score, 1);
}

#[test]
fn cognitive_complexity_of_a_reference_returns_the_score_of_its_operand() {
    // Arrange
    let block = parse_fn_block("let _ = &f(a && b);");

    // Act
    let score = cognitive_complexity(&block);

    // Assert
    assert_eq!(score, 1);
}

#[test]
fn cognitive_complexity_of_a_repeat_returns_the_score_of_its_length() {
    // Arrange
    let block = parse_fn_block("let _ = [0; f(a && b)];");

    // Act
    let score = cognitive_complexity(&block);

    // Assert
    assert_eq!(score, 1);
}

#[test]
fn cognitive_complexity_of_a_return_returns_the_score_of_its_value() {
    // Arrange
    let block = parse_fn_block("return a && b;");

    // Act
    let score = cognitive_complexity(&block);

    // Assert
    assert_eq!(score, 1);
}

#[test]
fn cognitive_complexity_of_a_struct_literal_returns_the_score_of_its_base() {
    // Arrange
    let block = parse_fn_block("let _ = S { x: 0, ..f(a && b) };");

    // Act
    let score = cognitive_complexity(&block);

    // Assert
    assert_eq!(score, 1);
}

#[test]
fn cognitive_complexity_of_a_try_block_body_returns_its_score_one_level_deeper() {
    // Arrange
    let block = parse_fn_block("let _ = try { if a { } };");

    // Act
    let score = cognitive_complexity(&block);

    // Assert
    assert_eq!(score, 3);
}

#[test]
fn cognitive_complexity_of_a_try_block_inside_an_if_returns_its_nesting() {
    // Arrange
    let block = parse_fn_block("if x { let _ = try { }; }");

    // Act
    let score = cognitive_complexity(&block);

    // Assert
    assert_eq!(score, 3);
}

#[test]
fn cognitive_complexity_of_a_tuple_returns_the_score_of_its_elements() {
    // Arrange
    let block = parse_fn_block("let _ = (a && b, 0);");

    // Act
    let score = cognitive_complexity(&block);

    // Assert
    assert_eq!(score, 1);
}

#[test]
fn cognitive_complexity_of_a_while_body_returns_its_score_one_level_deeper() {
    // Arrange
    let block = parse_fn_block("while a { if b { } }");

    // Act
    let score = cognitive_complexity(&block);

    // Assert
    assert_eq!(score, 3);
}

#[test]
fn cognitive_complexity_of_a_while_condition_returns_the_score_of_an_if_in_it() {
    // Arrange
    let block = parse_fn_block("while f(if a { 1 } else { 2 }) { }");

    // Act
    let score = cognitive_complexity(&block);

    // Assert
    assert_eq!(score, 2);
}

#[test]
fn cognitive_complexity_of_a_while_inside_an_if_returns_its_nesting() {
    // Arrange
    let block = parse_fn_block("if x { while a { } }");

    // Act
    let score = cognitive_complexity(&block);

    // Assert
    assert_eq!(score, 3);
}

#[test]
fn cognitive_complexity_of_a_while_let_condition_returns_the_score_of_a_match_in_it() {
    // Arrange
    let block = parse_fn_block("while let Some(x) = match y { _ => z } { }");

    // Act
    let score = cognitive_complexity(&block);

    // Assert
    assert_eq!(score, 2);
}

#[test]
fn cognitive_complexity_of_a_yield_returns_the_score_of_its_value() {
    // Arrange
    let block = parse_fn_block("yield a && b;");

    // Act
    let score = cognitive_complexity(&block);

    // Assert
    assert_eq!(score, 1);
}

#[test]
fn cognitive_complexity_of_an_array_returns_the_score_of_its_elements() {
    // Arrange
    let block = parse_fn_block("let _ = [a && b];");

    // Act
    let score = cognitive_complexity(&block);

    // Assert
    assert_eq!(score, 1);
}

#[test]
fn cognitive_complexity_of_an_assignment_returns_the_score_of_its_value() {
    // Arrange
    let block = parse_fn_block("x = a && b;");

    // Act
    let score = cognitive_complexity(&block);

    // Assert
    assert_eq!(score, 1);
}

#[test]
fn cognitive_complexity_of_an_async_block_returns_the_score_of_its_body() {
    // Arrange
    let block = parse_fn_block("let _ = async { a && b };");

    // Act
    let score = cognitive_complexity(&block);

    // Assert
    assert_eq!(score, 1);
}

#[test]
fn cognitive_complexity_of_an_await_returns_the_score_of_its_base() {
    // Arrange
    let block = parse_fn_block("f(a && b).await;");

    // Act
    let score = cognitive_complexity(&block);

    // Assert
    assert_eq!(score, 1);
}

#[test]
fn cognitive_complexity_of_an_else_block_returns_its_score_one_level_deeper() {
    // Arrange
    let block = parse_fn_block("if a { } else { if b { } }");

    // Act
    let score = cognitive_complexity(&block);

    // Assert
    assert_eq!(score, 3);
}

#[test]
fn cognitive_complexity_of_an_else_if_returns_the_score_of_both_conditions() {
    // Arrange
    let block = parse_fn_block("if a { } else if b { }");

    // Act
    let score = cognitive_complexity(&block);

    // Assert
    assert_eq!(score, 2);
}

#[test]
fn cognitive_complexity_of_an_if_condition_inside_an_if_returns_its_nesting() {
    // Arrange
    let block = parse_fn_block("if x { if f(if a { 1 } else { 2 }) { } }");

    // Act
    let score = cognitive_complexity(&block);

    // Assert
    assert_eq!(score, 5);
}

#[test]
fn cognitive_complexity_of_an_if_condition_returns_the_score_of_an_if_in_it() {
    // Arrange
    let block = parse_fn_block("if f(if a { 1 } else { 2 }) { }");

    // Act
    let score = cognitive_complexity(&block);

    // Assert
    assert_eq!(score, 2);
}

#[test]
fn cognitive_complexity_of_an_if_let_condition_returns_the_score_of_a_match_in_it() {
    // Arrange
    let block = parse_fn_block("if let Some(x) = match y { _ => z } { }");

    // Act
    let score = cognitive_complexity(&block);

    // Assert
    assert_eq!(score, 2);
}

#[test]
fn cognitive_complexity_of_an_index_returns_the_score_of_its_index() {
    // Arrange
    let block = parse_fn_block("let _ = x[a && b];");

    // Act
    let score = cognitive_complexity(&block);

    // Assert
    assert_eq!(score, 1);
}

#[test]
fn cognitive_complexity_of_an_unsafe_block_returns_the_score_of_its_body() {
    // Arrange
    let block = parse_fn_block("unsafe { a && b };");

    // Act
    let score = cognitive_complexity(&block);

    // Assert
    assert_eq!(score, 1);
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
fn for_loop_scores_one() {
    // Arrange
    let block = parse_fn_block("for _ in 0..10 { }");

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
fn loop_scores_one() {
    // Arrange
    let block = parse_fn_block("loop { break; }");

    // Act
    let score = cognitive_complexity(&block);

    // Assert
    assert_eq!(score, 1);
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
fn nested_if_scores_three() {
    // Arrange
    let block = parse_fn_block("if x { if y { } }");

    // Act
    let score = cognitive_complexity(&block);

    // Assert
    assert_eq!(score, 3);
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
fn try_block_scores_one() {
    // Arrange
    let block = parse_fn_block("let _ = try { 1 };");

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
fn while_with_logical_and_condition_scores_two() {
    // Arrange
    let block = parse_fn_block("while a && b { }");

    // Act
    let score = cognitive_complexity(&block);

    // Assert
    assert_eq!(score, 2);
}
