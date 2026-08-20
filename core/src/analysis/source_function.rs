// Copyright 2025 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

#[derive(Debug, Clone)]
pub struct SourceFunction {
    pub package_name: String,
    pub name: String,
    pub path_key: String,
    pub relative_file: String,
    pub line: usize,
    pub end_line: usize,
    pub complexity: u32,
}
