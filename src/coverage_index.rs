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
        let mut inner: HashMap<(String, usize), CoverageRecord> = HashMap::new();
        for record in records {
            let key = (record.path_key.clone(), record.line);
            match inner.remove(&key) {
                None => {
                    inner.insert(key, record);
                }
                Some(existing) => {
                    inner.insert(key, Self::merge_duplicate(existing, record));
                }
            }
        }
        Self { inner }
    }

    fn merge_duplicate(existing: CoverageRecord, incoming: CoverageRecord) -> CoverageRecord {
        if incoming.covered_regions == 0 && existing.covered_regions > 0 {
            return existing;
        }
        if existing.covered_regions == 0 && incoming.covered_regions > 0 {
            return incoming;
        }
        CoverageRecord {
            path_key: existing.path_key,
            line: existing.line,
            covered_regions: existing.covered_regions + incoming.covered_regions,
            total_regions: existing.total_regions + incoming.total_regions,
        }
    }

    pub fn match_function(&self, function: &SourceFunction) -> Option<f64> {
        match_function_coverage(function, &self.inner).map(|record| record.coverage_ratio())
    }

    pub fn get(&self, path_key: &str, line: usize) -> Option<&CoverageRecord> {
        self.inner.get(&(path_key.to_string(), line))
    }
}

pub fn match_function_coverage(
    function: &SourceFunction,
    coverage_index: &HashMap<(String, usize), CoverageRecord>,
) -> Option<CoverageRecord> {
    let matching: Vec<&CoverageRecord> = coverage_index
        .iter()
        .filter(|((path_key, line), _)| {
            path_key == &function.path_key && *line >= function.line && *line <= function.end_line
        })
        .map(|(_, record)| record)
        .collect();

    if matching.is_empty() {
        return None;
    }

    let covered_regions = matching.iter().map(|r| r.covered_regions).sum();
    let total_regions = matching.iter().map(|r| r.total_regions).sum();

    Some(CoverageRecord {
        path_key: function.path_key.clone(),
        line: function.line,
        covered_regions,
        total_regions,
    })
}
