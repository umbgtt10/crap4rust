// Copyright 2025 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use std::fs;
use std::iter;
use std::path::{Path, PathBuf};

use anyhow::{Context, Result};
use syn::spanned::Spanned;
use syn::{File, Item, ItemEnum, ItemFn, ItemMod, ItemStruct};
use walkdir::WalkDir;

use crate::complexity::cognitive_complexity;
use crate::impl_collector::{ImplCollector, end_line, is_test_attrs, qualified_name, start_line};
use crate::package_context::PackageContext;
use crate::source_function::SourceFunction;
use crate::test_module_registry::TestModuleRegistry;

type ParsedFile = (PathBuf, File);

pub struct FileWalker<'a> {
    package: &'a PackageContext,
    include_test_targets: bool,
    exclude_paths: &'a [String],
}

impl<'a> FileWalker<'a> {
    pub fn new(package: &'a PackageContext) -> Self {
        Self {
            package,
            include_test_targets: package.include_test_targets,
            exclude_paths: &package.exclude_paths,
        }
    }

    pub fn process_source_root(&self, source_root: &Path) -> Result<Vec<SourceFunction>> {
        if !source_root.exists() {
            return Ok(Vec::new());
        }

        let parsed_files = self.collect_parsed_files(source_root)?;
        let test_modules = TestModuleRegistry::build(&parsed_files);

        let mut functions = Vec::new();
        for (file_path, syntax) in &parsed_files {
            if test_modules.is_excluded(file_path) {
                continue;
            }
            functions.extend(self.visit_parsed_file(source_root, file_path, syntax));
        }
        Ok(functions)
    }

    fn collect_parsed_files(&self, source_root: &Path) -> Result<Vec<ParsedFile>> {
        let mut parsed_files = Vec::new();
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
            if !self.is_selected(file_path) {
                continue;
            }
            let source = fs::read_to_string(file_path)
                .with_context(|| format!("failed to read source file {}", file_path.display()))?;
            let syntax = syn::parse_file(&source)
                .with_context(|| format!("failed to parse source file {}", file_path.display()))?;
            parsed_files.push((file_path.to_path_buf(), syntax));
        }
        Ok(parsed_files)
    }

    fn is_selected(&self, file_path: &Path) -> bool {
        let relative_file = Self::relative_file(&self.package.manifest_dir, file_path);
        Self::is_selected_relative_file(&relative_file, self.include_test_targets)
            && Self::is_selected_source_file(
                &self.package.manifest_dir,
                file_path,
                self.include_test_targets,
            )
            && !Self::is_excluded_relative_file(&relative_file, self.exclude_paths)
    }

    fn visit_parsed_file(
        &self,
        source_root: &Path,
        file_path: &Path,
        syntax: &File,
    ) -> Vec<SourceFunction> {
        let relative_file = Self::relative_file(&self.package.manifest_dir, file_path);
        let module_prefix = Self::module_prefix(source_root, file_path);
        Self::visit_items(
            self.package,
            syntax,
            &normalize_path(file_path),
            &relative_file,
            &module_prefix,
            &[],
        )
    }

    pub fn relative_file(base_dir: &Path, file_path: &Path) -> String {
        file_path
            .strip_prefix(base_dir)
            .unwrap_or(file_path)
            .to_string_lossy()
            .replace('\\', "/")
    }

    pub fn is_selected_source_file(
        base_dir: &Path,
        file_path: &Path,
        include_test_targets: bool,
    ) -> bool {
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

    pub fn is_excluded_relative_file(relative_file: &str, exclude_paths: &[String]) -> bool {
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

    pub fn is_selected_relative_file(relative_file: &str, include_test_targets: bool) -> bool {
        !relative_file.starts_with("examples/")
            && !relative_file.starts_with("benches/")
            && relative_file != "build.rs"
            && (include_test_targets || !relative_file.starts_with("tests/"))
    }

    pub fn module_prefix(source_root: &Path, file_path: &Path) -> Vec<String> {
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

    pub fn visit_items(
        package: &PackageContext,
        syntax: &File,
        path_key: &str,
        relative_file: &str,
        module_prefix: &[String],
        inline_modules: &[String],
    ) -> Vec<SourceFunction> {
        syntax
            .items
            .iter()
            .flat_map(|item| {
                Self::visit_item(
                    package,
                    item,
                    path_key,
                    relative_file,
                    module_prefix,
                    inline_modules,
                )
            })
            .collect()
    }

    pub fn visit_item(
        package: &PackageContext,
        item: &Item,
        path_key: &str,
        relative_file: &str,
        module_prefix: &[String],
        inline_modules: &[String],
    ) -> Vec<SourceFunction> {
        match item {
            Item::Fn(item_fn) => Self::record_function(
                package,
                item_fn,
                None,
                path_key,
                relative_file,
                module_prefix,
                inline_modules,
            )
            .into_iter()
            .collect(),
            Item::Impl(item_impl) if !is_test_attrs(&item_impl.attrs) => ImplCollector::visit_impl(
                package,
                item_impl,
                path_key,
                relative_file,
                module_prefix,
                inline_modules,
            ),
            Item::Mod(item_mod) if !is_test_attrs(&item_mod.attrs) => Self::visit_module(
                package,
                item_mod,
                path_key,
                relative_file,
                module_prefix,
                inline_modules,
            ),
            Item::Enum(ItemEnum { .. }) | Item::Struct(ItemStruct { .. }) => Vec::new(),
            _ => Vec::new(),
        }
    }

    pub fn visit_module(
        package: &PackageContext,
        item_mod: &ItemMod,
        path_key: &str,
        relative_file: &str,
        module_prefix: &[String],
        inline_modules: &[String],
    ) -> Vec<SourceFunction> {
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
            .flat_map(|item| {
                Self::visit_item(
                    package,
                    item,
                    path_key,
                    relative_file,
                    module_prefix,
                    &nested_modules,
                )
            })
            .collect()
    }

    pub fn record_function(
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
