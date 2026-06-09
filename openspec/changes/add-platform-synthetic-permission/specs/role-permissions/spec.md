## ADDED Requirements

### Requirement: Platform synthetic-data permission
The system SHALL define a `PERMISSION_PLATFORM_SYNTHETIC` value in the `Permission` enum in `proto/pidgr/v1/common.proto` for staff operations that create synthetic (flagged) data on any org: seeding resources and simulating campaign outcomes. It SHALL be assignable only to roles within an `ORG_TYPE_STAFF` organization.

#### Scenario: Enum value and numbering
- **WHEN** the `Permission` enum is reviewed
- **THEN** it SHALL include `PERMISSION_PLATFORM_SYNTHETIC = 28`, the next free enum number, never reusing an existing number

#### Scenario: Naming convention
- **WHEN** the value is reviewed
- **THEN** it SHALL be prefixed with `PERMISSION_PLATFORM_` consistent with the other platform-staff permissions

#### Scenario: Staff-only assignability
- **WHEN** the doc comment for the value is reviewed
- **THEN** it SHALL note that it is assignable only to roles within an `ORG_TYPE_STAFF` organization

#### Scenario: Additive, non-breaking
- **WHEN** `buf breaking --against .git#branch=main` is run
- **THEN** it SHALL report no breaking changes
