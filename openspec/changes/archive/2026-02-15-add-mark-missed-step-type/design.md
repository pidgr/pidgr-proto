## Context

The workflow DAG needs an explicit step type for marking unacknowledged deliveries as `MISSED`. All step types are defined in the proto `StepType` enum.

## Decisions

### 1. New enum value, no config message

**Choice**: Add `STEP_TYPE_MARK_MISSED = 5` with no associated config in the `WorkflowStep.config` oneof.

**Rationale**: The step always marks `SENT`/`DELIVERED` as `MISSED` — there are no configurable parameters. Adding a config message would be over-engineering for a fixed behavior. If future requirements add parameters (e.g., configurable source statuses), a config message can be added then.
