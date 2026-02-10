## ADDED Requirements

### Requirement: GetMessage RPC
The system SHALL define GetMessage(GetMessageRequest) returning GetMessageResponse. GetMessageRequest SHALL contain: delivery_id (string). GetMessageResponse SHALL contain: entry (InboxEntry).

#### Scenario: Get message by valid delivery ID
- **WHEN** a client calls GetMessage with a valid delivery_id
- **THEN** the response SHALL contain the InboxEntry with full message content, delivery status, and read status

#### Scenario: Get message with non-existent delivery ID
- **WHEN** a client calls GetMessage with a non-existent delivery_id
- **THEN** the server SHALL return a NOT_FOUND error

## MODIFIED Requirements

### Requirement: InboxService proto definition
The system SHALL define an InboxService in `pidgr/v1/inbox.proto` with RPCs: Sync, MarkRead, GetMessage.

#### Scenario: Service definition exists
- **WHEN** the proto file is compiled
- **THEN** it SHALL produce a valid InboxService with Sync, MarkRead, and GetMessage RPCs
