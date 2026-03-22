## MODIFIED Requirements

### Requirement: TemplateVariable message includes PII flag
The TemplateVariable message SHALL include a `bool pii` field (field number 6) that indicates whether the variable's rendered value contains personally identifiable information.

#### Scenario: PII flag is additive
- **WHEN** `buf breaking --against .git#branch=main` is run
- **THEN** no breaking changes SHALL be detected (field 6 is a new optional bool, backward compatible)

#### Scenario: PII flag defaults to false
- **WHEN** a TemplateVariable is deserialized without a pii field (from older clients)
- **THEN** pii SHALL default to false (proto3 default for bool)

#### Scenario: PII flag documentation
- **WHEN** the field is reviewed
- **THEN** it SHALL have a doc comment: "When true, this variable's rendered value is masked in session replay and heatmap screenshots. Org admin controls per variable."

#### Scenario: Full message definition
- **WHEN** TemplateVariable is reviewed
- **THEN** it SHALL contain fields: name (1), description (2), required (3), source (4), default_value (5), pii (6)
