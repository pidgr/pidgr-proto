## Requirements

### Requirement: UserProfile message type
The system SHALL define a `UserProfile` message in `common.proto` with fields: `first_name` (string, field 1), `last_name` (string, field 2), `department` (string, field 3), `title` (string, field 4), `phone` (string, field 5), `location` (string, field 6), `employee_id` (string, field 7), `manager_name` (string, field 8), `start_date` (string, field 9), `custom_attributes` (map<string, string>, field 10).

#### Scenario: UserProfile with standard fields
- **WHEN** a UserProfile message is created with first_name="Maria" and department="Engineering"
- **THEN** the message SHALL contain those values in the corresponding fields

#### Scenario: UserProfile with custom attributes
- **WHEN** a UserProfile includes custom_attributes with key "cost_center" and value "CC-1234"
- **THEN** the custom_attributes map SHALL contain that entry

#### Scenario: Empty UserProfile
- **WHEN** a UserProfile message has no fields set
- **THEN** all string fields SHALL default to empty string and custom_attributes SHALL default to an empty map

### Requirement: UserProfile field constraints
All string fields on `UserProfile` SHALL have max length 200 characters. The `custom_attributes` map SHALL allow max 50 entries with key max length 100 characters and value max length 1000 characters. The `start_date` field SHALL use ISO 8601 date format (YYYY-MM-DD).

#### Scenario: Field within limits
- **WHEN** a UserProfile has first_name with 200 characters
- **THEN** the value SHALL be accepted

#### Scenario: Custom attributes within limits
- **WHEN** a UserProfile has 50 custom_attributes entries
- **THEN** the map SHALL be accepted

### Requirement: UpdateUserProfile RPC
The system SHALL define `UpdateUserProfileRequest` and `UpdateUserProfileResponse` messages in `member.proto`. `UpdateUserProfileRequest` SHALL contain: `user_id` (string, field 1) and `profile` (UserProfile, field 2). `UpdateUserProfileResponse` SHALL contain: `user` (User, field 1). The `MemberService` SHALL include an `UpdateUserProfile` RPC. Authorization: self-update (empty user_id or matching JWT sub) requires no special permission; updating another user requires `PERMISSION_MEMBERS_MANAGE`.

#### Scenario: User updates own profile
- **WHEN** an authenticated user calls UpdateUserProfile with empty user_id and profile.department="Engineering"
- **THEN** the response SHALL contain the updated User with profile.department="Engineering"

#### Scenario: Admin updates another user's profile
- **WHEN** an admin with PERMISSION_MEMBERS_MANAGE calls UpdateUserProfile with user_id="other-user" and profile.title="Senior Engineer"
- **THEN** the response SHALL contain the updated User with profile.title="Senior Engineer"

#### Scenario: Unauthorized profile update
- **WHEN** a user without PERMISSION_MEMBERS_MANAGE calls UpdateUserProfile for another user
- **THEN** the server SHALL return a PERMISSION_DENIED error
