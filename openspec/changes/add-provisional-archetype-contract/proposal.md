# Add provisional archetype contract metadata

## Why

pidgr-api is adding backend-generated provisional archetypes (openspec change `add-provisional-archetypes` in pidgr-api): low-confidence rule-based cohort summaries for sandbox organizations and opted-in standard organizations whose groups don't yet have trained ML archetypes. Clients must be able to tell provisional output apart from trained ML output so they can render an honest disclaimer, and admins need an org-level opt-in setting surfaced through the organization API.

## What Changes

- Add an `ArchetypeSource` enum (`UNSPECIFIED`, `ML`, `PROVISIONAL`) and an `Archetype.source` field (= 11) to `proto/pidgr/v1/insights.proto`.
- Add `GetGroupArchetypesResponse.confidence_level` (= 4) so clients get response-level confidence metadata; always `CONFIDENCE_LEVEL_LOW` when provisional archetypes are returned.
- Add `Organization.provisional_archetypes_enabled` (= 21) and `UpdateOrganizationRequest.provisional_archetypes_enabled` (`optional bool`, = 9) to `proto/pidgr/v1/organization.proto` for the standard-org opt-in.

## Capabilities

### Modified Capabilities
- `insights-service`: source + confidence metadata on the archetype read contract.
- `organization-service`: org-level provisional-archetypes opt-in setting.

## Impact

- **Proto files**: 2 modified (`insights.proto`, `organization.proto`).
- **Codegen**: Regenerated Go/Rust/TypeScript stubs + docs.
- **Downstream repos**: pidgr-api (provisional generation + org setting), pidgr-admin (settings toggle + disclaimer).
- **Breaking changes**: None — additive fields and a new enum (`buf breaking` clean). Clients that ignore the new fields keep existing behavior; `Archetype.source` is `UNSPECIFIED` from older servers and SHOULD be treated as ML.
- **Version**: Requires a proto version bump (next minor) before consumers can adopt.
