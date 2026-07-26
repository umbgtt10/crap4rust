// Copyright 2025 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License or Apache License, Version 2.0
// SPDX-License-Identifier: MIT OR Apache-2.0

use crate::config::Config;
use crate::project_report::ProjectReport;

pub trait Reporter {
    fn render(&self, report: &ProjectReport, config: &Config);
}
