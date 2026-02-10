## 1. Proto Changes

- [x] 1.1 Add `CreateOrganizationRequest` message to `user_org.proto` with fields: `name` (string), `admin_email` (string)
- [x] 1.2 Add `CreateOrganizationResponse` message with fields: `organization` (Organization), `admin_user` (User)
- [x] 1.3 Add `CreateOrganization` RPC to `UserOrgService`
- [x] 1.4 Run `buf build` and `buf lint` to verify

## 2. Code Generation

- [x] 2.1 Run `buf generate` to regenerate Go, Rust, and TypeScript stubs
- [x] 2.2 Verify generated Go code includes `CreateOrganization` in the service interface
- [x] 2.3 Verify `buf breaking` reports no breaking changes against main
