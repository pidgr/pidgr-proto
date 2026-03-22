## ADDED Requirements

### Requirement: AuditService proto file
The system SHALL define a new `audit.proto` file in `proto/pidgr/v1/` with package `pidgr.v1` and Go package option matching existing conventions.

#### Scenario: Proto file compiles
- **WHEN** `buf build` is run
- **THEN** `audit.proto` compiles without errors

### Requirement: AuditEvent message
The system SHALL define an `AuditEvent` message with fields for immutable, hash-chained audit logging.

#### Scenario: Message fields
- **WHEN** AuditEvent is reviewed
- **THEN** it SHALL contain: id (string), org_id (string), actor_id (string), event_type (AuditEventType), entity_type (string), entity_id (string), metadata (map<string, string>), previous_hash (string), hash (string), created_at (google.protobuf.Timestamp)

### Requirement: AuditEventType enum
The system SHALL define an `AuditEventType` enum covering all significant platform actions.

#### Scenario: Enum values
- **WHEN** the enum is reviewed
- **THEN** it SHALL include at minimum: UNSPECIFIED, CAMPAIGN_CREATED, MESSAGE_SENT, MESSAGE_OPENED, ACK_REGISTERED, ESCALATION_EXECUTED, USER_INVITED, USER_DEACTIVATED, DATA_EXPORT_REQUESTED, DATA_DELETION_REQUESTED, ROLE_CHANGED, SSO_CONFIGURED

#### Scenario: Enum naming convention
- **WHEN** the enum is reviewed
- **THEN** values SHALL be prefixed with `AUDIT_EVENT_TYPE_` per project conventions

### Requirement: ListAuditEvents RPC
The system SHALL define `ListAuditEvents` RPC with pagination and filtering.

#### Scenario: Request with filters
- **WHEN** ListAuditEventsRequest is sent
- **THEN** it SHALL accept org_id, page_token, page_size, optional event_type filter, optional actor_id filter, optional start_time and end_time range

#### Scenario: Paginated response
- **WHEN** the response is returned
- **THEN** ListAuditEventsResponse SHALL contain `repeated AuditEvent events` and `string next_page_token`

### Requirement: ExportAuditTrail RPC
The system SHALL define `ExportAuditTrail` RPC for async export to S3.

#### Scenario: Request with format
- **WHEN** ExportAuditTrailRequest is sent with org_id, format (AuditExportFormat), and optional date range
- **THEN** ExportAuditTrailResponse SHALL contain export_url (string) and status (PrivacyRequestStatus — reused from privacy.proto)

### Requirement: AuditExportFormat enum
The system SHALL define an `AuditExportFormat` enum with values: UNSPECIFIED (0), CSV (1), JSON (2), PARQUET (3).

#### Scenario: Enum naming convention
- **WHEN** the enum is reviewed
- **THEN** values SHALL be prefixed with `AUDIT_EXPORT_FORMAT_`
