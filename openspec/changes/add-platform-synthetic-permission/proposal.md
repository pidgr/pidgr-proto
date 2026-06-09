## Why

The forthcoming `SyntheticDataService` in pidgr-api needs a least-privilege platform permission to gate its RPCs. Staff tooling that seeds resources and simulates campaign outcomes on arbitrary orgs requires a dedicated permission so it can be assigned in isolation, without bundling it with provisioning, abuse-response, or compliance authority. A narrow `PERMISSION_PLATFORM_SYNTHETIC` keeps the blast radius of a compromised staff credential minimal.

## What Changes

- Add `PERMISSION_PLATFORM_SYNTHETIC` (= 28) to the `Permission` enum in `proto/pidgr/v1/common.proto` — staff permission for creating synthetic (flagged) data on any org: seeding resources and simulating campaign outcomes.
- Follows the existing `PERMISSION_PLATFORM_*` naming and numbering convention. Assignable only to roles within an `ORG_TYPE_STAFF` organization, matching all neighboring platform permissions.

## Capabilities

### Modified Capabilities
- `role-permissions`: Add one platform-staff permission enum value for least-privilege gating of synthetic-data staff RPCs.

## Impact

- **Proto files**: 1 modified (`proto/pidgr/v1/common.proto`).
- **Codegen**: Regenerated Go/Rust/TypeScript stubs + docs for the new enum value.
- **Downstream repos**: pidgr-api — after the proto module bump, the `SyntheticDataService` RPCs gate on `PERMISSION_PLATFORM_SYNTHETIC`.
- **Breaking changes**: None — appending an enum value is backward compatible (`buf breaking` clean).
- **Version**: Requires a proto version bump (next minor) before consumers can adopt.
