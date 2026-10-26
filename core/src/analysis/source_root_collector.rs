// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use std::collections::BTreeSet;

use crate::analysis::source_path::SourcePath;
use crate::invocation::target_description::TargetDescription;

pub struct SourceRootCollector<'a> {
    include_test_targets: bool,
    manifest_dir: &'a SourcePath,
    source_roots: BTreeSet<SourcePath>,
}

impl<'a> SourceRootCollector<'a> {
    pub fn new(include_test_targets: bool, manifest_dir: &'a SourcePath) -> Self {
        Self {
            include_test_targets,
            manifest_dir,
            source_roots: BTreeSet::new(),
        }
    }

    pub fn collect(&mut self, targets: &[TargetDescription]) {
        for target in targets {
            self.process_target(target);
        }
    }

    pub fn finalize(mut self) -> Vec<SourcePath> {
        if self.source_roots.is_empty() {
            self.source_roots.insert(self.manifest_dir.join("src"));
        }
        self.source_roots.into_iter().collect()
    }

    pub fn is_selected_target(target: &TargetDescription, include_test_targets: bool) -> bool {
        let kinds: &[String] = &target.kinds;

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

    fn process_target(&mut self, target: &TargetDescription) {
        if !Self::is_selected_target(target, self.include_test_targets) {
            return;
        }

        if target.src_path.extension() == Some("rs")
            && let Some(parent) = target.src_path.parent()
        {
            self.source_roots.insert(parent);
        }
    }
}
