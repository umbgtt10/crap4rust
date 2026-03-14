// Copyright 2025 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License or Apache License, Version 2.0
// SPDX-License-Identifier: MIT OR Apache-2.0

pub fn test_support_risky(a: bool, b: bool, c: bool, d: bool, e: bool) -> u32 {
    let mut score = 0;
    if a {
        score += 1;
    }
    if b {
        score += 1;
    }
    if c {
        score += 1;
    }
    if d {
        score += 1;
    }
    if e {
        score += 1;
    }
    score
}