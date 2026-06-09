## 1. Proto field additions

- [x] 1.1 Add `bool synthetic = 8` to `AuditEvent` in `proto/pidgr/v1/audit.proto`
- [x] 1.2 Add `bool synthetic = 9` to `Delivery` in `proto/pidgr/v1/campaign.proto`
- [x] 1.3 Add `bool synthetic = 20` to `Campaign` in `proto/pidgr/v1/campaign.proto`
- [x] 1.4 Add `optional bool include_synthetic_in_aggregates = 21` to `Organization` in `proto/pidgr/v1/organization.proto`
- [x] 1.5 Add `optional bool include_synthetic_in_aggregates = 9` to `UpdateOrganizationRequest` in `proto/pidgr/v1/organization.proto`
- [x] 1.6 Run `buf format -w`

## 2. Validation

- [x] 2.1 Run `buf lint` — clean
- [x] 2.2 Run `buf breaking --against .git#branch=main` — no breaking changes
- [x] 2.3 Run `buf generate` to regenerate Go/Rust/TypeScript stubs + docs

## 3. Code generation verification

- [x] 3.1 Verify all five fields appear in `gen/go/pidgr/v1/` with correct field numbers
- [x] 3.2 Verify all five fields appear in `gen/ts/pidgr/v1/` with correct field numbers
- [x] 3.3 Verify all five fields appear in `gen/rust/pidgr/v1/pidgr.v1.rs`
- [x] 3.4 Verify all five fields appear in `docs/index.md`
- [x] 3.5 Verify `gen/go` compiles (`GOWORK=off go build ./...`)
