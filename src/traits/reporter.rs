// Copyright 2025 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::config::Config;
use crate::project_report::ProjectReport;

pub trait Reporter {
    fn render(&self, report: &ProjectReport, config: &Config);
}
