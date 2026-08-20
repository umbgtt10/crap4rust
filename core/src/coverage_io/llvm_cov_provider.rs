// Copyright 2025 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use anyhow::Result;

use crate::coverage_io::coverage::{ensure_coverage_path, load_coverage_records};
use crate::coverage_io::coverage_record::CoverageRecord;
use crate::invocation::config::Config;
use crate::invocation::package_context::PackageContext;
use crate::traits::coverage_provider::CoverageProvider;

#[derive(Debug, Clone, Copy, Default)]
pub struct LlvmCovProvider;

impl LlvmCovProvider {
    #[must_use]
    pub const fn new() -> Self {
        Self
    }
}

impl CoverageProvider for LlvmCovProvider {
    fn provide(&self, config: &Config, packages: &[PackageContext]) -> Result<Vec<CoverageRecord>> {
        let coverage_path = ensure_coverage_path(config, packages)?;
        load_coverage_records(&coverage_path)
    }
}
