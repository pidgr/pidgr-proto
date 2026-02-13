## ADDED Requirements

### Requirement: Authorization comments on state-modifying RPCs
The proto files SHALL include `// Authorization:` doc comments on all RPCs that create, modify, or delete resources, specifying the minimum required `UserRole`.

#### Scenario: UpdateOrganization requires ADMIN role
- **WHEN** a developer reads the `UpdateOrganization` RPC comment in `user_org.proto`
- **THEN** it SHALL state `Authorization: Requires ADMIN role.`

#### Scenario: CreateCampaign requires MANAGER+ role
- **WHEN** a developer reads the `CreateCampaign` RPC comment in `campaign.proto`
- **THEN** it SHALL state `Authorization: Requires MANAGER+ role.`

#### Scenario: StartCampaign requires MANAGER+ role
- **WHEN** a developer reads the `StartCampaign` RPC comment in `campaign.proto`
- **THEN** it SHALL state `Authorization: Requires MANAGER+ role.`

#### Scenario: CancelCampaign requires MANAGER+ role
- **WHEN** a developer reads the `CancelCampaign` RPC comment in `campaign.proto`
- **THEN** it SHALL state `Authorization: Requires MANAGER+ role.`

#### Scenario: CreateTemplate requires MANAGER+ role
- **WHEN** a developer reads the `CreateTemplate` RPC comment in `template.proto`
- **THEN** it SHALL state `Authorization: Requires MANAGER+ role.`

#### Scenario: UpdateTemplate requires MANAGER+ role
- **WHEN** a developer reads the `UpdateTemplate` RPC comment in `template.proto`
- **THEN** it SHALL state `Authorization: Requires MANAGER+ role.`

#### Scenario: InviteUser requires ADMIN role
- **WHEN** a developer reads the `InviteUser` RPC comment in `user_org.proto`
- **THEN** it SHALL state `Authorization: Requires ADMIN role.`

### Requirement: Ownership validation comment on SubmitAction
The `SubmitAction` RPC comment SHALL document that the backend MUST validate the authenticated user owns the delivery before processing the action.

#### Scenario: SubmitAction ownership validation documented
- **WHEN** a developer reads the `SubmitAction` RPC comment in `action.proto`
- **THEN** it SHALL state that the backend must verify the authenticated user is the delivery recipient

### Requirement: Read-only RPCs document minimum authentication
Read-only RPCs (GetCampaign, ListCampaigns, GetTemplate, etc.) SHALL document that they require authentication but no specific role beyond org membership.

#### Scenario: GetCampaign documents authentication requirement
- **WHEN** a developer reads the `GetCampaign` RPC comment in `campaign.proto`
- **THEN** it SHALL state `Authorization: Authenticated user within the organization.`
