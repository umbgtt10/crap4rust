// Copyright 2025 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License or Apache License, Version 2.0
// SPDX-License-Identifier: MIT OR Apache-2.0

use anyhow::Result;

use crate::config::Config;
use crate::coverage::{ensure_coverage_path, load_coverage_records};
use crate::coverage_record::CoverageRecord;
use crate::package_context::PackageContext;
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
