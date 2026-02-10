## Why

The pidgr-api specs define `CancelCampaign`, `ListDeliveries`, and `GetMessage` RPCs with full service-layer implementations, but the corresponding proto definitions don't exist in pidgr-proto. This blocks gRPC handler wiring and E2E testing of campaign cancellation, delivery inspection, and push notification deep-linking.

## What Changes

- Add `CancelCampaign` RPC to `CampaignService` in `campaign.proto` with request/response messages
- Add `ListDeliveries` RPC to `CampaignService` in `campaign.proto` with request/response messages and a `Delivery` message
- Add `GetMessage` RPC to `InboxService` in `inbox.proto` with request/response messages

## Capabilities

### New Capabilities

_(none — this extends existing capabilities)_

### Modified Capabilities

- `campaign-service`: Adding `CancelCampaign` and `ListDeliveries` RPCs to the service definition
- `inbox-service`: Adding `GetMessage` RPC to the service definition

## Impact

- **Proto**: `campaign.proto` and `inbox.proto` get new messages and RPCs
- **Generated code**: Go, Rust, and TypeScript stubs will include the new RPCs
- **pidgr-api**: Handlers can be wired once proto is updated (service methods already exist)
- **Not breaking**: additive changes only (new RPCs + messages)
