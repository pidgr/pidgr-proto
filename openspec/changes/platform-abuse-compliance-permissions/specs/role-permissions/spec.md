## ADDED Requirements

### Requirement: Platform abuse-response permission
The system SHALL define a `PERMISSION_PLATFORM_ABUSE_RESPONSE` value in the `Permission` enum in `proto/pidgr/v1/common.proto` for staff abuse-response actions (suspend, revoke, quota overrides). It SHALL be assignable only to roles within an `ORG_TYPE_STAFF` organization.

#### Scenario: Enum value and numbering
- **WHEN** the `Permission` enum is reviewed
- **THEN** it SHALL include `PERMISSION_PLATFORM_ABUSE_RESPONSE = 26`, the next free enum number, never reusing an existing number

#### Scenario: Naming convention
- **WHEN** the value is reviewed
- **THEN** it SHALL be prefixed with `PERMISSION_PLATFORM_` consistent with the other platform-staff permissions

#### Scenario: Additive, non-breaking
- **WHEN** `buf breaking --against .git#branch=main` is run
- **THEN** it SHALL report no breaking changes

### Requirement: Platform compliance-write permission
The system SHALL define a `PERMISSION_PLATFORM_COMPLIANCE_WRITE` value in the `Permission` enum in `proto/pidgr/v1/common.proto` for staff subprocessor/compliance writes. It SHALL be assignable only to roles within an `ORG_TYPE_STAFF` organization.

#### Scenario: Enum value and numbering
- **WHEN** the `Permission` enum is reviewed
- **THEN** it SHALL include `PERMISSION_PLATFORM_COMPLIANCE_WRITE = 27`, the next free enum number, never reusing an existing number

#### Scenario: Naming convention
- **WHEN** the value is reviewed
- **THEN** it SHALL be prefixed with `PERMISSION_PLATFORM_` consistent with the other platform-staff permissions

#### Scenario: Additive, non-breaking
- **WHEN** `buf breaking --against .git#branch=main` is run
- **THEN** it SHALL report no breaking changes
