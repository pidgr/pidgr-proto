## ADDED Requirements

### Requirement: CI workflow file
The system SHALL define a GitHub Actions workflow at `.github/workflows/ci.yml` that triggers on `push` to all branches and on `pull_request` events (types: opened, synchronize, reopened).

#### Scenario: Push to any branch triggers CI
- **WHEN** a developer pushes commits to any branch
- **THEN** the CI workflow SHALL execute and validate all proto definitions

#### Scenario: Pull request triggers CI
- **WHEN** a pull request is opened or updated
- **THEN** the CI workflow SHALL execute and post results as PR status checks

### Requirement: Buf build validation
The CI workflow SHALL run `buf build` to verify all proto files compile without errors.

#### Scenario: All protos compile successfully
- **WHEN** all proto files are valid
- **THEN** the build step SHALL pass with exit code 0

#### Scenario: Proto compilation error
- **WHEN** a proto file has a syntax error or unresolved import
- **THEN** the build step SHALL fail and the error SHALL be visible in the GitHub Actions log

### Requirement: Buf lint validation
The CI workflow SHALL run `buf lint` against STANDARD rules to enforce proto style conventions.

#### Scenario: All protos pass lint
- **WHEN** all proto files follow STANDARD lint rules
- **THEN** the lint step SHALL pass with exit code 0

#### Scenario: Lint violation detected
- **WHEN** a proto file violates a STANDARD lint rule (e.g., enum value naming)
- **THEN** the lint step SHALL fail and report the specific violation with file and line number

### Requirement: Breaking change detection
The CI workflow SHALL run `buf breaking` against the main branch to detect backward-incompatible changes on pull requests.

#### Scenario: No breaking changes
- **WHEN** a PR modifies proto files without removing fields, changing field numbers, or renaming services
- **THEN** the breaking change step SHALL pass

#### Scenario: Breaking change detected
- **WHEN** a PR removes a message field or changes a field number
- **THEN** the breaking change step SHALL fail and report the breaking change with details

#### Scenario: First run with no main branch history
- **WHEN** breaking change detection runs but the main branch has no prior proto history
- **THEN** the step SHALL pass gracefully without errors

### Requirement: Use bufbuild/buf-action
The CI workflow SHALL use the official `bufbuild/buf-action@v1` unified action for all validation steps.

#### Scenario: Action executes build, lint, and breaking
- **WHEN** the CI workflow runs
- **THEN** it SHALL invoke `bufbuild/buf-action@v1` with build, lint, and breaking enabled, and BSR push disabled

#### Scenario: PR comment summary
- **WHEN** the CI workflow runs on a pull request with lint or breaking errors
- **THEN** the action SHALL post an inline summary comment on the PR
