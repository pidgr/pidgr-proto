## ADDED Requirements

### Requirement: TemplateType enum in common scope
The `TemplateType` enum SHALL be defined in `template.proto` (not `common.proto`) since it is specific to the template domain. It is referenced here for cross-cutting awareness. See `template-service` spec for the full definition.

#### Scenario: TemplateType available to downstream consumers
- **WHEN** code is generated from proto definitions
- **THEN** Go, TypeScript, and Rust stubs SHALL include the TemplateType enum

### Requirement: TemplateVariableSource enum in template scope
The `TemplateVariableSource` enum SHALL be defined in `template.proto` (not `common.proto`) since it is specific to the template variable domain. It is referenced here for cross-cutting awareness. See `template-variable-sources` spec for the full definition.

#### Scenario: TemplateVariableSource available to downstream consumers
- **WHEN** code is generated from proto definitions
- **THEN** Go, TypeScript, and Rust stubs SHALL include the TemplateVariableSource enum

## MODIFIED Requirements

### Requirement: SendNotificationConfig type
The system SHALL define SendNotificationConfig with fields: type (string, e.g., "push"), template_id (string, field 2), template_version (int32, field 3), action_label (string, field 4), action_type (ActionType, field 5), custom_variables (map<string, string>, field 6).

#### Scenario: Initial notification step
- **WHEN** a workflow begins with a send notification step
- **THEN** the config SHALL specify the notification type

#### Scenario: Notification step with template binding
- **WHEN** a workflow step uses a specific template
- **THEN** the SendNotificationConfig SHALL carry the template_id and template_version for that step

#### Scenario: Notification step with action override
- **WHEN** a workflow step overrides the default action
- **THEN** the SendNotificationConfig SHALL carry the action_label and action_type for that step

#### Scenario: Notification step with custom variables
- **WHEN** a workflow step provides campaign-specific variable values
- **THEN** the SendNotificationConfig SHALL carry the custom_variables map for template resolution
