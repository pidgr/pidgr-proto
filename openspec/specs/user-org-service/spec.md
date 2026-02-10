## Purpose

Defines the UserOrgService gRPC service contract for user invitation, organization management, and organization provisioning in the Pidgr platform.
## Requirements
### Requirement: UserOrgService proto definition
The system SHALL define a UserOrgService in `pidgr/v1/user_org.proto` with RPCs: InviteUser, GetUser, ListUsers, GetOrganization, UpdateOrganization.

#### Scenario: Service definition exists
- **WHEN** the proto file is compiled
- **THEN** it SHALL produce a valid UserOrgService with all five RPCs

### Requirement: User message type
The system SHALL define a User message with fields: id (string), email (string), name (string), role (UserRole), status (UserStatus), created_at (Timestamp).

#### Scenario: Invited user
- **WHEN** a user has been invited but has not confirmed
- **THEN** the User SHALL have status USER_STATUS_INVITED

#### Scenario: Active user
- **WHEN** a user has completed OTP verification
- **THEN** the User SHALL have status USER_STATUS_ACTIVE

### Requirement: Organization message type
The system SHALL define an Organization message with fields: id (string), name (string), default_workflow (WorkflowDefinition), created_at (Timestamp).

#### Scenario: Organization with default workflow
- **WHEN** an organization is set up with the MVP workflow
- **THEN** the default_workflow field SHALL contain the full WorkflowDefinition DAG

### Requirement: InviteUser RPC
The system SHALL define InviteUser(InviteUserRequest) returning InviteUserResponse. InviteUserRequest SHALL contain: email (string), name (string), role (UserRole). InviteUserResponse SHALL contain the created User.

#### Scenario: Invite a new user
- **WHEN** an admin calls InviteUser with a valid email
- **THEN** the server SHALL create a provisional Cognito user, send an OTP email, and return a User with status INVITED

#### Scenario: Invite an already existing email
- **WHEN** an admin calls InviteUser with an email already in the organization
- **THEN** the server SHALL return an ALREADY_EXISTS error

### Requirement: GetUser RPC
The system SHALL define GetUser(GetUserRequest) returning GetUserResponse. GetUserRequest SHALL contain: user_id (string). GetUserResponse SHALL contain the User.

#### Scenario: Get existing user
- **WHEN** a client calls GetUser with a valid user_id
- **THEN** the response SHALL contain the User with current status and role

#### Scenario: Get non-existent user
- **WHEN** a client calls GetUser with an invalid user_id
- **THEN** the server SHALL return a NOT_FOUND error

### Requirement: ListUsers RPC
The system SHALL define ListUsers(ListUsersRequest) returning ListUsersResponse. ListUsersRequest SHALL contain: pagination (Pagination). ListUsersResponse SHALL contain: users (repeated User), pagination_meta (PaginationMeta).

#### Scenario: List users with pagination
- **WHEN** a client calls ListUsers with page_size=20
- **THEN** the response SHALL contain at most 20 users from the caller's organization

### Requirement: GetOrganization RPC
The system SHALL define GetOrganization(GetOrganizationRequest) returning GetOrganizationResponse. GetOrganizationRequest SHALL contain no fields (org is identified via JWT). GetOrganizationResponse SHALL contain the Organization.

#### Scenario: Get organization details
- **WHEN** an admin calls GetOrganization
- **THEN** the response SHALL contain the Organization with its default_workflow

### Requirement: UpdateOrganization RPC
The system SHALL define UpdateOrganization(UpdateOrganizationRequest) returning UpdateOrganizationResponse. UpdateOrganizationRequest SHALL contain: name (string, optional), default_workflow (WorkflowDefinition, optional). UpdateOrganizationResponse SHALL contain the updated Organization.

#### Scenario: Update organization default workflow
- **WHEN** an admin calls UpdateOrganization with a new WorkflowDefinition
- **THEN** the organization's default_workflow SHALL be updated and returned

#### Scenario: Update organization name only
- **WHEN** an admin calls UpdateOrganization with only a name and no workflow
- **THEN** the name SHALL be updated and the default_workflow SHALL remain unchanged

### Requirement: CreateOrganization RPC
The system SHALL define CreateOrganization(CreateOrganizationRequest) returning CreateOrganizationResponse. CreateOrganizationRequest SHALL contain: name (string), admin_email (string). CreateOrganizationResponse SHALL contain: organization (Organization), admin_user (User).

#### Scenario: Create a new organization
- **WHEN** a service calls CreateOrganization with name="Acme Corp" and admin_email="admin@acme.com"
- **THEN** the response SHALL contain an Organization with the given name and a User with role ADMIN and status INVITED

#### Scenario: Create organization with duplicate name
- **WHEN** a service calls CreateOrganization with a name that already exists
- **THEN** the server SHALL return an ALREADY_EXISTS error

#### Scenario: Create organization with invalid email
- **WHEN** a service calls CreateOrganization with a malformed admin_email
- **THEN** the server SHALL return an INVALID_ARGUMENT error

#### Scenario: Authentication requires API key
- **WHEN** a client calls CreateOrganization with a JWT (not an API key)
- **THEN** the server SHALL return a PERMISSION_DENIED error (this RPC is for service-to-service use only)

