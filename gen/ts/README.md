# @pidgr/proto

[![npm](https://img.shields.io/npm/v/@pidgr/proto)](https://www.npmjs.com/package/@pidgr/proto)
[![GitHub Release](https://img.shields.io/github/v/release/pidgr/pidgr-proto)](https://github.com/pidgr/pidgr-proto/releases/latest)
[![License](https://img.shields.io/github/license/pidgr/pidgr-proto)](https://github.com/pidgr/pidgr-proto/blob/main/LICENSE)

Generated TypeScript protobuf/gRPC stubs for the [Pidgr](https://pidgr.com) platform. Built with [Protobuf-ES](https://github.com/bufbuild/protobuf-es) and [Connect](https://connectrpc.com/).

## Install

```bash
npm install @pidgr/proto
```

## Usage

```typescript
import { OrganizationService } from "@pidgr/proto/pidgr/v1/organization_pb";
import { CampaignService } from "@pidgr/proto/pidgr/v1/campaign_pb";
import { createClient } from "@connectrpc/connect";
import { createConnectTransport } from "@connectrpc/connect-web";

const transport = createConnectTransport({ baseUrl: "https://api.pidgr.com" });
const client = createClient(OrganizationService, transport);

const org = await client.getOrganization({});
```

## Available Services

| Service | Import |
|---------|--------|
| AccessCodeService | `@pidgr/proto/pidgr/v1/access_code_pb` |
| ActionService | `@pidgr/proto/pidgr/v1/action_pb` |
| ApiKeyService | `@pidgr/proto/pidgr/v1/api_key_pb` |
| CampaignService | `@pidgr/proto/pidgr/v1/campaign_pb` |
| DeviceService | `@pidgr/proto/pidgr/v1/device_pb` |
| GroupService | `@pidgr/proto/pidgr/v1/group_pb` |
| HeatmapService | `@pidgr/proto/pidgr/v1/heatmap_pb` |
| InboxService | `@pidgr/proto/pidgr/v1/inbox_pb` |
| InviteLinkService | `@pidgr/proto/pidgr/v1/invite_link_pb` |
| MemberService | `@pidgr/proto/pidgr/v1/member_pb` |
| OrganizationService | `@pidgr/proto/pidgr/v1/organization_pb` |
| RenderService | `@pidgr/proto/pidgr/v1/render_pb` |
| ReplayService | `@pidgr/proto/pidgr/v1/replay_pb` |
| RoleService | `@pidgr/proto/pidgr/v1/role_pb` |
| SSOService | `@pidgr/proto/pidgr/v1/sso_pb` |
| TeamService | `@pidgr/proto/pidgr/v1/team_pb` |
| TemplateService | `@pidgr/proto/pidgr/v1/template_pb` |
| UserService | `@pidgr/proto/pidgr/v1/user_pb` |

Shared types (enums, pagination, messages) are in `@pidgr/proto/pidgr/v1/common_pb`.

## Other Languages

| Language | Install |
|----------|---------|
| Go | `go get github.com/pidgr/pidgr-proto/gen/go@latest` |
| Rust | Git dependency — see [repo](https://github.com/pidgr/pidgr-proto) |

## License

[Apache-2.0](https://github.com/pidgr/pidgr-proto/blob/main/LICENSE)
