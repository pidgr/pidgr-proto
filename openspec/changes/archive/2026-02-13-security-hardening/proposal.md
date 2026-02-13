## Why

A security audit of the pidgr-proto definitions identified seven findings (one HIGH, five MEDIUM, one LOW) across authorization documentation, input validation constraints, sensitive data exposure, and SSRF risk. While proto3 does not enforce runtime constraints, proto comments serve as the canonical contract that all downstream implementations (pidgr-api, pidgr-mobile) must enforce. Missing security documentation means each consumer independently guesses at constraints, leading to inconsistent enforcement or none at all.

The most critical structural issue is that the `Device` message exposes raw FCM `push_token` values in `RegisterResponse` and `ListDevicesResponse`. Push tokens are sensitive credentials that allow sending arbitrary push notifications to a user's device. They should never leave the server boundary.

## What Changes

- Add authorization requirement comments to RPCs that modify state (`UpdateOrganization`, `CancelCampaign`, `CreateCampaign`, `SubmitAction`)
- Add maximum length doc comments to all unbounded user-facing string fields across all proto files
- Add maximum count doc comment to `CreateCampaignRequest.user_ids`
- Remove `push_token` from the `Device` response message and create a `DeviceSummary` message without the token for use in responses (breaking change)
- Add SSRF mitigation doc comments to `CallWebhookConfig.url`
- Add validation constraint doc comments to `WorkflowDefinition` (max steps, max transitions, DAG requirement)
- Add valid range doc comments to duration string fields (`DeadlineCheckConfig.delay`, `SendReminderConfig.repeat`, `SendReminderConfig.due_time`)

## Capabilities

### New Capabilities

_(none -- this modifies existing capabilities)_

### Modified Capabilities

- `authorization-docs`: Add authorization requirements to RPC comments
- `field-constraints`: Add max-length and max-count constraints to string and repeated fields
- `webhook-security`: Add SSRF prevention requirements to webhook URL comments
- `workflow-validation`: Add DAG validation constraints and duration bounds to workflow definitions

## Impact

- **Proto files**: `campaign.proto`, `common.proto`, `device.proto`, `action.proto`, `template.proto`, `user_org.proto`, `inbox.proto` all receive doc comment updates
- **Breaking change**: `device.proto` structural change -- `Device` message loses `push_token` field; new `DeviceSummary` message added; `RegisterResponse` and `ListDevicesResponse` use `DeviceSummary` instead of `Device`
- **Version bump**: Minor version bump required (breaking change for device responses)
- **Generated code**: Go, Rust, and TypeScript stubs must be regenerated
- **pidgr-api**: Device handlers must be updated for `DeviceSummary`; all handlers should add validation matching documented constraints
- **pidgr-mobile**: Device registration response handling must use `DeviceSummary` fields
