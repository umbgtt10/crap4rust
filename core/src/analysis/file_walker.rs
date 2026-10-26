// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use anyhow::{Context, Result};
use syn::File;
use syn::parse_file;

use crate::analysis::item_visitor::ItemVisitor;
use crate::analysis::source_function::SourceFunction;
use crate::analysis::source_path::SourcePath;
use crate::analysis::test_module_registry::TestModuleRegistry;
use crate::invocation::package_context::PackageContext;
use crate::traits::path_keys::PathKeys;
use crate::traits::source_tree::SourceTree;

type ParsedFile = (SourcePath, File);

pub struct FileWalker<'a> {
    package: &'a PackageContext,
    source_tree: &'a dyn SourceTree,
    path_keys: &'a dyn PathKeys,
    include_test_targets: bool,
    exclude_paths: &'a [String],
}

impl<'a> FileWalker<'a> {
    pub fn new(
        package: &'a PackageContext,
        source_tree: &'a dyn SourceTree,
        path_keys: &'a dyn PathKeys,
    ) -> Self {
        Self {
            package,
            source_tree,
            path_keys,
            include_test_targets: package.include_test_targets,
            exclude_paths: &package.exclude_paths,
        }
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

    pub fn process_source_root(&self, source_root: &SourcePath) -> Result<Vec<SourceFunction>> {
        let listed_files = self.source_tree.rust_files(source_root)?;
        let parsed_files = self.collect_parsed_files(&listed_files)?;
        let test_modules = TestModuleRegistry::build(&parsed_files, &listed_files, self.path_keys);

        let mut functions = Vec::new();
        for (file_path, syntax) in &parsed_files {
            if test_modules.is_excluded(file_path) {
                continue;
            }
            functions.extend(self.visit_parsed_file(source_root, file_path, syntax));
        }
        Ok(functions)
    }

    pub fn relative_file(base_dir: &SourcePath, file_path: &SourcePath) -> String {
        file_path
            .strip_prefix(base_dir)
            .unwrap_or(file_path.as_str())
            .replace('\\', "/")
    }

    fn collect_parsed_files(&self, listed_files: &[SourcePath]) -> Result<Vec<ParsedFile>> {
        let mut parsed_files = Vec::new();
        for file_path in listed_files {
            if !self.is_selected(file_path) {
                continue;
            }
            let source = self.source_tree.read_source(file_path)?;
            let syntax = parse_file(&source)
                .with_context(|| format!("failed to parse source file {file_path}"))?;
            parsed_files.push((file_path.clone(), syntax));
        }
        Ok(parsed_files)
    }

    fn is_selected(&self, file_path: &SourcePath) -> bool {
        let relative_file = Self::relative_file(&self.package.manifest_dir, file_path);
        Self::is_selected_relative_file(&relative_file, self.include_test_targets)
            && self.is_selected_source_file(file_path)
            && !Self::is_excluded_relative_file(&relative_file, self.exclude_paths)
    }

    fn is_selected_source_file(&self, file_path: &SourcePath) -> bool {
        let base_dir = self.path_keys.key_of(&self.package.manifest_dir);
        let file_path = self.path_keys.key_of(file_path);
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
            return self.include_test_targets;
        }

        !relative.ends_with("/build.rs") && relative != "build.rs"
    }

    fn module_prefix(source_root: &SourcePath, file_path: &SourcePath) -> Vec<String> {
        let relative = SourcePath::new(String::from(
            file_path
                .strip_prefix(source_root)
                .unwrap_or(file_path.as_str()),
        ));
        let mut prefix = relative
            .parent()
            .map(|parent| {
                parent
                    .components()
                    .into_iter()
                    .map(ToString::to_string)
                    .collect::<Vec<_>>()
            })
            .unwrap_or_default();
        let file_stem = file_path.file_stem().unwrap_or_default();
        if !matches!(file_stem, "lib" | "main" | "mod") {
            prefix.push(file_stem.to_string());
        }
        prefix
    }

    fn visit_parsed_file(
        &self,
        source_root: &SourcePath,
        file_path: &SourcePath,
        syntax: &File,
    ) -> Vec<SourceFunction> {
        let relative_file = Self::relative_file(&self.package.manifest_dir, file_path);
        let module_prefix = Self::module_prefix(source_root, file_path);
        let path_key = self.path_keys.key_of(file_path);
        let visitor = ItemVisitor::new(self.package, &path_key, &relative_file, &module_prefix);
        visitor.visit_items(syntax, &[])
    }
}
