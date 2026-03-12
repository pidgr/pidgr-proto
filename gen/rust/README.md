# pidgr-proto

[![crates.io](https://img.shields.io/crates/v/pidgr-proto.svg)](https://crates.io/crates/pidgr-proto)
[![docs.rs](https://docs.rs/pidgr-proto/badge.svg)](https://docs.rs/pidgr-proto)
[![License](https://img.shields.io/crates/l/pidgr-proto.svg)](https://github.com/pidgr/pidgr-proto/blob/main/LICENSE)

Generated [protobuf](https://protobuf.dev/) messages and [tonic](https://github.com/hyperium/tonic) gRPC clients/servers for the [Pidgr](https://pidgr.com) platform.

## Install

```bash
cargo add pidgr-proto
```

Or add to Cargo.toml:

```toml
[dependencies]
pidgr-proto = "0.39.0"
```

## Usage

```rust
use pidgr_proto::pidgr::v1::inbox_service_client::InboxServiceClient;
use pidgr_proto::pidgr::v1::SyncRequest;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = InboxServiceClient::connect("https://api.pidgr.com").await?;

    let response = client.sync(SyncRequest {
        last_sync_token: String::new(),
    }).await?;

    println!("Synced {} messages", response.into_inner().entries.len());
    Ok(())
}
```

## Available Services

| Service | Client | Description |
|---------|--------|-------------|
| OrganizationService | organization_service_client | Org CRUD |
| MemberService | member_service_client | User invitation, lookup, roles |
| RoleService | role_service_client | Role + permission management |
| InviteLinkService | invite_link_service_client | Shareable invite links |
| CampaignService | campaign_service_client | Campaign lifecycle |
| TemplateService | template_service_client | Markdown template CRUD |
| ActionService | action_service_client | User action submission |
| InboxService | inbox_service_client | Inbox sync + read tracking |
| DeviceService | device_service_client | Push token management |
| GroupService | group_service_client | Recipient group targeting |
| TeamService | team_service_client | Organizational units |
| RenderService | render_service_client | Batch template rendering |
| AccessCodeService | access_code_service_client | Early access codes |
| ApiKeyService | api_key_service_client | Scoped API keys |
| SSOService | sso_service_client | SSO provider configuration |
| HeatmapService | heatmap_service_client | Heatmap data queries |
| ReplayService | replay_service_client | Session replay data |

All message types and enums are under `pidgr_proto::pidgr::v1`.

## Proto Source

Proto definitions live in [proto/pidgr/v1/](https://github.com/pidgr/pidgr-proto/tree/main/proto/pidgr/v1) with shared types in `common.proto`. Code is generated using [buf](https://buf.build) with the [neoeinstein-prost](https://buf.build/community/neoeinstein-prost) and [neoeinstein-tonic](https://buf.build/community/neoeinstein-tonic) plugins.

## Also Available

- **Go:** `go get github.com/pidgr/pidgr-proto/gen/go@latest`
- **TypeScript:** `npm install @pidgr/proto`

## License

[Apache-2.0](https://github.com/pidgr/pidgr-proto/blob/main/LICENSE)
