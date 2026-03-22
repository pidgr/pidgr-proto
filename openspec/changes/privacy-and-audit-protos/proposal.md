## Why

Chikorita (v0.3) makes Pidgr commercially viable for LATAM enterprise customers. First contracts in UY, AR, CL, and BR require GDPR-aligned data protection, audit trails, and data subject rights. The API needs proto definitions for PrivacyService and AuditService before any GDPR implementation can begin across repos. Additionally, session replay is being refactored to mask PII template variables — this requires a `pii` flag on `TemplateVariable`.

## What Changes

- Add new `PrivacyService` with 5 RPCs for GDPR data subject rights (export, delete, rectify, restrict processing, existence confirmation)
- Add new `AuditService` with 2 RPCs for compliance audit trail (list events, export trail)
- Add `bool pii = 6` field to existing `TemplateVariable` message for session replay PII masking
- Add supporting messages: `AuditEvent`, `AuditEventType` enum, `PrivacyRequestStatus` enum, `AuditExportFormat` enum
- Add new proto files: `privacy.proto`, `audit.proto`

## Capabilities

### New Capabilities
- `privacy-service`: PrivacyService proto — ExportUserData, DeleteUserData, RectifyUserData, RestrictProcessing, GetDataExistenceConfirmation RPCs with request/response messages and PrivacyRequestStatus enum
- `audit-service`: AuditService proto — ListAuditEvents, ExportAuditTrail RPCs with AuditEvent message, AuditEventType enum, AuditExportFormat enum, hash chain fields

### Modified Capabilities
- `template-variable-pii`: Add `bool pii = 6` to TemplateVariable message. When true, the rendered value is wrapped in `{{pii}}...{{/pii}}` markers by the API renderer, enabling session replay and heatmap screenshot masking on clients.

## Impact

- **Proto files**: 2 new files (`privacy.proto`, `audit.proto`), 1 modified (`template.proto`)
- **Codegen**: New Go/Rust/TypeScript stubs generated for PrivacyService and AuditService
- **Downstream repos**: pidgr-api (implements handlers), pidgr-admin (privacy request UI, audit export UI, template PII toggle), pidgr-mobile (PII masking in renderer), pidgr-desktop (PII masking), pidgr-mcp (privacy tools)
- **Breaking changes**: None — all additions are backward compatible
- **Version**: Requires proto version bump (v0.43.0)
