// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use std::fs::read_to_string;
use std::path::Path;

// A fixture source file, read for the line its public functions start on. A
// coverage record is keyed by that line, so a test writing one asks here rather
// than hard-coding a number the fixture would silently drift away from.
pub struct Source {
    contents: String,
}

impl Source {
    pub fn new(path: &Path) -> Self {
        Self {
            contents: read_to_string(path).expect("read fixture source"),
        }
    }

    pub fn first_function_line(&self) -> usize {
        self.line_starting_with("pub fn ")
            .expect("fixture source contains a public function")
    }

    pub fn named_function_line(&self, function_name: &str) -> usize {
        self.line_starting_with(&format!("pub fn {function_name}"))
            .expect("fixture source contains the named public function")
    }

    fn line_starting_with(&self, needle: &str) -> Option<usize> {
        self.contents
            .lines()
            .enumerate()
            .find_map(|(index, line)| line.trim_start().starts_with(needle).then_some(index + 1))
    }
}
