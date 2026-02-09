# pidgr-proto

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

Set `GOPRIVATE` and use `go get`:

```bash
export GOPRIVATE=github.com/pidgr/*
go get github.com/pidgr/pidgr-proto/gen/go@v0.1.0
```

```go
import pidgrv1 "github.com/pidgr/pidgr-proto/gen/go/pidgr/v1"
```

### Rust

Add a Git dependency in `Cargo.toml`:

```toml
[dependencies]
pidgr-proto = { git = "https://github.com/pidgr/pidgr-proto", tag = "v0.1.0", subdirectory = "gen/rust" }
```

Requires `net.git-fetch-with-cli = true` in `.cargo/config.toml` for private repo auth.

### TypeScript

Configure `.npmrc` for GitHub Packages:

```
@pidgr:registry=https://npm.pkg.github.com
//npm.pkg.github.com/:_authToken=${GITHUB_TOKEN}
```

Then install:

```bash
npm install @pidgr/proto@0.1.0
```

## Design Conventions

- All enums have `UNSPECIFIED = 0` as safety net
- `org_id` is never in request messages — extracted from JWT in server middleware
- Canonical `Message` type shared across render, inbox, and delivery
- `WorkflowDefinition` is a data-driven DAG with typed step configs (`oneof`)
- `MarkRead` = analytics only; `SubmitAction` = drives Temporal workflows
- Templates are append-only versioned; campaigns pin a specific version
