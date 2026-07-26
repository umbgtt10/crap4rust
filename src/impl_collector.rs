// Copyright 2025 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use proc_macro2::Span;
use syn::spanned::Spanned;
use syn::{Attribute, ImplItem, ItemImpl, Path as SynPath, Type};

use crate::complexity::cognitive_complexity;
use crate::package_context::PackageContext;
use crate::source_function::SourceFunction;

pub(crate) struct ImplCollector<'a> {
    package: &'a PackageContext,
    path_key: &'a str,
    relative_file: &'a str,
    module_prefix: &'a [String],
    inline_modules: &'a [String],
}

impl<'a> ImplCollector<'a> {
    pub(crate) fn new(
        package: &'a PackageContext,
        path_key: &'a str,
        relative_file: &'a str,
        module_prefix: &'a [String],
        inline_modules: &'a [String],
    ) -> Self {
        Self {
            package,
            path_key,
            relative_file,
            module_prefix,
            inline_modules,
        }
    }

    pub(crate) fn collect(&self, item_impl: &ItemImpl) -> Vec<SourceFunction> {
        let receiver = impl_type_name(&item_impl.self_ty);
        let mut functions = Vec::new();
        for item in &item_impl.items {
            if let Some(function) = self.process_item(item, &receiver) {
                functions.push(function);
            }
        }
        functions
    }

    pub(crate) fn process_item(&self, item: &ImplItem, receiver: &str) -> Option<SourceFunction> {
        if let ImplItem::Fn(method) = item {
            if is_test_attrs(&method.attrs) {
                return None;
            }
            let name = qualified_name(
                self.module_prefix,
                self.inline_modules,
                Some(receiver),
                &method.sig.ident.to_string(),
            );
            Some(SourceFunction {
                package_name: self.package.name.clone(),
                name,
                path_key: self.path_key.to_string(),
                relative_file: self.relative_file.to_string(),
                line: start_line(method.sig.ident.span()),
                end_line: end_line(method.span()),
                complexity: cognitive_complexity(&method.block),
            })
        } else {
            None
        }
    }
}

pub(crate) fn visit_impl(
    package: &PackageContext,
    item_impl: &ItemImpl,
    path_key: &str,
    relative_file: &str,
    module_prefix: &[String],
    inline_modules: &[String],
) -> Vec<SourceFunction> {
    ImplCollector::new(
        package,
        path_key,
        relative_file,
        module_prefix,
        inline_modules,
    )
    .collect(item_impl)
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

pub(crate) fn qualified_name(
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

pub fn is_test_attrs(attrs: &[Attribute]) -> bool {
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

pub(crate) fn start_line(span: Span) -> usize {
    span.start().line
}

pub(crate) fn end_line(span: Span) -> usize {
    span.end().line
}
