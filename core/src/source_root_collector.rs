// Copyright 2025 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use std::collections::BTreeSet;
use std::path::{Path, PathBuf};

use cargo_metadata::Target;

pub struct SourceRootCollector<'a> {
    include_test_targets: bool,
    manifest_dir: &'a Path,
    source_roots: BTreeSet<PathBuf>,
}

impl<'a> SourceRootCollector<'a> {
    pub fn new(include_test_targets: bool, manifest_dir: &'a Path) -> Self {
        Self {
            include_test_targets,
            manifest_dir,
            source_roots: BTreeSet::new(),
        }
    }

    pub fn collect(&mut self, targets: &[Target]) {
        for target in targets {
            self.process_target(target);
        }
    }

    pub fn process_target(&mut self, target: &Target) {
        if !Self::is_selected_target(target, self.include_test_targets) {
            return;
        }

        if target
            .src_path
            .extension()
            .is_some_and(|extension| extension == "rs")
        {
            let path = target.src_path.clone().into_std_path_buf();
            if let Some(parent) = path.parent() {
                self.source_roots.insert(parent.to_path_buf());
            }
        }
    }

    pub fn finalize(mut self) -> Vec<PathBuf> {
        if self.source_roots.is_empty() {
            self.source_roots.insert(self.manifest_dir.join("src"));
        }
        self.source_roots.into_iter().collect()
    }

    pub fn is_selected_target(target: &Target, include_test_targets: bool) -> bool {
        let kinds = target
            .kind
            .iter()
            .map(|kind| kind.to_string())
            .collect::<Vec<_>>();

        if kinds.iter().any(|kind| kind == "custom-build") {
            return false;
        }

        if include_test_targets {
            return kinds.iter().any(|kind| {
                matches!(
                    kind.as_str(),
                    "lib"
                        | "bin"
                        | "proc-macro"
                        | "rlib"
                        | "dylib"
                        | "cdylib"
                        | "staticlib"
                        | "test"
                )
            });
        }

        if kinds
            .iter()
            .any(|kind| matches!(kind.as_str(), "test" | "bench" | "example"))
        {
            return false;
        }

        kinds.iter().any(|kind| {
            matches!(
                kind.as_str(),
                "lib" | "bin" | "proc-macro" | "rlib" | "dylib" | "cdylib" | "staticlib"
            )
        })
    }
}
