// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use std::collections::HashSet;
use std::path::{Path, PathBuf};

use syn::{File, Item, ItemMod};

use crate::analysis::impl_collector::is_test_attrs;
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
            Self::collect_excluded_mods(&syntax.items, &base_dir, &mut excluded_files);
        }
        Self { excluded_files }
    }

    #[must_use]
    pub fn is_excluded(&self, file_path: &Path) -> bool {
        self.excluded_files.contains(&normalize_path(file_path))
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

    fn collect_excluded_mods(
        items: &[Item],
        base_dir: &Path,
        excluded_files: &mut HashSet<String>,
    ) {
        for item in items {
            let Item::Mod(item_mod) = item else {
                continue;
            };
            match &item_mod.content {
                Some((_, nested_items)) => {
                    let nested_dir = base_dir.join(item_mod.ident.to_string());
                    Self::collect_excluded_mods(nested_items, &nested_dir, excluded_files);
                }
                None => Self::register_if_test_gated(item_mod, base_dir, excluded_files),
            }
        }
    }

    fn register_if_test_gated(
        item_mod: &ItemMod,
        base_dir: &Path,
        excluded_files: &mut HashSet<String>,
    ) {
        if !is_test_attrs(&item_mod.attrs) {
            return;
        }
        if let Some(target) = Self::resolve_file_module(base_dir, item_mod) {
            excluded_files.insert(normalize_path(&target));
        }
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
