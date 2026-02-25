## Why

Campaigns are currently immutable after creation — the admin can only start or cancel them. If a manager notices a typo in the sender name or wants to swap the template before launching, they must cancel the entire campaign and recreate it from scratch. Adding an `UpdateCampaign` RPC lets admins edit draft (CREATED) campaigns before starting them.

## What Changes

- Add `UpdateCampaign` RPC to `CampaignService` that accepts editable fields and returns the updated campaign
- Add `UpdateCampaignRequest` / `UpdateCampaignResponse` messages
- Editable fields (only when status is CREATED): `name`, `sender_name`, `title`, `template_id`, `template_version`, `workflow`
- Server rejects updates when campaign is not in CREATED status (FAILED_PRECONDITION)

## Capabilities

### New Capabilities

### Modified Capabilities
- `campaign-service`: Add UpdateCampaign RPC with request/response messages and state guard

## Impact

- `proto/pidgr/v1/campaign.proto` — new RPC, new messages
- Generated code: Go, Rust, TypeScript stubs regenerated
- Downstream: pidgr-api handler + service implementation, pidgr-admin edit UI
