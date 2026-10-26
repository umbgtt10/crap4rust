// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::coverage_io::coverage_failure::CoverageFailure;
use crate::coverage_io::export::Export;
use crate::invocation::package_context::PackageContext;

pub trait CoverageSource {
    fn export(&self, packages: &[PackageContext]) -> Result<Export, CoverageFailure>;
}
