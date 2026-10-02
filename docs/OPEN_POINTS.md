# Open Points

## TestModuleRegistry does not chase transitive test-module nesting

`TestModuleRegistry` (`ADR-CrossFileTestModuleExclusion.md`) resolves a
file-based `#[cfg(test)] mod name;` declaration to the file it gates, but
only one level: a further `mod nested;` declared *inside* that
already-excluded file, without repeating `#[cfg(test)]` on its own
declaration (which it does not need to, since it already inherits the gate
from its parent), is not itself resolved, so `nested`'s file stays
conservatively included. This mirrors the same non-recursive scope
boundary `grip`'s own `MethodPurityRegistry` deliberately accepted for
nested custom-accessor trust. Not started; the dominant real-world shape
(one gated `mod` statement, one sibling file) is already covered.

## Coverage region "kind" is not distinguished

`coverage.rs::load_coverage_records` treats every region in
`cargo-llvm-cov`'s JSON export identically when computing
`covered_regions`/`total_regions` — it does not read each region's `kind`
field (index 7 of the 8-element region array; `cargo-llvm-cov`'s own format
distinguishes Code/Expansion/Skipped/Gap/Branch regions). Every fixture and
real-world coverage file exercised so far has `kind == 0` (Code) throughout,
so this has not been observed to produce an incorrect ratio, and changing it
without a concrete, reproduced discrepancy would be speculative. Not
started; flagged here rather than silently assumed correct.

## Release 0.8.1 is committed but not published

Commit `83eb005` ("Release 0.8.1: ship the README to crates.io") is on
`main`. It bumps the package to 0.8.1, names `../README.md` in
`core/Cargo.toml` so that the README finally ships, and makes the README's
links absolute so that crates.io can resolve them. The release itself is not
done:

- **crates.io.** `cargo publish -p cargo-crap4rust` stopped at
  authentication on the Ubuntu laptop, which holds no crates.io token: no
  `~/.cargo/credentials.toml` and no `CARGO_REGISTRY_TOKEN`. Nothing was
  uploaded. A README reaches crates.io only with a published version, so the
  page keeps saying "No Readme" until 0.8.1 is out.
- **Tag.** `v0.8.1` does not exist yet. Earlier release tags are GPG-signed.
- **GitHub release.** Not created. As for `v0.8.0`, its title is the tag and
  its description is the version's `CHANGELOG.md` entry without the heading.

To finish, in this order, on a clean checkout whose `core/` still matches
`83eb005`, and with a crates.io token carrying the `publish-update` scope,
which `cargo login` reads at its prompt:

    cargo publish -p cargo-crap4rust
    git tag -s v0.8.1 -m v0.8.1 83eb005
    git push origin v0.8.1
    awk '/^## \[0\.8\.1\]/{f=1; next} /^## \[/{f=0} f' CHANGELOG.md > notes.md
    gh release create v0.8.1 --title v0.8.1 --notes-file notes.md

Publish first: a tag and a release for a version crates.io cannot serve would
point users at an install that fails. The entry is dated 2026-10-02; if the
release ships on a later day, the date should move with it. Started
2026-10-02; blocked on the token.
