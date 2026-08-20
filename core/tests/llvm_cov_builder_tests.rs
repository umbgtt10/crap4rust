// Copyright 2025 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crap4rust::llvm_cov_builder::LlvmCovBuilder;
use std::path::Path;

#[test]
fn builder_new_constructs_without_panicking() {
    // Arrange & Act
    let builder = LlvmCovBuilder::new(Path::new("out.json"));

    // Assert
    // Builder was created successfully — no panic
    drop(builder);
}
