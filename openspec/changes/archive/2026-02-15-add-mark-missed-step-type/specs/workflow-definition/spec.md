## ADDED Requirements

### Requirement: MARK_MISSED step type
The proto SHALL define a `STEP_TYPE_MARK_MISSED = 5` enum value in the `StepType` enum. This step type requires no configuration in the `WorkflowStep.config` oneof.

#### Scenario: Codegen includes new value
- **WHEN** `buf generate` is run
- **THEN** Go, TypeScript, and Rust generated code SHALL include the `MARK_MISSED` variant
