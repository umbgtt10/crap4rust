// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum Verdict {
    #[default]
    Clean,
    Warn,
    Crappy,
}

impl Verdict {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Clean => "clean",
            Self::Warn => "warn",
            Self::Crappy => "crappy",
        }
    }
}
