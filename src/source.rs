// Copyright 2025 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License or Apache License, Version 2.0
// SPDX-License-Identifier: MIT OR Apache-2.0

use std::fs;
use std::path::Path;

use anyhow::{Context, Result};
use proc_macro2::Span;
use syn::spanned::Spanned;
use syn::{
    Arm, Attribute, Block, Expr, ExprBinary, ExprBlock, ExprForLoop, ExprIf, ExprLoop, ExprMatch,
    ExprWhile, File, ImplItem, Item, ItemEnum, ItemFn, ItemImpl, ItemMod, ItemStruct, LocalInit,
    Pat, Path as SynPath, Stmt, Type, parse_file,
};
use walkdir::WalkDir;

use crate::model::{PackageContext, SourceFunction};

pub fn discover_functions(package: &PackageContext) -> Result<Vec<SourceFunction>> {
    let mut functions = Vec::new();

    for source_root in &package.source_roots {
        if !source_root.exists() {
            continue;
        }

        for entry in WalkDir::new(source_root)
            .into_iter()
            .filter_map(|entry| entry.ok())
            .filter(|entry| entry.file_type().is_file())
            .filter(|entry| {
                entry
                    .path()
                    .extension()
                    .is_some_and(|extension| extension == "rs")
            })
        {
            let file_path = entry.path();
            let relative_file = relative_file(&package.manifest_dir, file_path);
            if !is_production_relative_file(&relative_file)
                || !is_production_source_file(&package.manifest_dir, file_path)
            {
                continue;
            }
            let module_prefix = module_prefix(source_root, file_path);
            let source = fs::read_to_string(file_path)
                .with_context(|| format!("failed to read source file {}", file_path.display()))?;
            let syntax = parse_file(&source)
                .with_context(|| format!("failed to parse source file {}", file_path.display()))?;

            visit_items(
                package,
                &syntax,
                &normalize_path(file_path),
                &relative_file,
                &module_prefix,
                &mut Vec::new(),
                &mut functions,
            );
        }
    }

    Ok(functions)
}

pub fn normalize_path(path: &Path) -> String {
    let normalized = path
        .canonicalize()
        .unwrap_or_else(|_| path.to_path_buf())
        .to_string_lossy()
        .replace('\\', "/");
    if cfg!(windows) {
        normalized.to_lowercase()
    } else {
        normalized
    }
}

fn relative_file(base_dir: &Path, file_path: &Path) -> String {
    file_path
        .strip_prefix(base_dir)
        .unwrap_or(file_path)
        .to_string_lossy()
        .replace('\\', "/")
}

fn is_production_source_file(base_dir: &Path, file_path: &Path) -> bool {
    let base_dir = normalize_path(base_dir);
    let file_path = normalize_path(file_path);
    let Some(relative) = file_path.strip_prefix(&base_dir) else {
        return true;
    };
    let relative = relative.strip_prefix('/').unwrap_or(relative);

    let mut components = relative.split('/');
    let Some(first) = components.next() else {
        return true;
    };

    if matches!(first, "tests" | "examples" | "benches") {
        return false;
    }

    !relative.ends_with("/build.rs") && relative != "build.rs"
}

fn is_production_relative_file(relative_file: &str) -> bool {
    !relative_file.starts_with("tests/")
        && !relative_file.starts_with("examples/")
        && !relative_file.starts_with("benches/")
        && relative_file != "build.rs"
}

fn module_prefix(source_root: &Path, file_path: &Path) -> Vec<String> {
    let relative = file_path.strip_prefix(source_root).unwrap_or(file_path);
    let mut prefix = relative
        .parent()
        .map(|parent| {
            parent
                .components()
                .map(|component| component.as_os_str().to_string_lossy().to_string())
                .collect::<Vec<_>>()
        })
        .unwrap_or_default();
    let file_stem = file_path
        .file_stem()
        .and_then(|stem| stem.to_str())
        .unwrap_or_default();
    if !matches!(file_stem, "lib" | "main" | "mod") {
        prefix.push(file_stem.to_string());
    }
    prefix
}

fn visit_items(
    package: &PackageContext,
    syntax: &File,
    path_key: &str,
    relative_file: &str,
    module_prefix: &[String],
    inline_modules: &mut Vec<String>,
    functions: &mut Vec<SourceFunction>,
) {
    for item in &syntax.items {
        visit_item(
            package,
            item,
            path_key,
            relative_file,
            module_prefix,
            inline_modules,
            functions,
        );
    }
}

fn visit_item(
    package: &PackageContext,
    item: &Item,
    path_key: &str,
    relative_file: &str,
    module_prefix: &[String],
    inline_modules: &mut Vec<String>,
    functions: &mut Vec<SourceFunction>,
) {
    match item {
        Item::Fn(item_fn) => {
            if let Some(function) = record_function(
                package,
                item_fn,
                None,
                path_key,
                relative_file,
                module_prefix,
                inline_modules,
            ) {
                functions.push(function);
            }
        }
        Item::Impl(item_impl) => {
            if is_test_attrs(&item_impl.attrs) {
                return;
            }

            visit_impl(
                package,
                item_impl,
                path_key,
                relative_file,
                module_prefix,
                inline_modules,
                functions,
            )
        }
        Item::Mod(item_mod) => {
            if is_test_attrs(&item_mod.attrs) {
                return;
            }

            visit_module(
                package,
                item_mod,
                path_key,
                relative_file,
                module_prefix,
                inline_modules,
                functions,
            )
        }
        Item::Enum(ItemEnum { .. }) | Item::Struct(ItemStruct { .. }) => {}
        _ => {}
    }
}

fn visit_module(
    package: &PackageContext,
    item_mod: &ItemMod,
    path_key: &str,
    relative_file: &str,
    module_prefix: &[String],
    inline_modules: &mut Vec<String>,
    functions: &mut Vec<SourceFunction>,
) {
    let Some((_, items)) = &item_mod.content else {
        return;
    };

    inline_modules.push(item_mod.ident.to_string());
    for item in items {
        visit_item(
            package,
            item,
            path_key,
            relative_file,
            module_prefix,
            inline_modules,
            functions,
        );
    }
    inline_modules.pop();
}

fn visit_impl(
    package: &PackageContext,
    item_impl: &ItemImpl,
    path_key: &str,
    relative_file: &str,
    module_prefix: &[String],
    inline_modules: &[String],
    functions: &mut Vec<SourceFunction>,
) {
    let receiver = impl_type_name(&item_impl.self_ty);
    for item in &item_impl.items {
        if let ImplItem::Fn(method) = item {
            if is_test_attrs(&method.attrs) {
                continue;
            }
            let name = qualified_name(
                module_prefix,
                inline_modules,
                Some(&receiver),
                &method.sig.ident.to_string(),
            );
            functions.push(SourceFunction {
                package_name: package.name.clone(),
                name,
                path_key: path_key.to_string(),
                relative_file: relative_file.to_string(),
                line: start_line(method.sig.ident.span()),
                end_line: end_line(method.span()),
                complexity: cognitive_complexity(&method.block),
            });
        }
    }
}

fn record_function(
    package: &PackageContext,
    item_fn: &ItemFn,
    receiver: Option<&str>,
    path_key: &str,
    relative_file: &str,
    module_prefix: &[String],
    inline_modules: &[String],
) -> Option<SourceFunction> {
    if is_test_attrs(&item_fn.attrs) {
        return None;
    }

    let name = qualified_name(
        module_prefix,
        inline_modules,
        receiver,
        &item_fn.sig.ident.to_string(),
    );
    Some(SourceFunction {
        package_name: package.name.clone(),
        name,
        path_key: path_key.to_string(),
        relative_file: relative_file.to_string(),
        line: start_line(item_fn.sig.ident.span()),
        end_line: end_line(item_fn.span()),
        complexity: cognitive_complexity(&item_fn.block),
    })
}

fn impl_type_name(ty: &Type) -> String {
    match ty {
        Type::Path(path) => path
            .path
            .segments
            .last()
            .map(|segment| segment.ident.to_string())
            .unwrap_or_else(|| "impl".to_string()),
        Type::Reference(reference) => impl_type_name(&reference.elem),
        _ => "impl".to_string(),
    }
}

fn qualified_name(
    module_prefix: &[String],
    inline_modules: &[String],
    receiver: Option<&str>,
    function_name: &str,
) -> String {
    let mut parts = Vec::new();
    parts.extend(module_prefix.iter().cloned());
    parts.extend(inline_modules.iter().cloned());
    if let Some(receiver) = receiver {
        parts.push(receiver.to_string());
    }
    parts.push(function_name.to_string());
    parts.join("::")
}

fn is_test_attrs(attrs: &[syn::Attribute]) -> bool {
    attrs.iter().any(is_test_attr)
}

fn is_test_attr(attr: &Attribute) -> bool {
    if is_test_path(attr.path()) {
        return true;
    }

    let mut found = false;
    let _ = attr.parse_nested_meta(|meta| {
        if is_test_path(&meta.path) {
            found = true;
        }

        let _ = meta.parse_nested_meta(|nested| {
            if is_test_path(&nested.path) {
                found = true;
            }
            Ok(())
        });

        Ok(())
    });

    found
}

fn is_test_path(path: &SynPath) -> bool {
    path.is_ident("test")
        || path
            .segments
            .last()
            .is_some_and(|segment| segment.ident == "test")
}

fn start_line(span: Span) -> usize {
    span.start().line
}

fn end_line(span: Span) -> usize {
    span.end().line
}

fn cognitive_complexity(block: &Block) -> u32 {
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
        Expr::Try(expr_try) => 1 + nesting + score_expr(&expr_try.expr, nesting),
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
    arm.guard.as_ref().map_or(0, |(_, expr)| {
        logical_expr_score(expr) + score_expr(expr, nesting)
    }) + score_expr(&arm.body, nesting)
}

fn score_for(expr_for: &ExprForLoop, nesting: u32) -> u32 {
    1 + nesting
        + score_expr(&expr_for.expr, nesting)
        + pattern_complexity(&expr_for.pat)
        + score_block(&expr_for.body, nesting + 1)
}

fn score_while(expr_while: &ExprWhile, nesting: u32) -> u32 {
    1 + nesting
        + logical_expr_score(&expr_while.cond)
        + score_expr(&expr_while.cond, nesting)
        + score_block(&expr_while.body, nesting + 1)
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
