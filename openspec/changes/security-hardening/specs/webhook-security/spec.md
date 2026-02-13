## ADDED Requirements

### Requirement: SSRF prevention documentation on CallWebhookConfig.url
The `CallWebhookConfig.url` field SHALL have doc comments documenting SSRF prevention requirements that the backend MUST enforce.

#### Scenario: HTTPS required in production
- **WHEN** a developer reads the `url` field comment on `CallWebhookConfig`
- **THEN** it SHALL state that HTTPS is required in production environments

#### Scenario: Private IP ranges rejected
- **WHEN** a developer reads the `url` field comment on `CallWebhookConfig`
- **THEN** it SHALL list the private IP ranges that must be rejected: 10.0.0.0/8, 172.16.0.0/12, 192.168.0.0/16, 127.0.0.0/8, ::1

#### Scenario: Localhost rejected
- **WHEN** a developer reads the `url` field comment on `CallWebhookConfig`
- **THEN** it SHALL state that localhost URLs must be rejected in production

#### Scenario: Backend validation requirement documented
- **WHEN** a developer reads the `url` field comment on `CallWebhookConfig`
- **THEN** it SHALL state that the backend MUST validate the URL before executing the webhook request

### Requirement: Webhook name and URL max-length constraints
The `CallWebhookConfig` string fields SHALL have `// Constraints:` doc comments specifying maximum character lengths.

#### Scenario: Webhook name has max length
- **WHEN** a developer reads the `name` field comment on `CallWebhookConfig`
- **THEN** it SHALL state `Constraints: Max length 200 characters.`

#### Scenario: Webhook URL has max length
- **WHEN** a developer reads the `url` field comment on `CallWebhookConfig`
- **THEN** it SHALL state `Constraints: Max length 2048 characters.`
