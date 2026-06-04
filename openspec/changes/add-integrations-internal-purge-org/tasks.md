## 1. IntegrationsInternalService Proto

- [x] 1.1 Create `proto/pidgr/v1/integrations_internal.proto` with package `pidgr.v1` and the standard Go package option
- [x] 1.2 Add `PurgeOrgRequest { org_id, reason }` and `PurgeOrgResponse { reachabilities_deleted, dispatches_deleted, policies_deleted }` with per-field doc comments
- [x] 1.3 Add `IntegrationsInternalService` with the `PurgeOrg` RPC and doc comments documenting internal-mTLS-only auth + idempotency + the separation from `IntegrationsService`
- [x] 1.4 Run `buf format -w`
- [x] 1.5 Run `buf lint` and fix any issues
- [x] 1.6 Run `buf build` and verify compilation

## 2. Breaking-Change Check + Code Generation

- [x] 2.1 Run `buf breaking --against '.git#branch=main'` — confirm adding a new service is non-breaking
- [x] 2.2 Run `buf generate` to regenerate Go/Rust/TypeScript stubs
- [x] 2.3 Verify `gen/go/pidgr/v1/` contains `IntegrationsInternalService` + `PurgeOrg`
- [x] 2.4 Verify `gen/ts/` and `gen/rust/` contain the new service

## 3. Release

- [ ] 3.1 Version bump ships in a separate `release/` PR per the release rules (not part of this feature PR)
