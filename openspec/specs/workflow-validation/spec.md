## ADDED Requirements

### Requirement: WorkflowDefinition step count limit
The `WorkflowDefinition.steps` field SHALL have a doc comment specifying the maximum number of steps.

#### Scenario: Max steps documented
- **WHEN** a developer reads the `steps` field comment on `WorkflowDefinition`
- **THEN** it SHALL state `Constraints: Max 100 steps. Backend MUST validate the graph is a DAG (no cycles).`

### Requirement: WorkflowStep transitions count limit
The `WorkflowStep.transitions` field SHALL have a doc comment specifying the maximum number of transitions per step.

#### Scenario: Max transitions per step documented
- **WHEN** a developer reads the `transitions` field comment on `WorkflowStep`
- **THEN** it SHALL state `Constraints: Max 10 transitions per step.`

### Requirement: DAG validation requirement
The `WorkflowDefinition` message comment SHALL document that the backend MUST validate the workflow graph is a directed acyclic graph (no cycles allowed).

#### Scenario: DAG validation documented at message level
- **WHEN** a developer reads the `WorkflowDefinition` message comment
- **THEN** it SHALL state that the backend MUST validate no cycles exist before accepting the workflow

### Requirement: DeadlineCheckConfig.delay valid range
The `DeadlineCheckConfig.delay` field SHALL have a doc comment specifying the valid range for the duration string.

#### Scenario: Delay valid range documented
- **WHEN** a developer reads the `delay` field comment on `DeadlineCheckConfig`
- **THEN** it SHALL state `Constraints: Valid range 1m to 8760h (1 year).`

### Requirement: SendReminderConfig.repeat valid range
The `SendReminderConfig.repeat` field SHALL have a doc comment specifying the valid range for the ISO 8601 repeat interval.

#### Scenario: Repeat interval valid range documented
- **WHEN** a developer reads the `repeat` field comment on `SendReminderConfig`
- **THEN** it SHALL state `Constraints: Valid range PT1M to PT168H (1 week).`

### Requirement: SendReminderConfig.due_time valid range
The `SendReminderConfig.due_time` field SHALL have a doc comment specifying the valid range for the ISO 8601 duration.

#### Scenario: Due time valid range documented
- **WHEN** a developer reads the `due_time` field comment on `SendReminderConfig`
- **THEN** it SHALL state `Constraints: Valid range PT1M to PT168H (1 week).`

### Requirement: SendNotificationConfig.type constrained values
The `SendNotificationConfig.type` field SHALL have a doc comment specifying the accepted values.

#### Scenario: Notification type values documented
- **WHEN** a developer reads the `type` field comment on `SendNotificationConfig`
- **THEN** it SHALL state `Constraints: Accepted values: "push". Max length 50 characters.`

### Requirement: SendReminderConfig.type constrained values
The `SendReminderConfig.type` field SHALL have a doc comment specifying the accepted values.

#### Scenario: Reminder type values documented
- **WHEN** a developer reads the `type` field comment on `SendReminderConfig`
- **THEN** it SHALL state `Constraints: Accepted values: "push". Max length 50 characters.`
