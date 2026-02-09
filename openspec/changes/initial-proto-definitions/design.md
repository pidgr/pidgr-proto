## Context

Pidgr is a greenfield internal communication platform with no existing codebase. The pidgr-proto repo is the shared contract layer for all services. There are currently no proto files, no build tooling, and no generated code.

Four downstream repos will consume these definitions:
- **pidgr-api** (Go) — implements most services, acts as client for render-service
- **pidgr-renderer** (Rust) — implements render-service
- **pidgr-mobile** (React Native) — consumes inbox, action, device, and user-org services
- **future clients** — any additional consumer of the gRPC API

The platform uses Temporal for workflow orchestration, PostgreSQL for persistent state, S3 for immutable artifacts, and Cognito (LocalStack) for authentication. All inter-service communication uses gRPC with Protobuf.

## Goals / Non-Goals

**Goals:**
- Define a clean, versioned proto package structure that scales as services grow
- Establish buf tooling for linting, breaking change detection, and multi-language codegen
- Model all MVP services with correct message types and RPC signatures
- Design the WorkflowDefinition as a data-driven DAG that Go interprets into Temporal activities
- Design the Message and Action model as extensible primitives that support future interaction types without breaking changes
- Separate read tracking from action signals at the proto level

**Non-Goals:**
- Implementing any server or client code — this change is proto definitions only
- Defining database schemas — those belong in pidgr-api
- Defining Temporal workflow implementation details — protos define the contract, Go implements the mapping
- Publishing generated code packages — that will be set up in a follow-up change once CI/CD is in place
- Supporting proto editions or proto2 — all definitions use proto3

## Decisions

### 1. Package structure: flat `pidgr/v1/` namespace

All proto files live under a single `pidgr/v1/` package.

**Rationale:** With 8-9 service files plus a common types file, a flat structure is simple and avoids import complexity. Each service gets its own `.proto` file (e.g., `pidgr/v1/campaign.proto`) but shares the same Go/Rust/TS package.

**Alternative considered:** Per-service packages (`pidgr/campaign/v1/`, `pidgr/inbox/v1/`). This provides stricter isolation but introduces cross-package imports for shared types and complicates codegen configuration. Not justified at this scale.

**When to revisit:** If the number of proto files exceeds ~15 or if services need independent versioning, split into per-domain packages.

### 2. Buf over raw protoc

Use the Buf CLI (`buf lint`, `buf generate`, `buf breaking`) for all proto operations.

**Rationale:** Buf provides dependency management (buf.lock), consistent linting (DEFAULT category), breaking change detection against git history, and unified codegen config. It replaces the need for manual protoc invocations and Makefile chains.

**Alternative considered:** Raw `protoc` with Makefiles. More control but significantly more maintenance, no built-in breaking change detection, and harder dependency management.

### 3. WorkflowDefinition as a proto message with typed step configs

The workflow DAG is represented as a repeated list of `WorkflowStep` messages. Each step has a `StepType` enum and a `oneof` config block with typed configs per step type.

**Rationale:** Using typed configs (`SendNotificationConfig`, `WaitActionConfig`, etc.) provides compile-time safety in generated code. Go can switch on the `oneof` case and extract the correct config without type assertions or JSON parsing. Adding a new step type requires adding a new enum value + config message + Go handler, which is a controlled, reviewable change.

**Alternative considered:** `google.protobuf.Struct` for config (arbitrary JSON). More flexible but loses all type safety. Go code would need runtime assertions on every field. Bugs become runtime errors instead of compile errors.

**Alternative considered:** `bytes` config field with JSON schema validation. Same flexibility problem as Struct, plus requires maintaining a separate schema.

**Trade-off:** Adding new step types requires a proto change and regeneration. This is acceptable because new step types are infrequent and should be reviewed as contract changes.

### 4. Canonical Message type shared across all services

A single `Message` message type is used in inbox responses, render output, and delivery context. It contains the full rendered content plus attached actions.

**Rationale:** Prevents drift between what the renderer produces, what the inbox serves, and what delivery records reference. One type, one contract, one set of generated code.

**Alternative considered:** Separate `InboxMessage`, `RenderedMessage`, `DeliveryMessage` types. Would lead to mapping code and potential field drift across services.

### 5. ActionType enum with UNSPECIFIED safety value

All enums use `UNSPECIFIED = 0` as the default value, including `ActionType`, `CampaignStatus`, `DeliveryStatus`, etc.

**Rationale:** Proto3 default for enums is `0`. Without `UNSPECIFIED`, a missing or default field silently maps to the first real value, creating ambiguous state. `UNSPECIFIED` makes the "not set" case explicit. Servers must never produce `UNSPECIFIED` — if it appears, it signals a bug.

### 6. Server-streaming RPC for render-service

`RenderBatch` uses server-streaming (`returns stream RenderResult`) rather than unary request/response.

**Rationale:** The Rust renderer processes users in parallel and can begin streaming results before the entire batch completes. This allows Go to start persisting to S3/DB immediately, reducing end-to-end latency for large batches (up to 20,000 users). It also avoids building a single massive response message that could exceed gRPC size limits.

**Alternative considered:** Unary RPC returning `repeated RenderResult`. Simpler but requires the entire batch to complete before Go receives any data. For 20,000 users this adds unnecessary latency and memory pressure.

### 7. Read tracking as a separate RPC from action submission

`MarkRead` (inbox-service) and `SubmitAction` (action-service) are in different services with different responsibilities.

**Rationale:** Read events are analytics-only (OTEL + PostHog + optional audit). Action signals drive Temporal workflows. Mixing them in one service conflates observability with business logic. The mobile app calls `MarkRead` on message open (always) and `SubmitAction` on button press (only when actions exist). These are independent code paths.

### 8. Org-scoped auth via JWT claims, not request fields

Request messages do not include `org_id`. The org context is extracted from JWT claims in the gRPC interceptor/middleware on the server side.

**Rationale:** Prevents callers from spoofing org context. Simplifies request messages. The gRPC metadata (Authorization header) carries the JWT, and the server middleware extracts and validates org membership before the handler runs.

### 9. Codegen targets: Go, Rust, TypeScript

Three code generation targets configured in `buf.gen.yaml`:
- **Go:** `protocolbuffers/go` + `grpc/go` — consumed by pidgr-api
- **Rust:** `neoeinstein-prost` + `neoeinstein-tonic` — consumed by pidgr-renderer
- **TypeScript:** `timostamm-protobuf-ts` — consumed by pidgr-mobile

**Rationale:** These are the standard, well-maintained buf plugins for each language. `protobuf-ts` generates TypeScript-first code suitable for React Native (no grpc-web dependency required for types, can use connect-web or grpc-web transport layer separately).

## Risks / Trade-offs

**[Risk] WorkflowDefinition proto changes required for new step types** → Mitigated by keeping the `oneof` config extensible. Adding a new step type is a single PR adding an enum value, a config message, and a `oneof` case. Breaking change detection ensures backward compatibility.

**[Risk] Canonical Message type becomes too large as features grow** → Mitigated by keeping the MVP Message lean (content + actions only). Future fields can be added without breaking existing consumers. If Message diverges significantly per context, it can be split later, but premature splitting is worse.

**[Risk] Buf community plugins for Rust/TS may lag behind official plugins** → Mitigated by choosing the most widely adopted community plugins (neoeinstein-prost has 1M+ downloads, timostamm-protobuf-ts is the de facto TS plugin). If a plugin becomes unmaintained, switching to an alternative requires only a buf.gen.yaml change.

**[Risk] Flat package structure may not scale** → Mitigated by the "revisit at ~15 files" guideline. Current scope is ~10 files which is well within the comfort zone.

**[Risk] Server-streaming RenderBatch adds complexity to error handling** → Mitigated by including an `error` field per `RenderResult`. Individual user failures are reported inline without terminating the stream. The Go client collects errors and persists partial results.

## Open Questions

- Should generated code be committed to the repo (vendored) or generated in CI and published as packages? This depends on the CI/CD setup which is outside the scope of this change.
- What is the exact `buf.build` module path? Options: `buf.build/pidgr/proto` (if using BSR) or local-only module name. Deferred to buf-tooling implementation.
