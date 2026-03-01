# pidgr-proto

[![CI](https://github.com/pidgr/pidgr-proto/actions/workflows/ci.yml/badge.svg)](https://github.com/pidgr/pidgr-proto/actions/workflows/ci.yml)
[![Release](https://github.com/pidgr/pidgr-proto/actions/workflows/release.yml/badge.svg)](https://github.com/pidgr/pidgr-proto/actions/workflows/release.yml)
[![GitHub Release](https://img.shields.io/github/v/release/pidgr/pidgr-proto)](https://github.com/pidgr/pidgr-proto/releases/latest)
[![Go Reference](https://pkg.go.dev/badge/github.com/pidgr/pidgr-proto/gen/go.svg)](https://pkg.go.dev/github.com/pidgr/pidgr-proto/gen/go)
[![License](https://img.shields.io/github/license/pidgr/pidgr-proto)](LICENSE)

Shared Protocol Buffers definitions for the Pidgr platform. Single source of truth for all gRPC service contracts.

## Services

| Service | Proto File | RPCs | Consumer |
|---------|-----------|------|----------|
| CampaignService | `campaign.proto` | CreateCampaign, StartCampaign, GetCampaign, ListCampaigns | pidgr-api |
| TemplateService | `template.proto` | CreateTemplate, UpdateTemplate, GetTemplate, ListTemplates | pidgr-api |
| ActionService | `action.proto` | SubmitAction | pidgr-api, pidgr-mobile |
| InboxService | `inbox.proto` | Sync, MarkRead | pidgr-api, pidgr-mobile |
| DeviceService | `device.proto` | Register, Deactivate, ListDevices | pidgr-api, pidgr-mobile |
| UserOrgService | `user_org.proto` | InviteUser, GetUser, ListUsers, GetOrganization, UpdateOrganization | pidgr-api, pidgr-mobile |
| RenderService | `render.proto` | RenderBatch (server-streaming) | pidgr-api, pidgr-renderer |

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
| Go | `gen/go/` | protocolbuffers/go, grpc/go | pidgr-api |
| Rust | `gen/rust/` | neoeinstein-prost, neoeinstein-tonic | pidgr-renderer |
| TypeScript | `gen/ts/` | timostamm-protobuf-ts | pidgr-mobile |

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
