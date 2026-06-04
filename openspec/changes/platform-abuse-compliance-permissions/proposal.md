## Why

The platform-staff abuse-detection and subprocessors-compliance staff RPCs in pidgr-api currently gate on the broad `PERMISSION_PLATFORM_PROVISION` permission as a stopgap. Provisioning organizations and taking abuse-response actions (suspend, revoke, quota overrides) or writing compliance/subprocessor records are distinct, least-privilege concerns and should not share a single permission. Two dedicated platform permissions let those RPCs gate on exactly the authority they require.

## What Changes

- Add `PERMISSION_PLATFORM_ABUSE_RESPONSE` (= 26) to the `Permission` enum in `proto/pidgr/v1/common.proto` — staff abuse-response actions (suspend/revoke/quota overrides).
- Add `PERMISSION_PLATFORM_COMPLIANCE_WRITE` (= 27) to the `Permission` enum — staff subprocessor/compliance writes.
- Both follow the existing `PERMISSION_PLATFORM_*` naming + numbering convention and are assignable only to roles within an `ORG_TYPE_STAFF` organization, matching the neighboring platform permissions.

## Capabilities

### Modified Capabilities
- `role-permissions`: Add two platform-staff permission enum values for least-privilege gating of abuse-response and compliance-write staff RPCs.

## Impact

- **Proto files**: 1 modified (`proto/pidgr/v1/common.proto`).
- **Codegen**: Regenerated Go/Rust/TypeScript stubs + docs for the two new enum values.
- **Downstream repos**: pidgr-api — after the proto module bump, the abuse RPCs swap from `PERMISSION_PLATFORM_PROVISION` to `PERMISSION_PLATFORM_ABUSE_RESPONSE` and the subprocessor writes to `PERMISSION_PLATFORM_COMPLIANCE_WRITE` (a one-line gating swap each).
- **Breaking changes**: None — appending enum values is backward compatible (`buf breaking` clean).
- **Version**: Requires a proto version bump (next minor) before consumers can adopt.
