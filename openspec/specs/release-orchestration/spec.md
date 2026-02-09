## ADDED Requirements

### Requirement: Release workflow file
The system SHALL define a GitHub Actions workflow at `.github/workflows/release.yml` that triggers when a Git tag matching `v*` is pushed (e.g., `v0.1.0`, `v1.0.0-rc.1`).

#### Scenario: Version tag triggers release
- **WHEN** a developer pushes a tag matching `v*` (e.g., `git tag v0.1.0 && git push --tags`)
- **THEN** the release workflow SHALL execute

#### Scenario: Non-version tag does not trigger release
- **WHEN** a developer pushes a tag not matching `v*` (e.g., `test-123`)
- **THEN** the release workflow SHALL NOT execute

### Requirement: Code generation step
The release workflow SHALL run `buf generate` to produce fresh Go, Rust, and TypeScript stubs from the current proto definitions.

#### Scenario: All three language targets generated
- **WHEN** the code generation step completes
- **THEN** generated code SHALL exist under `gen/go/`, `gen/rust/`, and `gen/ts/`

#### Scenario: Generation uses tagged proto state
- **WHEN** the release workflow checks out the repo at the tag
- **THEN** `buf generate` SHALL use the proto files at that exact commit

### Requirement: Generated code commit
The release workflow SHALL commit the freshly generated code to the repository and update the tag to point to the new commit that includes generated stubs.

#### Scenario: Generated code committed
- **WHEN** `buf generate` produces updated stubs
- **THEN** the workflow SHALL commit the changes with message `chore: generate code for <tag>`

#### Scenario: No changes to commit
- **WHEN** `buf generate` produces identical output to what is already committed
- **THEN** the workflow SHALL skip the commit step and proceed to publishing

### Requirement: Version injection into package manifests
The release workflow SHALL extract the version from the Git tag (stripping the `v` prefix) and inject it into `gen/ts/package.json` and `gen/rust/Cargo.toml` before publishing.

#### Scenario: Version extracted from tag
- **WHEN** the tag is `v0.3.1`
- **THEN** the version injected into package manifests SHALL be `0.3.1`

#### Scenario: Pre-release tag
- **WHEN** the tag is `v1.0.0-rc.1`
- **THEN** the version injected SHALL be `1.0.0-rc.1`

### Requirement: npm publish step
The release workflow SHALL run `npm publish` in the `gen/ts/` directory to publish `@pidgr/proto` to GitHub Packages after version injection.

#### Scenario: TypeScript package published
- **WHEN** the release workflow reaches the npm publish step
- **THEN** it SHALL publish `@pidgr/proto` at the tag version to `https://npm.pkg.github.com`

### Requirement: GitHub Release creation
The release workflow SHALL create a GitHub Release for the tag using `gh release create`, including auto-generated release notes.

#### Scenario: Release created with notes
- **WHEN** the release workflow completes successfully
- **THEN** a GitHub Release SHALL exist for the tag with auto-generated release notes

#### Scenario: Release links to packages
- **WHEN** the GitHub Release is created
- **THEN** the release body SHALL include instructions for consuming the generated packages in Go, Rust, and TypeScript

### Requirement: Workflow permissions
The release workflow SHALL declare explicit permissions: `contents: write` (for commits and releases), `packages: write` (for npm publish).

#### Scenario: Permissions configured
- **WHEN** the release workflow YAML is inspected
- **THEN** the `permissions` block SHALL include `contents: write` and `packages: write`
