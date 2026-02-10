## Purpose

Package and distribute generated TypeScript code on GitHub Packages npm registry.

## Requirements

### Requirement: TypeScript package definition
The system SHALL include a `gen/ts/package.json` declaring the package as `@pidgr/proto` with `publishConfig.registry` set to `https://npm.pkg.github.com`.

#### Scenario: Valid npm package
- **WHEN** `gen/ts/package.json` is inspected
- **THEN** it SHALL declare `name` as `@pidgr/proto`, include a `version` field, and set `publishConfig.registry` to `https://npm.pkg.github.com`

#### Scenario: Package includes all generated files
- **WHEN** `gen/ts/package.json` is inspected
- **THEN** the `files` field SHALL include all generated `.ts` files under `pidgr/v1/`

### Requirement: Private npm package published to GitHub Packages
The release workflow SHALL publish `@pidgr/proto` to the GitHub Packages npm registry as a private package scoped to the pidgr organization.

#### Scenario: Package published on release
- **WHEN** the release workflow completes for tag `v0.1.0`
- **THEN** `@pidgr/proto@0.1.0` SHALL be available on the GitHub Packages npm registry

#### Scenario: Package version matches Git tag
- **WHEN** the release workflow runs for tag `v0.2.0`
- **THEN** the published package version SHALL be `0.2.0` (tag without the `v` prefix)

#### Scenario: Package is private to the pidgr org
- **WHEN** the package is published
- **THEN** it SHALL be visible only to members of the pidgr GitHub organization

### Requirement: npm publish authentication
The release workflow SHALL authenticate to GitHub Packages using `GITHUB_TOKEN` with `packages: write` permission and `NODE_AUTH_TOKEN` environment variable.

#### Scenario: Publish with GITHUB_TOKEN
- **WHEN** the release workflow runs `npm publish`
- **THEN** it SHALL use `NODE_AUTH_TOKEN` set to `GITHUB_TOKEN` for authentication

#### Scenario: Insufficient permissions
- **WHEN** the workflow token lacks `packages: write` permission
- **THEN** `npm publish` SHALL fail with a 403 authentication error

### Requirement: Private package consumption via .npmrc
Downstream TypeScript repos SHALL consume `@pidgr/proto` by configuring `.npmrc` with `@pidgr:registry=https://npm.pkg.github.com` and authenticating with a PAT or `GITHUB_TOKEN`.

#### Scenario: Downstream npm install resolves private package
- **WHEN** pidgr-mobile runs `npm install @pidgr/proto@0.1.0` with `.npmrc` configured
- **THEN** npm SHALL download the package from GitHub Packages

#### Scenario: Unauthenticated access denied
- **WHEN** a user without org membership attempts `npm install @pidgr/proto`
- **THEN** npm SHALL return a 401 or 404 error
