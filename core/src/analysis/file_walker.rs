// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use std::fs;
use std::path::{Path, PathBuf};

use anyhow::{Context, Result};
use syn::File;
use walkdir::WalkDir;

use crate::analysis::item_visitor::ItemVisitor;
use crate::analysis::normalize_path::normalize_path;
use crate::analysis::source_function::SourceFunction;
use crate::analysis::test_module_registry::TestModuleRegistry;
use crate::invocation::package_context::PackageContext;
use syn::parse_file;

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

    pub(crate) fn process_source_root(&self, source_root: &Path) -> Result<Vec<SourceFunction>> {
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
            let syntax = parse_file(&source)
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
        let path_key = normalize_path(file_path);
        let visitor = ItemVisitor::new(self.package, &path_key, &relative_file, &module_prefix);
        visitor.visit_items(syntax, &[])
    }

    pub fn relative_file(base_dir: &Path, file_path: &Path) -> String {
        file_path
            .strip_prefix(base_dir)
            .unwrap_or(file_path)
            .to_string_lossy()
            .replace('\\', "/")
    }

    fn is_selected_source_file(
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
}
