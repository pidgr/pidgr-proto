## Why

Slices D and E of the pidgr-api `ml-archetype-wiring-and-productization`
change need two proto additions to `InsightsService`:

1. **`TriggerArchetypeClustering`** — fine-grained manual retrigger that
   reruns archetype clustering for one group without forcing a full
   `TriggerMLPipeline` (SageMaker training + deploy). Used by support
   staff investigating a specific group's archetype output and by the
   admin UI's per-group "refresh clusters" affordance.

2. **`PipelineState` enum on `GetGroupArchetypesResponse`** — lets the
   admin UI render four distinct empty-state affordances instead of
   the current ambiguous "empty archetypes" fallback:
   - `NEVER_RUN` — ML pipeline hasn't fired for this org yet
   - `BELOW_THRESHOLD` — fired but the group is below the 50-vector
     k-anon minimum, so no clusters were produced
   - `NO_CLUSTERS` — enough vectors but the clustering provider
     returned zero archetypes (overly homogeneous audience)
   - `READY` — populated archetypes, render pills

Both land together to avoid a second proto version bump one week apart.

## What Changes

- Add `TriggerArchetypeClusteringRequest` with `string group_id` and
  `TriggerArchetypeClusteringResponse` with `workflow_id`,
  `remaining_this_month`, `last_clustered_at`
- Add `InsightsService.TriggerArchetypeClustering` RPC
- Add `PipelineState` enum with the four states above
- Add `PipelineState pipeline_state = 3` to `GetGroupArchetypesResponse`

## Capabilities

### Modified Capabilities
- `insights-service`: new `TriggerArchetypeClustering` RPC + messages,
  plus the `PipelineState` enum and the new field on
  `GetGroupArchetypesResponse`

## Impact

- **Proto files**: 1 modified (`insights.proto`)
- **Codegen**: Go/Rust/TypeScript stubs regenerated for the new RPC,
  messages, enum, and field
- **Downstream repos**: pidgr-api (implements RPC + populates
  PipelineState on responses), pidgr-admin (adds per-group retrigger
  button + renders the four empty-state affordances)
- **Breaking changes**: None — new RPC is additive, new enum field is
  optional (default `PIPELINE_STATE_UNSPECIFIED`)
- **Version**: Requires proto version bump (v0.63.0)

## Quota note

`TriggerArchetypeClustering` shares the same `ml_manual_limit_monthly`
counter that bounds `TriggerMLPipeline`. One quota governs both — "you
get N manual retrains per month, period". Tier-differentiated pricing
(archetype-only triggers being cheaper than full-pipeline) is a v0.5
concern alongside billing-aware quotas. Noted in the design doc.
