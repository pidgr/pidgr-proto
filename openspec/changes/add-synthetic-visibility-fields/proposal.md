## Why

The platform supports synthetic (artificially injected) data — used for demos, sandbox testing, and issue reproduction. Consumers of the API (admin UI, MCP tools, downstream analytics) need a machine-readable signal to distinguish real user actions from synthetic records. Without it, synthetic deliveries silently inflate campaign stats, audit trails, and ML training inputs.

Two concerns are addressed together:

1. **Per-record visibility flag**: callers reading `AuditEvent`, `Delivery`, or `Campaign` must be able to identify synthetic records without side-channel knowledge of which orgs ran synthetic seeding.
2. **Org-level aggregates control**: aggregate campaign stats (recipient counts, ack rates, missed counts) used by the admin UI and analytics pipeline should be governable per org. Sandbox orgs typically want synthetic data included for demo fidelity; production orgs want it excluded by default. Derived intelligence (ML training, differential-privacy analytics, compliance attestation evidence) always excludes synthetic data regardless of this setting — this invariant is non-negotiable.

## What Changes

- `proto/pidgr/v1/audit.proto` — `AuditEvent`: add `bool synthetic = 8`.
- `proto/pidgr/v1/campaign.proto` — `Delivery`: add `bool synthetic = 9`.
- `proto/pidgr/v1/campaign.proto` — `Campaign`: add `bool synthetic = 20`.
- `proto/pidgr/v1/organization.proto` — `Organization`: add `optional bool include_synthetic_in_aggregates = 21`.
- `proto/pidgr/v1/organization.proto` — `UpdateOrganizationRequest`: add `optional bool include_synthetic_in_aggregates = 9`.

## Invariant: Synthetic data visibility

Synthetic data is **visible everywhere it was written**, carrying its `synthetic = true` flag so consumers can filter. The `include_synthetic_in_aggregates` org setting governs only the aggregate stats surface (counts shown on campaign cards and overview dashboards). Derived intelligence — ML model training, k-anonymized analytics, attestation evidence — **always excludes synthetic data** regardless of the org setting.

## Capabilities

### Modified Capabilities
- `synthetic-data-visibility`: Expose per-record `synthetic` flag on audit events, deliveries, and campaigns; add org-level `include_synthetic_in_aggregates` governance setting.

## Impact

- **Proto files**: 3 modified (`audit.proto`, `campaign.proto`, `organization.proto`).
- **Codegen**: Regenerated Go/Rust/TypeScript stubs + docs for the five new fields.
- **Downstream repos**: pidgr-api — set `synthetic = true` when writing synthetic records; honour `include_synthetic_in_aggregates` when computing aggregate stats; always exclude synthetic from ML/analytics pipelines.
- **Breaking changes**: None — appending fields is backward compatible (`buf breaking` clean).
- **Version**: Requires a proto version bump (next minor) before consumers can adopt.
