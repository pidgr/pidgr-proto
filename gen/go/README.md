# pidgr-proto/gen/go

[![Go Reference](https://pkg.go.dev/badge/github.com/pidgr/pidgr-proto/gen/go.svg)](https://pkg.go.dev/github.com/pidgr/pidgr-proto/gen/go)
[![GitHub Release](https://img.shields.io/github/v/release/pidgr/pidgr-proto)](https://github.com/pidgr/pidgr-proto/releases/latest)
[![License](https://img.shields.io/github/license/pidgr/pidgr-proto)](https://github.com/pidgr/pidgr-proto/blob/main/LICENSE)

Generated Go protobuf/gRPC stubs for the [Pidgr](https://pidgr.com) platform. Includes standard gRPC and [Connect](https://connectrpc.com/) service definitions.

## Install

```bash
go get github.com/pidgr/pidgr-proto/gen/go@latest
```

## Usage

```go
import pidgrv1 "github.com/pidgr/pidgr-proto/gen/go/pidgr/v1"

// Messages
user := &pidgrv1.User{
    Email: "alice@company.com",
    Name:  "Alice",
}

// Connect client
import "github.com/pidgr/pidgr-proto/gen/go/pidgr/v1/pidgrv1connect"

client := pidgrv1connect.NewMemberServiceClient(
    http.DefaultClient,
    "https://api.pidgr.com",
)
```

## Available Services

| Service | Package |
|---------|---------|
| AccessCodeService | `pidgrv1` / `pidgrv1connect` |
| ActionService | `pidgrv1` / `pidgrv1connect` |
| ApiKeyService | `pidgrv1` / `pidgrv1connect` |
| CampaignService | `pidgrv1` / `pidgrv1connect` |
| DeviceService | `pidgrv1` / `pidgrv1connect` |
| GroupService | `pidgrv1` / `pidgrv1connect` |
| HeatmapService | `pidgrv1` / `pidgrv1connect` |
| InboxService | `pidgrv1` / `pidgrv1connect` |
| InviteLinkService | `pidgrv1` / `pidgrv1connect` |
| MemberService | `pidgrv1` / `pidgrv1connect` |
| OrganizationService | `pidgrv1` / `pidgrv1connect` |
| RenderService | `pidgrv1` / `pidgrv1connect` |
| ReplayService | `pidgrv1` / `pidgrv1connect` |
| RoleService | `pidgrv1` / `pidgrv1connect` |
| SSOService | `pidgrv1` / `pidgrv1connect` |
| TeamService | `pidgrv1` / `pidgrv1connect` |
| TemplateService | `pidgrv1` / `pidgrv1connect` |

Shared types (enums, pagination, messages) are in `pidgrv1`.

## Other Languages

| Language | Install |
|----------|---------|
| TypeScript | `npm install @pidgr/proto@latest` |
| Rust | Git dependency — see [repo](https://github.com/pidgr/pidgr-proto) |

## License

[Apache-2.0](https://github.com/pidgr/pidgr-proto/blob/main/LICENSE)
