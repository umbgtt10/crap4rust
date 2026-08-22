// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use anyhow::Result;

use crate::coverage_io::coverage_record::CoverageRecord;
use crate::invocation::config::Config;
use crate::invocation::package_context::PackageContext;

pub trait CoverageProvider {
    fn provide(&self, config: &Config, packages: &[PackageContext]) -> Result<Vec<CoverageRecord>>;
}
