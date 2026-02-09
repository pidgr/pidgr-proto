## ADDED Requirements

### Requirement: RenderService proto definition
The system SHALL define a RenderService in `pidgr/v1/render.proto` with RPC: RenderBatch (server-streaming). This service is internal only — it is consumed by pidgr-api (Go) and implemented by pidgr-renderer (Rust). It SHALL NOT be exposed to external clients.

#### Scenario: Service definition exists
- **WHEN** the proto file is compiled
- **THEN** it SHALL produce a valid RenderService with the RenderBatch server-streaming RPC

### Requirement: RenderBatch RPC
The system SHALL define RenderBatch(RenderBatchRequest) returning a stream of RenderResult. The Rust service SHALL process users in parallel and stream each result as it completes, allowing the Go client to begin persisting to S3 and DB without waiting for the entire batch.

#### Scenario: Render batch of 2000 users
- **WHEN** Go calls RenderBatch with 2000 UserRenderContext entries
- **THEN** the Rust service SHALL stream back 2000 RenderResult messages, each containing the rendered Message for one user

#### Scenario: Partial failure in batch
- **WHEN** rendering fails for some users (e.g., missing required variable)
- **THEN** the Rust service SHALL stream a RenderResult with an error field for each failed user and continue processing remaining users without terminating the stream

#### Scenario: Streaming enables early persistence
- **WHEN** the Rust service streams a RenderResult
- **THEN** the Go client MAY immediately persist the result to S3 and DB before the stream completes

### Requirement: RenderBatchRequest message type
The system SHALL define a RenderBatchRequest message with fields: template_id (string), version (int32), users (repeated UserRenderContext).

#### Scenario: Request with template and user batch
- **WHEN** Go sends a RenderBatchRequest
- **THEN** it SHALL specify the template_id, the pinned version, and the full list of users with their variable contexts

### Requirement: UserRenderContext message type
The system SHALL define a UserRenderContext message with fields: user_id (string), variables (map<string, string>).

#### Scenario: User with all required variables
- **WHEN** a UserRenderContext includes all variables required by the template
- **THEN** the render SHALL succeed and produce a valid Message

#### Scenario: User with missing required variable
- **WHEN** a UserRenderContext is missing a required variable
- **THEN** the RenderResult for that user SHALL contain an error describing the missing variable

### Requirement: RenderResult message type
The system SHALL define a RenderResult message with fields: user_id (string), message (Message), error (string). The message field SHALL use the canonical Message type. If error is non-empty, the message field MAY be empty.

#### Scenario: Successful render
- **WHEN** rendering succeeds for a user
- **THEN** the RenderResult SHALL contain a populated Message (content_id, body, summary, preview) and an empty error

#### Scenario: Failed render
- **WHEN** rendering fails for a user
- **THEN** the RenderResult SHALL contain a non-empty error string describing the failure
