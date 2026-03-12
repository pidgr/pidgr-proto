## 1. Database Migration

- [ ] 1.1 Create migration adding composite partial index `idx_users_cognito_sub_org_id` on `users (cognito_sub, org_id) WHERE cognito_sub IS NOT NULL`

## 2. Auth Interceptor — Multi-Audience

- [ ] 2.1 Add `COGNITO_APP_CLIENT_IDS` env var support (comma-separated) with fallback to `COGNITO_APP_CLIENT_ID`
- [ ] 2.2 Change `expectedAudience string` to `expectedAudiences map[string]bool` in interceptor struct
- [ ] 2.3 Update `validateJWT` to check `aud` against the audience map
- [ ] 2.4 Update `wire.go` to read new env var and pass audience set to interceptor
- [ ] 2.5 Write tests for multi-audience validation (accepted, rejected, fallback to single var)

## 3. Auth Interceptor — MembershipChecker Interface

- [ ] 3.1 Define `MembershipChecker` interface in auth package with `CheckMembership(ctx, cognitoSub, orgID string) (bool, error)`
- [ ] 3.2 Add `WithMembershipChecker` functional option to interceptor constructor
- [ ] 3.3 Implement `MembershipChecker` in userorg module using `SELECT id FROM users WHERE cognito_sub = $1 AND org_id = $2 LIMIT 1`
- [ ] 3.4 Wire `MembershipChecker` implementation into interceptor via `wire.go`
- [ ] 3.5 Write tests for membership checker (member found, not found, DB error)

## 4. Auth Interceptor — Header Org Resolution

- [ ] 4.1 Extract `x-org-id` from gRPC metadata in interceptor (after JWT validation)
- [ ] 4.2 Implement resolution priority: header → JWT claim → SSO → error
- [ ] 4.3 Call `MembershipChecker` when org comes from header; skip when org comes from JWT claim
- [ ] 4.4 Reject with PermissionDenied when membership check fails; reject with Internal on DB error
- [ ] 4.5 Reject with Internal when header provides org but no MembershipChecker is configured
- [ ] 4.6 Verify `auth.WithOrgID(ctx, orgID)` propagation works identically for all org sources
- [ ] 4.7 Write tests for header org resolution (header wins, JWT fallback, missing org, membership pass/fail, org-exempt bypass)

## 5. Client Updates — Desktop

- [ ] 5.1 Add `x-org-id` metadata to all authenticated gRPC requests in Rust commands
- [ ] 5.2 Source org ID from stored user profile (GetUser response) or cached value
- [ ] 5.3 Write tests for org ID metadata injection

## 6. Client Updates — Mobile

- [ ] 6.1 Add `x-org-id` header to Connect-Web transport interceptor
- [ ] 6.2 Source org ID from user profile store
- [ ] 6.3 Write tests for header injection

## 7. Client Updates — Admin

- [ ] 7.1 Add `x-org-id` header to Connect-Web transport interceptor
- [ ] 7.2 Source org ID from auth/user context
- [ ] 7.3 Write tests for header injection

## 8. Client Updates — MCP

- [ ] 8.1 Add `x-org-id` metadata to gRPC requests (from API key's associated org or explicit parameter)
- [ ] 8.2 Write tests for metadata injection

## 9. Integration Verification

- [ ] 9.1 Verify desktop app authenticates and syncs inbox end-to-end with multi-audience + header org
- [ ] 9.2 Verify mobile app continues to work with both JWT claim fallback and header
- [ ] 9.3 Verify admin app continues to work with both JWT claim fallback and header
- [ ] 9.4 Verify API key auth is unaffected by changes
