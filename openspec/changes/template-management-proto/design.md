## Context

The pidgr-proto repo defines all gRPC service contracts. Currently:
- `User` (common.proto) has only `id`, `email`, `name`, `status`, `role`, `role_id` — no structured profile attributes
- `TemplateVariable` (template.proto) has `name`, `description`, `required` — no source typing or defaults
- `Template` has no type field (all templates are implicitly markdown)
- `SendNotificationConfig` only has a `type` field ("push") — no template binding
- `Organization` has no SSO configuration

This change adds proto definitions that downstream repos (pidgr-api, pidgr-admin, pidgr-mobile) will consume. All changes are additive — new messages, fields, and enum values only.

## Goals / Non-Goals

**Goals:**
- Define `UserProfile` message with standard enterprise attributes collected at enrollment or synced from SSO
- Add `TemplateType` enum to distinguish markdown from future rich/HTML templates
- Add `TemplateVariableSource` enum so variables declare whether they auto-resolve from user profiles or require manual input
- Extend `SendNotificationConfig` with template binding fields for per-step template selection
- Define `SsoAttributeMapping` for IdP claim-to-profile mapping on Organization

**Non-Goals:**
- Runtime variable resolution logic (pidgr-api concern, not proto)
- WYSIWYG editor or admin UI (pidgr-admin concern)
- Mobile onboarding screens (pidgr-mobile concern)
- Profile field validation rules beyond max lengths (API-level enforcement)
- Custom per-org profile field definitions (fixed schema with `custom_attributes` overflow map)

## Decisions

### Decision 1: UserProfile as a separate message embedded in User

**Choice**: Define `UserProfile` as its own message, added as field 9 on `User`.

**Alternatives considered**:
- Flatten profile fields directly onto `User` — rejected because `User` is used everywhere (list responses, role operations) and not all callers need profile data. A nested message allows selective population.
- Separate `UserProfile` message returned only by a new `GetUserProfile` RPC — rejected as over-engineering; embedding in `User` is simpler and lets `GetUser`/`ListUsers` optionally include it.

**Rationale**: Nested message keeps `User` backward-compatible (field 9 is simply absent in old responses) while allowing the API to populate it selectively.

### Decision 2: TemplateVariableSource enum in template.proto

**Choice**: Add `TemplateVariableSource` enum with `UNSPECIFIED`, `PROFILE`, `CUSTOM` values in `template.proto`, and add `source` + `default_value` fields to the existing `TemplateVariable` message.

**Alternatives considered**:
- Separate `ProfileVariable` and `CustomVariable` message types — rejected because it would require two repeated fields on Template and complicate the API.
- Source as a string field — rejected in favor of enum for type safety and buf lint compliance.

**Rationale**: Extending the existing `TemplateVariable` is the least disruptive change. The `source` field defaults to `UNSPECIFIED` (0) for existing templates, which the API treats as `CUSTOM` for backward compatibility.

### Decision 3: Template binding on SendNotificationConfig (not a new message)

**Choice**: Add `template_id`, `template_version`, `action_label`, `action_type`, and `custom_variables` directly to `SendNotificationConfig`.

**Alternatives considered**:
- New `StepTemplateBinding` wrapper message — rejected as unnecessary indirection for a flat set of fields.
- Template binding on `WorkflowStep` itself — rejected because only `SEND_NOTIFICATION` steps need templates; other step types (deadline_check, call_webhook) don't.

**Rationale**: `SendNotificationConfig` is the natural home. Currently it only has `type = 1`. Adding template fields to it keeps the step config type-specific.

### Decision 4: SSO attribute mapping on Organization

**Choice**: Add `SsoAttributeMapping` message and a repeated field on `Organization`, with a dedicated `UpdateSsoAttributeMappings` RPC on `OrganizationService`.

**Alternatives considered**:
- Separate `SsoConfig` message with its own service — over-engineering for a simple mapping table.
- Storing mappings in `Organization.custom_attributes` — too unstructured; mappings need typed fields.

**Rationale**: Mappings are org-level configuration. Embedding in `Organization` is simple and allows `GetOrganization` to return them. A dedicated update RPC keeps the `UpdateOrganization` RPC clean.

### Decision 5: TemplateType enum values

**Choice**: `TEMPLATE_TYPE_UNSPECIFIED (0)`, `TEMPLATE_TYPE_MARKDOWN (1)`, `TEMPLATE_TYPE_RICH (2)`, `TEMPLATE_TYPE_HTML (3)`.

**Rationale**: Only `MARKDOWN` is implemented now. `RICH` and `HTML` reserve slots for future editor types. `UNSPECIFIED` defaults to `MARKDOWN` for backward compatibility with existing templates.

### Decision 6: Field numbering strategy

- `User.profile` = field 9 (next available after `role_id = 8`)
- `TemplateVariable.source` = field 4, `TemplateVariable.default_value` = field 5 (next after `required = 3`)
- `Template.type` = field 9 (next after `title = 8`)
- `CreateTemplateRequest.type` = field 5 (next after `title = 4`)
- `ListTemplatesRequest.type` = field 2 (next after `pagination = 1`)
- `SendNotificationConfig`: `template_id = 2`, `template_version = 3`, `action_label = 4`, `action_type = 5`, `custom_variables = 6` (after `type = 1`)
- `Organization.sso_attribute_mappings` = field 7 (next after `company_size = 6`)
- `InviteUserRequest.profile` = field 5 (next after `role_id = 4`, skipping reserved 3)

## Risks / Trade-offs

- **UserProfile schema rigidity**: Fixed fields (first_name, last_name, etc.) may not cover all enterprise use cases. → Mitigated by `custom_attributes` map for overflow. Can add more typed fields in future versions.
- **TemplateVariable backward compatibility**: Existing templates have `source = UNSPECIFIED`. → API must treat `UNSPECIFIED` as `CUSTOM` to maintain existing behavior (all variables manually provided).
- **SendNotificationConfig bloat**: Adding 5 fields to a config that had 1 field. → Acceptable trade-off; these fields are logically grouped and only relevant to notification steps.
- **SSO mapping granularity**: Mapping is per-field, not per-claim-format. Complex IdP claim transformations (e.g., splitting full name into first/last) are not supported at proto level. → API handles transformation logic; proto defines the mapping contract.
