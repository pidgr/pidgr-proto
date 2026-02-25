## MODIFIED Requirements

### Requirement: User message type
The system SHALL define a User message with fields: id (string), email (string), name (string), status (UserStatus), created_at (Timestamp), role (Role), role_id (string), profile (UserProfile, field 9).

#### Scenario: Invited user
- **WHEN** a user has been invited but has not confirmed
- **THEN** the User SHALL have status USER_STATUS_INVITED

#### Scenario: Active user
- **WHEN** a user has completed OTP verification
- **THEN** the User SHALL have status USER_STATUS_ACTIVE

#### Scenario: User with profile
- **WHEN** a user has completed their profile
- **THEN** the User SHALL have a populated profile field with their attributes

#### Scenario: User without profile
- **WHEN** a user has not completed their profile
- **THEN** the User SHALL have an empty or absent profile field

### Requirement: InviteUser RPC
The system SHALL define InviteUser(InviteUserRequest) returning InviteUserResponse. InviteUserRequest SHALL contain: email (string), name (string), role_id (string), profile (UserProfile, field 5). InviteUserResponse SHALL contain the created User.

#### Scenario: Invite a new user
- **WHEN** an admin calls InviteUser with a valid email
- **THEN** the server SHALL create a provisional Cognito user, send an OTP email, and return a User with status INVITED

#### Scenario: Invite with profile data
- **WHEN** an admin calls InviteUser with profile.department="Engineering" and profile.title="Staff Engineer"
- **THEN** the response SHALL contain a User with those profile fields pre-populated

#### Scenario: Invite without profile data
- **WHEN** an admin calls InviteUser without a profile field
- **THEN** the response SHALL contain a User with an empty profile (backward compatible)

#### Scenario: Invite an already existing email
- **WHEN** an admin calls InviteUser with an email already in the organization
- **THEN** the server SHALL return an ALREADY_EXISTS error
