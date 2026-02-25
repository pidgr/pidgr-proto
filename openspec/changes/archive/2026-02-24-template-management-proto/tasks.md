## 1. Common Types — UserProfile & Enums

- [x] 1.1 Add `UserProfile` message to `common.proto` with all 10 fields (first_name through custom_attributes)
- [x] 1.2 Add `UserProfile profile = 9` field to `User` message in `common.proto`
- [x] 1.3 Add template binding fields to `SendNotificationConfig` in `common.proto` (template_id=2, template_version=3, action_label=4, action_type=5, custom_variables=6)

## 2. Template Types & Variable Sources

- [x] 2.1 Add `TemplateType` enum to `template.proto` (UNSPECIFIED=0, MARKDOWN=1, RICH=2, HTML=3)
- [x] 2.2 Add `TemplateVariableSource` enum to `template.proto` (UNSPECIFIED=0, PROFILE=1, CUSTOM=2)
- [x] 2.3 Add `source = 4` (TemplateVariableSource) and `default_value = 5` (string) fields to `TemplateVariable` message
- [x] 2.4 Add `type = 9` (TemplateType) field to `Template` message
- [x] 2.5 Add `type = 5` (TemplateType) field to `CreateTemplateRequest` message
- [x] 2.6 Add `type = 2` (TemplateType) field to `ListTemplatesRequest` message

## 3. Member Service — Profile RPCs

- [x] 3.1 Add `UserProfile profile = 5` field to `InviteUserRequest` in `member.proto`
- [x] 3.2 Add `UpdateUserProfileRequest` message (user_id=1, profile=2) to `member.proto`
- [x] 3.3 Add `UpdateUserProfileResponse` message (user=1) to `member.proto`
- [x] 3.4 Add `UpdateUserProfile` RPC to `MemberService`

## 4. Organization Service — SSO Attribute Mapping

- [x] 4.1 Add `SsoAttributeMapping` message (idp_claim=1, profile_field=2) to `organization.proto`
- [x] 4.2 Add `repeated SsoAttributeMapping sso_attribute_mappings = 7` to `Organization` message
- [x] 4.3 Add `UpdateSsoAttributeMappingsRequest` message (sso_attribute_mappings=1) to `organization.proto`
- [x] 4.4 Add `UpdateSsoAttributeMappingsResponse` message (organization=1) to `organization.proto`
- [x] 4.5 Add `UpdateSsoAttributeMappings` RPC to `OrganizationService`

## 5. Validation & Release

- [x] 5.1 Run `buf build` — verify clean compilation
- [x] 5.2 Run `buf lint` — verify all naming conventions pass
- [x] 5.3 Run `buf breaking --against .git#branch=main` — verify no breaking changes
- [x] 5.4 Run `buf generate` — regenerate Go, Rust, TypeScript stubs
- [ ] 5.5 Bump version to 0.24.0 in `gen/ts/package.json` and `gen/rust/Cargo.toml`
- [ ] 5.6 Create `release/0.24.0` branch, commit, and open PR
