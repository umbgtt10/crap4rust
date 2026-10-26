// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use alloc::vec::Vec;

use crate::coverage_io::coverage_failure::CoverageFailure;
use crate::coverage_io::coverage_record::CoverageRecord;
use crate::invocation::package_context::PackageContext;

pub trait CoverageProvider {
    fn provide(&self, packages: &[PackageContext]) -> Result<Vec<CoverageRecord>, CoverageFailure>;
}
