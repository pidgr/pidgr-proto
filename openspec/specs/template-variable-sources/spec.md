## Requirements

### Requirement: TemplateVariableSource enum
The system SHALL define a `TemplateVariableSource` enum in `template.proto` with values: `TEMPLATE_VARIABLE_SOURCE_UNSPECIFIED` (0), `TEMPLATE_VARIABLE_SOURCE_PROFILE` (1), `TEMPLATE_VARIABLE_SOURCE_CUSTOM` (2).

#### Scenario: Profile-sourced variable
- **WHEN** a TemplateVariable has source=TEMPLATE_VARIABLE_SOURCE_PROFILE
- **THEN** the variable SHALL be understood as auto-resolvable from the user's profile at render time

#### Scenario: Custom-sourced variable
- **WHEN** a TemplateVariable has source=TEMPLATE_VARIABLE_SOURCE_CUSTOM
- **THEN** the variable SHALL be understood as requiring a value provided in the campaign/step configuration

#### Scenario: Unspecified source (backward compatibility)
- **WHEN** a TemplateVariable has source=TEMPLATE_VARIABLE_SOURCE_UNSPECIFIED
- **THEN** the API SHALL treat it as CUSTOM for backward compatibility with existing templates

#### Scenario: Enum follows naming convention
- **WHEN** the proto file is compiled
- **THEN** all enum values SHALL be prefixed with `TEMPLATE_VARIABLE_SOURCE_` per project conventions
