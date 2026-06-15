## 1. IntegrationsService Proto

- [x] 1.1 Add `CreateChannelConnectLinkRequest` (org_id, user_id, channel) and `CreateChannelConnectLinkResponse` (connect_url, token, expires_at) messages to `proto/pidgr/v1/integrations_service.proto`
- [x] 1.2 Add the `CreateChannelConnectLink` RPC to `IntegrationsService`
- [x] 1.3 Add doc comments (auth model, follow-style-channels-only constraint) on the RPC + messages
- [x] 1.4 Run `buf lint` — clean
- [x] 1.5 Run `buf build` — compiles

## 2. Breaking-change check

- [x] 2.1 Run `buf breaking --against .git#branch=main` — confirm additive (no breaking changes)

## 3. Code Generation

- [x] 3.1 Run `buf generate` to regenerate Go/Rust/TypeScript stubs + docs
- [x] 3.2 Verify Go (+ Connect-Go) contain `CreateChannelConnectLink`
- [x] 3.3 Verify Rust prost + tonic contain `create_channel_connect_link`
- [x] 3.4 Verify TS `integrations_service_pb.ts` updated and `docs/index.md` updated

## 4. Release (separate step — handled by the controller)

- [ ] 4.1 Cut `release/X.Y.Z` from main, bump versions, publish (NOT part of this PR)
