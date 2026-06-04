## Why

The org-wipe feature gives platform staff one action (`StaffService.ForceDeleteOrg`
in pidgr-api) that removes ALL trace of an org — its DB rows AND everything that
lives outside the DB. One of those external systems is pidgr-integrations, which
owns four org-scoped tables (`reachability_registry`, `channel_dispatches`,
`cost_cap_state`, `region_policy`) in a separate service/DB with no FK to the
`organizations` table. A DB-only wipe orphans this data.

pidgr-api needs a contract to ask pidgr-integrations to purge an org's data as
part of the wipe. This is slice A of the org-wipe work (proto contract first);
slice B implements it in pidgr-integrations and slice C wires it into pidgr-api's
`ForceDeleteOrg`.

## What Changes

- Add a new internal-only `IntegrationsInternalService` in a new
  `integrations_internal.proto` file, kept SEPARATE from the customer-proxied
  `IntegrationsService` so a destructive platform op is not mixed with the
  reachability/dispatch RPCs.
- Add `PurgeOrg(PurgeOrgRequest) returns (PurgeOrgResponse)` — internal-mTLS
  only, idempotent, returns per-table delete counts.
- Add `PurgeOrgRequest { string org_id, string reason }` and
  `PurgeOrgResponse { int64 reachabilities_deleted, int64 dispatches_deleted, int64 policies_deleted }`.

## Capabilities

### New Capabilities
- `integrations-internal-service`: `IntegrationsInternalService` proto — internal-mTLS-only
  service carrying the `PurgeOrg` RPC for the staff org-wipe, with its request/response
  messages and per-table delete-count semantics.

## Impact

- **Proto files**: 1 new file (`integrations_internal.proto`).
- **Codegen**: New Go/Rust/TypeScript stubs generated for `IntegrationsInternalService`.
- **Downstream repos**: pidgr-integrations (implements `PurgeOrg`, serves on the
  internal-mTLS listener — slice B), pidgr-api (calls `PurgeOrg` from staff
  `ForceDeleteOrg` via a new mTLS client — slice C).
- **Breaking changes**: None — adding a new service is non-breaking under PACKAGE rules.
- **Version**: Requires a proto version bump, shipped in a separate `release/` PR.
