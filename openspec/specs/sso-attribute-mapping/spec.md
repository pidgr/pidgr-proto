## Requirements

### Requirement: SsoAttributeMapping message type
The system SHALL define a `SsoAttributeMapping` message in `organization.proto` with fields: `idp_claim` (string, field 1) and `profile_field` (string, field 2).

#### Scenario: Standard SAML claim mapping
- **WHEN** an SsoAttributeMapping has idp_claim="urn:oid:2.5.4.11" and profile_field="department"
- **THEN** the mapping SHALL instruct the API to populate UserProfile.department from that SAML attribute on login

#### Scenario: OIDC standard claim mapping
- **WHEN** an SsoAttributeMapping has idp_claim="given_name" and profile_field="first_name"
- **THEN** the mapping SHALL instruct the API to populate UserProfile.first_name from the OIDC claim

### Requirement: SsoAttributeMapping field constraints
The `idp_claim` field SHALL have max length 500 characters. The `profile_field` field SHALL have max length 100 characters and MUST match a valid UserProfile field name (first_name, last_name, department, title, phone, location, employee_id, manager_name, start_date) or a custom_attributes key prefixed with "custom:".

#### Scenario: Valid profile field
- **WHEN** an SsoAttributeMapping has profile_field="department"
- **THEN** the mapping SHALL be accepted

#### Scenario: Custom attribute mapping
- **WHEN** an SsoAttributeMapping has profile_field="custom:cost_center"
- **THEN** the mapping SHALL target UserProfile.custom_attributes with key "cost_center"

### Requirement: SSO attribute mappings on Organization
The system SHALL add a `repeated SsoAttributeMapping sso_attribute_mappings` field (field 7) to the `Organization` message.

#### Scenario: Organization with SSO mappings
- **WHEN** an Organization has 3 SsoAttributeMapping entries
- **THEN** the sso_attribute_mappings repeated field SHALL contain all 3 mappings

#### Scenario: Organization without SSO
- **WHEN** an Organization has no SSO configured
- **THEN** the sso_attribute_mappings field SHALL be an empty repeated field

### Requirement: UpdateSsoAttributeMappings RPC
The system SHALL define `UpdateSsoAttributeMappingsRequest` and `UpdateSsoAttributeMappingsResponse` messages in `organization.proto`. `UpdateSsoAttributeMappingsRequest` SHALL contain: `sso_attribute_mappings` (repeated SsoAttributeMapping, field 1). `UpdateSsoAttributeMappingsResponse` SHALL contain: `organization` (Organization, field 1). The `OrganizationService` SHALL include an `UpdateSsoAttributeMappings` RPC. Authorization: requires `PERMISSION_ORG_WRITE`.

#### Scenario: Set SSO mappings
- **WHEN** an admin calls UpdateSsoAttributeMappings with 3 mappings
- **THEN** the response SHALL contain the Organization with exactly those 3 mappings (replacing any previous ones)

#### Scenario: Clear SSO mappings
- **WHEN** an admin calls UpdateSsoAttributeMappings with an empty list
- **THEN** the response SHALL contain the Organization with no SSO mappings

#### Scenario: Unauthorized update
- **WHEN** a user without PERMISSION_ORG_WRITE calls UpdateSsoAttributeMappings
- **THEN** the server SHALL return a PERMISSION_DENIED error
