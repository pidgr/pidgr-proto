# Rust Crate Publishing Implementation Plan

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** Publish `pidgr-proto` to crates.io with full metadata, tonic service stubs, usage README, and CI automation.

**Architecture:** Update `gen/rust/` to be a publishable crate (metadata, lib.rs re-exports, README), add `cargo publish` to the release workflow, and update repo-level docs.

**Tech Stack:** prost 0.14, tonic 0.14, crates.io, GitHub Actions.

---

### Task 1: Update Cargo.toml with crates.io metadata

**Files:**
- Modify: `gen/rust/Cargo.toml`

**Step 1: Update Cargo.toml**

Replace the full contents of `gen/rust/Cargo.toml` with:

```toml
[package]
name = "pidgr-proto"
version = "0.39.0"
edition = "2021"
description = "Generated protobuf/gRPC stubs for the Pidgr platform"
license = "Apache-2.0"
repository = "https://github.com/pidgr/pidgr-proto"
homepage = "https://github.com/pidgr/pidgr-proto"
readme = "README.md"
keywords = ["pidgr", "protobuf", "grpc", "tonic", "codegen"]
categories = ["api-bindings"]

[dependencies]
prost = "0.14"
prost-types = "0.14"
tonic = "0.14"
tonic-prost = "0.14"
```

**Step 2: Verify it compiles**

Run: `cd gen/rust && cargo check 2>&1 | tail -5`
Expected: `Finished` with no errors.

**Step 3: Commit**

```bash
git add gen/rust/Cargo.toml
git commit -m "feat: add crates.io metadata to Rust crate"
```

---

### Task 2: Include tonic service stubs in lib.rs

**Files:**
- Modify: `gen/rust/src/lib.rs`

**Step 1: Update lib.rs**

Replace the full contents of `gen/rust/src/lib.rs` with:

```rust
pub mod pidgr {
    pub mod v1 {
        include!("../pidgr/v1/pidgr.v1.rs");
        include!("../pidgr/v1/pidgr.v1.tonic.rs");
    }
}
```

**Step 2: Verify it compiles**

Run: `cd gen/rust && cargo check 2>&1 | tail -5`
Expected: `Finished` with no errors. There may be warnings from generated code — that's fine, the generated files already have `#![allow(...)]` annotations.

**Step 3: Verify tonic clients are accessible**

Run: `cd gen/rust && cargo doc --no-deps 2>&1 | tail -5`
Expected: `Finished` — confirms all types and service clients are publicly exported and documented.

**Step 4: Commit**

```bash
git add gen/rust/src/lib.rs
git commit -m "feat: include tonic service clients and servers in crate"
```

---

### Task 3: Create gen/rust/README.md

**Files:**
- Create: `gen/rust/README.md`

**Step 1: Create the README**

Create `gen/rust/README.md` with the following content:

````markdown
# pidgr-proto

[![crates.io](https://img.shields.io/crates/v/pidgr-proto.svg)](https://crates.io/crates/pidgr-proto)
[![docs.rs](https://docs.rs/pidgr-proto/badge.svg)](https://docs.rs/pidgr-proto)
[![License](https://img.shields.io/crates/l/pidgr-proto.svg)](https://github.com/pidgr/pidgr-proto/blob/main/LICENSE)

Generated [protobuf](https://protobuf.dev/) messages and [tonic](https://github.com/hyperium/tonic) gRPC clients/servers for the [Pidgr](https://pidgr.com) platform.

## Install

```bash
cargo add pidgr-proto
```

Or add to `Cargo.toml`:

```toml
[dependencies]
pidgr-proto = "0.39.0"
```

## Usage

```rust
use pidgr_proto::pidgr::v1::inbox_service_client::InboxServiceClient;
use pidgr_proto::pidgr::v1::SyncRequest;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = InboxServiceClient::connect("https://api.pidgr.com").await?;

    let response = client.sync(SyncRequest {
        last_sync_token: String::new(),
    }).await?;

    println!("Synced {} messages", response.into_inner().entries.len());
    Ok(())
}
```

## Available Services

| Service | Client | Description |
|---------|--------|-------------|
| OrganizationService | `organization_service_client` | Org CRUD |
| MemberService | `member_service_client` | User invitation, lookup, roles |
| RoleService | `role_service_client` | Role + permission management |
| InviteLinkService | `invite_link_service_client` | Shareable invite links |
| CampaignService | `campaign_service_client` | Campaign lifecycle |
| TemplateService | `template_service_client` | Markdown template CRUD |
| ActionService | `action_service_client` | User action submission |
| InboxService | `inbox_service_client` | Inbox sync + read tracking |
| DeviceService | `device_service_client` | Push token management |
| GroupService | `group_service_client` | Recipient group targeting |
| TeamService | `team_service_client` | Organizational units |
| RenderService | `render_service_client` | Batch template rendering |
| AccessCodeService | `access_code_service_client` | Early access codes |
| ApiKeyService | `api_key_service_client` | Scoped API keys |
| SSOService | `sso_service_client` | SSO provider configuration |
| HeatmapService | `heatmap_service_client` | Heatmap data queries |
| ReplayService | `replay_service_client` | Session replay data |

All message types and enums are under `pidgr_proto::pidgr::v1`.

## Proto Source

Proto definitions live in [`proto/pidgr/v1/`](https://github.com/pidgr/pidgr-proto/tree/main/proto/pidgr/v1) with shared types in `common.proto`. Code is generated using [buf](https://buf.build) with the [neoeinstein-prost](https://buf.build/community/neoeinstein-prost) and [neoeinstein-tonic](https://buf.build/community/neoeinstein-tonic) plugins.

## Also Available

- **Go:** `go get github.com/pidgr/pidgr-proto/gen/go@latest`
- **TypeScript:** `npm install @pidgr/proto`

## License

[Apache-2.0](https://github.com/pidgr/pidgr-proto/blob/main/LICENSE)
````

**Step 2: Verify the README is picked up by cargo**

Run: `cd gen/rust && cargo package --list 2>&1 | grep README`
Expected: `README.md` appears in the list.

**Step 3: Commit**

```bash
git add gen/rust/README.md
git commit -m "docs: add crate README for crates.io"
```

---

### Task 4: Add cargo publish to release workflow

**Files:**
- Modify: `.github/workflows/release.yml:73-94`

**Step 1: Add Rust publish steps**

In `.github/workflows/release.yml`, insert the following **after** the npm publish step (after line 80) and **before** the GitHub Release step (line 82):

```yaml
      - uses: dtolnay/rust-toolchain@efa25f7f19611383d5b0ccf2d1c8914531636bf9 # v1
        with:
          toolchain: stable

      - name: Publish crate
        working-directory: gen/rust
        run: cargo publish
        env:
          CARGO_REGISTRY_TOKEN: ${{ secrets.CARGO_REGISTRY_TOKEN }}
```

**Step 2: Verify YAML is valid**

Run: `python3 -c "import yaml; yaml.safe_load(open('.github/workflows/release.yml'))" && echo "Valid YAML"`
Expected: `Valid YAML`

**Step 3: Commit**

```bash
git add .github/workflows/release.yml
git commit -m "ci: add cargo publish to release workflow"
```

---

### Task 5: Update root README.md and CLAUDE.md

**Files:**
- Modify: `README.md:52-57`
- Modify: `CLAUDE.md` (Package Distribution table)

**Step 1: Update README.md Rust install section**

Replace the current Rust section (lines 52-57) with:

```markdown
### Rust

```bash
cargo add pidgr-proto
```

```rust
use pidgr_proto::pidgr::v1::{SyncRequest, inbox_service_client::InboxServiceClient};
```
```

Also add a crates.io badge to the badge row at the top (after the Go Reference badge, before the License badge):

```markdown
[![Crates.io](https://img.shields.io/crates/v/pidgr-proto.svg)](https://crates.io/crates/pidgr-proto)
```

**Step 2: Update CLAUDE.md Package Distribution table**

Find the Package Distribution table and update the Rust row from:

```
| Rust | Git dependency | `pidgr-proto = { git = "https://github.com/pidgr/pidgr-proto", tag = "v0.1.0", subdirectory = "gen/rust" }` |
```

To:

```
| Rust | crates.io (public) | `cargo add pidgr-proto` |
```

**Step 3: Commit**

```bash
git add README.md CLAUDE.md
git commit -m "docs: update Rust install instructions to crates.io"
```

---

### Task 6: Dry-run cargo publish and create PR

**Files:** None (verification only)

**Step 1: Dry-run cargo publish**

Run: `cd gen/rust && cargo publish --dry-run 2>&1 | tail -10`
Expected: Shows packaging output with no errors. May warn about `Cargo.lock` not being included — that's fine for library crates.

**Step 2: Push branch and create PR**

```bash
git push -u origin feat/rust-crate-publishing
```

Create PR with title: `feat: publish Rust crate to crates.io`

Body should include:
- Summary of changes (Cargo.toml metadata, lib.rs tonic stubs, README, CI publish step)
- Note that `CARGO_REGISTRY_TOKEN` secret must be added to the repo before the first release
- Test plan: dry-run passes, next release will publish to crates.io

---

### Post-merge: Manual setup (not automated)

After the PR is merged, before the next release:

1. **Create crates.io API token:** Go to https://crates.io/settings/tokens → create token scoped to `pidgr-proto` crate with `publish-update` permissions
2. **Add GitHub secret:** In pidgr-proto repo settings → Secrets → Actions → add `CARGO_REGISTRY_TOKEN` with the token value
3. **First publish:** The next `release/X.Y.Z` PR merge will automatically publish to crates.io alongside npm
