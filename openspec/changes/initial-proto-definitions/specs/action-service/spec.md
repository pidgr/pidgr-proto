## ADDED Requirements

### Requirement: ActionService proto definition
The system SHALL define an ActionService in `pidgr/v1/action.proto` with RPC: SubmitAction.

#### Scenario: Service definition exists
- **WHEN** the proto file is compiled
- **THEN** it SHALL produce a valid ActionService with the SubmitAction RPC

### Requirement: SubmitAction RPC
The system SHALL define SubmitAction(SubmitActionRequest) returning SubmitActionResponse. SubmitActionRequest SHALL contain: delivery_id (string), action_id (string), payload (bytes, optional). SubmitActionResponse SHALL contain: success (bool).

#### Scenario: Submit ACK action
- **WHEN** a mobile client submits an action with a valid delivery_id and action_id corresponding to an ACK action
- **THEN** the server SHALL validate the JWT, send a Temporal ActionSubmitted signal to the running workflow, and return success=true

#### Scenario: Submit action for non-existent delivery
- **WHEN** a client submits an action with an invalid delivery_id
- **THEN** the server SHALL return a NOT_FOUND error

#### Scenario: Submit action for already completed delivery
- **WHEN** a client submits an action on a delivery that is already ACKNOWLEDGED or MISSED
- **THEN** the server SHALL return a FAILED_PRECONDITION error

#### Scenario: Payload field for future extensibility
- **WHEN** a client submits an ACK action
- **THEN** the payload field SHALL be empty (zero bytes)

#### Scenario: Future action types with payload
- **WHEN** future action types (POLL, CTA) are added
- **THEN** the payload field SHALL carry the action-specific data (poll answer, form response) without changing the RPC contract

### Requirement: Temporal signal contract
When SubmitAction is called, the Go backend SHALL send a Temporal signal named ActionSubmitted containing: action_id (string), payload (bytes). The workflow SHALL use action_id to determine the action type and decide how to proceed.

#### Scenario: Workflow receives ACK signal
- **WHEN** a Temporal workflow receives an ActionSubmitted signal for an ACK action
- **THEN** the workflow SHALL mark the delivery as acknowledged and transition to the appropriate next step based on the WorkflowDefinition transitions

#### Scenario: Workflow receives unknown action type
- **WHEN** a Temporal workflow receives an ActionSubmitted signal with an unrecognized action_id
- **THEN** the workflow SHALL log the event and continue without crashing
