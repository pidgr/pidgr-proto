## ADDED Requirements

### Requirement: TemplateType enum
The system SHALL define a `TemplateType` enum in `template.proto` with values: `TEMPLATE_TYPE_UNSPECIFIED` (0), `TEMPLATE_TYPE_MARKDOWN` (1), `TEMPLATE_TYPE_RICH` (2), `TEMPLATE_TYPE_HTML` (3).

#### Scenario: Markdown template
- **WHEN** a Template has type=TEMPLATE_TYPE_MARKDOWN
- **THEN** the template body SHALL be interpreted as markdown with variable placeholders

#### Scenario: Unspecified type (backward compatibility)
- **WHEN** a Template has type=TEMPLATE_TYPE_UNSPECIFIED
- **THEN** the API SHALL treat it as TEMPLATE_TYPE_MARKDOWN for backward compatibility

#### Scenario: Enum follows naming convention
- **WHEN** the proto file is compiled
- **THEN** all enum values SHALL be prefixed with `TEMPLATE_TYPE_` per project conventions

## MODIFIED Requirements

### Requirement: TemplateVariable message type
The system SHALL define a TemplateVariable message with fields: name (string), description (string), required (bool), source (TemplateVariableSource, field 4), default_value (string, field 5).

#### Scenario: Required variable
- **WHEN** a template variable is marked as required=true
- **THEN** the render service SHALL fail for any user missing that variable in their render context

#### Scenario: Profile-sourced variable with default
- **WHEN** a TemplateVariable has source=PROFILE, name="department", and default_value="General"
- **THEN** the variable SHALL auto-resolve from user profile, falling back to "General" if the profile field is empty

#### Scenario: Custom variable with default
- **WHEN** a TemplateVariable has source=CUSTOM, name="deadline", and default_value="TBD"
- **THEN** the variable SHALL use the campaign/step-provided value, falling back to "TBD" if not provided

#### Scenario: Backward compatibility with existing templates
- **WHEN** a TemplateVariable has no source field set (UNSPECIFIED) and no default_value
- **THEN** the behavior SHALL be identical to pre-change behavior (treated as CUSTOM, required must be manually satisfied)

### Requirement: Template message type
The system SHALL define a Template message with fields: id (string), name (string), body (string, Markdown with variable placeholders), variables (repeated TemplateVariable), version (int32), created_at (Timestamp), updated_at (Timestamp), title (string), type (TemplateType, field 9).

#### Scenario: Template with variables
- **WHEN** a template body contains `{{name}}` and `{{date}}`
- **THEN** the variables field SHALL contain TemplateVariable entries for "name" and "date"

#### Scenario: Template with type
- **WHEN** a Template has type=TEMPLATE_TYPE_MARKDOWN
- **THEN** the type field SHALL indicate the template content format

#### Scenario: Existing template without type
- **WHEN** a Template was created before the type field was added
- **THEN** the type field SHALL default to TEMPLATE_TYPE_UNSPECIFIED (treated as MARKDOWN)

### Requirement: CreateTemplate RPC
The system SHALL define CreateTemplate(CreateTemplateRequest) returning CreateTemplateResponse. CreateTemplateRequest SHALL contain: name (string), body (string), variables (repeated TemplateVariable), title (string), type (TemplateType, field 5). CreateTemplateResponse SHALL contain the created Template with version=1.

#### Scenario: Create a new template
- **WHEN** a client calls CreateTemplate with a valid name and body
- **THEN** the response SHALL contain a Template with version 1

#### Scenario: Create template with type
- **WHEN** a client calls CreateTemplate with type=TEMPLATE_TYPE_MARKDOWN
- **THEN** the response SHALL contain a Template with type=TEMPLATE_TYPE_MARKDOWN

#### Scenario: Create template without type
- **WHEN** a client calls CreateTemplate without setting the type field
- **THEN** the response SHALL contain a Template with type=TEMPLATE_TYPE_UNSPECIFIED (API defaults to MARKDOWN)

### Requirement: ListTemplates RPC
The system SHALL define ListTemplates(ListTemplatesRequest) returning ListTemplatesResponse. ListTemplatesRequest SHALL contain: pagination (Pagination), type (TemplateType, field 2). ListTemplatesResponse SHALL contain: templates (repeated Template, latest version only), pagination_meta (PaginationMeta).

#### Scenario: List returns latest versions only
- **WHEN** a client calls ListTemplates
- **THEN** each template in the response SHALL be the latest version of that template

#### Scenario: Filter by type
- **WHEN** a client calls ListTemplates with type=TEMPLATE_TYPE_MARKDOWN
- **THEN** the response SHALL only contain templates with type MARKDOWN (or UNSPECIFIED, treated as MARKDOWN)

#### Scenario: No type filter
- **WHEN** a client calls ListTemplates with type=TEMPLATE_TYPE_UNSPECIFIED
- **THEN** the response SHALL contain all templates regardless of type
