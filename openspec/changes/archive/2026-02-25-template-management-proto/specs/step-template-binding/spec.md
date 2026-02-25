## ADDED Requirements

### Requirement: Template binding fields on SendNotificationConfig
The system SHALL extend `SendNotificationConfig` in `common.proto` with fields: `template_id` (string, field 2), `template_version` (int32, field 3), `action_label` (string, field 4), `action_type` (ActionType, field 5), `custom_variables` (map<string, string>, field 6).

#### Scenario: Step with template binding
- **WHEN** a SendNotificationConfig has template_id="tpl-123" and template_version=2
- **THEN** the workflow execution SHALL use that specific template version for this step's notification

#### Scenario: Step with action configuration
- **WHEN** a SendNotificationConfig has action_label="Got it" and action_type=ACTION_TYPE_ACK
- **THEN** the rendered message for this step SHALL use that action label and type

#### Scenario: Step with custom variables
- **WHEN** a SendNotificationConfig has custom_variables with key "deadline" and value "March 15"
- **THEN** the custom_variables map SHALL contain that entry for template variable resolution

#### Scenario: Step without template binding (backward compatibility)
- **WHEN** a SendNotificationConfig has no template_id set (empty string)
- **THEN** the API SHALL fall back to the campaign-level template_id and template_version

### Requirement: SendNotificationConfig field constraints
The `template_id` field SHALL have max length 36 characters (UUID). The `action_label` field SHALL have max length 50 characters. The `custom_variables` map SHALL allow max 100 entries with key max length 100 characters and value max length 10000 characters.

#### Scenario: Action label within limits
- **WHEN** a SendNotificationConfig has action_label with 50 characters
- **THEN** the value SHALL be accepted

#### Scenario: Custom variables within limits
- **WHEN** a SendNotificationConfig has 100 custom_variables entries
- **THEN** the map SHALL be accepted
