// Copyright 2025 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use anyhow::Result;

use crate::config::Config;
use crate::coverage_record::CoverageRecord;
use crate::package_context::PackageContext;

pub trait CoverageProvider {
    fn provide(&self, config: &Config, packages: &[PackageContext]) -> Result<Vec<CoverageRecord>>;
}
