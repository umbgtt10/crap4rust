// Copyright 2025 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License or Apache License, Version 2.0
// SPDX-License-Identifier: MIT OR Apache-2.0

use anyhow::Result;

use crate::config::Config;
use crate::package_context::PackageContext;

pub trait PackageResolver {
    fn resolve(&self, config: &Config) -> Result<Vec<PackageContext>>;
}
