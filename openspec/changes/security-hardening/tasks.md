## 1. Authorization Documentation (M1)

- [x] 1.1 Add `// Authorization: Requires MANAGER+ role.` comments to `CreateCampaign`, `StartCampaign`, `CancelCampaign` RPCs in `campaign.proto`
- [x] 1.2 Add `// Authorization: Authenticated user within the organization.` comments to `GetCampaign`, `ListCampaigns`, `ListDeliveries` RPCs in `campaign.proto`
- [x] 1.3 Add `// Authorization: Requires MANAGER+ role.` comments to `CreateTemplate`, `UpdateTemplate` RPCs in `template.proto`
- [x] 1.4 Add `// Authorization: Authenticated user within the organization.` comments to `GetTemplate`, `ListTemplates` RPCs in `template.proto`
- [x] 1.5 Add `// Authorization: Requires ADMIN role.` comment to `UpdateOrganization` RPC in `user_org.proto`
- [x] 1.6 Add `// Authorization: Requires ADMIN role.` comment to `InviteUser` RPC in `user_org.proto`
- [x] 1.7 Add `// Authorization: Authenticated user within the organization.` comments to `GetUser`, `ListUsers`, `GetOrganization` RPCs in `user_org.proto`
- [x] 1.8 Add ownership validation comment to `SubmitAction` RPC in `action.proto`: `// Backend MUST verify the authenticated user is the delivery recipient.`
- [x] 1.9 Add `// Authorization: Authenticated user (own devices only).` comments to `Register`, `Deactivate`, `ListDevices` RPCs in `device.proto`
- [x] 1.10 Add `// Authorization: Authenticated user (own inbox only).` comments to `Sync`, `MarkRead`, `GetMessage` RPCs in `inbox.proto`

## 2. Field Constraints — campaign.proto (M2, M3)

- [x] 2.1 Add `// Constraints: Max length 200 characters.` to `Campaign.name` and `CreateCampaignRequest.name`
- [x] 2.2 Add `// Constraints: Max 100000 items.` to `CreateCampaignRequest.user_ids`

## 3. Field Constraints — template.proto (M2)

- [x] 3.1 Add `// Constraints: Max length 100 characters.` to `Template.name`, `CreateTemplateRequest.name`
- [x] 3.2 Add `// Constraints: Max length 50000 characters.` to `Template.body`, `CreateTemplateRequest.body`, `UpdateTemplateRequest.body`
- [x] 3.3 Add `// Constraints: Max length 100 characters.` to `TemplateVariable.name`
- [x] 3.4 Add `// Constraints: Max length 500 characters.` to `TemplateVariable.description`

## 4. Field Constraints — common.proto (M2)

- [x] 4.1 Add `// Constraints: Max length 100000 characters.` to `Message.body`
- [x] 4.2 Add `// Constraints: Max length 500 characters.` to `Message.summary`
- [x] 4.3 Add `// Constraints: Max length 500 characters.` to `Message.preview`
- [x] 4.4 Add `// Constraints: Max length 200 characters.` to `Message.sender_name`
- [x] 4.5 Add `// Constraints: Max length 50 characters.` to `MessageAction.label`

## 5. Field Constraints — user_org.proto (M2)

- [x] 5.1 Add `// Constraints: Max length 254 characters (RFC 5321).` to `User.email`, `InviteUserRequest.email`, `CreateOrganizationRequest.admin_email`
- [x] 5.2 Add `// Constraints: Max length 200 characters.` to `User.name`, `InviteUserRequest.name`
- [x] 5.3 Add `// Constraints: Max length 200 characters.` to `Organization.name`, `UpdateOrganizationRequest.name`, `CreateOrganizationRequest.name`

## 6. Field Constraints — action.proto (M2)

- [x] 6.1 Add `// Constraints: Max size 10000 bytes.` to `SubmitActionRequest.payload`

## 7. Push Token Redaction — device.proto (M4)

- [x] 7.1 Add `// INTERNAL: This message is for server-side use only. Use DeviceSummary for API responses.` comment to the `Device` message
- [x] 7.2 Create `DeviceSummary` message with fields: `device_id` (string), `user_id` (string), `platform` (Platform), `active` (bool), `last_seen` (Timestamp), `created_at` (Timestamp) -- no `push_token`
- [x] 7.3 Change `RegisterResponse.device` field type from `Device` to `DeviceSummary`
- [x] 7.4 Change `ListDevicesResponse.devices` field type from `repeated Device` to `repeated DeviceSummary`
- [x] 7.5 Add `// Constraints: Max length 4096 characters.` to `RegisterRequest.push_token`

## 8. Webhook Security — common.proto (H4)

- [x] 8.1 Add SSRF prevention doc comment to `CallWebhookConfig.url`: HTTPS required in production, reject private IPs (10.0.0.0/8, 172.16.0.0/12, 192.168.0.0/16, 127.0.0.0/8, ::1), reject localhost, backend MUST validate
- [x] 8.2 Add `// Constraints: Max length 2048 characters.` to `CallWebhookConfig.url`
- [x] 8.3 Add `// Constraints: Max length 200 characters.` to `CallWebhookConfig.name`

## 9. Workflow Validation — common.proto (M5, L1)

- [x] 9.1 Add DAG validation requirement to `WorkflowDefinition` message comment
- [x] 9.2 Add `// Constraints: Max 100 steps. Backend MUST validate the graph is a DAG (no cycles).` to `WorkflowDefinition.steps`
- [x] 9.3 Add `// Constraints: Max 10 transitions per step.` to `WorkflowStep.transitions`
- [x] 9.4 Add `// Constraints: Valid range 1m to 8760h (1 year).` to `DeadlineCheckConfig.delay`
- [x] 9.5 Add `// Constraints: Accepted values: "push". Max length 50 characters.` to `SendNotificationConfig.type`
- [x] 9.6 Add `// Constraints: Accepted values: "push". Max length 50 characters.` to `SendReminderConfig.type`
- [x] 9.7 Add `// Constraints: Valid range PT1M to PT168H (1 week).` to `SendReminderConfig.repeat`
- [x] 9.8 Add `// Constraints: Valid range PT1M to PT168H (1 week).` to `SendReminderConfig.due_time`

## 10. Validation & Release

- [x] 10.1 Run `buf build` and `buf lint` to verify all changes compile
- [x] 10.2 Run `buf generate` to regenerate Go, Rust, and TypeScript stubs
- [x] 10.3 Run `buf breaking --against .git#branch=main` and confirm only the expected `DeviceSummary` breaking change is reported
- [x] 10.4 Verify `docs/index.md` reflects updated comments and new `DeviceSummary` message
- [x] 10.5 Bump version for minor release (breaking change: DeviceSummary)
