## ADDED Requirements

### Requirement: Provisional archetypes organization setting

`pidgr.v1.Organization` SHALL carry a `bool provisional_archetypes_enabled` field (= 21) reflecting the standard-organization opt-in for provisional archetypes. `pidgr.v1.UpdateOrganizationRequest` SHALL carry an `optional bool provisional_archetypes_enabled` field (= 9) using presence semantics: unset leaves the setting unchanged; `true`/`false` enable/disable it. The setting is only meaningful for `ORG_TYPE_STANDARD`; sandbox organizations are always eligible automatically and servers SHALL reject attempts to set it on sandbox organizations.

#### Scenario: Unset field leaves the setting unchanged
- **GIVEN** an `UpdateOrganizationRequest` that does not set `provisional_archetypes_enabled`
- **WHEN** the server applies the update
- **THEN** the organization's provisional-archetypes setting is unchanged

#### Scenario: Explicit false disables the setting
- **GIVEN** an organization with `provisional_archetypes_enabled = true`
- **WHEN** an `UpdateOrganizationRequest` sets the field to `false`
- **THEN** the organization's setting becomes `false` and subsequent `GetOrganization` responses reflect it
