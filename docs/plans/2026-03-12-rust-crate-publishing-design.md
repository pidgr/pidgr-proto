# Rust Crate Publishing Design

**Goal:** Publish `pidgr-proto` to crates.io alongside the existing npm (`@pidgr/proto`) and Go module releases, achieving full parity across all three language targets.

**Architecture:** Enable crates.io publishing by adding metadata to `Cargo.toml`, including tonic service stubs in `lib.rs`, adding a `cargo publish` step to the release workflow, and creating a usage README.

**Tech Stack:** prost 0.14 (messages), tonic 0.14 (gRPC clients/servers), crates.io (registry).

---

## Cargo.toml

Remove `publish = false`. Add crates.io metadata matching the npm package pattern:

- `license = "Apache-2.0"`
- `repository` / `homepage` pointing to GitHub
- `readme = "README.md"` (crate-level README)
- `keywords` and `categories` for discoverability

## lib.rs

Include both prost messages AND tonic service stubs:

```rust
pub mod pidgr {
    pub mod v1 {
        include!("../pidgr/v1/pidgr.v1.rs");
        include!("../pidgr/v1/pidgr.v1.tonic.rs");
    }
}
```

Both clients and servers are included. Dead code is eliminated by the compiler for consumers that only use one side.

## gen/rust/README.md

Usage README shown on crates.io. Covers:

- Install (`cargo add pidgr-proto`)
- Basic usage: connecting a tonic client, making an RPC call
- List of available services
- Link to proto source and full docs

## Release Workflow

Add after the npm publish step:

1. Install Rust toolchain (`dtolnay/rust-toolchain@stable`)
2. `cargo publish` in `gen/rust/` with `CARGO_REGISTRY_TOKEN` secret

The existing version consistency check already validates `gen/rust/Cargo.toml` matches the release branch version.

## Root README.md

Update the Rust install section from git dependency to crates.io:

```toml
[dependencies]
pidgr-proto = "0.39.0"
```

Add a crates.io badge alongside the existing Go Reference badge.

## CLAUDE.md

Update the Package Distribution table to reflect crates.io instead of git dependency.
