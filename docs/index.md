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
    - [SendNotificationConfig](#pidgr-v1-SendNotificationConfig)
    - [SendReminderConfig](#pidgr-v1-SendReminderConfig)
    - [WorkflowDefinition](#pidgr-v1-WorkflowDefinition)
    - [WorkflowStep](#pidgr-v1-WorkflowStep)
    - [WorkflowStep.TransitionsEntry](#pidgr-v1-WorkflowStep-TransitionsEntry)
  
    - [ActionType](#pidgr-v1-ActionType)
    - [CampaignStatus](#pidgr-v1-CampaignStatus)
    - [DeliveryStatus](#pidgr-v1-DeliveryStatus)
    - [Platform](#pidgr-v1-Platform)
    - [StepType](#pidgr-v1-StepType)
    - [UserRole](#pidgr-v1-UserRole)
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
  
- [pidgr/v1/inbox.proto](#pidgr_v1_inbox-proto)
    - [GetMessageRequest](#pidgr-v1-GetMessageRequest)
    - [GetMessageResponse](#pidgr-v1-GetMessageResponse)
    - [InboxEntry](#pidgr-v1-InboxEntry)
    - [MarkReadRequest](#pidgr-v1-MarkReadRequest)
    - [MarkReadResponse](#pidgr-v1-MarkReadResponse)
    - [SyncRequest](#pidgr-v1-SyncRequest)
    - [SyncResponse](#pidgr-v1-SyncResponse)
  
    - [InboxService](#pidgr-v1-InboxService)
  
- [pidgr/v1/render.proto](#pidgr_v1_render-proto)
    - [RenderBatchRequest](#pidgr-v1-RenderBatchRequest)
    - [RenderBatchResponse](#pidgr-v1-RenderBatchResponse)
    - [UserRenderContext](#pidgr-v1-UserRenderContext)
    - [UserRenderContext.VariablesEntry](#pidgr-v1-UserRenderContext-VariablesEntry)
  
    - [RenderService](#pidgr-v1-RenderService)
  
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
  
- [pidgr/v1/user_org.proto](#pidgr_v1_user_org-proto)
    - [CreateOrganizationRequest](#pidgr-v1-CreateOrganizationRequest)
    - [CreateOrganizationResponse](#pidgr-v1-CreateOrganizationResponse)
    - [GetOrganizationRequest](#pidgr-v1-GetOrganizationRequest)
    - [GetOrganizationResponse](#pidgr-v1-GetOrganizationResponse)
    - [GetUserRequest](#pidgr-v1-GetUserRequest)
    - [GetUserResponse](#pidgr-v1-GetUserResponse)
    - [InviteUserRequest](#pidgr-v1-InviteUserRequest)
    - [InviteUserResponse](#pidgr-v1-InviteUserResponse)
    - [ListUsersRequest](#pidgr-v1-ListUsersRequest)
    - [ListUsersResponse](#pidgr-v1-ListUsersResponse)
    - [Organization](#pidgr-v1-Organization)
    - [UpdateOrganizationRequest](#pidgr-v1-UpdateOrganizationRequest)
    - [UpdateOrganizationResponse](#pidgr-v1-UpdateOrganizationResponse)
    - [User](#pidgr-v1-User)
  
    - [UserOrgService](#pidgr-v1-UserOrgService)
  
- [Scalar Value Types](#scalar-value-types)



<a name="pidgr_v1_action-proto"></a>
<p align="right"><a href="#top">Top</a></p>

## pidgr/v1/action.proto



<a name="pidgr-v1-SubmitActionRequest"></a>

### SubmitActionRequest
Request to submit a user action on a delivered message.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| delivery_id | [string](#string) |  | ID of the delivery the user is acting on. |
| action_id | [string](#string) |  | ID of the action being performed (matches MessageAction.id). |
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
| headers | [CallWebhookConfig.HeadersEntry](#pidgr-v1-CallWebhookConfig-HeadersEntry) | repeated | Additional HTTP headers to include in the webhook request. |






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



<a name="pidgr-v1-UserRole"></a>

### UserRole
Role assigned to a user within an organization.

| Name | Number | Description |
| ---- | ------ | ----------- |
| USER_ROLE_UNSPECIFIED | 0 | Default value; not a valid role. |
| USER_ROLE_ADMIN | 1 | Full administrative privileges. |
| USER_ROLE_MANAGER | 2 | Can create and manage campaigns. |
| USER_ROLE_EMPLOYEE | 3 | Standard recipient; receives campaign messages. |



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
| id | [string](#string) |  | Unique identifier for the campaign. |
| name | [string](#string) |  | Human-readable campaign name. Constraints: Max length 200 characters. |
| template_id | [string](#string) |  | ID of the template used to render messages. |
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






<a name="pidgr-v1-CancelCampaignRequest"></a>

### CancelCampaignRequest
Request to cancel a running campaign.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| campaign_id | [string](#string) |  | ID of the campaign to cancel. |






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
| name | [string](#string) |  | Human-readable campaign name. Constraints: Max length 200 characters. |
| template_id | [string](#string) |  | ID of the template to use for rendering messages. |
| template_version | [int32](#int32) |  | Version of the template to pin for this campaign. |
| user_ids | [string](#string) | repeated | List of user IDs that form the campaign audience. Constraints: Max 100000 items. |
| workflow | [WorkflowDefinition](#pidgr-v1-WorkflowDefinition) |  | Workflow DAG defining the campaign&#39;s automation steps. |






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
| id | [string](#string) |  | Unique identifier for this delivery. |
| user_id | [string](#string) |  | ID of the recipient user. |
| campaign_id | [string](#string) |  | ID of the campaign this delivery belongs to. |
| status | [DeliveryStatus](#pidgr-v1-DeliveryStatus) |  | Current delivery status. |
| delivered_at | [google.protobuf.Timestamp](#google-protobuf-Timestamp) |  | Timestamp when the message was delivered to the device. |
| read_at | [google.protobuf.Timestamp](#google-protobuf-Timestamp) |  | Timestamp when the recipient read the message. |
| acted_at | [google.protobuf.Timestamp](#google-protobuf-Timestamp) |  | Timestamp when the recipient performed the required action. |






<a name="pidgr-v1-GetCampaignRequest"></a>

### GetCampaignRequest
Request to retrieve a single campaign by ID.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| campaign_id | [string](#string) |  | ID of the campaign to retrieve. |






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
| campaign_id | [string](#string) |  | ID of the campaign to list deliveries for. |
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
| campaign_id | [string](#string) |  | ID of the campaign to start. |






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
| device_id | [string](#string) |  | ID of the device to deactivate. |






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
| device_id | [string](#string) |  | Unique identifier for this device. |
| user_id | [string](#string) |  | ID of the user who owns this device. |
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
| device_id | [string](#string) |  | Client-generated unique device identifier. |
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

 



<a name="pidgr_v1_inbox-proto"></a>
<p align="right"><a href="#top">Top</a></p>

## pidgr/v1/inbox.proto



<a name="pidgr-v1-GetMessageRequest"></a>

### GetMessageRequest
Request to retrieve a single message by delivery ID.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| delivery_id | [string](#string) |  | ID of the delivery to retrieve. |






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
| delivery_id | [string](#string) |  | ID of the delivery record for this inbox entry. |
| message | [Message](#pidgr-v1-Message) |  | The fully rendered message content. |
| status | [DeliveryStatus](#pidgr-v1-DeliveryStatus) |  | Current delivery status (e.g. DELIVERED, ACKNOWLEDGED). |
| read | [bool](#bool) |  | Whether the user has read this message. |
| received_at | [google.protobuf.Timestamp](#google-protobuf-Timestamp) |  | Timestamp when the message was received in the inbox. |






<a name="pidgr-v1-MarkReadRequest"></a>

### MarkReadRequest
Request to mark a message as read.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| delivery_id | [string](#string) |  | ID of the delivery to mark as read. |






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
| users | [UserRenderContext](#pidgr-v1-UserRenderContext) | repeated | Per-user rendering contexts with variable substitutions. |






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
| variables | [UserRenderContext.VariablesEntry](#pidgr-v1-UserRenderContext-VariablesEntry) | repeated | Variable name-value pairs to substitute into the template. |






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
| RenderBatch | [RenderBatchRequest](#pidgr-v1-RenderBatchRequest) | [RenderBatchResponse](#pidgr-v1-RenderBatchResponse) stream | Render a template for multiple users, streaming results as each completes. |

 



<a name="pidgr_v1_template-proto"></a>
<p align="right"><a href="#top">Top</a></p>

## pidgr/v1/template.proto



<a name="pidgr-v1-CreateTemplateRequest"></a>

### CreateTemplateRequest
Request to create a new template.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| name | [string](#string) |  | Human-readable template name. Constraints: Max length 100 characters. |
| body | [string](#string) |  | Template body with {{variable}} placeholders. Constraints: Max length 50000 characters. |
| variables | [TemplateVariable](#pidgr-v1-TemplateVariable) | repeated | Variables available for substitution in the body. |






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
| name | [string](#string) |  | Human-readable template name. Constraints: Max length 100 characters. |
| body | [string](#string) |  | Template body with {{variable}} placeholders for substitution. Constraints: Max length 50000 characters. |
| variables | [TemplateVariable](#pidgr-v1-TemplateVariable) | repeated | Variables that can be substituted into the template body. |
| version | [int32](#int32) |  | Version number (auto-incremented on each update). |
| created_at | [google.protobuf.Timestamp](#google-protobuf-Timestamp) |  | Timestamp when this version was created. |
| updated_at | [google.protobuf.Timestamp](#google-protobuf-Timestamp) |  | Timestamp of the most recent update (same as created_at for the latest version). |






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

 



<a name="pidgr_v1_user_org-proto"></a>
<p align="right"><a href="#top">Top</a></p>

## pidgr/v1/user_org.proto



<a name="pidgr-v1-CreateOrganizationRequest"></a>

### CreateOrganizationRequest
Request to create a new organization with an admin user.
Requires API key authentication (service-to-service).


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| name | [string](#string) |  | Name for the new organization. Constraints: Max length 200 characters. |
| admin_email | [string](#string) |  | Email address for the initial admin user. Constraints: Max length 254 characters (RFC 5321). |






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
| role | [UserRole](#pidgr-v1-UserRole) |  | Role to assign to the new user. |






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






<a name="pidgr-v1-Organization"></a>

### Organization
An organization (tenant) in the Pidgr platform.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| id | [string](#string) |  | Unique identifier for the organization. |
| name | [string](#string) |  | Organization display name. Constraints: Max length 200 characters. |
| default_workflow | [WorkflowDefinition](#pidgr-v1-WorkflowDefinition) |  | Default workflow used when campaigns don&#39;t specify one. |
| created_at | [google.protobuf.Timestamp](#google-protobuf-Timestamp) |  | Timestamp when the organization was created. |






<a name="pidgr-v1-UpdateOrganizationRequest"></a>

### UpdateOrganizationRequest
Request to update organization settings.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| name | [string](#string) |  | New organization name. Empty string leaves unchanged. Constraints: Max length 200 characters. |
| default_workflow | [WorkflowDefinition](#pidgr-v1-WorkflowDefinition) |  | New default workflow definition. Null leaves unchanged. |






<a name="pidgr-v1-UpdateOrganizationResponse"></a>

### UpdateOrganizationResponse
Response after updating the organization.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| organization | [Organization](#pidgr-v1-Organization) |  | The updated organization. |






<a name="pidgr-v1-User"></a>

### User
A user within an organization.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| id | [string](#string) |  | Unique identifier for the user (internal platform UUID, not Cognito sub). |
| email | [string](#string) |  | User&#39;s email address. Constraints: Max length 254 characters (RFC 5321). |
| name | [string](#string) |  | User&#39;s display name. Constraints: Max length 200 characters. |
| role | [UserRole](#pidgr-v1-UserRole) |  | Role within the organization. |
| status | [UserStatus](#pidgr-v1-UserStatus) |  | Current account status. |
| created_at | [google.protobuf.Timestamp](#google-protobuf-Timestamp) |  | Timestamp when the user was created. |





 

 

 


<a name="pidgr-v1-UserOrgService"></a>

### UserOrgService
Manages users and organizations.
Most RPCs operate within the caller&#39;s org (extracted from JWT).
CreateOrganization requires API key authentication.

| Method Name | Request Type | Response Type | Description |
| ----------- | ------------ | ------------- | ------------|
| CreateOrganization | [CreateOrganizationRequest](#pidgr-v1-CreateOrganizationRequest) | [CreateOrganizationResponse](#pidgr-v1-CreateOrganizationResponse) | Create a new organization with an initial admin user. Requires API key auth. |
| InviteUser | [InviteUserRequest](#pidgr-v1-InviteUserRequest) | [InviteUserResponse](#pidgr-v1-InviteUserResponse) | Invite a new user to the organization via email. Authorization: Requires ADMIN role. |
| GetUser | [GetUserRequest](#pidgr-v1-GetUserRequest) | [GetUserResponse](#pidgr-v1-GetUserResponse) | Retrieve a user by ID within the organization. Authorization: Authenticated user within the organization. |
| ListUsers | [ListUsersRequest](#pidgr-v1-ListUsersRequest) | [ListUsersResponse](#pidgr-v1-ListUsersResponse) | List all users in the organization with pagination. Authorization: Authenticated user within the organization. |
| GetOrganization | [GetOrganizationRequest](#pidgr-v1-GetOrganizationRequest) | [GetOrganizationResponse](#pidgr-v1-GetOrganizationResponse) | Retrieve the organization for the authenticated user. Authorization: Authenticated user within the organization. |
| UpdateOrganization | [UpdateOrganizationRequest](#pidgr-v1-UpdateOrganizationRequest) | [UpdateOrganizationResponse](#pidgr-v1-UpdateOrganizationResponse) | Update organization settings (name, default workflow). Authorization: Requires ADMIN role. |

 



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

