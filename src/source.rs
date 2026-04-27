// Copyright 2025 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License or Apache License, Version 2.0
// SPDX-License-Identifier: MIT OR Apache-2.0

use std::fs;
use std::path::Path;

use anyhow::{Context, Result};
use proc_macro2::Span;
use syn::spanned::Spanned;
use syn::{
    Attribute, File, ImplItem, Item, ItemEnum, ItemFn, ItemImpl, ItemMod, ItemStruct,
    Path as SynPath, Type, parse_file,
};

use crate::complexity::cognitive_complexity;
use walkdir::WalkDir;

use crate::model::{PackageContext, SourceFunction};

pub fn discover_functions(package: &PackageContext) -> Result<Vec<SourceFunction>> {
    let mut functions = Vec::new();
    let include_test_targets = package.include_test_targets;
    let exclude_paths = &package.exclude_paths;

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
            if !is_selected_relative_file(&relative_file, include_test_targets)
                || !is_selected_source_file(&package.manifest_dir, file_path, include_test_targets)
                || is_excluded_relative_file(&relative_file, exclude_paths)
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

fn is_selected_source_file(base_dir: &Path, file_path: &Path, include_test_targets: bool) -> bool {
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

    if matches!(first, "examples" | "benches") {
        return false;
    }

    if first == "tests" {
        return include_test_targets;
    }

    !relative.ends_with("/build.rs") && relative != "build.rs"
}

fn is_excluded_relative_file(relative_file: &str, exclude_paths: &[String]) -> bool {
    exclude_paths.iter().any(|prefix| {
        let normalised = prefix.replace('\\', "/");
        let prefix_with_slash = if normalised.ends_with('/') {
            normalised.clone()
        } else {
            format!("{}/", normalised)
        };
        relative_file.starts_with(&prefix_with_slash) || relative_file == normalised
    })
}

fn is_selected_relative_file(relative_file: &str, include_test_targets: bool) -> bool {
    !relative_file.starts_with("examples/")
        && !relative_file.starts_with("benches/")
        && relative_file != "build.rs"
        && (include_test_targets || !relative_file.starts_with("tests/"))
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
