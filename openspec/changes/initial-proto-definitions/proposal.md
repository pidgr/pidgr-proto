## Why

Pidgr has no API contract layer yet. The platform requires four independent codebases (Go monolith, Rust renderer, React Native mobile, and future clients) to communicate through well-defined gRPC interfaces. Without a shared protobuf definitions repo, each service would define its own message types, leading to contract drift, duplicated models, and broken integrations. This change establishes the foundational `.proto` files, buf tooling, and package structure that every Pidgr service will depend on from day one.

## What Changes

- Define the complete set of protobuf service definitions and message types required by the Workflow Foundations PRD (Spinarak v0.1).
- Establish the proto package structure, buf configuration, and linting rules.
- Define code generation targets for Go, Rust, and TypeScript.
- Create shared common types (identifiers, timestamps, pagination, status enums, workflow definition model, message and action model) used across all services.
- Model campaign workflows as a JSON-representable DAG of steps, where each step maps to a Go function executed as a Temporal activity.
- Introduce a generic action system for message interactions, replacing hardcoded acknowledgment logic with an extensible action model.
- Separate read tracking (analytics-only) from action signals (workflow-driving) as independent interaction paths.

## Capabilities

### New Capabilities

- `common-types`: Shared message types, enums, and patterns used across all services. Includes org-scoped identifiers, timestamps, cursor pagination, status enums (CampaignStatus, DeliveryStatus, Platform, UserRole, UserStatus, ActionType, StepType), the canonical Message structure, MessageAction model, and the full WorkflowDefinition DAG model (WorkflowStep, typed step configs, transition maps).
- `campaign-service`: gRPC service for campaign lifecycle. Create a campaign with a template reference (pinned version), audience list, and workflow definition (DAG). Start a campaign to freeze the audience snapshot, pin the template version, and trigger the Temporal workflow. Query campaign status with aggregated delivery metrics.
- `template-service`: gRPC service for template management. Create and update Markdown templates with variable placeholders. Templates are append-only versioned — updates create new versions, never mutate existing ones. List and retrieve templates by ID with optional version pinning.
- `action-service`: gRPC service for explicit user interactions with messages. A single SubmitAction RPC handles all action types generically. For MVP the only action type is ACK. The submission triggers a Temporal ActionSubmitted signal with the action ID and optional payload. Future action types (polls, CTAs, forms) use the same RPC and signal contract without breaking changes.
- `inbox-service`: gRPC service for mobile inbox synchronization and read tracking. Sync fetches messages since a timestamp, returning canonical Message structures inside InboxEntry wrappers with delivery context and read state. MarkRead records that a user opened a message — this emits OTEL spans and analytics events but never triggers Temporal signals. Read tracking and action signals are independent paths.
- `device-service`: gRPC service for push notification token management. Register or update device tokens for FCM and APNs. Support multiple devices per user. Deactivate devices when tokens become invalid. List registered devices for a user.
- `user-org-service`: gRPC service for user and organization management. Invite users (triggers Cognito provisional user creation), list and retrieve user profiles, query and update organization settings including the default workflow definition used when campaigns omit a custom workflow.
- `render-service`: Internal gRPC service for Go to Rust communication during template compilation. A single server-streaming RPC receives a template ID, version, and user batch, then streams back rendered results as the Rust service completes each user in parallel. Output uses the canonical Message structure. Not exposed to external clients.
- `buf-tooling`: Buf module configuration, linting rules (DEFAULT category), breaking change detection, and code generation targets for Go (protocolbuffers/go + grpc/go), Rust (neoeinstein-prost + neoeinstein-tonic), and TypeScript (timostamm-protobuf-ts). Defines the proto package structure under pidgr/v1/.

### Modified Capabilities

_(none — this is a greenfield repo)_

## Impact

- **pidgr-api (Go monolith):** Imports generated Go stubs for all services. Implements server-side for campaign-service, template-service, action-service, inbox-service, device-service, and user-org-service. Acts as client for render-service. Interprets WorkflowDefinition DAGs by mapping each StepType to a Temporal activity function.
- **pidgr-renderer (Rust):** Imports generated Rust stubs. Implements server-side for render-service. Receives RenderBatchRequest, processes users in parallel, streams RenderResult messages back using the canonical Message structure.
- **pidgr-mobile (React Native):** Imports generated TypeScript stubs. Acts as client for inbox-service (Sync, MarkRead), action-service (SubmitAction), device-service (Register), and user-org-service (GetUser). Emits PostHog events for message_opened and ack_pressed. Reports reads via MarkRead and action submissions via SubmitAction as separate independent paths.
- **CI/CD:** Buf lint and breaking change detection gate all proto changes. Generated code packages are published for consumption by downstream repos.
- **All downstream repos** depend on this repo for API contracts. Proto definitions are the single source of truth for all inter-service communication.
