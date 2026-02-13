## Design Decisions

### D1: Proto comments as enforceable contract

**Decision:** Use proto doc comments (`//`) to specify authorization requirements, field constraints, and validation rules. These comments become the canonical contract that all downstream implementations must enforce.

**Rationale:** Proto3 has no built-in validation mechanism (unlike proto2 options or third-party validators like `buf validate`). Doc comments are already the established pattern in pidgr-proto for specifying behavior (e.g., "Omit for initial sync", "0 returns the latest version"). Adding security constraints as comments maintains consistency and ensures they appear in generated documentation (`docs/index.md`). The `buf validate` ecosystem is not yet adopted, and adding it would be a separate change.

### D2: DeviceSummary message for push token redaction (M4)

**Decision:** Create a new `DeviceSummary` message that mirrors `Device` but omits `push_token`. Use `DeviceSummary` in `RegisterResponse` and `ListDevicesResponse`. Keep the full `Device` message for internal/server-side use only.

**Rationale:** Push tokens are sensitive credentials. Returning them in API responses serves no purpose -- the client already has its own token (it sent it in `RegisterRequest`), and listing other devices' tokens is a security risk. A separate message type makes the API surface explicitly safe by construction, rather than relying on server-side field masking. This is a breaking change but the correct security boundary.

**Alternative considered:** Adding a `// INTERNAL: Do not return in responses` comment to `push_token`. Rejected because it relies on every implementation remembering to strip the field, which is error-prone.

### D3: Authorization comments use role-based format (M1)

**Decision:** Add `// Authorization: Requires <ROLE>+ role.` comments to RPCs that modify state. Use the existing `UserRole` enum values (ADMIN, MANAGER, EMPLOYEE) to specify minimum required roles. Add ownership validation comments to `SubmitAction`.

**Rationale:** The `UserRole` enum already defines the role hierarchy (ADMIN > MANAGER > EMPLOYEE). Documenting the minimum required role for each RPC gives implementers a clear, unambiguous requirement. The `+` suffix (e.g., "MANAGER+") means "this role or higher" and is a common convention in authorization documentation.

### D4: Field constraint format (M2, M3)

**Decision:** Add `// Constraints: Max length <N> characters.` or `// Constraints: Max <N> items.` comments to fields that need bounds.

**Rationale:** A consistent prefix ("Constraints:") makes constraints grep-able across proto files and distinguishable from behavioral documentation. Specific numeric limits are chosen based on reasonable application requirements:
- Campaign name: 200 characters (display-friendly)
- Template body: 50,000 characters (rich content)
- Template/variable names: 100 characters
- Email: 254 characters (RFC 5321)
- User name: 200 characters
- Organization name: 200 characters
- Message body: 100,000 characters (rendered content)
- Message summary/preview/sender_name: 500 characters
- Action label: 50 characters (button text)
- Push token: 4,096 characters (FCM tokens are ~163 chars, but allowing headroom)
- Action payload: 10,000 bytes
- user_ids: 100,000 items (per-campaign audience cap)

### D5: Webhook SSRF documentation (H4)

**Decision:** Add doc comments to `CallWebhookConfig.url` specifying: HTTPS required in production, private IP ranges (RFC 1918, RFC 4193, loopback) must be rejected, localhost must be rejected, and backend MUST validate before executing.

**Rationale:** SSRF is the highest-severity finding. While proto comments cannot enforce URL validation, documenting the requirement explicitly means any implementation that skips validation is clearly violating the contract. The specific IP ranges called out (10.0.0.0/8, 172.16.0.0/12, 192.168.0.0/16, 127.0.0.0/8, ::1) are the standard SSRF blocklist.

### D6: Workflow validation constraints (M5)

**Decision:** Add doc comments to `WorkflowDefinition` specifying: max 100 steps, max 10 transitions per step, and backend MUST validate that the graph is a DAG (no cycles). Add valid range comments to duration fields.

**Rationale:** Unbounded workflow definitions could cause resource exhaustion during execution. The limits (100 steps, 10 transitions) are generous enough for any reasonable campaign workflow while preventing abuse. DAG validation is critical because cyclic workflows would cause infinite Temporal execution. Duration bounds (1m to 8760h for delays, 1m to 168h for reminders) prevent both trivially short timers and unreasonably long ones.

### D7: Version bump strategy

**Decision:** This change requires a minor version bump due to the breaking `DeviceSummary` structural change in `device.proto`. All other changes are doc-comment-only (non-breaking).

**Rationale:** The `DeviceSummary` message introduction changes `RegisterResponse` and `ListDevicesResponse` field types, which breaks existing generated code. A minor version bump signals to consumers that they need to update their code. The release follows the established pidgr-proto release flow (release branch, version bumps, CI handles tagging and publishing).
