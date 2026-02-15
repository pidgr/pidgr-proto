## Why

The `DEADLINE_CHECK` workflow step sleeps for its configured delay and transitions, but no step type exists to mark unacknowledged deliveries as `MISSED`. Users see messages stuck in `SENT` status after the campaign completes. An explicit step type is needed so the API can implement the behavior.

## What Changes

- **New `STEP_TYPE_MARK_MISSED = 5` enum value**: Added to `StepType` in `common.proto`. No config message needed — the step always marks `SENT`/`DELIVERED` deliveries as `MISSED`.

## Impact

- `proto/pidgr/v1/common.proto`: One new enum value
- Generated code updated for Go, TypeScript, and Rust
- Requires a new pidgr-proto release for pidgr-api to consume
