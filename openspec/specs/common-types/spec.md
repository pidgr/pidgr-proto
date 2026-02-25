## Purpose

Define shared enums, messages, and canonical types used across all services.

## Requirements

### Requirement: CampaignStatus enum
The system SHALL define a CampaignStatus enum with values: CAMPAIGN_STATUS_UNSPECIFIED (0), CAMPAIGN_STATUS_CREATED (1), CAMPAIGN_STATUS_RUNNING (2), CAMPAIGN_STATUS_COMPLETED (3), CAMPAIGN_STATUS_FAILED (4), CAMPAIGN_STATUS_CANCELLED (5).

#### Scenario: Campaign is created
- **WHEN** a campaign is created via CreateCampaign
- **THEN** its status SHALL be CAMPAIGN_STATUS_CREATED

#### Scenario: Campaign is started
- **WHEN** a campaign is started via StartCampaign
- **THEN** its status SHALL transition to CAMPAIGN_STATUS_RUNNING

#### Scenario: Default value safety
- **WHEN** a CampaignStatus field is not explicitly set
- **THEN** its value SHALL be CAMPAIGN_STATUS_UNSPECIFIED (0), indicating a bug

### Requirement: DeliveryStatus enum
The system SHALL define a DeliveryStatus enum with values: DELIVERY_STATUS_UNSPECIFIED (0), DELIVERY_STATUS_PENDING (1), DELIVERY_STATUS_SENT (2), DELIVERY_STATUS_DELIVERED (3), DELIVERY_STATUS_ACKNOWLEDGED (4), DELIVERY_STATUS_MISSED (5), DELIVERY_STATUS_NO_DEVICE (6), DELIVERY_STATUS_FAILED (7).

#### Scenario: User has no registered device
- **WHEN** a campaign targets a user with no device token
- **THEN** the delivery record SHALL have status DELIVERY_STATUS_NO_DEVICE

#### Scenario: User acknowledges a message
- **WHEN** a user submits an ACK action
- **THEN** the delivery status SHALL transition to DELIVERY_STATUS_ACKNOWLEDGED

#### Scenario: Reminder period expires without acknowledgment
- **WHEN** the reminder period (2 weeks) expires without an action submission
- **THEN** the delivery status SHALL transition to DELIVERY_STATUS_MISSED

### Requirement: Platform enum
The system SHALL define a Platform enum with values: PLATFORM_UNSPECIFIED (0), PLATFORM_IOS (1), PLATFORM_ANDROID (2).

#### Scenario: iOS device registration
- **WHEN** a device registers with platform "ios"
- **THEN** the platform field SHALL be PLATFORM_IOS

#### Scenario: Android device registration
- **WHEN** a device registers with platform "android"
- **THEN** the platform field SHALL be PLATFORM_ANDROID

### Requirement: UserRole enum
The system SHALL define a UserRole enum with values: USER_ROLE_UNSPECIFIED (0), USER_ROLE_ADMIN (1), USER_ROLE_MANAGER (2), USER_ROLE_EMPLOYEE (3).

#### Scenario: Admin invites a user with a specific role
- **WHEN** an admin invites a user with role EMPLOYEE
- **THEN** the user record SHALL have role USER_ROLE_EMPLOYEE

### Requirement: UserStatus enum
The system SHALL define a UserStatus enum with values: USER_STATUS_UNSPECIFIED (0), USER_STATUS_INVITED (1), USER_STATUS_ACTIVE (2), USER_STATUS_DEACTIVATED (3).

#### Scenario: User is invited but has not confirmed
- **WHEN** an admin invites a user and the user has not completed OTP verification
- **THEN** the user status SHALL be USER_STATUS_INVITED

#### Scenario: User completes OTP verification
- **WHEN** an invited user confirms their identity via OTP
- **THEN** the user status SHALL transition to USER_STATUS_ACTIVE

### Requirement: ActionType enum
The system SHALL define an ActionType enum with values: ACTION_TYPE_UNSPECIFIED (0), ACTION_TYPE_ACK (1).

#### Scenario: MVP message with acknowledge action
- **WHEN** a message is created with an acknowledge button
- **THEN** the action type SHALL be ACTION_TYPE_ACK

#### Scenario: Future extensibility
- **WHEN** new action types are added in future versions (POLL, CTA, LINK)
- **THEN** they SHALL be appended to the enum without changing existing values

### Requirement: StepType enum
The system SHALL define a StepType enum with values: STEP_TYPE_UNSPECIFIED (0), STEP_TYPE_SEND_NOTIFICATION (1), STEP_TYPE_WAIT_ACTION (2), STEP_TYPE_SEND_REMINDER (3), STEP_TYPE_CALL_WEBHOOK (4).

#### Scenario: Workflow step sends a notification
- **WHEN** a workflow step has type STEP_TYPE_SEND_NOTIFICATION
- **THEN** Go SHALL map it to a Temporal activity that dispatches the initial message

#### Scenario: Workflow step waits for user action
- **WHEN** a workflow step has type STEP_TYPE_WAIT_ACTION
- **THEN** Go SHALL map it to a Temporal activity that waits for an ActionSubmitted signal with the configured timeout

### Requirement: Pagination messages
The system SHALL define Pagination (page_size int32, page_token string) and PaginationMeta (next_page_token string, total_count int32) messages for cursor-based pagination across all list RPCs.

#### Scenario: First page request
- **WHEN** a client sends a list request with page_size=20 and empty page_token
- **THEN** the server SHALL return the first 20 results and a next_page_token if more results exist

#### Scenario: Subsequent page request
- **WHEN** a client sends a list request with a non-empty page_token
- **THEN** the server SHALL return results starting after the cursor position

### Requirement: Message type
The system SHALL define a canonical Message type with fields: content_id (string), campaign_id (string), sender_name (string), summary (string), preview (string), body (string, rendered Markdown), critical (bool), actions (repeated MessageAction), created_at (google.protobuf.Timestamp).

#### Scenario: Message with actions
- **WHEN** a campaign message includes an ACK action
- **THEN** the Message SHALL contain one MessageAction with type ACTION_TYPE_ACK and a label

#### Scenario: Message without actions
- **WHEN** a campaign message is informational only
- **THEN** the Message SHALL contain zero MessageAction entries

#### Scenario: Canonical usage across services
- **WHEN** the render-service produces output, the inbox-service returns entries, or the action-service references a message
- **THEN** all SHALL use the same Message type definition

### Requirement: MessageAction type
The system SHALL define a MessageAction type with fields: id (string), type (ActionType), label (string).

#### Scenario: ACK action in MVP
- **WHEN** a message has an acknowledge button
- **THEN** the MessageAction SHALL have type ACTION_TYPE_ACK and label "Acknowledge Message" (or custom label)

### Requirement: WorkflowDefinition type
The system SHALL define a WorkflowDefinition message containing a repeated list of WorkflowStep messages. This represents a full DAG of campaign workflow execution.

#### Scenario: MVP workflow definition
- **WHEN** a campaign uses the default workflow
- **THEN** the WorkflowDefinition SHALL contain 4 steps: send-notification, wait-ack, send-reminder, call-webhook with appropriate transitions

#### Scenario: JSON serialization
- **WHEN** a WorkflowDefinition is serialized to JSON
- **THEN** it SHALL produce a valid JSON representation that can be stored, transmitted, and deserialized without loss

### Requirement: WorkflowStep type
The system SHALL define a WorkflowStep message with fields: id (string), type (StepType), a oneof config block containing typed configs (SendNotificationConfig, WaitActionConfig, SendReminderConfig, CallWebhookConfig), and transitions (map<string, string>) mapping outcome names to next step IDs where "end" is a reserved terminal value.

#### Scenario: Step with single transition
- **WHEN** a step has one outcome (e.g., "done")
- **THEN** transitions SHALL map "done" to the next step ID or "end"

#### Scenario: Step with branching transitions
- **WHEN** a step has multiple outcomes (e.g., "true"/"false")
- **THEN** transitions SHALL map each outcome to a different next step ID or "end"

### Requirement: SendNotificationConfig type
The system SHALL define SendNotificationConfig with fields: type (string, e.g., "push"), template_id (string, field 2), template_version (int32, field 3), action_label (string, field 4), action_type (ActionType, field 5), custom_variables (map<string, string>, field 6).

#### Scenario: Initial notification step
- **WHEN** a workflow begins with a send notification step
- **THEN** the config SHALL specify the notification type

#### Scenario: Notification step with template binding
- **WHEN** a workflow step uses a specific template
- **THEN** the SendNotificationConfig SHALL carry the template_id and template_version for that step

#### Scenario: Notification step with action override
- **WHEN** a workflow step overrides the default action
- **THEN** the SendNotificationConfig SHALL carry the action_label and action_type for that step

#### Scenario: Notification step with custom variables
- **WHEN** a workflow step provides campaign-specific variable values
- **THEN** the SendNotificationConfig SHALL carry the custom_variables map for template resolution

### Requirement: WaitActionConfig type
The system SHALL define WaitActionConfig with fields: action_type (string, e.g., "ack"), due_time (string, e.g., "1d").

#### Scenario: Wait for ACK with 1 day timeout
- **WHEN** a workflow step waits for user action
- **THEN** the config SHALL specify action_type "ack" and due_time "1d"

### Requirement: SendReminderConfig type
The system SHALL define SendReminderConfig with fields: type (string, e.g., "push"), repeat (string, e.g., "1d"), due_time (string, e.g., "2w").

#### Scenario: Push reminder with daily repeat and 2 week limit
- **WHEN** a workflow step sends reminders
- **THEN** the config SHALL specify type "push", repeat "1d", due_time "2w"

### Requirement: CallWebhookConfig type
The system SHALL define CallWebhookConfig with fields: name (string), url (string), headers (map<string, string>).

#### Scenario: Webhook call on missed delivery
- **WHEN** a workflow step calls a webhook
- **THEN** the config SHALL specify the webhook name and URL

### Requirement: Org context via JWT
Request messages SHALL NOT include org_id fields. Org context SHALL be extracted from JWT claims in the gRPC server middleware.

#### Scenario: Authenticated request
- **WHEN** a client sends a gRPC request with a valid JWT
- **THEN** the server SHALL extract org_id from the token claims and scope all operations to that organization

#### Scenario: Request without org context
- **WHEN** a gRPC request arrives without a valid JWT or without org claims
- **THEN** the server SHALL reject the request with an UNAUTHENTICATED error

### Requirement: TemplateType enum in common scope
The `TemplateType` enum SHALL be defined in `template.proto` (not `common.proto`) since it is specific to the template domain. It is referenced here for cross-cutting awareness. See `template-service` spec for the full definition.

#### Scenario: TemplateType available to downstream consumers
- **WHEN** code is generated from proto definitions
- **THEN** Go, TypeScript, and Rust stubs SHALL include the TemplateType enum

### Requirement: TemplateVariableSource enum in template scope
The `TemplateVariableSource` enum SHALL be defined in `template.proto` (not `common.proto`) since it is specific to the template variable domain. It is referenced here for cross-cutting awareness. See `template-variable-sources` spec for the full definition.

#### Scenario: TemplateVariableSource available to downstream consumers
- **WHEN** code is generated from proto definitions
- **THEN** Go, TypeScript, and Rust stubs SHALL include the TemplateVariableSource enum
