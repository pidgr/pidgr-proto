## Design Decisions

### D1: Request and response message design

**Decision:** `CreateOrganizationRequest` contains `name` (string) and `admin_email` (string). `CreateOrganizationResponse` returns both the `Organization` and the admin `User`.

**Rationale:** The caller needs both the org ID (for subsequent API calls) and the admin user ID (to track invitation status). Returning both avoids a follow-up GetUser call. The admin_email field triggers Cognito user creation server-side — the caller doesn't need to manage Cognito directly.

### D2: API key auth only

**Decision:** This RPC is restricted to API key authentication (service-to-service). JWT-authenticated users cannot create organizations.

**Rationale:** Org creation is a platform operation, not a user operation. It should only be callable from admin tooling or onboarding automation, never from the mobile app. The pidgr-api interceptor already distinguishes API key vs JWT auth via `auth.IsService(ctx)`.

### D3: Additive proto change

**Decision:** Add new messages and RPC to existing `user_org.proto`. No changes to existing messages or RPCs.

**Rationale:** This is a purely additive change — no breaking changes, no version bump required for existing consumers. New RPC is ignored by clients that don't use it.
