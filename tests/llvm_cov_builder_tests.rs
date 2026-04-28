// Copyright 2025 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License or Apache License, Version 2.0
// SPDX-License-Identifier: MIT OR Apache-2.0

use std::path::Path;

use crap4rust::llvm_cov_builder::LlvmCovBuilder;

#[test]
fn builder_new_constructs_without_panicking() {
    // Arrange & Act
    let builder = LlvmCovBuilder::new(Path::new("out.json"));

    // Assert
    // Builder was created successfully — no panic
    drop(builder);
}
