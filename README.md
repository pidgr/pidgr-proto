# pidgr-proto

[![CI](https://github.com/pidgr/pidgr-proto/actions/workflows/ci.yml/badge.svg)](https://github.com/pidgr/pidgr-proto/actions/workflows/ci.yml)
[![CodeQL](https://github.com/pidgr/pidgr-proto/actions/workflows/codeql.yml/badge.svg)](https://github.com/pidgr/pidgr-proto/actions/workflows/codeql.yml)
[![Release](https://github.com/pidgr/pidgr-proto/actions/workflows/release.yml/badge.svg)](https://github.com/pidgr/pidgr-proto/actions/workflows/release.yml)
[![GitHub Release](https://img.shields.io/github/v/release/pidgr/pidgr-proto)](https://github.com/pidgr/pidgr-proto/releases/latest)
[![Go Reference](https://pkg.go.dev/badge/github.com/pidgr/pidgr-proto/gen/go.svg)](https://pkg.go.dev/github.com/pidgr/pidgr-proto/gen/go)
[![License](https://img.shields.io/github/license/pidgr/pidgr-proto)](LICENSE)

Shared Protocol Buffers definitions for the Pidgr platform. Single source of truth for all gRPC service contracts.

## Services

| Service | Proto File | Description |
|---------|-----------|-------------|
| OrganizationService | `organization.proto` | Org CRUD, industry/size enums |
| MemberService | `member.proto` | User invitation, lookup, role changes, bulk invite |
| RoleService | `role.proto` | Role listing + permission management |
| InviteLinkService | `invite_link.proto` | Shareable invite links (create, list, revoke, redeem) |
| CampaignService | `campaign.proto` | Campaign lifecycle management |
| TemplateService | `template.proto` | Markdown template CRUD + versioning |
| ActionService | `action.proto` | Generic user action submission |
| InboxService | `inbox.proto` | Mobile inbox sync + read tracking |
| DeviceService | `device.proto` | Push token management |
| GroupService | `group.proto` | Recipient groups for audience targeting |
| TeamService | `team.proto` | Organizational units (departments, divisions) |
| RenderService | `render.proto` | Internal batch template rendering (server-streaming) |
| AccessCodeService | `access_code.proto` | Early access code management |
| ApiKeyService | `api_key.proto` | Scoped API key management |
| SSOService | `sso.proto` | SSO identity provider configuration |

All proto files live under `proto/pidgr/v1/` with shared types in `common.proto`.

## Build

Requires [Buf CLI](https://buf.build/docs/installation).

```bash
buf build                                  # Compile all proto files
buf lint                                   # Lint against STANDARD rules
buf breaking --against .git#branch=main    # Check for breaking changes
buf generate                               # Generate Go, Rust, and TypeScript stubs
```

## Code Generation

| Language | Output | Plugins | Consumer |
|----------|--------|---------|----------|
| Go | `gen/go/` | protocolbuffers/go, grpc/go, connectrpc/go | pidgr-api, pidgr-mcp |
| Rust | `gen/rust/` | neoeinstein-prost, neoeinstein-tonic | pidgr-renderer |
| TypeScript | `gen/ts/` | bufbuild/es | pidgr-mobile, pidgr-admin |

## Consuming Generated Code

### Go

```bash
go get github.com/pidgr/pidgr-proto/gen/go@latest
```

```go
import pidgrv1 "github.com/pidgr/pidgr-proto/gen/go/pidgr/v1"
```

### Rust

Add a Git dependency in `Cargo.toml`:

```toml
[dependencies]
pidgr-proto = { git = "https://github.com/pidgr/pidgr-proto", tag = "v0.20.0", subdirectory = "gen/rust" }
```

### TypeScript

```bash
npm install @pidgr/proto@latest
```

## Design Conventions

- All enums have `UNSPECIFIED = 0` as safety net
- `org_id` is never in request messages — extracted from JWT in server middleware
- Canonical `Message` type shared across render, inbox, and delivery
- `WorkflowDefinition` is a data-driven DAG with typed step configs (`oneof`)
- `MarkRead` = analytics only; `SubmitAction` = drives workflow orchestration
- Templates are append-only versioned; campaigns pin a specific version

## License

Licensed under the [Apache License, Version 2.0](LICENSE). See [LICENSE](LICENSE) for the full text.
