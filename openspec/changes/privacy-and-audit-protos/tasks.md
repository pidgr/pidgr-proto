## 1. PrivacyService Proto

- [ ] 1.1 Create `proto/pidgr/v1/privacy.proto` with PrivacyRequestStatus enum, all request/response messages
- [ ] 1.2 Add PrivacyService with 5 RPCs (ExportUserData, DeleteUserData, RectifyUserData, RestrictProcessing, GetDataExistenceConfirmation)
- [ ] 1.3 Add doc comments with auth requirements on each RPC
- [ ] 1.4 Run `buf lint` and fix any issues
- [ ] 1.5 Run `buf build` and verify compilation

## 2. AuditService Proto

- [ ] 2.1 Create `proto/pidgr/v1/audit.proto` with AuditEventType enum, AuditExportFormat enum, AuditEvent message (including hash chain fields)
- [ ] 2.2 Add AuditService with 2 RPCs (ListAuditEvents, ExportAuditTrail)
- [ ] 2.3 Import PrivacyRequestStatus from privacy.proto for ExportAuditTrail response status
- [ ] 2.4 Add doc comments with auth requirements
- [ ] 2.5 Run `buf lint` + `buf build`

## 3. TemplateVariable PII Field

- [ ] 3.1 Add `bool pii = 6` to TemplateVariable in `proto/pidgr/v1/template.proto` with doc comment
- [ ] 3.2 Run `buf breaking --against .git#branch=main` — confirm no breaking changes
- [ ] 3.3 Run `buf lint` + `buf build`

## 4. Code Generation + Validation

- [ ] 4.1 Run `buf generate` to regenerate Go/Rust/TypeScript stubs
- [ ] 4.2 Verify `gen/go/pidgr/v1/` has new privacy and audit files
- [ ] 4.3 Verify `gen/ts/` has new privacy and audit exports
- [ ] 4.4 Verify `gen/rust/` has new privacy and audit modules
- [ ] 4.5 Verify docs/ index.md is updated by buf doc plugin

## 5. Release

- [ ] 5.1 Create branch `release/0.43.0` from main
- [ ] 5.2 Bump `gen/ts/package.json` to 0.43.0
- [ ] 5.3 Bump `gen/rust/Cargo.toml` to 0.43.0
- [ ] 5.4 Run `npm install` in `gen/ts/`
- [ ] 5.5 Commit: `chore: bump version to 0.43.0`
- [ ] 5.6 Push branch, create PR titled `release: v0.43.0`
- [ ] 5.7 On merge, verify CI auto-creates tags + publishes npm
