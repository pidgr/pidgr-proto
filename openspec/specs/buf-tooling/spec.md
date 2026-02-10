## Purpose

Configure buf tooling, code generation, and proto style conventions.

## Requirements

### Requirement: buf.yaml module configuration
The system SHALL define a buf.yaml at the repository root configuring the buf module with a proto source directory, lint rules using the DEFAULT category, and breaking change detection enabled against the git main branch.

#### Scenario: Lint passes on valid proto files
- **WHEN** `buf lint` is run against the proto directory
- **THEN** all proto files SHALL pass with zero lint errors

#### Scenario: Breaking change detected
- **WHEN** a proto change removes a field or changes a field number
- **THEN** `buf breaking --against .git#branch=main` SHALL report the breaking change

### Requirement: Proto package structure
All proto files SHALL be organized under `proto/pidgr/v1/`. The package name for all files SHALL be `pidgr.v1`. The following proto files SHALL exist:
- `common.proto` — shared types, enums, pagination, Message, MessageAction, WorkflowDefinition
- `campaign.proto` — CampaignService and campaign-related messages
- `template.proto` — TemplateService and template-related messages
- `action.proto` — ActionService and action-related messages
- `inbox.proto` — InboxService and inbox-related messages
- `device.proto` — DeviceService and device-related messages
- `user_org.proto` — UserOrgService and user/organization-related messages
- `render.proto` — RenderService and render-related messages (internal)

#### Scenario: All proto files compile
- **WHEN** `buf build` is run
- **THEN** all proto files SHALL compile without errors

#### Scenario: Cross-file imports
- **WHEN** campaign.proto imports common.proto types
- **THEN** the import SHALL resolve correctly within the `pidgr.v1` package

### Requirement: buf.gen.yaml code generation targets
The system SHALL define a buf.gen.yaml configuring code generation for three languages:
- **Go**: using `protocolbuffers/go` and `grpc/go` plugins, output to `gen/go/`
- **Rust**: using `neoeinstein-prost` and `neoeinstein-tonic` plugins, output to `gen/rust/`
- **TypeScript**: using `timostamm-protobuf-ts` plugin, output to `gen/ts/`

#### Scenario: Generate Go stubs
- **WHEN** `buf generate` is run
- **THEN** Go protobuf and gRPC stubs SHALL be generated in `gen/go/pidgr/v1/`

#### Scenario: Generate Rust stubs
- **WHEN** `buf generate` is run
- **THEN** Rust prost and tonic stubs SHALL be generated in `gen/rust/`

#### Scenario: Generate TypeScript stubs
- **WHEN** `buf generate` is run
- **THEN** TypeScript protobuf stubs SHALL be generated in `gen/ts/pidgr/v1/`

### Requirement: Proto style conventions
All proto files SHALL follow these conventions:
- Use proto3 syntax
- Import `google/protobuf/timestamp.proto` for all timestamp fields
- Enum values SHALL be prefixed with the enum name in UPPER_SNAKE_CASE (e.g., CAMPAIGN_STATUS_CREATED)
- Message fields SHALL use lower_snake_case
- Service names SHALL use PascalCase
- RPC names SHALL use PascalCase
- All enums SHALL have an UNSPECIFIED value at position 0

#### Scenario: Enum naming convention
- **WHEN** a new enum value is added
- **THEN** it SHALL follow the pattern `ENUM_NAME_VALUE_NAME` (e.g., DELIVERY_STATUS_SENT)

#### Scenario: Proto3 syntax
- **WHEN** any proto file is inspected
- **THEN** it SHALL declare `syntax = "proto3";` as the first non-comment line

### Requirement: Dependency management
The system SHALL use buf.lock for dependency management. The only external dependency for the MVP SHALL be `google/protobuf` well-known types (Timestamp, Struct).

#### Scenario: No unnecessary dependencies
- **WHEN** buf.lock is inspected
- **THEN** it SHALL contain only the buf.build well-known types module and no other external modules
