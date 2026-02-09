## ADDED Requirements

### Requirement: TemplateService proto definition
The system SHALL define a TemplateService in `pidgr/v1/template.proto` with RPCs: CreateTemplate, UpdateTemplate, GetTemplate, ListTemplates.

#### Scenario: Service definition exists
- **WHEN** the proto file is compiled
- **THEN** it SHALL produce a valid TemplateService with all four RPCs

### Requirement: Template message type
The system SHALL define a Template message with fields: id (string), name (string), body (string, Markdown with variable placeholders), variables (repeated TemplateVariable), version (int32), created_at (Timestamp), updated_at (Timestamp).

#### Scenario: Template with variables
- **WHEN** a template body contains `{{name}}` and `{{date}}`
- **THEN** the variables field SHALL contain TemplateVariable entries for "name" and "date"

### Requirement: TemplateVariable message type
The system SHALL define a TemplateVariable message with fields: name (string), description (string), required (bool).

#### Scenario: Required variable
- **WHEN** a template variable is marked as required=true
- **THEN** the render service SHALL fail for any user missing that variable in their render context

### Requirement: CreateTemplate RPC
The system SHALL define CreateTemplate(CreateTemplateRequest) returning CreateTemplateResponse. CreateTemplateRequest SHALL contain: name (string), body (string), variables (repeated TemplateVariable). CreateTemplateResponse SHALL contain the created Template with version=1.

#### Scenario: Create a new template
- **WHEN** a client calls CreateTemplate with a valid name and body
- **THEN** the response SHALL contain a Template with version 1

### Requirement: UpdateTemplate RPC
The system SHALL define UpdateTemplate(UpdateTemplateRequest) returning UpdateTemplateResponse. UpdateTemplateRequest SHALL contain: template_id (string), body (string), variables (repeated TemplateVariable). UpdateTemplateResponse SHALL contain the updated Template with an incremented version number. Updates SHALL be append-only — existing versions are never mutated.

#### Scenario: Update an existing template
- **WHEN** a client calls UpdateTemplate on a template at version 2
- **THEN** the response SHALL contain a Template with version 3

#### Scenario: Previous versions remain accessible
- **WHEN** a template is updated from version 1 to version 2
- **THEN** GetTemplate with version=1 SHALL still return the original content

### Requirement: GetTemplate RPC
The system SHALL define GetTemplate(GetTemplateRequest) returning GetTemplateResponse. GetTemplateRequest SHALL contain: template_id (string), version (int32). GetTemplateResponse SHALL contain the Template.

#### Scenario: Get latest version
- **WHEN** a client calls GetTemplate with version=0
- **THEN** the response SHALL return the most recent version of the template

#### Scenario: Get specific version
- **WHEN** a client calls GetTemplate with version=2
- **THEN** the response SHALL return exactly version 2 of the template

#### Scenario: Get non-existent version
- **WHEN** a client calls GetTemplate with a version that does not exist
- **THEN** the server SHALL return a NOT_FOUND error

### Requirement: ListTemplates RPC
The system SHALL define ListTemplates(ListTemplatesRequest) returning ListTemplatesResponse. ListTemplatesRequest SHALL contain: pagination (Pagination). ListTemplatesResponse SHALL contain: templates (repeated Template, latest version only), pagination_meta (PaginationMeta).

#### Scenario: List returns latest versions only
- **WHEN** a client calls ListTemplates
- **THEN** each template in the response SHALL be the latest version of that template
