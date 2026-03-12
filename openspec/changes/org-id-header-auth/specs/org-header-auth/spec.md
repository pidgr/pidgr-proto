## ADDED Requirements

### Requirement: Multi-audience JWT validation
The auth interceptor SHALL validate JWT `aud` claims against a configurable set of allowed Cognito app client IDs. The set of allowed audiences SHALL be configured via the `COGNITO_APP_CLIENT_IDS` environment variable as a comma-separated list. If `COGNITO_APP_CLIENT_IDS` is not set, the interceptor SHALL fall back to reading `COGNITO_APP_CLIENT_ID` (single value) for backward compatibility.

#### Scenario: Token from allowed desktop client is accepted
- **WHEN** a request carries a JWT with `aud` matching the desktop Cognito client ID AND the client ID is in the allowed set
- **THEN** the interceptor SHALL accept the token and proceed with authentication

#### Scenario: Token from allowed mobile client is accepted
- **WHEN** a request carries a JWT with `aud` matching the mobile/admin Cognito client ID AND the client ID is in the allowed set
- **THEN** the interceptor SHALL accept the token and proceed with authentication

#### Scenario: Token from unknown client is rejected
- **WHEN** a request carries a JWT with `aud` not present in the allowed set
- **THEN** the interceptor SHALL reject the request with an Unauthenticated error

#### Scenario: Fallback to single audience env var
- **WHEN** `COGNITO_APP_CLIENT_IDS` is not set AND `COGNITO_APP_CLIENT_ID` is set
- **THEN** the interceptor SHALL use the single client ID as the only allowed audience

#### Scenario: Multiple client IDs configured
- **WHEN** `COGNITO_APP_CLIENT_IDS` is set to `clientA,clientB,clientC`
- **THEN** the interceptor SHALL accept JWTs with `aud` matching any of `clientA`, `clientB`, or `clientC`

### Requirement: Org resolution from x-org-id metadata
The auth interceptor SHALL read the organization ID from `x-org-id` gRPC metadata (or `X-Org-Id` HTTP header) on every authenticated request. The header value SHALL take precedence over the `custom:org_id` JWT claim when both are present.

#### Scenario: Org ID provided via header
- **WHEN** an authenticated request includes `x-org-id` metadata with value `org-123`
- **THEN** the interceptor SHALL use `org-123` as the organization ID for the request context

#### Scenario: Header takes precedence over JWT claim
- **WHEN** an authenticated request includes `x-org-id` metadata with value `org-A` AND the JWT contains `custom:org_id` with value `org-B`
- **THEN** the interceptor SHALL use `org-A` (the header value) as the organization ID

#### Scenario: Fallback to JWT claim during transition
- **WHEN** an authenticated request does NOT include `x-org-id` metadata AND the JWT contains `custom:org_id` with value `org-123`
- **THEN** the interceptor SHALL use `org-123` from the JWT claim as the organization ID

#### Scenario: SSO fallback when no org available
- **WHEN** an authenticated request has no `x-org-id` metadata AND no `custom:org_id` JWT claim AND the method is not org-exempt AND an SSO resolver is configured
- **THEN** the interceptor SHALL attempt SSO auto-registration to resolve the organization

#### Scenario: Missing org ID on non-exempt method
- **WHEN** an authenticated request has no `x-org-id` metadata AND no `custom:org_id` JWT claim AND SSO resolution fails or is not configured AND the method is not org-exempt
- **THEN** the interceptor SHALL reject the request with an error indicating missing org_id

#### Scenario: Org-exempt method without org ID
- **WHEN** an authenticated request targets an org-exempt method AND no org ID is available from header or JWT
- **THEN** the interceptor SHALL proceed without setting an org ID in the context

### Requirement: Membership validation for header-provided org
When the organization ID is provided via the `x-org-id` header (untrusted client input), the interceptor SHALL validate that the authenticated user is a member of the requested organization by querying the users table. The interceptor SHALL NOT perform membership validation when the org ID comes from the JWT `custom:org_id` claim (trusted, Cognito-issued).

#### Scenario: User is a member of the requested org
- **WHEN** the org ID comes from `x-org-id` header AND the authenticated user (by Cognito sub) has a row in the users table with the matching org_id
- **THEN** the interceptor SHALL accept the request and set the org ID in the context

#### Scenario: User is NOT a member of the requested org
- **WHEN** the org ID comes from `x-org-id` header AND the authenticated user (by Cognito sub) does NOT have a row in the users table with the matching org_id
- **THEN** the interceptor SHALL reject the request with a PermissionDenied error

#### Scenario: Membership check skipped for JWT claim org
- **WHEN** the org ID comes from the `custom:org_id` JWT claim (no `x-org-id` header present)
- **THEN** the interceptor SHALL NOT query the users table for membership and SHALL accept the org ID as authoritative

#### Scenario: Membership check database error
- **WHEN** the org ID comes from `x-org-id` header AND the membership query fails due to a database error
- **THEN** the interceptor SHALL reject the request with an Internal error

### Requirement: MembershipChecker interface
The auth package SHALL define a `MembershipChecker` interface with a `CheckMembership(ctx context.Context, cognitoSub, orgID string) (bool, error)` method. The interceptor SHALL accept a `MembershipChecker` implementation via a `WithMembershipChecker` functional option. The membership checker SHALL be implemented by the userorg module using a database query against the users table.

#### Scenario: MembershipChecker injected at startup
- **WHEN** the interceptor is created with `WithMembershipChecker(checker)`
- **THEN** the interceptor SHALL use the provided checker for all header-based org membership validation

#### Scenario: MembershipChecker not configured
- **WHEN** the interceptor is created without a `MembershipChecker` AND a request provides org via `x-org-id` header
- **THEN** the interceptor SHALL reject the request with an Internal error indicating membership validation is not configured

### Requirement: Composite index for membership lookup
A database migration SHALL add a partial composite index on the users table for the membership lookup query: `CREATE INDEX idx_users_cognito_sub_org_id ON users (cognito_sub, org_id) WHERE cognito_sub IS NOT NULL`.

#### Scenario: Index supports membership query
- **WHEN** the membership checker executes `SELECT id FROM users WHERE cognito_sub = $1 AND org_id = $2 LIMIT 1`
- **THEN** the query SHALL use the composite index for sub-millisecond lookups

### Requirement: Context propagation unchanged
The interceptor SHALL continue to use `auth.WithOrgID(ctx, orgID)` and `auth.WithUserID(ctx, userID)` to propagate identity to downstream handlers, regardless of whether the org ID was resolved from header, JWT claim, or SSO. All downstream handlers SHALL continue to use `auth.OrgID(ctx)` without modification.

#### Scenario: Downstream handler reads org from context
- **WHEN** the interceptor resolves org ID from any source (header, JWT, SSO) and sets it in context
- **THEN** downstream handlers calling `auth.OrgID(ctx)` SHALL receive the resolved org ID

### Requirement: API key auth unaffected
The API key authentication flow SHALL remain unchanged. API keys already resolve the organization from the key record, not from JWT claims or headers.

#### Scenario: API key request without x-org-id header
- **WHEN** a request authenticates via API key (no JWT)
- **THEN** the interceptor SHALL resolve the org from the API key record as before, ignoring any `x-org-id` header
