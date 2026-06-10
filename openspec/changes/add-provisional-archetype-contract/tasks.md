## 1. Proto changes

- [x] 1.1 Add `ArchetypeSource` enum + `Archetype.source = 11` + `GetGroupArchetypesResponse.confidence_level = 4` to `proto/pidgr/v1/insights.proto`
- [x] 1.2 Add `Organization.provisional_archetypes_enabled = 21` + `UpdateOrganizationRequest.provisional_archetypes_enabled = 9` (optional bool) to `proto/pidgr/v1/organization.proto`
- [x] 1.3 Run `buf format -w`
- [x] 1.4 Run `buf lint` — clean
- [x] 1.5 Run `buf breaking --against .git#branch=main` — confirm additive/non-breaking

## 2. Code generation + validation

- [x] 2.1 Run `buf generate` to regenerate Go/Rust/TypeScript stubs + docs
- [x] 2.2 Verify the new fields appear in `gen/go/pidgr/v1/insights.pb.go`, `gen/go/pidgr/v1/organization.pb.go`, and the TS/Rust equivalents
- [x] 2.3 Verify `gen/go` compiles (`go build ./...`)
