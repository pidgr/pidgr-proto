## Why

The pidgr-proto repo has no CI/CD pipeline. Proto definitions compile and lint locally but there's no automated validation on push/PR, and generated code (Go, Rust, TypeScript) must be consumed manually by downstream repos (pidgr-api, pidgr-renderer, pidgr-mobile). Adding GitHub Actions for testing and automatic private package/module publishing ensures contract integrity on every change and gives downstream repos versioned, dependency-managed access to generated stubs — all private within the pidgr organization.

## What Changes

- Add a **CI workflow** that runs on every push and PR to validate proto definitions (`buf build`, `buf lint`, `buf breaking`)
- Add a **release workflow** that triggers on version tags (e.g., `v0.1.0`) to generate code and publish:
  - **Go**: private Git-based module consumption — downstream repos use `GOPRIVATE=github.com/pidgr/*` and `go get` resolves the tagged version directly from the private repo
  - **Rust**: private Git-based dependency — downstream repos reference `pidgr-proto = { git = "...", tag = "v0.1.0" }` in `Cargo.toml`, authenticated via GitHub PAT
  - **TypeScript**: private npm package `@pidgr/proto` published to GitHub Packages npm registry, scoped to the pidgr org
- Add release scripting to automate the tag → generate → commit → tag flow
- Add `.github/` directory structure with reusable workflow configuration
- Add downstream consumption documentation (`.npmrc`, `GOPRIVATE`, Cargo Git auth)

## Capabilities

### New Capabilities
- `ci-validation`: GitHub Actions workflow for proto validation on push/PR — runs `buf build`, `buf lint`, and `buf breaking --against .git#branch=main`
- `go-module-release`: Release workflow that generates Go stubs, commits to the repo, and tags the release so downstream repos can `go get github.com/pidgr/pidgr-proto@v0.1.0` via private Git authentication (`GOPRIVATE`)
- `rust-git-release`: Release workflow that generates Rust stubs and includes them in the tagged release so downstream repos can reference the crate as a private Git dependency in `Cargo.toml`
- `ts-package-release`: Automated private npm package (`@pidgr/proto`) publishing to GitHub Packages npm registry, scoped to the pidgr org, consumable via `.npmrc` authentication
- `release-orchestration`: Release workflow and scripting that coordinates code generation, generated code commit, version tagging, and TypeScript package publishing

### Modified Capabilities

_(none — no existing specs)_

## Impact

- **New files**: `.github/workflows/ci.yml`, `.github/workflows/release.yml`, release scripts, `gen/ts/package.json`
- **Dependencies**: Requires GitHub Packages enabled on the pidgr org (for npm), `GITHUB_TOKEN` permissions for package publishing
- **Downstream repo setup**:
  - **pidgr-api (Go)**: set `GOPRIVATE=github.com/pidgr/*`, configure Git credential helper with PAT
  - **pidgr-renderer (Rust)**: add Git dependency in `Cargo.toml` with tag reference, configure `net.git-fetch-with-cli` in Cargo config
  - **pidgr-mobile (TypeScript)**: add `.npmrc` pointing `@pidgr:registry` to `https://npm.pkg.github.com` with PAT auth
- **Repository settings**: Branch protection rules should require the CI workflow to pass before merging
