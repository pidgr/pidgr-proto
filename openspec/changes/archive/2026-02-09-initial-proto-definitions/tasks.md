## 1. Buf Tooling Setup

- [x] 1.1 Create `proto/pidgr/v1/` directory structure
- [x] 1.2 Create `buf.yaml` with module config, DEFAULT lint rules, and breaking change detection against main branch
- [x] 1.3 Create `buf.gen.yaml` with codegen targets for Go (`gen/go/`), Rust (`gen/rust/`), and TypeScript (`gen/ts/`)
- [x] 1.4 Create `buf.lock` by running `buf dep update` to resolve `google/protobuf` well-known types
- [x] 1.5 Verify `buf build` compiles with zero errors

## 2. Common Types

- [x] 2.1 Create `proto/pidgr/v1/common.proto` with package declaration, proto3 syntax, and google.protobuf.Timestamp import
- [x] 2.2 Define enums: CampaignStatus, DeliveryStatus, Platform, UserRole, UserStatus, ActionType, StepType (all with UNSPECIFIED at 0)
- [x] 2.3 Define Pagination and PaginationMeta messages
- [x] 2.4 Define MessageAction message (id, type, label)
- [x] 2.5 Define canonical Message message (content_id, campaign_id, sender_name, summary, preview, body, critical, actions, created_at)
- [x] 2.6 Define WorkflowDefinition and WorkflowStep messages (id, type, oneof config, transitions map)
- [x] 2.7 Define step config messages: SendNotificationConfig, WaitActionConfig, SendReminderConfig, CallWebhookConfig
- [x] 2.8 Verify `buf lint` passes on common.proto

## 3. Campaign Service

- [x] 3.1 Create `proto/pidgr/v1/campaign.proto` importing common.proto
- [x] 3.2 Define Campaign message (id, name, template_id, template_version, audience_snapshot_ref, status, workflow, aggregated counts, timestamps)
- [x] 3.3 Define CreateCampaignRequest/Response, StartCampaignRequest/Response, GetCampaignRequest/Response, ListCampaignsRequest/Response
- [x] 3.4 Define CampaignService with all four RPCs
- [x] 3.5 Verify `buf lint` passes

## 4. Template Service

- [x] 4.1 Create `proto/pidgr/v1/template.proto` importing common.proto
- [x] 4.2 Define Template and TemplateVariable messages
- [x] 4.3 Define CreateTemplateRequest/Response, UpdateTemplateRequest/Response, GetTemplateRequest/Response, ListTemplatesRequest/Response
- [x] 4.4 Define TemplateService with all four RPCs
- [x] 4.5 Verify `buf lint` passes

## 5. Action Service

- [x] 5.1 Create `proto/pidgr/v1/action.proto` importing common.proto
- [x] 5.2 Define SubmitActionRequest (delivery_id, action_id, payload bytes) and SubmitActionResponse (success)
- [x] 5.3 Define ActionService with SubmitAction RPC
- [x] 5.4 Verify `buf lint` passes

## 6. Inbox Service

- [x] 6.1 Create `proto/pidgr/v1/inbox.proto` importing common.proto
- [x] 6.2 Define InboxEntry message (delivery_id, message, status, read, received_at)
- [x] 6.3 Define SyncInboxRequest/Response and MarkReadRequest/Response
- [x] 6.4 Define InboxService with Sync and MarkRead RPCs
- [x] 6.5 Verify `buf lint` passes

## 7. Device Service

- [x] 7.1 Create `proto/pidgr/v1/device.proto` importing common.proto
- [x] 7.2 Define Device message
- [x] 7.3 Define RegisterDeviceRequest/Response, DeactivateDeviceRequest/Response, ListDevicesRequest/Response
- [x] 7.4 Define DeviceService with Register, Deactivate, ListDevices RPCs
- [x] 7.5 Verify `buf lint` passes

## 8. User & Org Service

- [x] 8.1 Create `proto/pidgr/v1/user_org.proto` importing common.proto
- [x] 8.2 Define User and Organization messages (Organization includes default_workflow)
- [x] 8.3 Define InviteUserRequest/Response, GetUserRequest/Response, ListUsersRequest/Response, GetOrganizationRequest/Response, UpdateOrganizationRequest/Response
- [x] 8.4 Define UserOrgService with all five RPCs
- [x] 8.5 Verify `buf lint` passes

## 9. Render Service (Internal)

- [x] 9.1 Create `proto/pidgr/v1/render.proto` importing common.proto
- [x] 9.2 Define RenderBatchRequest, UserRenderContext, and RenderResult messages (RenderResult uses canonical Message)
- [x] 9.3 Define RenderService with server-streaming RenderBatch RPC
- [x] 9.4 Verify `buf lint` passes

## 10. Final Validation

- [x] 10.1 Run `buf build` — all proto files compile with zero errors
- [x] 10.2 Run `buf lint` — all files pass DEFAULT lint rules
- [x] 10.3 Run `buf generate` — Go, Rust, and TypeScript stubs generate successfully
- [x] 10.4 Verify cross-file imports resolve correctly (all services importing common.proto)
