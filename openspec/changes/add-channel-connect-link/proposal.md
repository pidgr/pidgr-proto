## Why

The Telegram, Slack, and LINE connect flows are currently closed. pidgr-api
already mints the exact opt-in token internally (`internal/linktoken`, merged in
api #441), but there is no RPC to expose it to clients (admin/mobile/desktop).
Without an RPC, a user has no way to obtain the deep link that binds a
third-party follow-style channel to their (org, user). This change opens those
flows by adding a client-facing RPC that wraps the existing api-side minter.

## What Changes

- Add `CreateChannelConnectLink` RPC to the existing `IntegrationsService` in
  `proto/pidgr/v1/integrations_service.proto`.
- Add `CreateChannelConnectLinkRequest` (org_id, user_id, channel) and
  `CreateChannelConnectLinkResponse` (connect_url, token, expires_at) messages.
- Extend the service auth-model doc comment to cover the new admin RPC.

## Capabilities

### Modified Capabilities
- `integrations-service`: Add the `CreateChannelConnectLink` RPC + its
  request/response messages. Follow-style channels only
  (CHANNEL_NAME_TELEGRAM, CHANNEL_NAME_SLACK, CHANNEL_NAME_LINE); other
  channels are rejected server-side with `invalid_argument`.

## Impact

- **Proto files**: 1 modified (`integrations_service.proto`). No new files.
- **Codegen**: Regenerated Go (+ Connect-Go), Rust (prost + tonic), and
  TypeScript stubs, plus `docs/index.md`.
- **Downstream repos**: pidgr-api (implements the handler, lands after the
  proto release — wraps the existing `internal/linktoken` minter), pidgr-admin
  / pidgr-mobile / pidgr-desktop (render the connect link / QR), pidgr-mcp
  (optional tool surface).
- **Breaking changes**: None — additive RPC + messages only
  (`buf breaking --against .git#branch=main` passes).
- **Version**: Requires a proto version bump (handled by the release step,
  not this change).

## Non-goals

- No api-side handler implementation here — that lands in pidgr-api after the
  proto release.
- No new channel types; the enum is unchanged.
