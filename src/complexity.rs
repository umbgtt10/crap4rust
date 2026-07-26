// Copyright 2025 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License or Apache License, Version 2.0
// SPDX-License-Identifier: MIT OR Apache-2.0

use syn::{
    Arm, Block, Expr, ExprBinary, ExprBlock, ExprForLoop, ExprIf, ExprLoop, ExprMatch, ExprWhile,
    ImplItem, Item, LocalInit, Pat, Stmt,
};

#[must_use]
pub fn cognitive_complexity(block: &Block) -> u32 {
    score_block(block, 0)
}

fn score_block(block: &Block, nesting: u32) -> u32 {
    block
        .stmts
        .iter()
        .map(|stmt| score_stmt(stmt, nesting))
        .sum()
}

fn score_stmt(stmt: &Stmt, nesting: u32) -> u32 {
    match stmt {
        Stmt::Local(local) => score_local_init(&local.init, nesting),
        Stmt::Item(item) => match item {
            Item::Fn(item_fn) => score_block(&item_fn.block, nesting),
            Item::Mod(item_mod) => item_mod.content.as_ref().map_or(0, |(_, items)| {
                items
                    .iter()
                    .map(|item| score_nested_item(item, nesting))
                    .sum()
            }),
            _ => 0,
        },
        Stmt::Expr(expr, _) => score_expr(expr, nesting),
        Stmt::Macro(_) => 0,
    }
}

fn score_nested_item(item: &Item, nesting: u32) -> u32 {
    match item {
        Item::Fn(item_fn) => score_block(&item_fn.block, nesting),
        Item::Impl(item_impl) => item_impl
            .items
            .iter()
            .map(|item| match item {
                ImplItem::Fn(method) => score_block(&method.block, nesting),
                _ => 0,
            })
            .sum(),
        Item::Mod(item_mod) => item_mod.content.as_ref().map_or(0, |(_, items)| {
            items
                .iter()
                .map(|item| score_nested_item(item, nesting))
                .sum()
        }),
        _ => 0,
    }
}

fn score_local_init(init: &Option<LocalInit>, nesting: u32) -> u32 {
    init.as_ref().map_or(0, |init| {
        score_expr(&init.expr, nesting)
            + init
                .diverge
                .as_ref()
                .map_or(0, |(_, expr)| score_expr(expr, nesting))
    })
}

fn score_expr(expr: &Expr, nesting: u32) -> u32 {
    match expr {
        Expr::If(expr_if) => score_if(expr_if, nesting),
        Expr::Match(expr_match) => score_match(expr_match, nesting),
        Expr::ForLoop(expr_for) => score_for(expr_for, nesting),
        Expr::While(expr_while) => score_while(expr_while, nesting),
        Expr::Loop(expr_loop) => score_loop(expr_loop, nesting),
        Expr::Block(ExprBlock { block, .. }) => score_block(block, nesting),
        Expr::Binary(expr_binary) => {
            logical_ops(expr_binary)
                + score_expr(&expr_binary.left, nesting)
                + score_expr(&expr_binary.right, nesting)
        }
        Expr::Call(expr_call) => {
            score_expr(&expr_call.func, nesting)
                + expr_call
                    .args
                    .iter()
                    .map(|argument| score_expr(argument, nesting))
                    .sum::<u32>()
        }
        Expr::MethodCall(expr_call) => {
            score_expr(&expr_call.receiver, nesting)
                + expr_call
                    .args
                    .iter()
                    .map(|argument| score_expr(argument, nesting))
                    .sum::<u32>()
        }
        Expr::Closure(expr_closure) => score_expr(&expr_closure.body, nesting),
        Expr::Async(expr_async) => score_block(&expr_async.block, nesting),
        Expr::Await(expr_await) => score_expr(&expr_await.base, nesting),
        Expr::Try(expr_try) => score_expr(&expr_try.expr, nesting),
        Expr::TryBlock(expr_try_block) => {
            1 + nesting + score_block(&expr_try_block.block, nesting + 1)
        }
        Expr::Unary(expr_unary) => score_expr(&expr_unary.expr, nesting),
        Expr::Reference(expr_reference) => score_expr(&expr_reference.expr, nesting),
        Expr::Return(expr_return) => expr_return
            .expr
            .as_ref()
            .map_or(0, |expr| score_expr(expr, nesting)),
        Expr::Break(expr_break) => expr_break
            .expr
            .as_ref()
            .map_or(0, |expr| score_expr(expr, nesting)),
        Expr::Paren(expr_paren) => score_expr(&expr_paren.expr, nesting),
        Expr::Array(expr_array) => expr_array
            .elems
            .iter()
            .map(|expr| score_expr(expr, nesting))
            .sum(),
        Expr::Assign(expr_assign) => {
            score_expr(&expr_assign.left, nesting) + score_expr(&expr_assign.right, nesting)
        }
        Expr::Field(expr_field) => score_expr(&expr_field.base, nesting),
        Expr::Index(expr_index) => {
            score_expr(&expr_index.expr, nesting) + score_expr(&expr_index.index, nesting)
        }
        Expr::Let(expr_let) => score_expr(&expr_let.expr, nesting),
        Expr::Macro(_) => 0,
        Expr::Range(expr_range) => {
            expr_range
                .start
                .as_ref()
                .map_or(0, |expr| score_expr(expr, nesting))
                + expr_range
                    .end
                    .as_ref()
                    .map_or(0, |expr| score_expr(expr, nesting))
        }
        Expr::Repeat(expr_repeat) => {
            score_expr(&expr_repeat.expr, nesting) + score_expr(&expr_repeat.len, nesting)
        }
        Expr::Struct(expr_struct) => {
            expr_struct
                .fields
                .iter()
                .map(|field| score_expr(&field.expr, nesting))
                .sum::<u32>()
                + expr_struct
                    .rest
                    .as_ref()
                    .map_or(0, |expr| score_expr(expr, nesting))
        }
        Expr::Tuple(expr_tuple) => expr_tuple
            .elems
            .iter()
            .map(|expr| score_expr(expr, nesting))
            .sum(),
        Expr::Unsafe(expr_unsafe) => score_block(&expr_unsafe.block, nesting),
        Expr::Yield(expr_yield) => expr_yield
            .expr
            .as_ref()
            .map_or(0, |expr| score_expr(expr, nesting)),
        _ => 0,
    }
}

fn score_if(expr_if: &ExprIf, nesting: u32) -> u32 {
    let mut score = 1
        + nesting
        + logical_expr_score(&expr_if.cond)
        + score_block(&expr_if.then_branch, nesting + 1);
    if let Some((_, else_branch)) = &expr_if.else_branch {
        score += match else_branch.as_ref() {
            Expr::If(else_if) => score_if(else_if, nesting),
            other => score_expr(other, nesting + 1),
        };
    }
    score
}

fn score_match(expr_match: &ExprMatch, nesting: u32) -> u32 {
    1 + nesting
        + score_expr(&expr_match.expr, nesting)
        + expr_match
            .arms
            .iter()
            .map(|arm| score_arm(arm, nesting + 1))
            .sum::<u32>()
}

fn score_arm(arm: &Arm, nesting: u32) -> u32 {
    arm.guard
        .as_ref()
        .map_or(0, |(_, expr)| logical_expr_score(expr))
        + score_expr(&arm.body, nesting)
}

fn score_for(expr_for: &ExprForLoop, nesting: u32) -> u32 {
    1 + nesting
        + score_expr(&expr_for.expr, nesting)
        + pattern_complexity(&expr_for.pat)
        + score_block(&expr_for.body, nesting + 1)
}

fn score_while(expr_while: &ExprWhile, nesting: u32) -> u32 {
    1 + nesting + logical_expr_score(&expr_while.cond) + score_block(&expr_while.body, nesting + 1)
}

fn score_loop(expr_loop: &ExprLoop, nesting: u32) -> u32 {
    1 + nesting + score_block(&expr_loop.body, nesting + 1)
}

fn pattern_complexity(pattern: &Pat) -> u32 {
    match pattern {
        Pat::Or(pattern_or) => pattern_or.cases.len().saturating_sub(1) as u32,
        _ => 0,
    }
}

fn logical_expr_score(expr: &Expr) -> u32 {
    match expr {
        Expr::Binary(binary) => {
            let current = logical_ops(binary);
            current + logical_expr_score(&binary.left) + logical_expr_score(&binary.right)
        }
        Expr::Paren(paren) => logical_expr_score(&paren.expr),
        Expr::Group(group) => logical_expr_score(&group.expr),
        _ => 0,
    }
}

fn logical_ops(expr_binary: &ExprBinary) -> u32 {
    if matches!(expr_binary.op, syn::BinOp::And(_) | syn::BinOp::Or(_)) {
        1
    } else {
        0
    }
}
