// Copyright 2025 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License or Apache License, Version 2.0
// SPDX-License-Identifier: MIT OR Apache-2.0

use std::collections::HashMap;

use crate::model::{CoverageRecord, SourceFunction};

pub struct CoverageIndex {
    inner: HashMap<(String, usize), CoverageRecord>,
}

impl CoverageIndex {
    pub fn from_records(records: Vec<CoverageRecord>) -> Self {
        let mut inner = HashMap::new();
        for record in records {
            let key = (record.path_key.clone(), record.line);
            inner
                .entry(key)
                .and_modify(|existing: &mut CoverageRecord| {
                    existing.covered_regions += record.covered_regions;
                    existing.total_regions += record.total_regions;
                })
                .or_insert(record);
        }
        Self { inner }
    }

    pub fn match_function(&self, function: &SourceFunction) -> Option<f64> {
        match_function_coverage(function, &self.inner).map(|record| record.coverage_ratio())
    }
}

pub fn match_function_coverage(
    function: &SourceFunction,
    coverage_index: &HashMap<(String, usize), CoverageRecord>,
) -> Option<CoverageRecord> {
    if let Some(record) = coverage_index.get(&(function.path_key.clone(), function.line)) {
        return Some(record.clone());
    }

    coverage_index
        .iter()
        .filter(|((path_key, line), _)| {
            path_key == &function.path_key && *line >= function.line && *line <= function.end_line
        })
        .min_by_key(|((_, line), _)| *line - function.line)
        .map(|(_, record)| record.clone())
}
