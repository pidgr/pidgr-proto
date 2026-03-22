## Context

pidgr-proto is the single source of truth for all gRPC service contracts. It currently has 17 services across 16 proto files. Chikorita (v0.3) requires two new services (PrivacyService, AuditService) and one field addition (TemplateVariable.pii) to enable GDPR compliance and PII-aware session replay across all downstream repos.

Current TemplateVariable has fields 1-5 (name, description, required, source, default_value) in `proto/pidgr/v1/template.proto`. Adding field 6 is backward compatible.

## Goals / Non-Goals

**Goals:**
- Define PrivacyService with 5 RPCs covering all GDPR data subject rights (Art. 15-18, 20) and LGPD confirmação de existência
- Define AuditService with 2 RPCs for append-only audit trail access and export
- Add `bool pii` flag to TemplateVariable for client-side masking in session replay and heatmap screenshots
- Maintain backward compatibility with all existing consumers

**Non-Goals:**
- ConsentService (removed from Chikorita scope — no consent required)
- InsightsService (deferred to v0.4 with AI Foundations)
- Implementation of any handlers — this is proto definitions only

## Decisions

### 1. Two new proto files (not one combined)

PrivacyService and AuditService are separate proto files (`privacy.proto`, `audit.proto`). They serve different personas (privacy for data subjects/admins, audit for compliance officers) and have independent message types.

Alternative considered: single `compliance.proto`. Rejected because it conflates two distinct concerns and produces a large, unfocused file.

### 2. AuditEvent hash chain fields

`AuditEvent` includes `previous_hash` and `hash` fields for tamper-evident integrity verification. The hash chain is computed server-side (SHA-256 of previous_hash + event data). This enables offline audit trail verification without trusting the database.

Alternative considered: no hash chain, rely on PostgreSQL immutability trigger only. Rejected because the trigger is a database-level control — the hash chain provides application-level integrity that survives database migrations or exports.

### 3. DeleteUserData supports anonymization

`DeleteUserDataRequest` includes a `bool anonymize` flag. When true, PII is replaced with placeholders instead of hard-deleted. This preserves audit trail integrity (events still reference a valid entity) while removing personal data.

### 4. TemplateVariable.pii as bool (not enum)

A simple boolean flag is sufficient — a variable either is PII or isn't. No need for PII categories or sensitivity levels at the proto layer. Clients use this flag to wrap rendered values in masking markers.

## Risks / Trade-offs

- **Proto version bump required** — all downstream repos must bump to v0.43.0 before implementing GDPR features. Coordinated across 5+ repos. → Mitigation: proto is always the first step, downstream bumps are routine.
- **AuditEventType enum may need extension** — the initial set (12 event types) may not cover all future auditable actions. → Mitigation: enums are additive in proto3, new values can be added without breaking.
- **No streaming for ExportUserData** — returns a single response with a download URL rather than streaming data. → Mitigation: export is async (Temporal workflow), URL points to S3 ZIP. Streaming adds complexity without benefit for this use case.
