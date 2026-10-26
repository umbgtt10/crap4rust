// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use alloc::collections::BTreeSet;
use alloc::format;
use alloc::string::{String, ToString};

use syn::{File, Item, ItemMod};

use crate::analysis::impl_collector::ImplCollector;
use crate::analysis::source_path::SourcePath;
use crate::traits::path_keys::PathKeys;

// The files a `#[cfg(test)] mod name;` declaration brings in, which are test
// code however they are named. A module file is the one rustc would load --
// `name.rs` beside the declaring file, else `name/mod.rs` -- if the source
// tree lists it; files are compared by their path keys.
pub struct TestModuleRegistry<'a> {
    path_keys: &'a dyn PathKeys,
    excluded_files: BTreeSet<String>,
}

impl<'a> TestModuleRegistry<'a> {
    #[must_use]
    pub fn build(
        files: &[(SourcePath, File)],
        listed_files: &[SourcePath],
        path_keys: &'a dyn PathKeys,
    ) -> Self {
        let listed_keys = listed_files
            .iter()
            .map(|file| path_keys.key_of(file))
            .collect::<BTreeSet<_>>();
        let mut excluded_files = BTreeSet::new();
        for (file_path, syntax) in files {
            let base_dir = Self::own_base_dir(file_path);
            excluded_files.extend(Self::collect_excluded_mods(
                &syntax.items,
                &base_dir,
                &listed_keys,
                path_keys,
            ));
        }
        Self {
            path_keys,
            excluded_files,
        }
    }

    #[must_use]
    pub fn is_excluded(&self, file_path: &SourcePath) -> bool {
        self.excluded_files
            .contains(&self.path_keys.key_of(file_path))
    }

    fn collect_excluded_mods(
        items: &[Item],
        base_dir: &SourcePath,
        listed_keys: &BTreeSet<String>,
        path_keys: &dyn PathKeys,
    ) -> BTreeSet<String> {
        let mut excluded_files = BTreeSet::new();
        for item in items {
            let Item::Mod(item_mod) = item else {
                continue;
            };
            match &item_mod.content {
                Some((_, nested_items)) => {
                    let nested_dir = base_dir.join(&item_mod.ident.to_string());
                    excluded_files.extend(Self::collect_excluded_mods(
                        nested_items,
                        &nested_dir,
                        listed_keys,
                        path_keys,
                    ));
                }
                None => excluded_files.extend(Self::register_if_test_gated(
                    item_mod,
                    base_dir,
                    listed_keys,
                    path_keys,
                )),
            }
        }
        excluded_files
    }

    fn own_base_dir(file_path: &SourcePath) -> SourcePath {
        let parent = file_path
            .parent()
            .unwrap_or_else(|| SourcePath::new(String::new()));
        let stem = file_path.file_stem().unwrap_or_default();
        if matches!(stem, "lib" | "main" | "mod") {
            parent
        } else {
            parent.join(stem)
        }
    }

    fn register_if_test_gated(
        item_mod: &ItemMod,
        base_dir: &SourcePath,
        listed_keys: &BTreeSet<String>,
        path_keys: &dyn PathKeys,
    ) -> Option<String> {
        if !ImplCollector::is_test_attrs(&item_mod.attrs) {
            return None;
        }
        Self::resolve_file_module(base_dir, item_mod, listed_keys, path_keys)
    }

    fn resolve_file_module(
        base_dir: &SourcePath,
        item_mod: &ItemMod,
        listed_keys: &BTreeSet<String>,
        path_keys: &dyn PathKeys,
    ) -> Option<String> {
        let name = item_mod.ident.to_string();

        let sibling_file = path_keys.key_of(&base_dir.join(&format!("{name}.rs")));
        if listed_keys.contains(&sibling_file) {
            return Some(sibling_file);
        }

        let directory_file = path_keys.key_of(&base_dir.join(&name).join("mod.rs"));
        if listed_keys.contains(&directory_file) {
            return Some(directory_file);
        }

        None
    }
}
