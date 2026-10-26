# crap4rust-core

The analysis behind [`cargo-crap4rust`](https://crates.io/crates/cargo-crap4rust):
CRAP scores for Rust functions from their cognitive complexity and test
coverage. This crate is function discovery, the complexity score, the coverage
arithmetic, the formula, the verdict and the human report; it reaches cargo,
cargo-llvm-cov and the file system only through traits. The command,
`cargo crap4rust`, is the `cargo-crap4rust` crate.

What it computes and why are in the
[main README](https://github.com/umbgtt10/crap4rust#readme) and
[`docs/FORMULA.md`](https://github.com/umbgtt10/crap4rust/blob/main/docs/FORMULA.md).

MIT -- see [`LICENSE`](https://github.com/umbgtt10/crap4rust/blob/main/LICENSE).
