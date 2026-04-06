// Copyright 2025 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License or Apache License, Version 2.0
// SPDX-License-Identifier: MIT OR Apache-2.0

pub fn validation_only_risky(flag_a: bool, flag_b: bool, flag_c: bool, flag_d: bool) -> u32 {
    let mut score = 0;

    if flag_a {
        score += 1;
    }

    if flag_b {
        score += 1;
    }

    if flag_c {
        score += 1;
    }

    if flag_d {
        score += 1;
    }

    score
}

#[test]
fn validation_only_risky_smoke_test() {
    assert_eq!(validation_only_risky(true, false, true, false), 2);
}
