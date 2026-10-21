// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use std::collections::HashSet;
use std::path::{Path, PathBuf};

use syn::{File, Item, ItemMod};

use crate::analysis::impl_collector::ImplCollector;
use crate::analysis::normalize_path::normalize_path;

pub struct TestModuleRegistry {
    excluded_files: HashSet<String>,
}

impl TestModuleRegistry {
    #[must_use]
    pub fn build(files: &[(PathBuf, File)]) -> Self {
        let mut excluded_files = HashSet::new();
        for (file_path, syntax) in files {
            let base_dir = Self::own_base_dir(file_path);
            excluded_files.extend(Self::collect_excluded_mods(&syntax.items, &base_dir));
        }
        Self { excluded_files }
    }

    #[must_use]
    pub fn is_excluded(&self, file_path: &Path) -> bool {
        self.excluded_files.contains(&normalize_path(file_path))
    }

    fn collect_excluded_mods(items: &[Item], base_dir: &Path) -> HashSet<String> {
        let mut excluded_files = HashSet::new();
        for item in items {
            let Item::Mod(item_mod) = item else {
                continue;
            };
            match &item_mod.content {
                Some((_, nested_items)) => {
                    let nested_dir = base_dir.join(item_mod.ident.to_string());
                    excluded_files.extend(Self::collect_excluded_mods(nested_items, &nested_dir));
                }
                None => excluded_files.extend(Self::register_if_test_gated(item_mod, base_dir)),
            }
        }
        excluded_files
    }

    fn own_base_dir(file_path: &Path) -> PathBuf {
        let parent = file_path
            .parent()
            .map(Path::to_path_buf)
            .unwrap_or_default();
        let stem = file_path
            .file_stem()
            .and_then(|stem| stem.to_str())
            .unwrap_or_default();
        if matches!(stem, "lib" | "main" | "mod") {
            parent
        } else {
            parent.join(stem)
        }
    }

    fn register_if_test_gated(item_mod: &ItemMod, base_dir: &Path) -> Option<String> {
        if !ImplCollector::is_test_attrs(&item_mod.attrs) {
            return None;
        }
        Self::resolve_file_module(base_dir, item_mod).map(|target| normalize_path(&target))
    }

    fn resolve_file_module(base_dir: &Path, item_mod: &ItemMod) -> Option<PathBuf> {
        let name = item_mod.ident.to_string();

        let sibling_file = base_dir.join(format!("{name}.rs"));
        if sibling_file.exists() {
            return Some(sibling_file);
        }

        let directory_file = base_dir.join(&name).join("mod.rs");
        if directory_file.exists() {
            return Some(directory_file);
        }

        None
    }
}
