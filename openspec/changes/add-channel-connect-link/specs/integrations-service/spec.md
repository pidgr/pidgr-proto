## ADDED Requirements

### Requirement: CreateChannelConnectLink RPC
The system SHALL define a `CreateChannelConnectLink` RPC on the existing
`IntegrationsService` in `proto/pidgr/v1/integrations_service.proto` that mints
a short-lived, HMAC-signed opt-in link binding a third-party follow-style
channel to a (org, user). It wraps the pidgr-api `internal/linktoken` minter.

#### Scenario: Proto compiles
- **WHEN** `buf build` is run
- **THEN** `integrations_service.proto` compiles without errors

#### Scenario: Additive change only
- **WHEN** `buf breaking --against .git#branch=main` is run
- **THEN** no breaking changes are reported

### Requirement: CreateChannelConnectLinkRequest message
The system SHALL define a `CreateChannelConnectLinkRequest` message scoped to
an org and a user for a single channel.

#### Scenario: Message fields
- **WHEN** `CreateChannelConnectLinkRequest` is reviewed
- **THEN** it SHALL contain: org_id (string), user_id (string — internal user
  UUID resolved via UserResolver on the server), channel (ChannelName)

#### Scenario: Follow-style channels only
- **WHEN** the request's `channel` is not one of CHANNEL_NAME_TELEGRAM,
  CHANNEL_NAME_SLACK, or CHANNEL_NAME_LINE
- **THEN** the documented server behavior SHALL reject it with `invalid_argument`

### Requirement: CreateChannelConnectLinkResponse message
The system SHALL define a `CreateChannelConnectLinkResponse` message carrying
the renderable deep link, the raw token, and the token expiry.

#### Scenario: Message fields
- **WHEN** `CreateChannelConnectLinkResponse` is reviewed
- **THEN** it SHALL contain: connect_url (string — deep link the client
  renders), token (string — raw base64url opt-in token for QR/copy),
  expires_at (google.protobuf.Timestamp)

### Requirement: Auth model
The system SHALL document `CreateChannelConnectLink` as a Cognito-JWT,
admin-only, org-scoped RPC, consistent with the other reachability/config RPCs
on `IntegrationsService`; cross-org access is denied with `permission_denied`.

#### Scenario: Auth documented
- **WHEN** the `IntegrationsService` doc comment is reviewed
- **THEN** `CreateChannelConnectLink` SHALL be listed under the Cognito-JWT
  admin-only, org-scoped group
