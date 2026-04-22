# Tasks

- [ ] 1. Define `PipelineState` enum in `insights.proto` above the
      message section, alongside `ConfidenceLevel`. Values:
      `PIPELINE_STATE_UNSPECIFIED`, `PIPELINE_STATE_NEVER_RUN`,
      `PIPELINE_STATE_BELOW_THRESHOLD`, `PIPELINE_STATE_NO_CLUSTERS`,
      `PIPELINE_STATE_READY`.
- [ ] 2. Add `PipelineState pipeline_state = 3` to
      `GetGroupArchetypesResponse`. Comment that it lets the UI
      differentiate the four empty-state cases.
- [ ] 3. Add `TriggerArchetypeClusteringRequest` message with
      `string group_id = 1`. Comment that org is JWT-extracted, same
      as the surrounding RPCs.
- [ ] 4. Add `TriggerArchetypeClusteringResponse` message with
      `string workflow_id = 1`,
      `int32 remaining_this_month = 2`,
      `google.protobuf.Timestamp last_clustered_at = 3`.
- [ ] 5. Add `rpc TriggerArchetypeClustering(...) returns (...)`
      under `InsightsService` near `TriggerMLPipeline`. Comment that
      it shares the `ml_manual_limit_monthly` quota.
- [ ] 6. Run `buf build`, `buf lint`, `buf breaking --against .git#branch=main`
      — `breaking` should PASS (additions only).
- [ ] 7. Run `buf generate` — commit the regenerated Go / Rust / TS
      stubs in `gen/`.
- [ ] 8. Bump version to 0.63.0 in `gen/ts/package.json` and
      `gen/rust/Cargo.toml`. Commit on `release/0.63.0` branch per
      the repo's version-bump convention.
