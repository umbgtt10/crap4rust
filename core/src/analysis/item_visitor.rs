// Copyright 2025 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use std::iter;

use syn::spanned::Spanned;
use syn::{File, Item, ItemEnum, ItemFn, ItemMod, ItemStruct};

use crate::analysis::complexity::cognitive_complexity;
use crate::analysis::impl_collector::{
    ImplCollector, end_line, is_test_attrs, qualified_name, start_line,
};
use crate::analysis::source_function::SourceFunction;
use crate::invocation::package_context::PackageContext;

pub struct ItemVisitor<'a> {
    package: &'a PackageContext,
    path_key: &'a str,
    relative_file: &'a str,
    module_prefix: &'a [String],
}

impl<'a> ItemVisitor<'a> {
    pub fn new(
        package: &'a PackageContext,
        path_key: &'a str,
        relative_file: &'a str,
        module_prefix: &'a [String],
    ) -> Self {
        Self {
            package,
            path_key,
            relative_file,
            module_prefix,
        }
    }

    pub fn visit_items(&self, syntax: &File, inline_modules: &[String]) -> Vec<SourceFunction> {
        syntax
            .items
            .iter()
            .flat_map(|item| self.visit_item(item, inline_modules))
            .collect()
    }

    fn visit_item(&self, item: &Item, inline_modules: &[String]) -> Vec<SourceFunction> {
        match item {
            Item::Fn(item_fn) => self
                .record_function(item_fn, None, inline_modules)
                .into_iter()
                .collect(),
            Item::Impl(item_impl) if !is_test_attrs(&item_impl.attrs) => ImplCollector::visit_impl(
                self.package,
                item_impl,
                self.path_key,
                self.relative_file,
                self.module_prefix,
                inline_modules,
            ),
            Item::Mod(item_mod) if !is_test_attrs(&item_mod.attrs) => {
                self.visit_module(item_mod, inline_modules)
            }
            Item::Enum(ItemEnum { .. }) | Item::Struct(ItemStruct { .. }) => Vec::new(),
            _ => Vec::new(),
        }
    }

    fn visit_module(&self, item_mod: &ItemMod, inline_modules: &[String]) -> Vec<SourceFunction> {
        let Some((_, items)) = &item_mod.content else {
            return Vec::new();
        };

        let nested_modules = inline_modules
            .iter()
            .cloned()
            .chain(iter::once(item_mod.ident.to_string()))
            .collect::<Vec<_>>();

        items
            .iter()
            .flat_map(|item| self.visit_item(item, &nested_modules))
            .collect()
    }

    fn record_function(
        &self,
        item_fn: &ItemFn,
        receiver: Option<&str>,
        inline_modules: &[String],
    ) -> Option<SourceFunction> {
        if is_test_attrs(&item_fn.attrs) {
            return None;
        }

        let name = qualified_name(
            self.module_prefix,
            inline_modules,
            receiver,
            &item_fn.sig.ident.to_string(),
        );
        Some(SourceFunction {
            package_name: self.package.name.clone(),
            name,
            path_key: self.path_key.to_string(),
            relative_file: self.relative_file.to_string(),
            line: start_line(item_fn.sig.ident.span()),
            end_line: end_line(item_fn.span()),
            complexity: cognitive_complexity(&item_fn.block),
        })
    }
}
