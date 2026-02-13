## ADDED Requirements

### Requirement: String field max-length constraints in campaign.proto
All user-facing string fields in `campaign.proto` SHALL have `// Constraints:` doc comments specifying maximum character lengths.

#### Scenario: Campaign name has max length
- **WHEN** a developer reads the `name` field comment on `Campaign` and `CreateCampaignRequest`
- **THEN** it SHALL state `Constraints: Max length 200 characters.`

### Requirement: String field max-length constraints in template.proto
All user-facing string fields in `template.proto` SHALL have `// Constraints:` doc comments specifying maximum character lengths.

#### Scenario: Template name has max length
- **WHEN** a developer reads the `name` field comment on `Template` and `CreateTemplateRequest`
- **THEN** it SHALL state `Constraints: Max length 100 characters.`

#### Scenario: Template body has max length
- **WHEN** a developer reads the `body` field comment on `Template`, `CreateTemplateRequest`, and `UpdateTemplateRequest`
- **THEN** it SHALL state `Constraints: Max length 50000 characters.`

#### Scenario: Template variable name has max length
- **WHEN** a developer reads the `name` field comment on `TemplateVariable`
- **THEN** it SHALL state `Constraints: Max length 100 characters.`

#### Scenario: Template variable description has max length
- **WHEN** a developer reads the `description` field comment on `TemplateVariable`
- **THEN** it SHALL state `Constraints: Max length 500 characters.`

### Requirement: String field max-length constraints in common.proto
All user-facing string fields in `common.proto` SHALL have `// Constraints:` doc comments specifying maximum character lengths.

#### Scenario: Message body has max length
- **WHEN** a developer reads the `body` field comment on `Message`
- **THEN** it SHALL state `Constraints: Max length 100000 characters.`

#### Scenario: Message summary has max length
- **WHEN** a developer reads the `summary` field comment on `Message`
- **THEN** it SHALL state `Constraints: Max length 500 characters.`

#### Scenario: Message preview has max length
- **WHEN** a developer reads the `preview` field comment on `Message`
- **THEN** it SHALL state `Constraints: Max length 500 characters.`

#### Scenario: Message sender_name has max length
- **WHEN** a developer reads the `sender_name` field comment on `Message`
- **THEN** it SHALL state `Constraints: Max length 200 characters.`

#### Scenario: MessageAction label has max length
- **WHEN** a developer reads the `label` field comment on `MessageAction`
- **THEN** it SHALL state `Constraints: Max length 50 characters.`

### Requirement: String field max-length constraints in user_org.proto
All user-facing string fields in `user_org.proto` SHALL have `// Constraints:` doc comments specifying maximum character lengths.

#### Scenario: User email has max length
- **WHEN** a developer reads the `email` field comment on `User` and `InviteUserRequest`
- **THEN** it SHALL state `Constraints: Max length 254 characters (RFC 5321).`

#### Scenario: User name has max length
- **WHEN** a developer reads the `name` field comment on `User` and `InviteUserRequest`
- **THEN** it SHALL state `Constraints: Max length 200 characters.`

#### Scenario: Organization name has max length
- **WHEN** a developer reads the `name` field comment on `Organization`, `UpdateOrganizationRequest`, and `CreateOrganizationRequest`
- **THEN** it SHALL state `Constraints: Max length 200 characters.`

### Requirement: String field max-length constraints in device.proto
All user-facing string fields in `device.proto` SHALL have `// Constraints:` doc comments specifying maximum character lengths.

#### Scenario: Push token has max length
- **WHEN** a developer reads the `push_token` field comment on `RegisterRequest`
- **THEN** it SHALL state `Constraints: Max length 4096 characters.`

### Requirement: String field max-length constraints in action.proto
All user-facing string fields in `action.proto` SHALL have `// Constraints:` doc comments specifying maximum character lengths.

#### Scenario: Action payload has max size
- **WHEN** a developer reads the `payload` field comment on `SubmitActionRequest`
- **THEN** it SHALL state `Constraints: Max size 10000 bytes.`

### Requirement: Repeated field max-count constraint on user_ids
The `CreateCampaignRequest.user_ids` field SHALL have a `// Constraints:` doc comment specifying the maximum number of items.

#### Scenario: user_ids has max count
- **WHEN** a developer reads the `user_ids` field comment on `CreateCampaignRequest`
- **THEN** it SHALL state `Constraints: Max 100000 items.`

### Requirement: DeviceSummary message for push token redaction
A new `DeviceSummary` message SHALL be defined in `device.proto` that contains all `Device` fields except `push_token`. Response messages (`RegisterResponse`, `ListDevicesResponse`) SHALL use `DeviceSummary` instead of `Device`.

#### Scenario: DeviceSummary excludes push_token
- **WHEN** the `DeviceSummary` message is defined
- **THEN** it SHALL contain `device_id`, `user_id`, `platform`, `active`, `last_seen`, `created_at` but NOT `push_token`

#### Scenario: RegisterResponse uses DeviceSummary
- **WHEN** a developer reads `RegisterResponse`
- **THEN** it SHALL contain a `DeviceSummary device` field instead of `Device device`

#### Scenario: ListDevicesResponse uses DeviceSummary
- **WHEN** a developer reads `ListDevicesResponse`
- **THEN** it SHALL contain `repeated DeviceSummary devices` instead of `repeated Device devices`

#### Scenario: Device message retained for internal use
- **WHEN** a developer reads `device.proto`
- **THEN** the `Device` message SHALL still exist with `push_token` for internal/server-side use, with a comment indicating it is internal-only
