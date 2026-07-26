// Copyright 2025 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

pub fn helper_risky(a: bool, b: bool, c: bool, d: bool, e: bool) -> u32 {
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

#[cfg(test)]
mod tests {
    #[test]
    fn helper_member_test_should_not_run_for_root_only_analysis() {
        panic!("helper-member tests should not run during root-only automatic coverage");
    }
}
