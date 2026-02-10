## ADDED Requirements

### Requirement: HTML documentation generation
The system SHALL generate a browsable HTML documentation page from proto file comments using `protoc-gen-doc` via `buf generate`.

#### Scenario: Documentation includes all services
- **WHEN** `buf generate` is run
- **THEN** `docs/index.html` SHALL list all 7 gRPC services with their RPCs, request/response types, and field descriptions

#### Scenario: Documentation includes enum descriptions
- **WHEN** `buf generate` is run
- **THEN** `docs/index.html` SHALL list all enums with their values and descriptions

### Requirement: Proto files have doc comments
The system SHALL include leading comments on all services, RPCs, messages, enums, and fields in every proto file.

#### Scenario: Every service has a doc comment
- **WHEN** a developer opens any `.proto` file
- **THEN** each `service`, `rpc`, `message`, `enum`, and field SHALL have a leading `//` comment describing its purpose

### Requirement: Generated docs hidden from diffs
The system SHALL include `docs/` in `.gitattributes` with `linguist-generated=true` so generated HTML does not appear in PR diffs.

#### Scenario: PR diff excludes docs
- **WHEN** a PR modifies proto files and regenerates docs
- **THEN** the `docs/index.html` changes SHALL be hidden from the GitHub diff view
