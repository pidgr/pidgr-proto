## Purpose

Package and distribute generated Rust code as a private Git dependency.

## Requirements

### Requirement: Rust crate definition
The system SHALL include a `gen/rust/Cargo.toml` declaring the crate name as `pidgr-proto` with version, edition, and appropriate metadata.

#### Scenario: Valid Cargo crate
- **WHEN** a downstream repo references `pidgr-proto` as a Git dependency
- **THEN** Cargo SHALL resolve the crate from the `gen/rust/` subdirectory of the private Git repo at the tagged commit

#### Scenario: Crate metadata is correct
- **WHEN** `gen/rust/Cargo.toml` is inspected
- **THEN** it SHALL declare `name = "pidgr-proto"`, a valid `edition`, and include `prost` and `tonic` as dependencies

### Requirement: Generated Rust code included in tagged release
The release workflow SHALL run `buf generate` and commit the generated Rust stubs under `gen/rust/` before creating the version tag.

#### Scenario: Rust stubs present at tagged commit
- **WHEN** a version tag (e.g., `v0.1.0`) is created by the release workflow
- **THEN** the tagged commit SHALL contain up-to-date generated Rust files under `gen/rust/`

#### Scenario: Rust stubs compile
- **WHEN** the generated Rust code is included in a downstream project
- **THEN** `cargo build` SHALL compile the crate without errors

### Requirement: Private crate consumption via Git dependency
Downstream Rust repos SHALL consume the crate by adding a Git dependency in `Cargo.toml`: `pidgr-proto = { git = "https://github.com/pidgr/pidgr-proto", tag = "v0.1.0", subdirectory = "gen/rust" }`.

#### Scenario: Downstream Cargo resolves private crate
- **WHEN** pidgr-renderer includes the Git dependency with a valid tag and has `net.git-fetch-with-cli = true` in `.cargo/config.toml`
- **THEN** Cargo SHALL clone the private repo via Git credentials and resolve the crate at the specified tag

#### Scenario: Unauthenticated access denied
- **WHEN** a user without org membership attempts to resolve the Git dependency
- **THEN** Git SHALL deny access and `cargo build` SHALL fail with a fetch error
