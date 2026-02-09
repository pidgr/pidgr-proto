## Context

The pidgr-proto repo contains shared Protocol Buffers definitions consumed by three downstream repos: pidgr-api (Go), pidgr-renderer (Rust), and pidgr-mobile (TypeScript/React Native). Currently there is no CI validation and no automated distribution of generated code. All repos are private within the pidgr GitHub organization.

Buf provides an official unified GitHub Action (`bufbuild/buf-action`) that consolidates build, lint, format, and breaking change detection into a single step with PR comment integration.

For distribution, each language ecosystem has different private package patterns:
- **Go**: private modules via Git tags + `GOPRIVATE`
- **Rust**: private crates via Git dependencies in `Cargo.toml`
- **TypeScript**: private npm packages via GitHub Packages npm registry

## Goals / Non-Goals

**Goals:**
- Validate proto definitions automatically on every push and PR
- Detect breaking changes against the main branch before merge
- Automate code generation and version tagging on release
- Publish TypeScript stubs as a private npm package to GitHub Packages
- Ensure Go and Rust stubs are accessible via tagged releases
- Keep all packages private within the pidgr organization

**Non-Goals:**
- Publishing to the Buf Schema Registry (BSR) — not needed for private org use
- Setting up downstream repo CI/CD — each repo handles its own pipeline
- Automating semantic versioning — tags are manually chosen for now
- Publishing Go or Rust to dedicated package registries — Git-based consumption is standard for private repos

## Decisions

### 1. Use `bufbuild/buf-action@v1` for CI validation

Use the official unified Buf GitHub Action instead of individual `buf-setup` + manual commands.

**Rationale:** The unified action provides build, lint, format, and breaking change detection in a single step with automatic PR comment summaries. Less configuration, built-in best practices, and inline annotations on PRs.

**Alternative considered:** Manual `buf-setup-action` + individual `buf lint` / `buf breaking` steps. More control but significantly more YAML boilerplate and no PR comment integration.

**Configuration:** Disable BSR push (we don't use it) and format check (not configured yet). Enable build, lint, and breaking.

### 2. Two-workflow architecture: CI + Release

Separate workflows for validation (CI) and publishing (Release).

**Rationale:** CI runs on every push/PR and must be fast (lint + build + breaking only). Release runs only on version tags and performs the heavier work of code generation and package publishing. Separating them keeps CI fast and release logic isolated.

**Alternative considered:** Single workflow with conditional jobs. More complex YAML, harder to debug, and mixes concerns.

### 3. Release trigger: manual version tags

The release workflow triggers on `v*` tags pushed to the repo (e.g., `v0.1.0`). The developer creates the tag manually or via `gh release create`.

**Rationale:** Keeps versioning intentional and human-controlled. Automated semver detection adds complexity and is unnecessary for a proto definitions repo where changes are deliberate.

**Alternative considered:** Automated versioning via conventional commits. Overkill for a proto repo with infrequent, deliberate changes.

### 4. Generated code committed to repo on release

The release workflow runs `buf generate`, commits the generated code to the repo, and creates/updates the Git tag to include generated stubs.

**Rationale:** Go modules resolve via Git — the generated Go code must be in the repo at the tagged commit. Rust Git dependencies also clone the repo. Committing generated code ensures all three languages can consume from the same tagged commit.

**Alternative considered:** Generated code as release artifacts (zip uploads). Go modules can't consume release artifacts, and Rust Git dependencies need files in the repo tree. Only TypeScript (npm publish) doesn't need in-repo files, but consistency is simpler.

**Trade-off:** Generated code in the repo increases repo size. Mitigated by `.gitattributes` marking `gen/` as generated (hides from PR diffs) and the fact that proto repos are small.

### 5. TypeScript: private npm package on GitHub Packages

Publish `@pidgr/proto` to the GitHub Packages npm registry. Scope it to the pidgr org. Visibility: private (org members only).

**Rationale:** npm is the native package manager for React Native / TypeScript. GitHub Packages npm registry provides private scoped packages with `GITHUB_TOKEN` auth — no external registry needed. pidgr-mobile adds a `.npmrc` with `@pidgr:registry=https://npm.pkg.github.com` and authenticates with a PAT or `GITHUB_TOKEN`.

**Alternative considered:** Publishing to npmjs.com as a private package. Requires a paid npm org plan and manages auth separately from GitHub. GitHub Packages is free for private repos within the org.

### 6. Go: Git-based module consumption via `GOPRIVATE`

No registry publication for Go. Downstream repos set `GOPRIVATE=github.com/pidgr/*` and consume via `go get github.com/pidgr/pidgr-proto@v0.1.0`.

**Rationale:** This is the standard pattern for private Go modules. Go modules resolve directly from Git tags — no registry needed. The `go.mod` and `go.sum` in the repo (under `gen/go/`) make it a valid Go module.

**Requirement:** The repo needs a `gen/go/go.mod` file declaring the module path `github.com/pidgr/pidgr-proto/gen/go`.

### 7. Rust: Git dependency in Cargo.toml

No registry publication for Rust. Downstream repos reference the crate as a Git dependency: `pidgr-proto = { git = "https://github.com/pidgr/pidgr-proto", tag = "v0.1.0", subdirectory = "gen/rust" }`.

**Rationale:** Cargo supports Git dependencies natively. There is no GitHub Packages Cargo registry. Private Cargo registries (Cloudsmith, Shipyard) add external dependencies and cost. Git-based consumption is the standard for private Rust crates.

**Requirement:** The repo needs a `gen/rust/Cargo.toml` declaring the crate with proper metadata.

### 8. `package.json` for TypeScript package metadata

Add a `gen/ts/package.json` with:
- `name`: `@pidgr/proto`
- `version`: injected from the Git tag during release
- `publishConfig.registry`: `https://npm.pkg.github.com`
- `files`: include all generated `.ts` files

**Rationale:** npm publish requires a `package.json`. The version is injected dynamically from the Git tag to avoid manual version bumps.

## Risks / Trade-offs

**[Risk] Generated code committed to repo bloats Git history** → Mitigated by `.gitattributes` marking `gen/` as linguist-generated. Proto repos are small; generated code for 8 proto files is ~100KB total. Acceptable at this scale.

**[Risk] Tag manipulation during release (commit + re-tag) may confuse downstream** → Mitigated by using a two-step process: generate → commit → tag (not re-tagging). The release workflow creates the tag only after generated code is committed.

**[Risk] `GITHUB_TOKEN` permissions insufficient for package publishing** → Mitigated by explicitly setting `permissions: packages: write` in the release workflow. GitHub Actions `GITHUB_TOKEN` has this capability by default for the repo's org.

**[Risk] Go module path mismatch** → The Go module must be at `github.com/pidgr/pidgr-proto/gen/go` (subdirectory module). Downstream uses `go get github.com/pidgr/pidgr-proto/gen/go/pidgr/v1`. Verified by having `go.mod` in `gen/go/`.

**[Risk] Breaking change detection fails on first run (no main branch history)** → The `buf-action` handles this gracefully by skipping breaking checks when no comparison target exists.

## Open Questions

- Should we add a `buf format` check to CI? Currently not configured but could enforce consistent proto formatting.
- Should the release workflow also create a GitHub Release (with release notes) or just tag?
