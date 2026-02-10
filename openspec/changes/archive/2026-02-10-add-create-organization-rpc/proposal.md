## Why

The `UserOrgService` currently has no RPC for creating organizations. Organization bootstrap is an internal-only Go method in pidgr-api, requiring direct SQL inserts for testing and onboarding. This blocks E2E testing and prevents programmatic org provisioning via the API (e.g., from an admin dashboard or onboarding automation).

## What Changes

- Add `CreateOrganization` RPC to `UserOrgService` in `user_org.proto`
- Add `CreateOrganizationRequest` and `CreateOrganizationResponse` messages
- The RPC accepts an org name and admin email, returns the created organization and admin user
- Designed for API key auth (service-to-service), not end-user JWT auth

## Capabilities

### New Capabilities

_(none — this extends an existing capability)_

### Modified Capabilities

- `user-org-service`: Adding `CreateOrganization` RPC to the service definition

## Impact

- **Proto**: `user_org.proto` gets new messages and RPC
- **Generated code**: Go, Rust, and TypeScript stubs will include the new RPC
- **pidgr-api**: Handler needs to be implemented (already has the service method, just needs the gRPC handler wired)
- **Not breaking**: additive change only (new RPC + messages)
