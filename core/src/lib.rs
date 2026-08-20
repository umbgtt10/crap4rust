// Copyright 2025 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

pub mod app;
pub mod cargo_package_resolver;
pub mod cli;
pub mod complexity;
pub mod config;
pub mod coverage;
pub mod coverage_index;
pub mod coverage_record;
pub mod crap_formula;
pub mod default_scorer;
pub mod entry_point;
pub mod export;
pub mod file_walker;
pub mod function_report;
pub mod impl_collector;
pub mod item_visitor;
pub mod llvm_cov_builder;
pub mod llvm_cov_provider;
pub mod normalize_path;
pub mod output_format;
pub mod package_context;
pub mod project_metrics;
pub mod project_report;
pub mod source_function;
pub mod source_function_discovery;
pub mod source_root_collector;
pub mod stdout_reporter;
pub mod test_module_registry;
pub mod traits;
pub mod verdict;
