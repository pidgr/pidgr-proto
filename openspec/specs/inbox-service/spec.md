## Purpose

Define gRPC service for mobile inbox synchronization and read tracking.
## Requirements
### Requirement: InboxService proto definition
The system SHALL define an InboxService in `pidgr/v1/inbox.proto` with RPCs: Sync, MarkRead, GetMessage.

#### Scenario: Service definition exists
- **WHEN** the proto file is compiled
- **THEN** it SHALL produce a valid InboxService with Sync, MarkRead, and GetMessage RPCs

### Requirement: InboxEntry message type
The system SHALL define an InboxEntry message with fields: delivery_id (string), message (Message), status (DeliveryStatus), read (bool), received_at (Timestamp). The message field SHALL use the canonical Message type from common-types.

#### Scenario: Unread message entry
- **WHEN** a user has not opened a message
- **THEN** the InboxEntry SHALL have read=false

#### Scenario: Read message entry
- **WHEN** a user has opened a message via MarkRead
- **THEN** the InboxEntry SHALL have read=true

#### Scenario: Message with actions
- **WHEN** an inbox entry contains a message with an ACK action
- **THEN** the embedded Message SHALL include the MessageAction with type ACTION_TYPE_ACK

#### Scenario: Message without actions
- **WHEN** an inbox entry contains an informational message
- **THEN** the embedded Message SHALL have an empty actions list

### Requirement: Sync RPC
The system SHALL define Sync(SyncInboxRequest) returning SyncInboxResponse. SyncInboxRequest SHALL contain: since (Timestamp), limit (int32). SyncInboxResponse SHALL contain: entries (repeated InboxEntry), next_since (Timestamp).

#### Scenario: Initial sync on app open
- **WHEN** a mobile client sends a Sync request with a zero/empty since timestamp
- **THEN** the server SHALL return the most recent messages up to the limit

#### Scenario: Incremental sync
- **WHEN** a mobile client sends a Sync request with a non-zero since timestamp
- **THEN** the server SHALL return only messages received after that timestamp

#### Scenario: No new messages
- **WHEN** a mobile client syncs and there are no new messages since the given timestamp
- **THEN** the response SHALL contain an empty entries list and next_since equal to the request's since value

#### Scenario: Default limit
- **WHEN** a client sends a Sync request with limit=0
- **THEN** the server SHALL use a default limit of 50

### Requirement: MarkRead RPC
The system SHALL define MarkRead(MarkReadRequest) returning MarkReadResponse. MarkReadRequest SHALL contain: delivery_id (string). MarkReadResponse SHALL contain: success (bool).

#### Scenario: Mark a message as read
- **WHEN** a mobile client opens a message and calls MarkRead
- **THEN** the server SHALL record the read event, emit an OTEL span with type message_read, and return success=true

#### Scenario: MarkRead does not trigger Temporal
- **WHEN** MarkRead is called
- **THEN** the server SHALL NOT send any Temporal signal — read events are analytics only

#### Scenario: MarkRead is idempotent
- **WHEN** MarkRead is called multiple times for the same delivery_id
- **THEN** the server SHALL return success=true without creating duplicate events

#### Scenario: MarkRead emits observability data
- **WHEN** MarkRead is called
- **THEN** the server SHALL emit an OTEL span with attributes: campaign.id, delivery.id, user.id, org.id

### Requirement: GetMessage RPC
The system SHALL define GetMessage(GetMessageRequest) returning GetMessageResponse. GetMessageRequest SHALL contain: delivery_id (string). GetMessageResponse SHALL contain: entry (InboxEntry).

#### Scenario: Get message by valid delivery ID
- **WHEN** a client calls GetMessage with a valid delivery_id
- **THEN** the response SHALL contain the InboxEntry with full message content, delivery status, and read status

#### Scenario: Get message with non-existent delivery ID
- **WHEN** a client calls GetMessage with a non-existent delivery_id
- **THEN** the server SHALL return a NOT_FOUND error

