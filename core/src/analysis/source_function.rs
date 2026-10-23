// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

#[derive(Debug, Clone)]
pub struct SourceFunction {
    package_name: String,
    name: String,
    path_key: String,
    relative_file: String,
    line: usize,
    end_line: usize,
    complexity: u32,
}

impl SourceFunction {
    pub fn new(
        package_name: String,
        name: String,
        path_key: String,
        relative_file: String,
        line: usize,
        end_line: usize,
        complexity: u32,
    ) -> Self {
        Self {
            package_name,
            name,
            path_key,
            relative_file,
            line,
            end_line,
            complexity,
        }
    }

    pub fn complexity(&self) -> u32 {
        self.complexity
    }

    pub fn end_line(&self) -> usize {
        self.end_line
    }

    pub fn line(&self) -> usize {
        self.line
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn package_name(&self) -> &str {
        &self.package_name
    }

    pub fn path_key(&self) -> &str {
        &self.path_key
    }

    pub fn relative_file(&self) -> &str {
        &self.relative_file
    }
}
