## Purpose

Package and distribute generated Go code as a private module.

## Requirements

### Requirement: Go module definition
The system SHALL include a `gen/go/go.mod` file declaring the Go module path as `github.com/pidgr/pidgr-proto/gen/go` with the appropriate Go version.

#### Scenario: Valid Go module
- **WHEN** a downstream repo runs `go get github.com/pidgr/pidgr-proto/gen/go@v0.1.0`
- **THEN** Go SHALL resolve the module from the private Git repo at the tagged commit

#### Scenario: Module path matches generated code
- **WHEN** the `go.mod` module path is `github.com/pidgr/pidgr-proto/gen/go`
- **THEN** generated Go packages SHALL be importable as `github.com/pidgr/pidgr-proto/gen/go/pidgr/v1`

### Requirement: Generated Go code included in tagged release
The release workflow SHALL run `buf generate` and commit the generated Go stubs under `gen/go/` before creating the version tag.

#### Scenario: Go stubs present at tagged commit
- **WHEN** a version tag (e.g., `v0.1.0`) is created by the release workflow
- **THEN** the tagged commit SHALL contain up-to-date generated Go files under `gen/go/pidgr/v1/`

#### Scenario: Go stubs match proto definitions
- **WHEN** the release workflow generates Go code
- **THEN** the generated `.pb.go` and `_grpc.pb.go` files SHALL reflect the current proto definitions

### Requirement: Private module consumption via GOPRIVATE
Downstream Go repos SHALL consume the module by setting `GOPRIVATE=github.com/pidgr/*` and authenticating with a GitHub PAT via Git credential helper.

#### Scenario: Downstream go get resolves private module
- **WHEN** pidgr-api runs `go get github.com/pidgr/pidgr-proto/gen/go@v0.1.0` with `GOPRIVATE` configured
- **THEN** Go SHALL clone the private repo and resolve the module at the specified tag

#### Scenario: Unauthenticated access denied
- **WHEN** a user without org membership attempts `go get` on the module
- **THEN** Git SHALL deny access and `go get` SHALL fail with an authentication error
