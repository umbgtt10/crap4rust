// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use anyhow::Result;

use crate::coverage_io::coverage::{ensure_coverage_path, load_coverage_records};
use crate::coverage_io::coverage_record::CoverageRecord;
use crate::invocation::config::Config;
use crate::invocation::package_context::PackageContext;
use crate::traits::coverage_provider::CoverageProvider;
use crate::traits::path_keys::PathKeys;

pub struct LlvmCovProvider<'a> {
    path_keys: &'a dyn PathKeys,
}

impl<'a> LlvmCovProvider<'a> {
    #[must_use]
    pub fn new(path_keys: &'a dyn PathKeys) -> Self {
        Self { path_keys }
    }
}

impl CoverageProvider for LlvmCovProvider<'_> {
    fn provide(&self, config: &Config, packages: &[PackageContext]) -> Result<Vec<CoverageRecord>> {
        let coverage_path = ensure_coverage_path(config, packages)?;
        load_coverage_records(&coverage_path, self.path_keys)
    }
}
