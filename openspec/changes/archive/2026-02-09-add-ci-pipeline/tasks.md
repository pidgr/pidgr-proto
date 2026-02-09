## 1. Repository Setup

- [x] 1.1 Create `.github/workflows/` directory
- [x] 1.2 Create `.gitattributes` with `gen/** linguist-generated=true` to hide generated code from PR diffs

## 2. CI Validation Workflow

- [x] 2.1 Create `.github/workflows/ci.yml` triggered on `push` and `pull_request` (opened, synchronize, reopened)
- [x] 2.2 Configure `bufbuild/buf-action@v1` with build, lint, and breaking enabled; push and format disabled
- [x] 2.3 Set breaking change detection against `main` branch
- [x] 2.4 Verify CI workflow YAML is valid (act or manual push test)

## 3. Go Module Setup

- [x] 3.1 Create `gen/go/go.mod` with module path `github.com/pidgr/pidgr-proto/gen/go` and appropriate Go version
- [x] 3.2 Create `gen/go/go.sum` (empty or with resolved dependencies if needed)
- [x] 3.3 Verify `go get` resolves the module path correctly against the local generated code

## 4. Rust Crate Setup

- [x] 4.1 Create `gen/rust/Cargo.toml` with `name = "pidgr-proto"`, edition, and `prost`/`tonic` dependencies
- [x] 4.2 Create `gen/rust/src/lib.rs` that re-exports generated modules (if not already produced by buf generate)
- [x] 4.3 Verify `cargo check` passes against the generated crate

## 5. TypeScript Package Setup

- [x] 5.1 Create `gen/ts/package.json` with `name: "@pidgr/proto"`, `publishConfig.registry: "https://npm.pkg.github.com"`, and `files` field
- [x] 5.2 Create `gen/ts/.npmrc` with `@pidgr:registry=https://npm.pkg.github.com`
- [x] 5.3 Verify `npm pack --dry-run` lists the expected files

## 6. Release Workflow

- [x] 6.1 Create `.github/workflows/release.yml` triggered on tags matching `v*`
- [x] 6.2 Add checkout step with `fetch-depth: 0` for full history
- [x] 6.3 Add `buf generate` step to produce fresh stubs for all three languages
- [x] 6.4 Add version extraction step (strip `v` prefix from tag → `0.1.0`)
- [x] 6.5 Add version injection step — update `gen/ts/package.json` and `gen/rust/Cargo.toml` with extracted version
- [x] 6.6 Add commit step — commit generated code with message `chore: generate code for <tag>`, skip if no changes
- [x] 6.7 Add npm publish step — `npm publish` in `gen/ts/` with `NODE_AUTH_TOKEN` set to `GITHUB_TOKEN`
- [x] 6.8 Add GitHub Release step — `gh release create <tag> --generate-notes` with consumption instructions in body
- [x] 6.9 Set workflow `permissions: contents: write, packages: write`

## 7. Validation

- [x] 7.1 Verify CI workflow runs on a test PR (buf build + lint + breaking all pass)
- [x] 7.2 Verify release workflow runs on a test tag (generates, commits, publishes npm, creates release)
- [x] 7.3 Verify downstream Go consumption: `GOPRIVATE` + `go get` resolves the module
- [x] 7.4 Verify downstream Rust consumption: Git dependency in `Cargo.toml` resolves at tag
- [x] 7.5 Verify downstream TypeScript consumption: `npm install @pidgr/proto` from GitHub Packages
