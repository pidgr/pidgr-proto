## Context

The auth interceptor (`internal/auth/interceptor.go`) currently:

1. **Single audience**: `expectedAudience string` validated against `claims["aud"]`. Configured from `COGNITO_APP_CLIENT_ID` env var (mobile/admin client `22oupk04kslf7c48go4h0tennp`). Desktop client `2fqc2hfp3gb7qnnf1var6ms7ah` is rejected.

2. **Org from JWT**: `custom:org_id` extracted from JWT claims (line 160). If missing and method is not org-exempt, SSO resolver is attempted, then fails with "missing org_id claim". Desktop Cognito client doesn't include `custom:org_id` in readable attributes.

3. **Users table**: `users.org_id` is NOT NULL FK to organizations. `users.cognito_sub` maps Cognito identity to internal user. This is already the membership table — a user row with `(cognito_sub, org_id)` proves membership.

4. **Context propagation**: `auth.WithOrgID(ctx, orgID)` and `auth.OrgID(ctx)` used by all downstream handlers.

## Goals / Non-Goals

**Goals:**
- Accept JWTs from multiple Cognito app clients (mobile, desktop, admin)
- Determine org from `x-org-id` gRPC metadata (or `X-Org-Id` HTTP header) instead of JWT claim
- Validate that the authenticated user is a member of the requested org via DB lookup
- Maintain backward compatibility during transition (accept both JWT claim and header)
- Keep the same `auth.OrgID(ctx)` interface for all downstream handlers

**Non-Goals:**
- Multi-org UI (org switcher) — that's a future feature; this just enables the backend
- Removing `custom:org_id` from Cognito — keep it writable for enrollment, just stop reading it for auth
- Changing the users table schema — current `(cognito_sub, org_id)` already serves as membership
- Modifying API key auth flow — API keys already resolve org from the key, not JWT

## Decisions

### 1. Multi-audience validation

**Change**: `expectedAudience string` → `expectedAudiences map[string]bool`

**Config**: `COGNITO_APP_CLIENT_IDS` env var (comma-separated) replaces `COGNITO_APP_CLIENT_ID`. The old single var is still read as fallback for backward compat.

**Rationale**: Simplest change. Map lookup is O(1). Comma-separated env var avoids needing structured config.

**Alternative considered**: Separate env var per client (e.g., `COGNITO_MOBILE_CLIENT_ID`, `COGNITO_DESKTOP_CLIENT_ID`). Rejected — less flexible, more config surface.

### 2. Org resolution from header

**Change**: After JWT validation, read `x-org-id` from gRPC metadata. Fall back to `custom:org_id` JWT claim during transition.

**Resolution order:**
1. `x-org-id` metadata/header (new, preferred)
2. `custom:org_id` JWT claim (legacy, transition only)
3. SSO auto-registration (existing behavior, unchanged)
4. Error: "missing org_id" (if all above fail and method is not org-exempt)

**Rationale**: Header-first with JWT fallback allows gradual client migration. Clients can adopt at their own pace.

### 3. Membership validation via existing users table

**Change**: When org comes from header (not JWT), query `users` table: `SELECT id FROM users WHERE cognito_sub = $1 AND org_id = $2 LIMIT 1`.

**When JWT claim provides org**: Skip membership check (Cognito is authoritative for claims it issues — the claim was set during enrollment).

**When header provides org**: Always check membership (untrusted input from client).

**Rationale**: No new tables needed. The `users` table with `(cognito_sub, org_id)` uniquely identifies membership. Index `idx_users_cognito_sub` already exists. Adding a composite index `(cognito_sub, org_id)` optimizes the lookup.

**Alternative considered**: Separate `org_members` junction table. Rejected — would require data migration and the users table already models membership (a user IS a member by virtue of having a row in that org).

### 4. New DB migration: composite index

Add index for the membership lookup query:
```sql
CREATE INDEX idx_users_cognito_sub_org_id ON users (cognito_sub, org_id) WHERE cognito_sub IS NOT NULL;
```

### 5. MembershipChecker interface

New interface injected into the interceptor:
```go
type MembershipChecker interface {
    CheckMembership(ctx context.Context, cognitoSub, orgID string) (bool, error)
}
```

Implemented by the userorg module (DB query). Injected via `WithMembershipChecker(checker)` option.

**Rationale**: Keeps the auth package free of DB dependencies. Same pattern as `SSOUserResolver` and `ApiKeyLookup`.

## Risks / Trade-offs

**[Risk] DB query on every authenticated request** → The membership check adds one DB query per request when org comes from header. Mitigation: The query hits an indexed lookup (`cognito_sub, org_id`), sub-millisecond. Can add in-memory caching (TTL ~60s) later if needed, but premature now.

**[Risk] Transition period complexity** → During migration, both JWT claim and header are accepted. Mitigation: Clear priority order (header wins over claim). Remove JWT claim support in a follow-up change after all clients are updated.

**[Risk] Clients must know their org_id** → Clients need the org ID to send in the header. Mitigation: The `GetUser` RPC (org-exempt) already returns the user's org. Desktop/mobile can call this on login, cache the org ID, and send it on subsequent requests. For multi-org future: a new `ListUserOrgs` RPC.

**[Risk] Header spoofing** → A client could send any org ID in the header. Mitigation: This is the entire point of the membership check — the DB query rejects unauthorized org access.

## Migration Plan

### Phase 1: API changes (this change)
1. Add `COGNITO_APP_CLIENT_IDS` env var with both client IDs
2. Update interceptor: multi-audience, header extraction, membership check
3. Deploy API — immediately unblocks desktop (JWT fallback still works for mobile)

### Phase 2: Client updates (separate changes per repo)
1. Desktop: add `x-org-id` to gRPC metadata (source from GetUser or stored value)
2. Mobile: add `x-org-id` to Connect-Web transport
3. Admin: add `x-org-id` to Connect-Web transport
4. MCP: already has org from API key, no change needed

### Phase 3: Cleanup (future)
1. Remove JWT claim fallback from interceptor
2. Update specs to reflect header-only org resolution

### Rollback
Revert the API deploy. The old single-audience interceptor still works for mobile/admin. Desktop remains blocked but that's the pre-change state.

## Open Questions

1. **Cache membership checks?** — Deferred. Start without cache, measure latency, add if needed.
2. **Rate limit the membership query?** — Unlikely needed since it's per-authenticated-request, not per-anonymous-request. Existing rate limiter covers this.
