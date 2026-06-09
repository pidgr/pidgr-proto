## ADDED Requirements

### Requirement: AuditEvent synthetic flag
The system SHALL include a `bool synthetic = 8` field on `AuditEvent` in `proto/pidgr/v1/audit.proto`. This field SHALL be `true` when the event was synthetic (artificially injected) data rather than the record of a real user action.

#### Scenario: Field presence and number
- **WHEN** the `AuditEvent` message is reviewed
- **THEN** it SHALL include `bool synthetic = 8`, using the next free field number after the existing `metadata = 7` / `created_at = 10` range

#### Scenario: Default value
- **WHEN** a real-user-produced audit event is read
- **THEN** `synthetic` SHALL be `false` (proto3 default) — no wire bytes required for the common case

---

### Requirement: Delivery synthetic flag
The system SHALL include a `bool synthetic = 9` field on `Delivery` in `proto/pidgr/v1/campaign.proto`. This field SHALL be `true` when the delivery's outcome is synthetic (artificially injected) data.

#### Scenario: Field presence and number
- **WHEN** the `Delivery` message is reviewed
- **THEN** it SHALL include `bool synthetic = 9`, using the next free field number (fields 9–11 are unassigned in the existing message)

#### Scenario: Default value
- **WHEN** a real delivery record is read
- **THEN** `synthetic` SHALL be `false` — no wire bytes required for the common case

---

### Requirement: Campaign synthetic flag
The system SHALL include a `bool synthetic = 20` field on `Campaign` in `proto/pidgr/v1/campaign.proto`. This field SHALL be `true` when the campaign contains synthetic (artificially injected) data.

#### Scenario: Field presence and number
- **WHEN** the `Campaign` message is reviewed
- **THEN** it SHALL include `bool synthetic = 20`, immediately after `originating_archetype = 19`

#### Scenario: Default value
- **WHEN** a real campaign is read
- **THEN** `synthetic` SHALL be `false` — no wire bytes required for the common case

---

### Requirement: Organization include_synthetic_in_aggregates setting
The system SHALL include an `optional bool include_synthetic_in_aggregates = 21` field on `Organization` in `proto/pidgr/v1/organization.proto`. This field SHALL govern whether aggregate campaign stats (recipient/ack/missed counts) include synthetic data for the org.

#### Scenario: Field presence and number
- **WHEN** the `Organization` message is reviewed
- **THEN** it SHALL include `optional bool include_synthetic_in_aggregates = 21`, immediately after `last_ml_training_at = 20`

#### Scenario: Unset semantics
- **WHEN** `include_synthetic_in_aggregates` is unset
- **THEN** the server SHALL apply the org-type default: `true` for `ORG_TYPE_SANDBOX`, `false` for `ORG_TYPE_STANDARD`

#### Scenario: Intelligence exclusion invariant
- **WHEN** `include_synthetic_in_aggregates` is `true`
- **THEN** derived intelligence (ML model training, k-anonymized analytics, attestation evidence) SHALL still exclude synthetic data — the setting governs aggregate counts only

---

### Requirement: UpdateOrganizationRequest include_synthetic_in_aggregates field
The system SHALL include an `optional bool include_synthetic_in_aggregates = 9` field on `UpdateOrganizationRequest` in `proto/pidgr/v1/organization.proto`. This field SHALL allow callers to set or clear the org-level synthetic-aggregates override.

#### Scenario: Field presence and number
- **WHEN** the `UpdateOrganizationRequest` message is reviewed
- **THEN** it SHALL include `optional bool include_synthetic_in_aggregates = 9`, after `ml_manual_limit_monthly = 8`, mirroring the style of `optional bool ml_cancelled_counts = 7`

#### Scenario: Unset leaves unchanged
- **WHEN** `include_synthetic_in_aggregates` is omitted from an `UpdateOrganization` call
- **THEN** the server SHALL leave the org's existing setting unchanged

---

### Requirement: Additive, non-breaking
All five field additions SHALL be additive only.

#### Scenario: buf breaking check
- **WHEN** `buf breaking --against .git#branch=main` is run
- **THEN** it SHALL report no breaking changes
