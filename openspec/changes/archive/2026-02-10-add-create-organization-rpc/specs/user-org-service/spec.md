## ADDED Requirements

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
