## Why

The desktop app authenticates via a separate Cognito app client (`2fqc2hfp3gb7qnnf1var6ms7ah`) whose tokens have a different `aud` claim than mobile/admin (`22oupk04kslf7c48go4h0tennp`). The API currently rejects desktop tokens because it validates against a single audience. Additionally, the API extracts `custom:org_id` from the JWT — this couples org identification to Cognito, blocks multi-org switching, and fails for clients whose Cognito app client doesn't include the custom attribute in readable attributes. Moving org identification to a request header unblocks the desktop app, enables future multi-org support, and decouples the API from Cognito-specific claims.

## What Changes

- **BREAKING**: API stops reading `custom:org_id` from JWT for org identification. All clients must send `X-Org-Id` gRPC metadata (or HTTP header) on authenticated requests.
- API validates JWT audience against a configurable list of allowed Cognito client IDs (mobile, desktop, admin).
- API adds per-request org membership validation: every request with `X-Org-Id` is checked against the org_members table to confirm the authenticated user belongs to that org.
- gRPC clients (desktop, mobile, admin) include `x-org-id` metadata on all authenticated requests.
- Proto service definitions add documentation noting the `x-org-id` metadata requirement for authenticated RPCs.

## Capabilities

### New Capabilities
- `org-header-auth`: Per-request org identification via `X-Org-Id` header/metadata with server-side membership validation. Replaces `custom:org_id` JWT claim. Includes multi-audience JWT validation.

### Modified Capabilities
<!-- No existing specs to modify -->

## Impact

- **pidgr-api**: Auth middleware rewrite — extract org from header instead of JWT, add membership check, accept multiple audiences. All authenticated handlers affected.
- **pidgr-proto**: No schema changes. Documentation-only update for metadata convention.
- **pidgr-desktop**: Add `x-org-id` to gRPC metadata in Rust commands. Source org ID from user profile (GetUser response) or stored value.
- **pidgr-mobile**: Add `x-org-id` to Connect-Web transport interceptor.
- **pidgr-admin**: Add `x-org-id` to Connect-Web transport interceptor.
- **pidgr-mcp**: Add `x-org-id` to gRPC metadata (from API key's associated org or explicit parameter).
- **Cognito**: No changes required. `custom:org_id` attribute stays writable (used during enrollment) but is no longer read for auth.
- **Migration**: Coordinated deploy — API must accept both old (JWT claim) and new (header) during transition, then drop JWT claim support.
