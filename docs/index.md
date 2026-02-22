# Protocol Documentation
<a name="top"></a>

## Table of Contents

- [pidgr/v1/action.proto](#pidgr_v1_action-proto)
    - [SubmitActionRequest](#pidgr-v1-SubmitActionRequest)
    - [SubmitActionResponse](#pidgr-v1-SubmitActionResponse)
  
    - [ActionService](#pidgr-v1-ActionService)
  
- [pidgr/v1/common.proto](#pidgr_v1_common-proto)
    - [CallWebhookConfig](#pidgr-v1-CallWebhookConfig)
    - [CallWebhookConfig.HeadersEntry](#pidgr-v1-CallWebhookConfig-HeadersEntry)
    - [DeadlineCheckConfig](#pidgr-v1-DeadlineCheckConfig)
    - [Message](#pidgr-v1-Message)
    - [MessageAction](#pidgr-v1-MessageAction)
    - [Pagination](#pidgr-v1-Pagination)
    - [PaginationMeta](#pidgr-v1-PaginationMeta)
    - [Role](#pidgr-v1-Role)
    - [SendNotificationConfig](#pidgr-v1-SendNotificationConfig)
    - [SendReminderConfig](#pidgr-v1-SendReminderConfig)
    - [User](#pidgr-v1-User)
    - [WorkflowDefinition](#pidgr-v1-WorkflowDefinition)
    - [WorkflowStep](#pidgr-v1-WorkflowStep)
    - [WorkflowStep.TransitionsEntry](#pidgr-v1-WorkflowStep-TransitionsEntry)
  
    - [ActionType](#pidgr-v1-ActionType)
    - [CampaignStatus](#pidgr-v1-CampaignStatus)
    - [DeliveryStatus](#pidgr-v1-DeliveryStatus)
    - [Permission](#pidgr-v1-Permission)
    - [Platform](#pidgr-v1-Platform)
    - [StepType](#pidgr-v1-StepType)
    - [UserStatus](#pidgr-v1-UserStatus)
  
- [pidgr/v1/campaign.proto](#pidgr_v1_campaign-proto)
    - [Campaign](#pidgr-v1-Campaign)
    - [CancelCampaignRequest](#pidgr-v1-CancelCampaignRequest)
    - [CancelCampaignResponse](#pidgr-v1-CancelCampaignResponse)
    - [CreateCampaignRequest](#pidgr-v1-CreateCampaignRequest)
    - [CreateCampaignResponse](#pidgr-v1-CreateCampaignResponse)
    - [Delivery](#pidgr-v1-Delivery)
    - [GetCampaignRequest](#pidgr-v1-GetCampaignRequest)
    - [GetCampaignResponse](#pidgr-v1-GetCampaignResponse)
    - [ListCampaignsRequest](#pidgr-v1-ListCampaignsRequest)
    - [ListCampaignsResponse](#pidgr-v1-ListCampaignsResponse)
    - [ListDeliveriesRequest](#pidgr-v1-ListDeliveriesRequest)
    - [ListDeliveriesResponse](#pidgr-v1-ListDeliveriesResponse)
    - [StartCampaignRequest](#pidgr-v1-StartCampaignRequest)
    - [StartCampaignResponse](#pidgr-v1-StartCampaignResponse)
  
    - [CampaignService](#pidgr-v1-CampaignService)
  
- [pidgr/v1/device.proto](#pidgr_v1_device-proto)
    - [DeactivateRequest](#pidgr-v1-DeactivateRequest)
    - [DeactivateResponse](#pidgr-v1-DeactivateResponse)
    - [Device](#pidgr-v1-Device)
    - [DeviceSummary](#pidgr-v1-DeviceSummary)
    - [ListDevicesRequest](#pidgr-v1-ListDevicesRequest)
    - [ListDevicesResponse](#pidgr-v1-ListDevicesResponse)
    - [RegisterRequest](#pidgr-v1-RegisterRequest)
    - [RegisterResponse](#pidgr-v1-RegisterResponse)
  
    - [DeviceService](#pidgr-v1-DeviceService)
  
- [pidgr/v1/heatmap.proto](#pidgr_v1_heatmap-proto)
    - [HeatmapDataPoint](#pidgr-v1-HeatmapDataPoint)
    - [IngestTouchEventsRequest](#pidgr-v1-IngestTouchEventsRequest)
    - [IngestTouchEventsResponse](#pidgr-v1-IngestTouchEventsResponse)
    - [ListScreenshotsRequest](#pidgr-v1-ListScreenshotsRequest)
    - [ListScreenshotsResponse](#pidgr-v1-ListScreenshotsResponse)
    - [QueryHeatmapDataRequest](#pidgr-v1-QueryHeatmapDataRequest)
    - [QueryHeatmapDataResponse](#pidgr-v1-QueryHeatmapDataResponse)
    - [ScreenScreenshot](#pidgr-v1-ScreenScreenshot)
    - [TouchEvent](#pidgr-v1-TouchEvent)
    - [UserTouchCount](#pidgr-v1-UserTouchCount)
  
    - [HeatmapMode](#pidgr-v1-HeatmapMode)
    - [TouchEventType](#pidgr-v1-TouchEventType)
  
    - [HeatmapService](#pidgr-v1-HeatmapService)
  
- [pidgr/v1/inbox.proto](#pidgr_v1_inbox-proto)
    - [GetMessageRequest](#pidgr-v1-GetMessageRequest)
    - [GetMessageResponse](#pidgr-v1-GetMessageResponse)
    - [InboxEntry](#pidgr-v1-InboxEntry)
    - [MarkReadRequest](#pidgr-v1-MarkReadRequest)
    - [MarkReadResponse](#pidgr-v1-MarkReadResponse)
    - [SyncRequest](#pidgr-v1-SyncRequest)
    - [SyncResponse](#pidgr-v1-SyncResponse)
  
    - [InboxService](#pidgr-v1-InboxService)
  
- [pidgr/v1/member.proto](#pidgr_v1_member-proto)
    - [DeactivateUserRequest](#pidgr-v1-DeactivateUserRequest)
    - [DeactivateUserResponse](#pidgr-v1-DeactivateUserResponse)
    - [GetUserRequest](#pidgr-v1-GetUserRequest)
    - [GetUserResponse](#pidgr-v1-GetUserResponse)
    - [InviteUserRequest](#pidgr-v1-InviteUserRequest)
    - [InviteUserResponse](#pidgr-v1-InviteUserResponse)
    - [ListUsersRequest](#pidgr-v1-ListUsersRequest)
    - [ListUsersResponse](#pidgr-v1-ListUsersResponse)
    - [UpdateUserRoleRequest](#pidgr-v1-UpdateUserRoleRequest)
    - [UpdateUserRoleResponse](#pidgr-v1-UpdateUserRoleResponse)
  
    - [MemberService](#pidgr-v1-MemberService)
  
- [pidgr/v1/organization.proto](#pidgr_v1_organization-proto)
    - [CreateOrganizationRequest](#pidgr-v1-CreateOrganizationRequest)
    - [CreateOrganizationResponse](#pidgr-v1-CreateOrganizationResponse)
    - [GetOrganizationRequest](#pidgr-v1-GetOrganizationRequest)
    - [GetOrganizationResponse](#pidgr-v1-GetOrganizationResponse)
    - [Organization](#pidgr-v1-Organization)
    - [UpdateOrganizationRequest](#pidgr-v1-UpdateOrganizationRequest)
    - [UpdateOrganizationResponse](#pidgr-v1-UpdateOrganizationResponse)
  
    - [CompanySize](#pidgr-v1-CompanySize)
    - [Industry](#pidgr-v1-Industry)
  
    - [OrganizationService](#pidgr-v1-OrganizationService)
  
- [pidgr/v1/render.proto](#pidgr_v1_render-proto)
    - [RenderBatchRequest](#pidgr-v1-RenderBatchRequest)
    - [RenderBatchResponse](#pidgr-v1-RenderBatchResponse)
    - [UserRenderContext](#pidgr-v1-UserRenderContext)
    - [UserRenderContext.VariablesEntry](#pidgr-v1-UserRenderContext-VariablesEntry)
  
    - [RenderService](#pidgr-v1-RenderService)
  
- [pidgr/v1/replay.proto](#pidgr_v1_replay-proto)
    - [GetSessionSnapshotsRequest](#pidgr-v1-GetSessionSnapshotsRequest)
    - [GetSessionSnapshotsResponse](#pidgr-v1-GetSessionSnapshotsResponse)
    - [ListSessionRecordingsRequest](#pidgr-v1-ListSessionRecordingsRequest)
    - [ListSessionRecordingsResponse](#pidgr-v1-ListSessionRecordingsResponse)
    - [SessionRecording](#pidgr-v1-SessionRecording)
  
    - [ReplayService](#pidgr-v1-ReplayService)
  
- [pidgr/v1/role.proto](#pidgr_v1_role-proto)
    - [CreateRoleRequest](#pidgr-v1-CreateRoleRequest)
    - [CreateRoleResponse](#pidgr-v1-CreateRoleResponse)
    - [DeleteRoleRequest](#pidgr-v1-DeleteRoleRequest)
    - [DeleteRoleResponse](#pidgr-v1-DeleteRoleResponse)
    - [ListRolesRequest](#pidgr-v1-ListRolesRequest)
    - [ListRolesResponse](#pidgr-v1-ListRolesResponse)
    - [UpdateRoleRequest](#pidgr-v1-UpdateRoleRequest)
    - [UpdateRoleResponse](#pidgr-v1-UpdateRoleResponse)
  
    - [RoleService](#pidgr-v1-RoleService)
  
- [pidgr/v1/sso.proto](#pidgr_v1_sso-proto)
    - [CheckSSOByDomainRequest](#pidgr-v1-CheckSSOByDomainRequest)
    - [CheckSSOByDomainResponse](#pidgr-v1-CheckSSOByDomainResponse)
    - [CreateSSOProviderRequest](#pidgr-v1-CreateSSOProviderRequest)
    - [CreateSSOProviderResponse](#pidgr-v1-CreateSSOProviderResponse)
    - [DeleteSSOProviderRequest](#pidgr-v1-DeleteSSOProviderRequest)
    - [DeleteSSOProviderResponse](#pidgr-v1-DeleteSSOProviderResponse)
    - [GetSSOProviderRequest](#pidgr-v1-GetSSOProviderRequest)
    - [GetSSOProviderResponse](#pidgr-v1-GetSSOProviderResponse)
    - [SSOAttributeMapping](#pidgr-v1-SSOAttributeMapping)
    - [SSOProvider](#pidgr-v1-SSOProvider)
  
    - [SSOProviderType](#pidgr-v1-SSOProviderType)
  
    - [SSOService](#pidgr-v1-SSOService)
  
- [pidgr/v1/template.proto](#pidgr_v1_template-proto)
    - [CreateTemplateRequest](#pidgr-v1-CreateTemplateRequest)
    - [CreateTemplateResponse](#pidgr-v1-CreateTemplateResponse)
    - [GetTemplateRequest](#pidgr-v1-GetTemplateRequest)
    - [GetTemplateResponse](#pidgr-v1-GetTemplateResponse)
    - [ListTemplatesRequest](#pidgr-v1-ListTemplatesRequest)
    - [ListTemplatesResponse](#pidgr-v1-ListTemplatesResponse)
    - [Template](#pidgr-v1-Template)
    - [TemplateVariable](#pidgr-v1-TemplateVariable)
    - [UpdateTemplateRequest](#pidgr-v1-UpdateTemplateRequest)
    - [UpdateTemplateResponse](#pidgr-v1-UpdateTemplateResponse)
  
    - [TemplateService](#pidgr-v1-TemplateService)
  
- [Scalar Value Types](#scalar-value-types)



<a name="pidgr_v1_action-proto"></a>
<p align="right"><a href="#top">Top</a></p>

## pidgr/v1/action.proto



<a name="pidgr-v1-SubmitActionRequest"></a>

### SubmitActionRequest
Request to submit a user action on a delivered message.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| delivery_id | [string](#string) |  | ID of the delivery the user is acting on. Constraints: UUID format (36 characters). |
| action_id | [string](#string) |  | ID of the action being performed (matches MessageAction.id). Constraints: Max length 100 characters. |
| payload | [bytes](#bytes) |  | Optional action-specific payload (e.g. poll response data). Empty for ACK. Constraints: Max size 10000 bytes. |






<a name="pidgr-v1-SubmitActionResponse"></a>

### SubmitActionResponse
Response after submitting an action.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| success | [bool](#bool) |  | Whether the action was successfully recorded and forwarded to the workflow. |





 

 

 


<a name="pidgr-v1-ActionService"></a>

### ActionService
Handles user actions on delivered messages.
Actions drive Temporal workflow progression (e.g. ACK completes a wait step).

| Method Name | Request Type | Response Type | Description |
| ----------- | ------------ | ------------- | ------------|
| SubmitAction | [SubmitActionRequest](#pidgr-v1-SubmitActionRequest) | [SubmitActionResponse](#pidgr-v1-SubmitActionResponse) | Submit an action for a specific delivery, advancing the campaign workflow. Backend MUST verify the authenticated user is the delivery recipient. |

 



<a name="pidgr_v1_common-proto"></a>
<p align="right"><a href="#top">Top</a></p>

## pidgr/v1/common.proto



<a name="pidgr-v1-CallWebhookConfig"></a>

### CallWebhookConfig
Configuration for a step that calls an external webhook.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| name | [string](#string) |  | Human-readable name for this webhook (for logging/display). Constraints: Max length 200 characters. |
| url | [string](#string) |  | URL to POST campaign context to. Constraints: Max length 2048 characters. Security: HTTPS required in production. Backend MUST reject private IPs (10.0.0.0/8, 172.16.0.0/12, 192.168.0.0/16, 127.0.0.0/8, ::1) and localhost to prevent SSRF attacks. Backend MUST validate before executing. |
| headers | [CallWebhookConfig.HeadersEntry](#pidgr-v1-CallWebhookConfig-HeadersEntry) | repeated | Additional HTTP headers to include in the webhook request. Constraints: Max 20 entries. Key max length 200 characters, value max length 2000 characters. |






<a name="pidgr-v1-CallWebhookConfig-HeadersEntry"></a>

### CallWebhookConfig.HeadersEntry



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| key | [string](#string) |  |  |
| value | [string](#string) |  |  |






<a name="pidgr-v1-DeadlineCheckConfig"></a>

### DeadlineCheckConfig
Configuration for a deadline-based timer step that sleeps for a configured
delay before proceeding. Acknowledgments happen independently at the delivery
level and are evaluated by subsequent steps (e.g. SEND_REMINDER).


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| delay | [string](#string) |  | Go duration string for the deadline delay (e.g. &#34;120h&#34;, &#34;72h&#34;). Constraints: Valid range 1m to 8760h (1 year). |






<a name="pidgr-v1-Message"></a>

### Message
Canonical message type used across rendering, inbox, and delivery.
Represents the fully rendered content delivered to a recipient.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| content_id | [string](#string) |  | SHA-256 hash of the rendered content, used as a content-addressable ID. |
| campaign_id | [string](#string) |  | ID of the campaign this message belongs to. |
| sender_name | [string](#string) |  | Display name of the sender (e.g. organization or campaign name). Constraints: Max length 200 characters. |
| summary | [string](#string) |  | Short one-line summary shown in notification banners. Constraints: Max length 500 characters. |
| preview | [string](#string) |  | Preview text shown in inbox list views. Constraints: Max length 500 characters. |
| body | [string](#string) |  | Full message body content. Constraints: Max length 100000 characters. |
| critical | [bool](#bool) |  | Whether this message requires immediate attention from the recipient. |
| actions | [MessageAction](#pidgr-v1-MessageAction) | repeated | Actions available to the recipient (e.g. acknowledge button). |
| created_at | [google.protobuf.Timestamp](#google-protobuf-Timestamp) |  | Timestamp when the message was created. |
| title | [string](#string) |  | User-facing title of the message (resolved from campaign or template). Constraints: Max length 200 characters. |






<a name="pidgr-v1-MessageAction"></a>

### MessageAction
An action button attached to a message that a recipient can interact with.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| id | [string](#string) |  | Unique identifier for this action within the message. |
| type | [ActionType](#pidgr-v1-ActionType) |  | The type of action (e.g. ACK). |
| label | [string](#string) |  | Display label shown to the recipient (e.g. &#34;Got it&#34;). Constraints: Max length 50 characters. |






<a name="pidgr-v1-Pagination"></a>

### Pagination
Cursor-based pagination parameters for list requests.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| page_size | [int32](#int32) |  | Maximum number of items to return per page. |
| page_token | [string](#string) |  | Opaque token from a previous response to fetch the next page. |






<a name="pidgr-v1-PaginationMeta"></a>

### PaginationMeta
Pagination metadata returned alongside list responses.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| next_page_token | [string](#string) |  | Token to pass in the next request to get the following page. Empty if no more pages. |
| total_count | [int32](#int32) |  | Total number of items matching the query (across all pages). |






<a name="pidgr-v1-Role"></a>

### Role
A named role within an organization with a set of permissions.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| id | [string](#string) |  | Unique identifier for the role. |
| slug | [string](#string) |  | URL-safe slug (unique within the organization, e.g. &#34;admin&#34;, &#34;manager&#34;). |
| name | [string](#string) |  | Human-readable display name. |
| is_default | [bool](#bool) |  | Whether this role was seeded by the system on organization creation. |
| permissions | [Permission](#pidgr-v1-Permission) | repeated | Permissions granted to users with this role. |
| is_system | [bool](#bool) |  | Whether this role is system-managed and immutable (e.g. super_admin). |






<a name="pidgr-v1-SendNotificationConfig"></a>

### SendNotificationConfig
Configuration for a step that sends the initial push notification.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| type | [string](#string) |  | Notification delivery type (e.g. &#34;push&#34;). Constraints: Accepted values: &#34;push&#34;. Max length 50 characters. |






<a name="pidgr-v1-SendReminderConfig"></a>

### SendReminderConfig
Configuration for a step that sends reminders to non-responsive recipients.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| type | [string](#string) |  | Reminder delivery type (e.g. &#34;push&#34;). Constraints: Accepted values: &#34;push&#34;. Max length 50 characters. |
| repeat | [string](#string) |  | ISO 8601 repeat interval between reminders (e.g. &#34;PT8H&#34;). Constraints: Valid range PT1M to PT168H (1 week). |
| due_time | [string](#string) |  | ISO 8601 duration after which reminders stop (e.g. &#34;PT24H&#34;). Constraints: Valid range PT1M to PT168H (1 week). |






<a name="pidgr-v1-User"></a>

### User
A user within an organization.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| id | [string](#string) |  | Unique identifier for the user (internal platform UUID, not Cognito sub). |
| email | [string](#string) |  | User&#39;s email address. Constraints: Max length 254 characters (RFC 5321). |
| name | [string](#string) |  | User&#39;s display name. Constraints: Max length 200 characters. |
| status | [UserStatus](#pidgr-v1-UserStatus) |  | Current account status. |
| created_at | [google.protobuf.Timestamp](#google-protobuf-Timestamp) |  | Timestamp when the user was created. |
| role | [Role](#pidgr-v1-Role) |  | The user&#39;s role with its permission set. |
| role_id | [string](#string) |  | ID of the user&#39;s role (for assignment operations). |






<a name="pidgr-v1-WorkflowDefinition"></a>

### WorkflowDefinition
A data-driven workflow represented as a directed acyclic graph (DAG) of steps.
Defines the automation logic for a campaign&#39;s lifecycle.
Backend MUST validate the graph is a DAG (no cycles) before execution.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| steps | [WorkflowStep](#pidgr-v1-WorkflowStep) | repeated | Ordered list of steps in the workflow DAG. Constraints: Max 100 steps. Backend MUST validate the graph is a DAG (no cycles). |






<a name="pidgr-v1-WorkflowStep"></a>

### WorkflowStep
A single step in a workflow DAG with typed configuration and transitions.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| id | [string](#string) |  | Unique identifier for this step within the workflow. |
| type | [StepType](#pidgr-v1-StepType) |  | The type of operation this step performs. |
| send_notification | [SendNotificationConfig](#pidgr-v1-SendNotificationConfig) |  | Configuration for SEND_NOTIFICATION steps. |
| deadline_check | [DeadlineCheckConfig](#pidgr-v1-DeadlineCheckConfig) |  | Configuration for DEADLINE_CHECK steps. |
| send_reminder | [SendReminderConfig](#pidgr-v1-SendReminderConfig) |  | Configuration for SEND_REMINDER steps. |
| call_webhook | [CallWebhookConfig](#pidgr-v1-CallWebhookConfig) |  | Configuration for CALL_WEBHOOK steps. |
| transitions | [WorkflowStep.TransitionsEntry](#pidgr-v1-WorkflowStep-TransitionsEntry) | repeated | Map of outcome labels to the next step ID (e.g. &#34;completed&#34; -&gt; &#34;step_3&#34;). Constraints: Max 10 transitions per step. |






<a name="pidgr-v1-WorkflowStep-TransitionsEntry"></a>

### WorkflowStep.TransitionsEntry



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| key | [string](#string) |  |  |
| value | [string](#string) |  |  |





 


<a name="pidgr-v1-ActionType"></a>

### ActionType
Type of action a recipient can perform on a message.

| Name | Number | Description |
| ---- | ------ | ----------- |
| ACTION_TYPE_UNSPECIFIED | 0 | Default value; not a valid action type. |
| ACTION_TYPE_ACK | 1 | Simple acknowledgment — recipient confirms they received the message. |



<a name="pidgr-v1-CampaignStatus"></a>

### CampaignStatus
Lifecycle status of a campaign.

| Name | Number | Description |
| ---- | ------ | ----------- |
| CAMPAIGN_STATUS_UNSPECIFIED | 0 | Default value; not a valid status. |
| CAMPAIGN_STATUS_CREATED | 1 | Campaign has been created but not yet started. |
| CAMPAIGN_STATUS_RUNNING | 2 | Campaign is actively delivering messages and processing actions. |
| CAMPAIGN_STATUS_COMPLETED | 3 | All recipients have been processed; campaign is finished. |
| CAMPAIGN_STATUS_FAILED | 4 | Campaign terminated due to an unrecoverable error. |
| CAMPAIGN_STATUS_CANCELLED | 5 | Campaign was manually cancelled before completion. |



<a name="pidgr-v1-DeliveryStatus"></a>

### DeliveryStatus
Delivery status for a single message sent to a recipient.

| Name | Number | Description |
| ---- | ------ | ----------- |
| DELIVERY_STATUS_UNSPECIFIED | 0 | Default value; not a valid status. |
| DELIVERY_STATUS_PENDING | 1 | Message is queued but has not been sent yet. |
| DELIVERY_STATUS_SENT | 2 | Push notification was sent to the delivery provider. |
| DELIVERY_STATUS_DELIVERED | 3 | Message was confirmed delivered to the device. |
| DELIVERY_STATUS_ACKNOWLEDGED | 4 | Recipient completed the required action (e.g. acknowledged). |
| DELIVERY_STATUS_MISSED | 5 | Recipient did not act before the deadline. |
| DELIVERY_STATUS_NO_DEVICE | 6 | Recipient has no registered device; delivery was skipped. |
| DELIVERY_STATUS_FAILED | 7 | Delivery failed due to a provider or system error. |



<a name="pidgr-v1-Permission"></a>

### Permission
Granular permission for authorization checks.
Stored in the database as enum names (e.g. &#34;PERMISSION_ORG_READ&#34;).
New values MUST be appended with the next sequential number; existing values
MUST NOT be renumbered or removed (enforced by buf breaking).

| Name | Number | Description |
| ---- | ------ | ----------- |
| PERMISSION_UNSPECIFIED | 0 | Default value; not a valid permission. |
| PERMISSION_ORG_READ | 1 | View organization settings. |
| PERMISSION_ORG_WRITE | 2 | Modify organization settings. |
| PERMISSION_MEMBERS_READ | 3 | View organization members. |
| PERMISSION_MEMBERS_INVITE | 4 | Invite new users to the organization. |
| PERMISSION_MEMBERS_MANAGE | 5 | Change user roles, deactivate users. |
| PERMISSION_CAMPAIGNS_READ | 6 | View campaigns and deliveries. |
| PERMISSION_CAMPAIGNS_WRITE | 7 | Create and edit campaigns. |
| PERMISSION_CAMPAIGNS_START | 8 | Start campaign execution. |
| PERMISSION_TEMPLATES_READ | 9 | View templates. |
| PERMISSION_TEMPLATES_WRITE | 10 | Create and edit templates. |
| PERMISSION_WORKFLOWS_READ | 11 | View workflow definitions. |
| PERMISSION_WORKFLOWS_WRITE | 12 | Create and edit workflow definitions. |
| PERMISSION_INBOX_READ | 13 | View inbox messages and deliveries. |
| PERMISSION_INBOX_ACT | 14 | Submit actions on deliveries. |



<a name="pidgr-v1-Platform"></a>

### Platform
Mobile platform for device registration.

| Name | Number | Description |
| ---- | ------ | ----------- |
| PLATFORM_UNSPECIFIED | 0 | Default value; not a valid platform. |
| PLATFORM_IOS | 1 | Apple iOS. |
| PLATFORM_ANDROID | 2 | Google Android. |



<a name="pidgr-v1-StepType"></a>

### StepType
Type of step within a workflow definition DAG.

| Name | Number | Description |
| ---- | ------ | ----------- |
| STEP_TYPE_UNSPECIFIED | 0 | Default value; not a valid step type. |
| STEP_TYPE_SEND_NOTIFICATION | 1 | Send the initial push notification to all recipients. |
| STEP_TYPE_DEADLINE_CHECK | 2 | Sleep for a configurable deadline, then proceed to the next step. |
| STEP_TYPE_SEND_REMINDER | 3 | Send a follow-up reminder to recipients who have not acted. |
| STEP_TYPE_CALL_WEBHOOK | 4 | Call an external webhook with campaign context. |
| STEP_TYPE_MARK_MISSED | 5 | Mark unacknowledged deliveries (SENT/DELIVERED) as MISSED. No config required. |



<a name="pidgr-v1-UserStatus"></a>

### UserStatus
Lifecycle status of a user account.

| Name | Number | Description |
| ---- | ------ | ----------- |
| USER_STATUS_UNSPECIFIED | 0 | Default value; not a valid status. |
| USER_STATUS_INVITED | 1 | User has been invited but has not completed onboarding. |
| USER_STATUS_ACTIVE | 2 | User is active and can receive messages. |
| USER_STATUS_DEACTIVATED | 3 | User has been deactivated and will not receive messages. |


 

 

 



<a name="pidgr_v1_campaign-proto"></a>
<p align="right"><a href="#top">Top</a></p>

## pidgr/v1/campaign.proto



<a name="pidgr-v1-Campaign"></a>

### Campaign
A campaign that delivers structured messages to a set of recipients
and tracks their engagement through a workflow.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| id | [string](#string) |  | Unique identifier for the campaign. Constraints: UUID format (36 characters). |
| name | [string](#string) |  | Human-readable campaign name. Constraints: Max length 200 characters. |
| template_id | [string](#string) |  | ID of the template used to render messages. Constraints: UUID format (36 characters). |
| template_version | [int32](#int32) |  | Pinned version of the template used for this campaign. |
| audience_snapshot_ref | [string](#string) |  | S3 reference to the audience snapshot taken at campaign creation. |
| status | [CampaignStatus](#pidgr-v1-CampaignStatus) |  | Current lifecycle status of the campaign. |
| workflow | [WorkflowDefinition](#pidgr-v1-WorkflowDefinition) |  | Workflow DAG that drives the campaign&#39;s automation logic. |
| total_recipients | [int32](#int32) |  | Total number of recipients in the audience snapshot. |
| action_completed_count | [int32](#int32) |  | Number of recipients who completed the required action. |
| missed_count | [int32](#int32) |  | Number of recipients who did not act before the deadline. |
| created_at | [google.protobuf.Timestamp](#google-protobuf-Timestamp) |  | Timestamp when the campaign was created. |
| started_at | [google.protobuf.Timestamp](#google-protobuf-Timestamp) |  | Timestamp when the campaign was started (workflow execution began). |
| completed_at | [google.protobuf.Timestamp](#google-protobuf-Timestamp) |  | Timestamp when the campaign finished (completed, failed, or cancelled). |
| sender_name | [string](#string) |  | Display name of the sender shown to recipients (e.g. &#34;HR Team&#34;). Constraints: Max length 200 characters. |
| title | [string](#string) |  | Optional user-facing title override. If set, takes precedence over the template title. Constraints: Max length 200 characters. |






<a name="pidgr-v1-CancelCampaignRequest"></a>

### CancelCampaignRequest
Request to cancel a running campaign.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| campaign_id | [string](#string) |  | ID of the campaign to cancel. Constraints: UUID format (36 characters). |






<a name="pidgr-v1-CancelCampaignResponse"></a>

### CancelCampaignResponse
Response after cancelling a campaign.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| campaign | [Campaign](#pidgr-v1-Campaign) |  | The campaign with updated status (CANCELLED). |






<a name="pidgr-v1-CreateCampaignRequest"></a>

### CreateCampaignRequest
Request to create a new campaign.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| name | [string](#string) |  | Human-readable campaign name (admin-facing label). Constraints: Max length 200 characters. |
| template_id | [string](#string) |  | ID of the template to use for rendering messages. Constraints: UUID format (36 characters). |
| template_version | [int32](#int32) |  | Version of the template to pin for this campaign. |
| user_ids | [string](#string) | repeated | List of user IDs that form the campaign audience. Constraints: Max 100000 items. |
| workflow | [WorkflowDefinition](#pidgr-v1-WorkflowDefinition) |  | Workflow DAG defining the campaign&#39;s automation steps. |
| sender_name | [string](#string) |  | Display name of the sender shown to recipients (e.g. &#34;HR Team&#34;). Constraints: Max length 200 characters. |
| title | [string](#string) |  | Optional user-facing title override. If empty, the template title is used. Constraints: Max length 200 characters. |






<a name="pidgr-v1-CreateCampaignResponse"></a>

### CreateCampaignResponse
Response after creating a campaign.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| campaign | [Campaign](#pidgr-v1-Campaign) |  | The newly created campaign. |






<a name="pidgr-v1-Delivery"></a>

### Delivery
A single delivery record tracking message delivery to one recipient.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| id | [string](#string) |  | Unique identifier for this delivery. Constraints: UUID format (36 characters). |
| user_id | [string](#string) |  | ID of the recipient user. Constraints: UUID format (36 characters). |
| campaign_id | [string](#string) |  | ID of the campaign this delivery belongs to. Constraints: UUID format (36 characters). |
| status | [DeliveryStatus](#pidgr-v1-DeliveryStatus) |  | Current delivery status. |
| delivered_at | [google.protobuf.Timestamp](#google-protobuf-Timestamp) |  | Timestamp when the message was delivered to the device. |
| read_at | [google.protobuf.Timestamp](#google-protobuf-Timestamp) |  | Timestamp when the recipient read the message. |
| acted_at | [google.protobuf.Timestamp](#google-protobuf-Timestamp) |  | Timestamp when the recipient performed the required action. |






<a name="pidgr-v1-GetCampaignRequest"></a>

### GetCampaignRequest
Request to retrieve a single campaign by ID.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| campaign_id | [string](#string) |  | ID of the campaign to retrieve. Constraints: UUID format (36 characters). |






<a name="pidgr-v1-GetCampaignResponse"></a>

### GetCampaignResponse
Response containing the requested campaign.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| campaign | [Campaign](#pidgr-v1-Campaign) |  | The requested campaign. |






<a name="pidgr-v1-ListCampaignsRequest"></a>

### ListCampaignsRequest
Request to list campaigns with pagination.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| pagination | [Pagination](#pidgr-v1-Pagination) |  | Pagination parameters. |






<a name="pidgr-v1-ListCampaignsResponse"></a>

### ListCampaignsResponse
Response containing a page of campaigns.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| campaigns | [Campaign](#pidgr-v1-Campaign) | repeated | List of campaigns in this page. |
| pagination_meta | [PaginationMeta](#pidgr-v1-PaginationMeta) |  | Pagination metadata for fetching subsequent pages. |






<a name="pidgr-v1-ListDeliveriesRequest"></a>

### ListDeliveriesRequest
Request to list deliveries for a campaign with optional status filtering.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| campaign_id | [string](#string) |  | ID of the campaign to list deliveries for. Constraints: UUID format (36 characters). |
| status_filter | [DeliveryStatus](#pidgr-v1-DeliveryStatus) |  | Optional filter by delivery status. UNSPECIFIED returns all. |
| pagination | [Pagination](#pidgr-v1-Pagination) |  | Pagination parameters. |






<a name="pidgr-v1-ListDeliveriesResponse"></a>

### ListDeliveriesResponse
Response containing a page of delivery records.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| deliveries | [Delivery](#pidgr-v1-Delivery) | repeated | List of deliveries in this page. |
| pagination_meta | [PaginationMeta](#pidgr-v1-PaginationMeta) |  | Pagination metadata for fetching subsequent pages. |






<a name="pidgr-v1-StartCampaignRequest"></a>

### StartCampaignRequest
Request to start a campaign&#39;s workflow execution.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| campaign_id | [string](#string) |  | ID of the campaign to start. Constraints: UUID format (36 characters). |






<a name="pidgr-v1-StartCampaignResponse"></a>

### StartCampaignResponse
Response after starting a campaign.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| campaign | [Campaign](#pidgr-v1-Campaign) |  | The campaign with updated status. |





 

 

 


<a name="pidgr-v1-CampaignService"></a>

### CampaignService
Manages the full lifecycle of communication campaigns — creation,
execution, monitoring, and cancellation.

| Method Name | Request Type | Response Type | Description |
| ----------- | ------------ | ------------- | ------------|
| CreateCampaign | [CreateCampaignRequest](#pidgr-v1-CreateCampaignRequest) | [CreateCampaignResponse](#pidgr-v1-CreateCampaignResponse) | Create a new campaign with a template, audience, and workflow. Authorization: Requires MANAGER&#43; role. |
| StartCampaign | [StartCampaignRequest](#pidgr-v1-StartCampaignRequest) | [StartCampaignResponse](#pidgr-v1-StartCampaignResponse) | Start a created campaign, triggering its workflow execution via Temporal. Authorization: Requires MANAGER&#43; role. |
| GetCampaign | [GetCampaignRequest](#pidgr-v1-GetCampaignRequest) | [GetCampaignResponse](#pidgr-v1-GetCampaignResponse) | Retrieve a single campaign by ID. Authorization: Authenticated user within the organization. |
| ListCampaigns | [ListCampaignsRequest](#pidgr-v1-ListCampaignsRequest) | [ListCampaignsResponse](#pidgr-v1-ListCampaignsResponse) | List campaigns for the organization with pagination. Authorization: Authenticated user within the organization. |
| CancelCampaign | [CancelCampaignRequest](#pidgr-v1-CancelCampaignRequest) | [CancelCampaignResponse](#pidgr-v1-CancelCampaignResponse) | Cancel a running campaign, stopping further deliveries and reminders. Authorization: Requires MANAGER&#43; role. |
| ListDeliveries | [ListDeliveriesRequest](#pidgr-v1-ListDeliveriesRequest) | [ListDeliveriesResponse](#pidgr-v1-ListDeliveriesResponse) | List delivery records for a campaign, optionally filtered by status. Authorization: Authenticated user within the organization. |

 



<a name="pidgr_v1_device-proto"></a>
<p align="right"><a href="#top">Top</a></p>

## pidgr/v1/device.proto



<a name="pidgr-v1-DeactivateRequest"></a>

### DeactivateRequest
Request to deactivate a device, stopping push notifications.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| device_id | [string](#string) |  | ID of the device to deactivate. Constraints: UUID format (36 characters). |






<a name="pidgr-v1-DeactivateResponse"></a>

### DeactivateResponse
Response after deactivating a device.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| success | [bool](#bool) |  | Whether the device was successfully deactivated. |






<a name="pidgr-v1-Device"></a>

### Device
A registered device that can receive push notifications.
INTERNAL: This message is for server-side use only. Use DeviceSummary for API responses.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| device_id | [string](#string) |  | Unique identifier for this device. Constraints: UUID format (36 characters). |
| user_id | [string](#string) |  | ID of the user who owns this device. Constraints: UUID format (36 characters). |
| platform | [Platform](#pidgr-v1-Platform) |  | Mobile platform (iOS or Android). |
| push_token | [string](#string) |  | FCM push token used to send notifications to this device. |
| active | [bool](#bool) |  | Whether the device is currently active and eligible for push delivery. |
| last_seen | [google.protobuf.Timestamp](#google-protobuf-Timestamp) |  | Timestamp of the last activity from this device. |
| created_at | [google.protobuf.Timestamp](#google-protobuf-Timestamp) |  | Timestamp when the device was first registered. |






<a name="pidgr-v1-DeviceSummary"></a>

### DeviceSummary
A device summary safe for API responses — excludes sensitive push_token.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| device_id | [string](#string) |  | Unique identifier for this device. |
| user_id | [string](#string) |  | ID of the user who owns this device. |
| platform | [Platform](#pidgr-v1-Platform) |  | Mobile platform (iOS or Android). |
| active | [bool](#bool) |  | Whether the device is currently active and eligible for push delivery. |
| last_seen | [google.protobuf.Timestamp](#google-protobuf-Timestamp) |  | Timestamp of the last activity from this device. |
| created_at | [google.protobuf.Timestamp](#google-protobuf-Timestamp) |  | Timestamp when the device was first registered. |






<a name="pidgr-v1-ListDevicesRequest"></a>

### ListDevicesRequest
Request to list all devices for the authenticated user.






<a name="pidgr-v1-ListDevicesResponse"></a>

### ListDevicesResponse
Response containing all devices for the user.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| devices | [DeviceSummary](#pidgr-v1-DeviceSummary) | repeated | List of devices registered to the authenticated user. |






<a name="pidgr-v1-RegisterRequest"></a>

### RegisterRequest
Request to register a device for push notifications.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| device_id | [string](#string) |  | Client-generated unique device identifier. Constraints: UUID format (36 characters). |
| platform | [Platform](#pidgr-v1-Platform) |  | Mobile platform of the device. |
| push_token | [string](#string) |  | FCM push token obtained from Firebase on the client. Constraints: Max length 4096 characters. |






<a name="pidgr-v1-RegisterResponse"></a>

### RegisterResponse
Response after registering a device.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| device | [DeviceSummary](#pidgr-v1-DeviceSummary) |  | The registered device summary (excludes push_token). |





 

 

 


<a name="pidgr-v1-DeviceService"></a>

### DeviceService
Manages push notification device registration.
Used by the mobile app to register FCM tokens and manage device lifecycle.

| Method Name | Request Type | Response Type | Description |
| ----------- | ------------ | ------------- | ------------|
| Register | [RegisterRequest](#pidgr-v1-RegisterRequest) | [RegisterResponse](#pidgr-v1-RegisterResponse) | Register a device with its FCM push token for receiving notifications. Authorization: Authenticated user (own devices only). |
| Deactivate | [DeactivateRequest](#pidgr-v1-DeactivateRequest) | [DeactivateResponse](#pidgr-v1-DeactivateResponse) | Deactivate a device, preventing further push notifications. Authorization: Authenticated user (own devices only). |
| ListDevices | [ListDevicesRequest](#pidgr-v1-ListDevicesRequest) | [ListDevicesResponse](#pidgr-v1-ListDevicesResponse) | List all devices registered to the authenticated user. Authorization: Authenticated user (own devices only). |

 



<a name="pidgr_v1_heatmap-proto"></a>
<p align="right"><a href="#top">Top</a></p>

## pidgr/v1/heatmap.proto



<a name="pidgr-v1-HeatmapDataPoint"></a>

### HeatmapDataPoint
A single aggregated data point in a heatmap grid cell.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| x_pct | [float](#float) |  | Grid cell horizontal center as a percentage (0.0–1.0). |
| y_pct | [float](#float) |  | Grid cell vertical center as a percentage (0.0–1.0). |
| value | [float](#float) |  | Aggregated value for this cell (count, median, or z-score depending on mode). |






<a name="pidgr-v1-IngestTouchEventsRequest"></a>

### IngestTouchEventsRequest
Request to ingest a batch of touch events from the mobile app.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| events | [TouchEvent](#pidgr-v1-TouchEvent) | repeated | Batch of touch events to ingest. Constraints: Max 100 events per batch. |






<a name="pidgr-v1-IngestTouchEventsResponse"></a>

### IngestTouchEventsResponse
Response after ingesting touch events.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| ingested_count | [int32](#int32) |  | Number of events successfully ingested. |






<a name="pidgr-v1-ListScreenshotsRequest"></a>

### ListScreenshotsRequest
Request to list available screen screenshots.






<a name="pidgr-v1-ListScreenshotsResponse"></a>

### ListScreenshotsResponse
Response containing available screen screenshots.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| screenshots | [ScreenScreenshot](#pidgr-v1-ScreenScreenshot) | repeated | Available screen screenshots with their URLs and versions. |






<a name="pidgr-v1-QueryHeatmapDataRequest"></a>

### QueryHeatmapDataRequest
Request to query aggregated heatmap data for a screen.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| screen_name | [string](#string) |  | Screen name to query. Constraints: Max length 200 characters. |
| date_from | [google.protobuf.Timestamp](#google-protobuf-Timestamp) |  | Start of the time range filter (inclusive). |
| date_to | [google.protobuf.Timestamp](#google-protobuf-Timestamp) |  | End of the time range filter (inclusive). |
| campaign_id | [string](#string) |  | Optional: filter by campaign ID. Constraints: UUID format (36 characters). |
| user_id | [string](#string) |  | Optional: filter by user ID (required for OUTLIER mode). Constraints: UUID format (36 characters). |
| grid_resolution | [float](#float) |  | Grid resolution for coordinate rounding. Default: 0.02 (50×50 grid). Constraints: Range 0.005 to 0.1. |
| mode | [HeatmapMode](#pidgr-v1-HeatmapMode) |  | Aggregation mode. |






<a name="pidgr-v1-QueryHeatmapDataResponse"></a>

### QueryHeatmapDataResponse
Response containing aggregated heatmap data.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| data_points | [HeatmapDataPoint](#pidgr-v1-HeatmapDataPoint) | repeated | Aggregated data points for heatmap rendering. |
| user_touch_counts | [UserTouchCount](#pidgr-v1-UserTouchCount) | repeated | Per-user touch counts for distribution chart rendering. Only populated when mode is TOTAL or MEDIAN. |






<a name="pidgr-v1-ScreenScreenshot"></a>

### ScreenScreenshot
A screen screenshot stored as a static asset.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| screen_name | [string](#string) |  | Screen name matching React Navigation route. |
| url | [string](#string) |  | S3 URL to the screenshot image. |
| app_version | [string](#string) |  | App version this screenshot corresponds to. |






<a name="pidgr-v1-TouchEvent"></a>

### TouchEvent
A single touch event captured from the mobile app.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| screen_name | [string](#string) |  | Screen name from React Navigation route. Constraints: Max length 200 characters. |
| x_pct | [float](#float) |  | Horizontal coordinate as a percentage of screen width (0.0–1.0). Constraints: Range 0.0 to 1.0 inclusive. |
| y_pct | [float](#float) |  | Vertical coordinate as a percentage of screen height (0.0–1.0). Constraints: Range 0.0 to 1.0 inclusive. |
| event_type | [TouchEventType](#pidgr-v1-TouchEventType) |  | Type of touch event. |
| screen_width | [int32](#int32) |  | Screen width in device pixels at the time of capture. |
| screen_height | [int32](#int32) |  | Screen height in device pixels at the time of capture. |
| client_timestamp | [google.protobuf.Timestamp](#google-protobuf-Timestamp) |  | Client-side timestamp when the touch occurred. |






<a name="pidgr-v1-UserTouchCount"></a>

### UserTouchCount
Per-user touch count for distribution analysis.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| user_id | [string](#string) |  | User ID. |
| count | [int32](#int32) |  | Total touch count for the user in the query range. |





 


<a name="pidgr-v1-HeatmapMode"></a>

### HeatmapMode
Aggregation mode for heatmap data queries.

| Name | Number | Description |
| ---- | ------ | ----------- |
| HEATMAP_MODE_UNSPECIFIED | 0 | Default value; not a valid mode. |
| HEATMAP_MODE_TOTAL | 1 | Sum of all users&#39; touches per grid cell (default). |
| HEATMAP_MODE_MEDIAN | 2 | Median touch count per grid cell across all users. |
| HEATMAP_MODE_OUTLIER | 3 | Highlight cells where a specific user deviates more than 2σ from the median. Requires user_id to be set in the query request. |



<a name="pidgr-v1-TouchEventType"></a>

### TouchEventType
Type of touch event captured on the mobile app.

| Name | Number | Description |
| ---- | ------ | ----------- |
| TOUCH_EVENT_TYPE_UNSPECIFIED | 0 | Default value; not a valid event type. |
| TOUCH_EVENT_TYPE_TAP | 1 | A single tap on the screen. |
| TOUCH_EVENT_TYPE_LONG_PRESS | 2 | A long press (held for 500ms&#43;). |


 

 


<a name="pidgr-v1-HeatmapService"></a>

### HeatmapService
Manages touch event ingestion, heatmap data aggregation, and screen screenshots
for mobile app interaction analytics.

| Method Name | Request Type | Response Type | Description |
| ----------- | ------------ | ------------- | ------------|
| IngestTouchEvents | [IngestTouchEventsRequest](#pidgr-v1-IngestTouchEventsRequest) | [IngestTouchEventsResponse](#pidgr-v1-IngestTouchEventsResponse) | Ingest a batch of touch events from the mobile app. Authorization: Authenticated mobile user. |
| QueryHeatmapData | [QueryHeatmapDataRequest](#pidgr-v1-QueryHeatmapDataRequest) | [QueryHeatmapDataResponse](#pidgr-v1-QueryHeatmapDataResponse) | Query aggregated touch data for heatmap rendering. Authorization: Requires CAMPAIGNS_READ permission. |
| ListScreenshots | [ListScreenshotsRequest](#pidgr-v1-ListScreenshotsRequest) | [ListScreenshotsResponse](#pidgr-v1-ListScreenshotsResponse) | List available screen screenshots for heatmap backgrounds. Authorization: Requires CAMPAIGNS_READ permission. |

 



<a name="pidgr_v1_inbox-proto"></a>
<p align="right"><a href="#top">Top</a></p>

## pidgr/v1/inbox.proto



<a name="pidgr-v1-GetMessageRequest"></a>

### GetMessageRequest
Request to retrieve a single message by delivery ID.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| delivery_id | [string](#string) |  | ID of the delivery to retrieve. Constraints: UUID format (36 characters). |






<a name="pidgr-v1-GetMessageResponse"></a>

### GetMessageResponse
Response containing the requested inbox entry.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| entry | [InboxEntry](#pidgr-v1-InboxEntry) |  | The inbox entry for the requested delivery. |






<a name="pidgr-v1-InboxEntry"></a>

### InboxEntry
A single entry in a user&#39;s inbox, combining a message with its delivery state.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| delivery_id | [string](#string) |  | ID of the delivery record for this inbox entry. Constraints: UUID format (36 characters). |
| message | [Message](#pidgr-v1-Message) |  | The fully rendered message content. |
| status | [DeliveryStatus](#pidgr-v1-DeliveryStatus) |  | Current delivery status (e.g. DELIVERED, ACKNOWLEDGED). |
| read | [bool](#bool) |  | Whether the user has read this message. |
| received_at | [google.protobuf.Timestamp](#google-protobuf-Timestamp) |  | Timestamp when the message was received in the inbox. |






<a name="pidgr-v1-MarkReadRequest"></a>

### MarkReadRequest
Request to mark a message as read.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| delivery_id | [string](#string) |  | ID of the delivery to mark as read. Constraints: UUID format (36 characters). |






<a name="pidgr-v1-MarkReadResponse"></a>

### MarkReadResponse
Response after marking a message as read.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| success | [bool](#bool) |  | Whether the read status was successfully updated. |






<a name="pidgr-v1-SyncRequest"></a>

### SyncRequest
Request to sync inbox entries since a given timestamp.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| since | [google.protobuf.Timestamp](#google-protobuf-Timestamp) |  | Fetch entries newer than this timestamp. Omit for initial sync. |
| limit | [int32](#int32) |  | Maximum number of entries to return. Constraints: Valid range 1 to 200. |






<a name="pidgr-v1-SyncResponse"></a>

### SyncResponse
Response containing synced inbox entries.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| entries | [InboxEntry](#pidgr-v1-InboxEntry) | repeated | Inbox entries newer than the requested timestamp. |
| next_since | [google.protobuf.Timestamp](#google-protobuf-Timestamp) |  | Cursor timestamp to use for the next sync call. |





 

 

 


<a name="pidgr-v1-InboxService"></a>

### InboxService
Provides the mobile app&#39;s inbox experience — syncing messages,
tracking read status, and retrieving individual entries.

| Method Name | Request Type | Response Type | Description |
| ----------- | ------------ | ------------- | ------------|
| Sync | [SyncRequest](#pidgr-v1-SyncRequest) | [SyncResponse](#pidgr-v1-SyncResponse) | Sync inbox entries since a given timestamp for incremental updates. Authorization: Authenticated user (own inbox only). |
| MarkRead | [MarkReadRequest](#pidgr-v1-MarkReadRequest) | [MarkReadResponse](#pidgr-v1-MarkReadResponse) | Mark a delivered message as read (analytics-only, does not affect workflow). Authorization: Authenticated user (own inbox only). |
| GetMessage | [GetMessageRequest](#pidgr-v1-GetMessageRequest) | [GetMessageResponse](#pidgr-v1-GetMessageResponse) | Retrieve a single inbox entry by delivery ID. Authorization: Authenticated user (own inbox only). |

 



<a name="pidgr_v1_member-proto"></a>
<p align="right"><a href="#top">Top</a></p>

## pidgr/v1/member.proto



<a name="pidgr-v1-DeactivateUserRequest"></a>

### DeactivateUserRequest
Request to deactivate a user within the organization.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| user_id | [string](#string) |  | ID of the user to deactivate. |






<a name="pidgr-v1-DeactivateUserResponse"></a>

### DeactivateUserResponse
Response after deactivating a user.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| user | [User](#pidgr-v1-User) |  | The deactivated user (status: DEACTIVATED). |






<a name="pidgr-v1-GetUserRequest"></a>

### GetUserRequest
Request to retrieve a user by ID.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| user_id | [string](#string) |  | ID of the user to retrieve. |






<a name="pidgr-v1-GetUserResponse"></a>

### GetUserResponse
Response containing the requested user.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| user | [User](#pidgr-v1-User) |  | The requested user. |






<a name="pidgr-v1-InviteUserRequest"></a>

### InviteUserRequest
Request to invite a new user to the organization.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| email | [string](#string) |  | Email address to send the invitation to. Constraints: Max length 254 characters (RFC 5321). |
| name | [string](#string) |  | Display name for the invited user. Constraints: Max length 200 characters. |
| role_id | [string](#string) |  | ID of the role to assign. Defaults to the organization&#39;s employee role if empty. |






<a name="pidgr-v1-InviteUserResponse"></a>

### InviteUserResponse
Response after inviting a user.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| user | [User](#pidgr-v1-User) |  | The newly created user (status: INVITED). |






<a name="pidgr-v1-ListUsersRequest"></a>

### ListUsersRequest
Request to list users in the organization with pagination.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| pagination | [Pagination](#pidgr-v1-Pagination) |  | Pagination parameters. |






<a name="pidgr-v1-ListUsersResponse"></a>

### ListUsersResponse
Response containing a page of users.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| users | [User](#pidgr-v1-User) | repeated | List of users in this page. |
| pagination_meta | [PaginationMeta](#pidgr-v1-PaginationMeta) |  | Pagination metadata for fetching subsequent pages. |






<a name="pidgr-v1-UpdateUserRoleRequest"></a>

### UpdateUserRoleRequest
Request to change a user&#39;s role within the organization.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| user_id | [string](#string) |  | ID of the user whose role to update. |
| role_id | [string](#string) |  | ID of the new role to assign. |






<a name="pidgr-v1-UpdateUserRoleResponse"></a>

### UpdateUserRoleResponse
Response after updating a user&#39;s role.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| user | [User](#pidgr-v1-User) |  | The updated user with the new role. |





 

 

 


<a name="pidgr-v1-MemberService"></a>

### MemberService
Manages members (users) within an organization.
All RPCs operate within the caller&#39;s org (extracted from JWT).

| Method Name | Request Type | Response Type | Description |
| ----------- | ------------ | ------------- | ------------|
| InviteUser | [InviteUserRequest](#pidgr-v1-InviteUserRequest) | [InviteUserResponse](#pidgr-v1-InviteUserResponse) | Invite a new user to the organization via email. Authorization: Requires PERMISSION_MEMBERS_INVITE. |
| GetUser | [GetUserRequest](#pidgr-v1-GetUserRequest) | [GetUserResponse](#pidgr-v1-GetUserResponse) | Retrieve a user by ID within the organization. Self-lookup (empty user_id) is allowed for any authenticated user. Authorization: Requires PERMISSION_MEMBERS_READ for other users. |
| ListUsers | [ListUsersRequest](#pidgr-v1-ListUsersRequest) | [ListUsersResponse](#pidgr-v1-ListUsersResponse) | List all users in the organization with pagination. Authorization: Requires PERMISSION_MEMBERS_READ. |
| UpdateUserRole | [UpdateUserRoleRequest](#pidgr-v1-UpdateUserRoleRequest) | [UpdateUserRoleResponse](#pidgr-v1-UpdateUserRoleResponse) | Change a user&#39;s role within the organization. Authorization: Requires PERMISSION_MEMBERS_MANAGE. |
| DeactivateUser | [DeactivateUserRequest](#pidgr-v1-DeactivateUserRequest) | [DeactivateUserResponse](#pidgr-v1-DeactivateUserResponse) | Deactivate a user within the organization. Authorization: Requires PERMISSION_MEMBERS_MANAGE. |

 



<a name="pidgr_v1_organization-proto"></a>
<p align="right"><a href="#top">Top</a></p>

## pidgr/v1/organization.proto



<a name="pidgr-v1-CreateOrganizationRequest"></a>

### CreateOrganizationRequest
Request to create a new organization with an admin user.
Supports API key auth (service-to-service) and JWT auth (self-service onboarding).


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| name | [string](#string) |  | Name for the new organization. Constraints: Max length 200 characters. |
| admin_email | [string](#string) |  | Email address for the initial admin user. Only used with API key auth; ignored with JWT auth (email derived from Cognito sub). |
| industry | [Industry](#pidgr-v1-Industry) |  | Industry vertical for the organization. |
| company_size | [CompanySize](#pidgr-v1-CompanySize) |  | Employee headcount range. |






<a name="pidgr-v1-CreateOrganizationResponse"></a>

### CreateOrganizationResponse
Response after creating an organization.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| organization | [Organization](#pidgr-v1-Organization) |  | The newly created organization. |
| admin_user | [User](#pidgr-v1-User) |  | The admin user created for the organization. |






<a name="pidgr-v1-GetOrganizationRequest"></a>

### GetOrganizationRequest
Request to retrieve the organization for the authenticated user.






<a name="pidgr-v1-GetOrganizationResponse"></a>

### GetOrganizationResponse
Response containing the organization.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| organization | [Organization](#pidgr-v1-Organization) |  | The organization the authenticated user belongs to. |






<a name="pidgr-v1-Organization"></a>

### Organization
An organization (tenant) in the Pidgr platform.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| id | [string](#string) |  | Unique identifier for the organization. |
| name | [string](#string) |  | Organization display name. Constraints: Max length 200 characters. |
| default_workflow | [WorkflowDefinition](#pidgr-v1-WorkflowDefinition) |  | Default workflow used when campaigns don&#39;t specify one. |
| created_at | [google.protobuf.Timestamp](#google-protobuf-Timestamp) |  | Timestamp when the organization was created. |
| industry | [Industry](#pidgr-v1-Industry) |  | Industry vertical. |
| company_size | [CompanySize](#pidgr-v1-CompanySize) |  | Employee headcount range. |






<a name="pidgr-v1-UpdateOrganizationRequest"></a>

### UpdateOrganizationRequest
Request to update organization settings.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| name | [string](#string) |  | New organization name. Empty string leaves unchanged. Constraints: Max length 200 characters. |
| default_workflow | [WorkflowDefinition](#pidgr-v1-WorkflowDefinition) |  | New default workflow definition. Null leaves unchanged. |
| industry | [Industry](#pidgr-v1-Industry) |  | New industry vertical. UNSPECIFIED leaves unchanged. |
| company_size | [CompanySize](#pidgr-v1-CompanySize) |  | New employee headcount range. UNSPECIFIED leaves unchanged. |






<a name="pidgr-v1-UpdateOrganizationResponse"></a>

### UpdateOrganizationResponse
Response after updating the organization.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| organization | [Organization](#pidgr-v1-Organization) |  | The updated organization. |





 


<a name="pidgr-v1-CompanySize"></a>

### CompanySize
Employee headcount range for an organization.

| Name | Number | Description |
| ---- | ------ | ----------- |
| COMPANY_SIZE_UNSPECIFIED | 0 |  |
| COMPANY_SIZE_1_200 | 1 |  |
| COMPANY_SIZE_200_500 | 2 |  |
| COMPANY_SIZE_500_1000 | 3 |  |
| COMPANY_SIZE_1000_5000 | 4 |  |
| COMPANY_SIZE_5000_PLUS | 5 |  |



<a name="pidgr-v1-Industry"></a>

### Industry
Industry vertical for an organization.

| Name | Number | Description |
| ---- | ------ | ----------- |
| INDUSTRY_UNSPECIFIED | 0 |  |
| INDUSTRY_TECHNOLOGY | 1 |  |
| INDUSTRY_FINANCE | 2 |  |
| INDUSTRY_HEALTHCARE | 3 |  |
| INDUSTRY_EDUCATION | 4 |  |
| INDUSTRY_RETAIL | 5 |  |
| INDUSTRY_MANUFACTURING | 6 |  |
| INDUSTRY_MEDIA | 7 |  |
| INDUSTRY_OTHER | 8 |  |


 

 


<a name="pidgr-v1-OrganizationService"></a>

### OrganizationService
Manages organizations (tenants) in the Pidgr platform.
Most RPCs operate within the caller&#39;s org (extracted from JWT).
CreateOrganization supports API key auth or JWT auth (self-service onboarding).

| Method Name | Request Type | Response Type | Description |
| ----------- | ------------ | ------------- | ------------|
| CreateOrganization | [CreateOrganizationRequest](#pidgr-v1-CreateOrganizationRequest) | [CreateOrganizationResponse](#pidgr-v1-CreateOrganizationResponse) | Create a new organization with an initial admin user. Supports API key auth (service-to-service) and JWT auth (self-service onboarding). |
| GetOrganization | [GetOrganizationRequest](#pidgr-v1-GetOrganizationRequest) | [GetOrganizationResponse](#pidgr-v1-GetOrganizationResponse) | Retrieve the organization for the authenticated user. Authorization: Requires PERMISSION_ORG_READ. |
| UpdateOrganization | [UpdateOrganizationRequest](#pidgr-v1-UpdateOrganizationRequest) | [UpdateOrganizationResponse](#pidgr-v1-UpdateOrganizationResponse) | Update organization settings (name, default workflow, industry, company size). Authorization: Requires PERMISSION_ORG_WRITE. |

 



<a name="pidgr_v1_render-proto"></a>
<p align="right"><a href="#top">Top</a></p>

## pidgr/v1/render.proto



<a name="pidgr-v1-RenderBatchRequest"></a>

### RenderBatchRequest
Request to render a template for a batch of users.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| template_id | [string](#string) |  | ID of the template to render. |
| version | [int32](#int32) |  | Version of the template to render. |
| users | [UserRenderContext](#pidgr-v1-UserRenderContext) | repeated | Per-user rendering contexts with variable substitutions. Constraints: Max 10000 users per batch. |






<a name="pidgr-v1-RenderBatchResponse"></a>

### RenderBatchResponse
Streamed response for each user&#39;s rendered message.
One response is emitted per user in the batch.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| user_id | [string](#string) |  | ID of the user this result is for. |
| message | [Message](#pidgr-v1-Message) |  | The rendered message (set on success). |
| error | [string](#string) |  | Error message if rendering failed for this user (empty on success). |






<a name="pidgr-v1-UserRenderContext"></a>

### UserRenderContext
Per-user rendering context containing variable substitutions.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| user_id | [string](#string) |  | ID of the user being rendered for. |
| variables | [UserRenderContext.VariablesEntry](#pidgr-v1-UserRenderContext-VariablesEntry) | repeated | Variable name-value pairs to substitute into the template. Constraints: Max 100 entries. Key max length 100 characters, value max length 10000 characters. |






<a name="pidgr-v1-UserRenderContext-VariablesEntry"></a>

### UserRenderContext.VariablesEntry



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| key | [string](#string) |  |  |
| value | [string](#string) |  |  |





 

 

 


<a name="pidgr-v1-RenderService"></a>

### RenderService
Internal service for batch template rendering.
Currently implemented in-process in Go; proto preserved for future
extraction to a dedicated Rust rendering service.

| Method Name | Request Type | Response Type | Description |
| ----------- | ------------ | ------------- | ------------|
| RenderBatch | [RenderBatchRequest](#pidgr-v1-RenderBatchRequest) | [RenderBatchResponse](#pidgr-v1-RenderBatchResponse) stream | Render a template for multiple users, streaming results as each completes. Authorization: Internal server-to-server only. Not exposed to external clients. |

 



<a name="pidgr_v1_replay-proto"></a>
<p align="right"><a href="#top">Top</a></p>

## pidgr/v1/replay.proto



<a name="pidgr-v1-GetSessionSnapshotsRequest"></a>

### GetSessionSnapshotsRequest
Request to fetch rrweb snapshot events for a recording.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| recording_id | [string](#string) |  | PostHog recording ID. Constraints: Max length 200 characters. |






<a name="pidgr-v1-GetSessionSnapshotsResponse"></a>

### GetSessionSnapshotsResponse
Response containing rrweb snapshot events.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| snapshot_data | [string](#string) |  | JSON-encoded array of rrweb eventWithTime objects. Clients parse this JSON to feed into rrweb-player. |






<a name="pidgr-v1-ListSessionRecordingsRequest"></a>

### ListSessionRecordingsRequest
Request to list session recordings.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| campaign_id | [string](#string) |  | Optional: filter recordings by campaign ID (mapped to PostHog property filter). Constraints: UUID format (36 characters). |
| date_from | [google.protobuf.Timestamp](#google-protobuf-Timestamp) |  | Optional: start of the time range filter (inclusive). |
| date_to | [google.protobuf.Timestamp](#google-protobuf-Timestamp) |  | Optional: end of the time range filter (inclusive). |
| pagination | [Pagination](#pidgr-v1-Pagination) |  | Pagination parameters. |






<a name="pidgr-v1-ListSessionRecordingsResponse"></a>

### ListSessionRecordingsResponse
Response containing a page of session recordings.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| recordings | [SessionRecording](#pidgr-v1-SessionRecording) | repeated | List of session recordings in this page. |
| pagination_meta | [PaginationMeta](#pidgr-v1-PaginationMeta) |  | Pagination metadata for fetching subsequent pages. |






<a name="pidgr-v1-SessionRecording"></a>

### SessionRecording
A session recording summary from PostHog.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| id | [string](#string) |  | PostHog recording ID. |
| person_distinct_id | [string](#string) |  | PostHog person distinct ID (maps to a pidgr user). |
| start_time | [google.protobuf.Timestamp](#google-protobuf-Timestamp) |  | Timestamp when the recording started. |
| end_time | [google.protobuf.Timestamp](#google-protobuf-Timestamp) |  | Timestamp when the recording ended. |
| duration_seconds | [int32](#int32) |  | Duration of the recording in seconds. |
| activity_score | [float](#float) |  | PostHog activity score (0.0–1.0). |





 

 

 


<a name="pidgr-v1-ReplayService"></a>

### ReplayService
Proxies PostHog&#39;s session recording API, keeping the Personal API Key server-side.
All data is fetched from PostHog on demand; no recording data is stored in pidgr.

| Method Name | Request Type | Response Type | Description |
| ----------- | ------------ | ------------- | ------------|
| ListSessionRecordings | [ListSessionRecordingsRequest](#pidgr-v1-ListSessionRecordingsRequest) | [ListSessionRecordingsResponse](#pidgr-v1-ListSessionRecordingsResponse) | List recent session recordings with optional campaign and time range filters. Authorization: Requires CAMPAIGNS_READ permission. |
| GetSessionSnapshots | [GetSessionSnapshotsRequest](#pidgr-v1-GetSessionSnapshotsRequest) | [GetSessionSnapshotsResponse](#pidgr-v1-GetSessionSnapshotsResponse) | Fetch the full rrweb snapshot data for a single recording. Authorization: Requires CAMPAIGNS_READ permission. |

 



<a name="pidgr_v1_role-proto"></a>
<p align="right"><a href="#top">Top</a></p>

## pidgr/v1/role.proto



<a name="pidgr-v1-CreateRoleRequest"></a>

### CreateRoleRequest
Request to create a new role in the caller&#39;s organization.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| name | [string](#string) |  | Display name for the role (e.g. &#34;Team Lead&#34;). Required. A slug is auto-generated from the name. |
| permissions | [Permission](#pidgr-v1-Permission) | repeated | Initial permission set for the role. PERMISSION_UNSPECIFIED values are rejected. |






<a name="pidgr-v1-CreateRoleResponse"></a>

### CreateRoleResponse
Response after creating a role.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| role | [Role](#pidgr-v1-Role) |  | The newly created role with its generated slug and permission set. |






<a name="pidgr-v1-DeleteRoleRequest"></a>

### DeleteRoleRequest
Request to delete a role.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| role_id | [string](#string) |  | ID of the role to delete. Required. |






<a name="pidgr-v1-DeleteRoleResponse"></a>

### DeleteRoleResponse
Response after deleting a role.






<a name="pidgr-v1-ListRolesRequest"></a>

### ListRolesRequest
Request to list all roles in the caller&#39;s organization.






<a name="pidgr-v1-ListRolesResponse"></a>

### ListRolesResponse
Response containing the organization&#39;s roles.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| roles | [Role](#pidgr-v1-Role) | repeated | All roles in the organization, including their permission sets. |






<a name="pidgr-v1-UpdateRoleRequest"></a>

### UpdateRoleRequest
Request to update a role&#39;s name and/or permissions.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| role_id | [string](#string) |  | ID of the role to update. Required. |
| name | [string](#string) |  | New display name. If empty, the name is not changed. |
| permissions | [Permission](#pidgr-v1-Permission) | repeated | New permission set (replaces existing permissions entirely). If empty, permissions are not changed. PERMISSION_UNSPECIFIED values are rejected. |






<a name="pidgr-v1-UpdateRoleResponse"></a>

### UpdateRoleResponse
Response after updating a role.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| role | [Role](#pidgr-v1-Role) |  | The updated role. |





 

 

 


<a name="pidgr-v1-RoleService"></a>

### RoleService
Manages roles and their permissions within an organization.
All RPCs operate within the caller&#39;s org (extracted from JWT).

| Method Name | Request Type | Response Type | Description |
| ----------- | ------------ | ------------- | ------------|
| ListRoles | [ListRolesRequest](#pidgr-v1-ListRolesRequest) | [ListRolesResponse](#pidgr-v1-ListRolesResponse) | List all roles in the organization with their permission sets. Authorization: Requires PERMISSION_ORG_READ. |
| CreateRole | [CreateRoleRequest](#pidgr-v1-CreateRoleRequest) | [CreateRoleResponse](#pidgr-v1-CreateRoleResponse) | Create a new custom role with a name and initial permissions. Slug is auto-generated from the name and immutable after creation. Authorization: Requires PERMISSION_MEMBERS_MANAGE. |
| UpdateRole | [UpdateRoleRequest](#pidgr-v1-UpdateRoleRequest) | [UpdateRoleResponse](#pidgr-v1-UpdateRoleResponse) | Update a role&#39;s name and/or permissions. System roles (is_system=true) cannot be updated. Authorization: Requires PERMISSION_MEMBERS_MANAGE. |
| DeleteRole | [DeleteRoleRequest](#pidgr-v1-DeleteRoleRequest) | [DeleteRoleResponse](#pidgr-v1-DeleteRoleResponse) | Delete a role. Fails if any users are assigned to it. System roles (is_system=true) cannot be deleted. Authorization: Requires PERMISSION_MEMBERS_MANAGE. |

 



<a name="pidgr_v1_sso-proto"></a>
<p align="right"><a href="#top">Top</a></p>

## pidgr/v1/sso.proto



<a name="pidgr-v1-CheckSSOByDomainRequest"></a>

### CheckSSOByDomainRequest
Request to check if an email domain has SSO configured.
This RPC is pre-authentication — no JWT required.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| email | [string](#string) |  | Email address to check. The domain part is extracted. Constraints: Max length 254 characters (RFC 5321). |






<a name="pidgr-v1-CheckSSOByDomainResponse"></a>

### CheckSSOByDomainResponse
Response for SSO domain check.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| sso_enabled | [bool](#bool) |  | Whether SSO is enabled for the email&#39;s domain. |
| provider_name | [string](#string) |  | Cognito identity provider name for signInWithRedirect. Empty if sso_enabled is false. |






<a name="pidgr-v1-CreateSSOProviderRequest"></a>

### CreateSSOProviderRequest
Request to create an SSO provider for the organization.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| domain | [string](#string) |  | Email domain to associate (e.g. &#34;acme.com&#34;). Constraints: Max length 253 characters (RFC 1035). |
| type | [SSOProviderType](#pidgr-v1-SSOProviderType) |  | Type of identity provider. |
| metadata_url | [string](#string) |  | SAML metadata URL or OIDC discovery URL. Constraints: Max length 2048 characters. HTTPS required. |
| attribute_mapping | [SSOAttributeMapping](#pidgr-v1-SSOAttributeMapping) |  | Optional custom SAML attribute name overrides. When omitted, attribute names are auto-detected from the metadata URL. |






<a name="pidgr-v1-CreateSSOProviderResponse"></a>

### CreateSSOProviderResponse
Response after creating an SSO provider.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| provider | [SSOProvider](#pidgr-v1-SSOProvider) |  | The newly created SSO provider. |






<a name="pidgr-v1-DeleteSSOProviderRequest"></a>

### DeleteSSOProviderRequest
Request to delete the organization&#39;s SSO provider.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| provider_id | [string](#string) |  | ID of the provider to delete. |






<a name="pidgr-v1-DeleteSSOProviderResponse"></a>

### DeleteSSOProviderResponse
Response after deleting an SSO provider.






<a name="pidgr-v1-GetSSOProviderRequest"></a>

### GetSSOProviderRequest
Request to get the SSO provider for the organization.
Returns the provider if one is configured, or empty if not.






<a name="pidgr-v1-GetSSOProviderResponse"></a>

### GetSSOProviderResponse
Response containing the organization&#39;s SSO provider.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| provider | [SSOProvider](#pidgr-v1-SSOProvider) |  | The organization&#39;s SSO provider, or null if not configured. |






<a name="pidgr-v1-SSOAttributeMapping"></a>

### SSOAttributeMapping
Custom SAML attribute name overrides for identity providers that use
non-standard attribute names. When provided, these override the
auto-detected values from the metadata URL host.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| email | [string](#string) |  | SAML attribute name for the user&#39;s email address. |
| given_name | [string](#string) |  | SAML attribute name for the user&#39;s first name. |
| family_name | [string](#string) |  | SAML attribute name for the user&#39;s last name. |






<a name="pidgr-v1-SSOProvider"></a>

### SSOProvider
An SSO identity provider configured for an organization.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| id | [string](#string) |  | Unique identifier for the provider. |
| domain | [string](#string) |  | Email domain that triggers this SSO provider (e.g. &#34;acme.com&#34;). Constraints: Max length 253 characters (RFC 1035). |
| type | [SSOProviderType](#pidgr-v1-SSOProviderType) |  | Type of identity provider. |
| metadata_url | [string](#string) |  | SAML metadata URL or OIDC discovery URL. Constraints: Max length 2048 characters. HTTPS required. |
| cognito_provider_name | [string](#string) |  | Name of the identity provider in Cognito (used for signInWithRedirect). Set by the API when the Cognito IdP is created. |
| created_at | [google.protobuf.Timestamp](#google-protobuf-Timestamp) |  | Timestamp when the provider was created. |
| updated_at | [google.protobuf.Timestamp](#google-protobuf-Timestamp) |  | Timestamp when the provider was last updated. |
| attribute_mapping | [SSOAttributeMapping](#pidgr-v1-SSOAttributeMapping) |  | Optional custom SAML attribute name overrides. |





 


<a name="pidgr-v1-SSOProviderType"></a>

### SSOProviderType
Type of SSO identity provider.

| Name | Number | Description |
| ---- | ------ | ----------- |
| SSO_PROVIDER_TYPE_UNSPECIFIED | 0 | Default value; not a valid type. |
| SSO_PROVIDER_TYPE_SAML | 1 | SAML 2.0 identity provider (e.g. Okta, Azure AD). |
| SSO_PROVIDER_TYPE_OIDC | 2 | OpenID Connect identity provider (e.g. Google Workspace, Auth0). |


 

 


<a name="pidgr-v1-SSOService"></a>

### SSOService
Manages SSO identity provider configuration for organizations.

| Method Name | Request Type | Response Type | Description |
| ----------- | ------------ | ------------- | ------------|
| CheckSSOByDomain | [CheckSSOByDomainRequest](#pidgr-v1-CheckSSOByDomainRequest) | [CheckSSOByDomainResponse](#pidgr-v1-CheckSSOByDomainResponse) | Check if an email domain has SSO configured. This is a pre-authentication endpoint — no JWT required. Authorization: None (public). |
| CreateSSOProvider | [CreateSSOProviderRequest](#pidgr-v1-CreateSSOProviderRequest) | [CreateSSOProviderResponse](#pidgr-v1-CreateSSOProviderResponse) | Create an SSO provider for the organization. Validates the metadata URL before saving. Creates the corresponding Cognito identity provider. Authorization: Requires PERMISSION_ORG_WRITE. |
| GetSSOProvider | [GetSSOProviderRequest](#pidgr-v1-GetSSOProviderRequest) | [GetSSOProviderResponse](#pidgr-v1-GetSSOProviderResponse) | Get the organization&#39;s SSO provider configuration. Authorization: Requires PERMISSION_ORG_READ. |
| DeleteSSOProvider | [DeleteSSOProviderRequest](#pidgr-v1-DeleteSSOProviderRequest) | [DeleteSSOProviderResponse](#pidgr-v1-DeleteSSOProviderResponse) | Delete the organization&#39;s SSO provider. Deletes the corresponding Cognito identity provider. Users with that domain fall back to passkey/OTP. Authorization: Requires PERMISSION_ORG_WRITE. |

 



<a name="pidgr_v1_template-proto"></a>
<p align="right"><a href="#top">Top</a></p>

## pidgr/v1/template.proto



<a name="pidgr-v1-CreateTemplateRequest"></a>

### CreateTemplateRequest
Request to create a new template.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| name | [string](#string) |  | Human-readable template name (admin-facing label). Constraints: Max length 200 characters. |
| body | [string](#string) |  | Template body with {{variable}} placeholders. Constraints: Max length 50000 characters. |
| variables | [TemplateVariable](#pidgr-v1-TemplateVariable) | repeated | Variables available for substitution in the body. |
| title | [string](#string) |  | User-facing title shown as the message subject to recipients. Constraints: Max length 200 characters. |






<a name="pidgr-v1-CreateTemplateResponse"></a>

### CreateTemplateResponse
Response after creating a template.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| template | [Template](#pidgr-v1-Template) |  | The newly created template (version 1). |






<a name="pidgr-v1-GetTemplateRequest"></a>

### GetTemplateRequest
Request to retrieve a specific template version.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| template_id | [string](#string) |  | ID of the template to retrieve. |
| version | [int32](#int32) |  | Version to retrieve. 0 returns the latest version. |






<a name="pidgr-v1-GetTemplateResponse"></a>

### GetTemplateResponse
Response containing the requested template.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| template | [Template](#pidgr-v1-Template) |  | The requested template. |






<a name="pidgr-v1-ListTemplatesRequest"></a>

### ListTemplatesRequest
Request to list templates with pagination.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| pagination | [Pagination](#pidgr-v1-Pagination) |  | Pagination parameters. |






<a name="pidgr-v1-ListTemplatesResponse"></a>

### ListTemplatesResponse
Response containing a page of templates.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| templates | [Template](#pidgr-v1-Template) | repeated | List of templates in this page (latest version of each). |
| pagination_meta | [PaginationMeta](#pidgr-v1-PaginationMeta) |  | Pagination metadata for fetching subsequent pages. |






<a name="pidgr-v1-Template"></a>

### Template
A versioned message template with variable placeholders.
Templates are append-only — updates create new versions.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| id | [string](#string) |  | Unique identifier for the template. |
| name | [string](#string) |  | Human-readable template name (admin-facing label). Constraints: Max length 200 characters. |
| body | [string](#string) |  | Template body with {{variable}} placeholders for substitution. Constraints: Max length 50000 characters. |
| variables | [TemplateVariable](#pidgr-v1-TemplateVariable) | repeated | Variables that can be substituted into the template body. |
| version | [int32](#int32) |  | Version number (auto-incremented on each update). |
| created_at | [google.protobuf.Timestamp](#google-protobuf-Timestamp) |  | Timestamp when this version was created. |
| updated_at | [google.protobuf.Timestamp](#google-protobuf-Timestamp) |  | Timestamp of the most recent update (same as created_at for the latest version). |
| title | [string](#string) |  | User-facing title shown as the message subject to recipients. Serves as the default title; campaigns can override it. Constraints: Max length 200 characters. |






<a name="pidgr-v1-TemplateVariable"></a>

### TemplateVariable
A variable placeholder within a template that gets substituted during rendering.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| name | [string](#string) |  | Variable name used in the template body (e.g. &#34;employee_name&#34;). Constraints: Max length 100 characters. |
| description | [string](#string) |  | Human-readable description of what this variable represents. Constraints: Max length 500 characters. |
| required | [bool](#bool) |  | Whether this variable must be provided during rendering. |






<a name="pidgr-v1-UpdateTemplateRequest"></a>

### UpdateTemplateRequest
Request to update a template, creating a new version.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| template_id | [string](#string) |  | ID of the template to update. |
| body | [string](#string) |  | New template body with {{variable}} placeholders. Constraints: Max length 50000 characters. |
| variables | [TemplateVariable](#pidgr-v1-TemplateVariable) | repeated | Updated variables for substitution. |






<a name="pidgr-v1-UpdateTemplateResponse"></a>

### UpdateTemplateResponse
Response after updating a template.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| template | [Template](#pidgr-v1-Template) |  | The updated template with incremented version number. |





 

 

 


<a name="pidgr-v1-TemplateService"></a>

### TemplateService
Manages versioned message templates used by campaigns.
Templates are append-only — updates create new versions while preserving history.

| Method Name | Request Type | Response Type | Description |
| ----------- | ------------ | ------------- | ------------|
| CreateTemplate | [CreateTemplateRequest](#pidgr-v1-CreateTemplateRequest) | [CreateTemplateResponse](#pidgr-v1-CreateTemplateResponse) | Create a new template with a body and variable definitions. Authorization: Requires MANAGER&#43; role. |
| UpdateTemplate | [UpdateTemplateRequest](#pidgr-v1-UpdateTemplateRequest) | [UpdateTemplateResponse](#pidgr-v1-UpdateTemplateResponse) | Update an existing template, creating a new version. Authorization: Requires MANAGER&#43; role. |
| GetTemplate | [GetTemplateRequest](#pidgr-v1-GetTemplateRequest) | [GetTemplateResponse](#pidgr-v1-GetTemplateResponse) | Retrieve a specific template by ID and optional version. Authorization: Authenticated user within the organization. |
| ListTemplates | [ListTemplatesRequest](#pidgr-v1-ListTemplatesRequest) | [ListTemplatesResponse](#pidgr-v1-ListTemplatesResponse) | List all templates for the organization with pagination. Authorization: Authenticated user within the organization. |

 



## Scalar Value Types

| .proto Type | Notes | C++ | Java | Python | Go | C# | PHP | Ruby |
| ----------- | ----- | --- | ---- | ------ | -- | -- | --- | ---- |
| <a name="double" /> double |  | double | double | float | float64 | double | float | Float |
| <a name="float" /> float |  | float | float | float | float32 | float | float | Float |
| <a name="int32" /> int32 | Uses variable-length encoding. Inefficient for encoding negative numbers – if your field is likely to have negative values, use sint32 instead. | int32 | int | int | int32 | int | integer | Bignum or Fixnum (as required) |
| <a name="int64" /> int64 | Uses variable-length encoding. Inefficient for encoding negative numbers – if your field is likely to have negative values, use sint64 instead. | int64 | long | int/long | int64 | long | integer/string | Bignum |
| <a name="uint32" /> uint32 | Uses variable-length encoding. | uint32 | int | int/long | uint32 | uint | integer | Bignum or Fixnum (as required) |
| <a name="uint64" /> uint64 | Uses variable-length encoding. | uint64 | long | int/long | uint64 | ulong | integer/string | Bignum or Fixnum (as required) |
| <a name="sint32" /> sint32 | Uses variable-length encoding. Signed int value. These more efficiently encode negative numbers than regular int32s. | int32 | int | int | int32 | int | integer | Bignum or Fixnum (as required) |
| <a name="sint64" /> sint64 | Uses variable-length encoding. Signed int value. These more efficiently encode negative numbers than regular int64s. | int64 | long | int/long | int64 | long | integer/string | Bignum |
| <a name="fixed32" /> fixed32 | Always four bytes. More efficient than uint32 if values are often greater than 2^28. | uint32 | int | int | uint32 | uint | integer | Bignum or Fixnum (as required) |
| <a name="fixed64" /> fixed64 | Always eight bytes. More efficient than uint64 if values are often greater than 2^56. | uint64 | long | int/long | uint64 | ulong | integer/string | Bignum |
| <a name="sfixed32" /> sfixed32 | Always four bytes. | int32 | int | int | int32 | int | integer | Bignum or Fixnum (as required) |
| <a name="sfixed64" /> sfixed64 | Always eight bytes. | int64 | long | int/long | int64 | long | integer/string | Bignum |
| <a name="bool" /> bool |  | bool | boolean | boolean | bool | bool | boolean | TrueClass/FalseClass |
| <a name="string" /> string | A string must always contain UTF-8 encoded or 7-bit ASCII text. | string | String | str/unicode | string | string | string | String (UTF-8) |
| <a name="bytes" /> bytes | May contain any arbitrary sequence of bytes. | string | ByteString | str | []byte | ByteString | string | String (ASCII-8BIT) |

