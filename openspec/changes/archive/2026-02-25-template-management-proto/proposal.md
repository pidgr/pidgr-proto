## Why

Templates today are untyped markdown strings with flat variable lists, and user profiles only carry `name` and `email`. Campaign creators cannot personalize messages with standard employee attributes (department, title, location) because that data doesn't exist in the system. Template variables have no source tracking — every variable must be manually provided at campaign launch, even for data that should auto-resolve from user profiles. Additionally, templates are bound at the campaign level, making multi-step workflows with different messages per step impossible. This change introduces the proto foundation for user profiles, template typing, variable source resolution, step-level template binding, and SSO attribute mapping.

## What Changes

- Add `UserProfile` message to `common.proto` with structured fields: `first_name`, `last_name`, `department`, `title`, `phone`, `location`, `employee_id`, `manager_name`, `start_date`, and extensible `custom_attributes` map
- Add `UserProfile profile` field to `User` message and `InviteUserRequest`
- Add `UpdateUserProfileRequest`/`UpdateUserProfileResponse` messages and `UpdateUserProfile` RPC to `MemberService`
- Add `TemplateType` enum (`MARKDOWN`, `RICH` reserved, `HTML` reserved) and `type` field to `Template` and `CreateTemplateRequest`
- Add `TemplateVariableSource` enum (`PROFILE`, `CUSTOM`) to `template.proto`
- Extend `TemplateVariable` with `source` (TemplateVariableSource) and `default_value` fields
- Add `ListTemplatesRequest.type` filter field
- Add template binding fields to `SendNotificationConfig`: `template_id`, `template_version`, `action_label`, `action_type`, `custom_variables`
- Add `SsoAttributeMapping` message and `repeated SsoAttributeMapping sso_attribute_mappings` to `Organization`
- Add `UpdateSsoAttributeMappingsRequest`/Response and RPC to `OrganizationService`

## Capabilities

### New Capabilities
- `user-profile`: UserProfile message with structured employee attributes, UpdateUserProfile RPC on MemberService, profile field on User and InviteUserRequest
- `template-variable-sources`: TemplateVariableSource enum (PROFILE, CUSTOM), source and default_value fields on TemplateVariable, enabling auto-resolution of profile-backed variables at render time
- `step-template-binding`: Template binding fields on SendNotificationConfig (template_id, template_version, action_label, action_type, custom_variables), enabling per-step template selection in multi-message workflows
- `sso-attribute-mapping`: SsoAttributeMapping message, mappings on Organization, UpdateSsoAttributeMappings RPC, enabling IdP claim-to-profile-field auto-sync

### Modified Capabilities
- `template-service`: Template gains TemplateType field; TemplateVariable gains source and default_value; ListTemplatesRequest gains type filter; CreateTemplateRequest gains type field
- `user-org-service`: User gains UserProfile profile field; InviteUserRequest gains optional UserProfile for pre-filling at invitation time
- `common-types`: New TemplateType and TemplateVariableSource enums; SendNotificationConfig extended with template binding fields

## Impact

- **Proto files**: `common.proto` (User, SendNotificationConfig, new enums), `template.proto` (TemplateVariable, Template, CreateTemplateRequest, ListTemplatesRequest), `member.proto` (InviteUserRequest, new UpdateUserProfile RPC), `organization.proto` (Organization, new SsoAttributeMapping, new RPC)
- **All downstream consumers**: Go (pidgr-api), TypeScript (pidgr-admin, pidgr-mobile) stubs regenerated. All changes are additive — no breaking changes.
- **Field numbering**: All new fields use next available numbers on existing messages. No reserved field conflicts.
- **Release**: v0.24.0 (minor bump, backward compatible)
