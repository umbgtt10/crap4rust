// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::invocation::config::Config;
use crate::reporting::project_report::ProjectReport;

pub trait Reporter {
    fn render(&self, report: &ProjectReport, config: &Config);
}
