// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use alloc::vec::Vec;

use crate::analysis::source_path::SourcePath;
use crate::coverage_io::coverage_failure::CoverageFailure;
use crate::coverage_io::coverage_record::CoverageRecord;
use crate::coverage_io::export::Export;
use crate::invocation::package_context::PackageContext;
use crate::traits::coverage_provider::CoverageProvider;
use crate::traits::coverage_source::CoverageSource;
use crate::traits::path_keys::PathKeys;

// A record per exported function: its file's key, the line its first region
// starts on, and how many of its regions ran at least once.
pub struct ExportCoverageProvider<'a> {
    source: &'a dyn CoverageSource,
    path_keys: &'a dyn PathKeys,
}

impl<'a> ExportCoverageProvider<'a> {
    #[must_use]
    pub fn new(source: &'a dyn CoverageSource, path_keys: &'a dyn PathKeys) -> Self {
        Self { source, path_keys }
    }

    fn records(&self, export: Export) -> Vec<CoverageRecord> {
        let mut records = Vec::new();
        for chunk in export.data {
            for function in chunk.functions {
                let Some(filename) = function.filenames.first() else {
                    continue;
                };
                let Some(first_region) = function.regions.first() else {
                    continue;
                };
                if first_region.len() < 5 {
                    continue;
                }

                let total_regions = function.regions.len() as u32;
                let covered_regions = function
                    .regions
                    .iter()
                    .filter(|region| region.get(4).copied().unwrap_or(0) > 0)
                    .count() as u32;

                records.push(CoverageRecord::new(
                    self.path_keys.key_of(&SourcePath::new(filename.clone())),
                    first_region[0] as usize,
                    covered_regions,
                    total_regions,
                ));
            }
        }
        records
    }
}

impl CoverageProvider for ExportCoverageProvider<'_> {
    fn provide(&self, packages: &[PackageContext]) -> Result<Vec<CoverageRecord>, CoverageFailure> {
        let export = self.source.export(packages)?;
        Ok(self.records(export))
    }
}
