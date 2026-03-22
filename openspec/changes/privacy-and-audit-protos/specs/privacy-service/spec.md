## ADDED Requirements

### Requirement: PrivacyService proto file
The system SHALL define a new `privacy.proto` file in `proto/pidgr/v1/` with package `pidgr.v1` and Go package option matching existing conventions.

#### Scenario: Proto file compiles
- **WHEN** `buf build` is run
- **THEN** `privacy.proto` compiles without errors

### Requirement: ExportUserData RPC
The system SHALL define `ExportUserData` RPC that accepts a user_id and returns an export status with a result URL.

#### Scenario: Request and response messages
- **WHEN** ExportUserDataRequest is sent with a user_id string
- **THEN** ExportUserDataResponse SHALL contain status (PrivacyRequestStatus enum), result_url string, and export_id string

#### Scenario: Auth documentation
- **WHEN** the proto file is reviewed
- **THEN** the RPC SHALL have a doc comment specifying "Requires JWT. Callable by the user themselves or an org admin."

### Requirement: DeleteUserData RPC
The system SHALL define `DeleteUserData` RPC that accepts a user_id and an anonymize flag, returning deletion status.

#### Scenario: Request with anonymize flag
- **WHEN** DeleteUserDataRequest is sent with user_id and `bool anonymize = true`
- **THEN** DeleteUserDataResponse SHALL contain status, deleted_at timestamp

#### Scenario: Auth documentation
- **WHEN** the proto file is reviewed
- **THEN** the RPC SHALL have a doc comment specifying "Requires JWT. Admin only."

### Requirement: RectifyUserData RPC
The system SHALL define `RectifyUserData` RPC that accepts a user_id and a map of field corrections.

#### Scenario: Request and response messages
- **WHEN** RectifyUserDataRequest is sent with user_id and `map<string, string> corrections`
- **THEN** RectifyUserDataResponse SHALL contain `repeated string rectified_fields`

### Requirement: RestrictProcessing RPC
The system SHALL define `RestrictProcessing` RPC that sets or clears a processing restriction on a user.

#### Scenario: Request and response messages
- **WHEN** RestrictProcessingRequest is sent with user_id and `bool restricted`
- **THEN** RestrictProcessingResponse SHALL contain `bool restricted` and `google.protobuf.Timestamp restricted_at`

### Requirement: GetDataExistenceConfirmation RPC
The system SHALL define `GetDataExistenceConfirmation` RPC for LGPD confirmação de existência compliance.

#### Scenario: Request and response messages
- **WHEN** GetDataExistenceConfirmationRequest is sent with user_id
- **THEN** GetDataExistenceConfirmationResponse SHALL contain `bool exists` and `repeated string data_categories`

### Requirement: PrivacyRequestStatus enum
The system SHALL define a `PrivacyRequestStatus` enum with values: UNSPECIFIED (0), PENDING (1), PROCESSING (2), COMPLETED (3), FAILED (4).

#### Scenario: Enum follows naming convention
- **WHEN** the enum is reviewed
- **THEN** values SHALL be prefixed with `PRIVACY_REQUEST_STATUS_` per project conventions
