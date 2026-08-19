// Copyright 2025 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

pub mod target;

#[cfg(test)]
mod tests {
    #[test]
    fn helper_member_test_should_not_run_for_root_only_analysis() {
        panic!("helper-member tests should not run during root-only automatic coverage");
    }
}
