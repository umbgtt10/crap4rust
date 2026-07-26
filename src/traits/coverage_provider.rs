// Copyright 2025 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License or Apache License, Version 2.0
// SPDX-License-Identifier: MIT OR Apache-2.0

use anyhow::Result;

use crate::config::Config;
use crate::coverage_record::CoverageRecord;
use crate::package_context::PackageContext;

pub trait CoverageProvider {
    fn provide(&self, config: &Config, packages: &[PackageContext]) -> Result<Vec<CoverageRecord>>;
}
