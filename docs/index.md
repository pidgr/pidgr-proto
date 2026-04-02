# Protocol Documentation
<a name="top"></a>

## Table of Contents

- [pidgr/v1/access_code.proto](#pidgr_v1_access_code-proto)
    - [AccessCode](#pidgr-v1-AccessCode)
    - [GenerateAccessCodesRequest](#pidgr-v1-GenerateAccessCodesRequest)
    - [GenerateAccessCodesResponse](#pidgr-v1-GenerateAccessCodesResponse)
    - [ListAccessCodesRequest](#pidgr-v1-ListAccessCodesRequest)
    - [ListAccessCodesResponse](#pidgr-v1-ListAccessCodesResponse)
    - [RevokeAccessCodeRequest](#pidgr-v1-RevokeAccessCodeRequest)
    - [RevokeAccessCodeResponse](#pidgr-v1-RevokeAccessCodeResponse)
  
    - [AccessCodeService](#pidgr-v1-AccessCodeService)
  
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
    - [SendNotificationConfig.CustomVariablesEntry](#pidgr-v1-SendNotificationConfig-CustomVariablesEntry)
    - [SendReminderConfig](#pidgr-v1-SendReminderConfig)
    - [WorkflowDefinition](#pidgr-v1-WorkflowDefinition)
    - [WorkflowStep](#pidgr-v1-WorkflowStep)
    - [WorkflowStep.TransitionsEntry](#pidgr-v1-WorkflowStep-TransitionsEntry)
  
    - [ActionType](#pidgr-v1-ActionType)
    - [CampaignStatus](#pidgr-v1-CampaignStatus)
    - [DeliveryStatus](#pidgr-v1-DeliveryStatus)
    - [Permission](#pidgr-v1-Permission)
    - [Platform](#pidgr-v1-Platform)
    - [StepType](#pidgr-v1-StepType)
  
- [pidgr/v1/api_key.proto](#pidgr_v1_api_key-proto)
    - [ApiKey](#pidgr-v1-ApiKey)
    - [CreateApiKeyRequest](#pidgr-v1-CreateApiKeyRequest)
    - [CreateApiKeyResponse](#pidgr-v1-CreateApiKeyResponse)
    - [ListApiKeysRequest](#pidgr-v1-ListApiKeysRequest)
    - [ListApiKeysResponse](#pidgr-v1-ListApiKeysResponse)
    - [RevokeApiKeyRequest](#pidgr-v1-RevokeApiKeyRequest)
    - [RevokeApiKeyResponse](#pidgr-v1-RevokeApiKeyResponse)
  
    - [ApiKeyService](#pidgr-v1-ApiKeyService)
  
- [pidgr/v1/privacy.proto](#pidgr_v1_privacy-proto)
    - [CancelDeletionRequest](#pidgr-v1-CancelDeletionRequest)
    - [CancelDeletionResponse](#pidgr-v1-CancelDeletionResponse)
    - [DeleteUserDataRequest](#pidgr-v1-DeleteUserDataRequest)
    - [DeleteUserDataResponse](#pidgr-v1-DeleteUserDataResponse)
    - [ExportUserDataRequest](#pidgr-v1-ExportUserDataRequest)
    - [ExportUserDataResponse](#pidgr-v1-ExportUserDataResponse)
    - [GetDataExistenceConfirmationRequest](#pidgr-v1-GetDataExistenceConfirmationRequest)
    - [GetDataExistenceConfirmationResponse](#pidgr-v1-GetDataExistenceConfirmationResponse)
    - [ImmediateDeleteRequest](#pidgr-v1-ImmediateDeleteRequest)
    - [ImmediateDeleteResponse](#pidgr-v1-ImmediateDeleteResponse)
    - [ListPrivacyRequestsRequest](#pidgr-v1-ListPrivacyRequestsRequest)
    - [ListPrivacyRequestsResponse](#pidgr-v1-ListPrivacyRequestsResponse)
    - [PrivacyRequest](#pidgr-v1-PrivacyRequest)
    - [PrivacyRequest.MetadataEntry](#pidgr-v1-PrivacyRequest-MetadataEntry)
    - [RectifyUserDataRequest](#pidgr-v1-RectifyUserDataRequest)
    - [RectifyUserDataRequest.CorrectionsEntry](#pidgr-v1-RectifyUserDataRequest-CorrectionsEntry)
    - [RectifyUserDataResponse](#pidgr-v1-RectifyUserDataResponse)
    - [RestrictProcessingRequest](#pidgr-v1-RestrictProcessingRequest)
    - [RestrictProcessingResponse](#pidgr-v1-RestrictProcessingResponse)
  
    - [PrivacyRequestStatus](#pidgr-v1-PrivacyRequestStatus)
  
    - [PrivacyService](#pidgr-v1-PrivacyService)
  
- [pidgr/v1/audit.proto](#pidgr_v1_audit-proto)
    - [AuditEvent](#pidgr-v1-AuditEvent)
    - [AuditEvent.MetadataEntry](#pidgr-v1-AuditEvent-MetadataEntry)
    - [AuditExport](#pidgr-v1-AuditExport)
    - [ExportAuditTrailRequest](#pidgr-v1-ExportAuditTrailRequest)
    - [ExportAuditTrailResponse](#pidgr-v1-ExportAuditTrailResponse)
    - [ListAuditEventsRequest](#pidgr-v1-ListAuditEventsRequest)
    - [ListAuditEventsResponse](#pidgr-v1-ListAuditEventsResponse)
    - [ListAuditExportsRequest](#pidgr-v1-ListAuditExportsRequest)
    - [ListAuditExportsResponse](#pidgr-v1-ListAuditExportsResponse)
  
    - [AuditEventType](#pidgr-v1-AuditEventType)
    - [AuditExportFormat](#pidgr-v1-AuditExportFormat)
  
    - [AuditService](#pidgr-v1-AuditService)
  
- [pidgr/v1/campaign.proto](#pidgr_v1_campaign-proto)
    - [AudienceMember](#pidgr-v1-AudienceMember)
    - [AudienceMember.VariablesEntry](#pidgr-v1-AudienceMember-VariablesEntry)
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
    - [UpdateCampaignRequest](#pidgr-v1-UpdateCampaignRequest)
    - [UpdateCampaignResponse](#pidgr-v1-UpdateCampaignResponse)
  
    - [CampaignService](#pidgr-v1-CampaignService)
  
- [pidgr/v1/device.proto](#pidgr_v1_device-proto)
    - [DeactivateRequest](#pidgr-v1-DeactivateRequest)
    - [DeactivateResponse](#pidgr-v1-DeactivateResponse)
    - [Device](#pidgr-v1-Device)
    - [DeviceSummary](#pidgr-v1-DeviceSummary)
    - [ListDevicesRequest](#pidgr-v1-ListDevicesRequest)
    - [ListDevicesResponse](#pidgr-v1-ListDevicesResponse)
    - [ListMemberDevicesRequest](#pidgr-v1-ListMemberDevicesRequest)
    - [ListMemberDevicesResponse](#pidgr-v1-ListMemberDevicesResponse)
    - [RegisterRequest](#pidgr-v1-RegisterRequest)
    - [RegisterResponse](#pidgr-v1-RegisterResponse)
  
    - [DeviceService](#pidgr-v1-DeviceService)
  
- [pidgr/v1/user.proto](#pidgr_v1_user-proto)
    - [User](#pidgr-v1-User)
    - [UserProfile](#pidgr-v1-UserProfile)
    - [UserProfile.CustomAttributesEntry](#pidgr-v1-UserProfile-CustomAttributesEntry)
    - [UserSettings](#pidgr-v1-UserSettings)
  
    - [ThemePreference](#pidgr-v1-ThemePreference)
    - [UserStatus](#pidgr-v1-UserStatus)
  
- [pidgr/v1/group.proto](#pidgr_v1_group-proto)
    - [AddGroupMembersRequest](#pidgr-v1-AddGroupMembersRequest)
    - [AddGroupMembersResponse](#pidgr-v1-AddGroupMembersResponse)
    - [CreateGroupRequest](#pidgr-v1-CreateGroupRequest)
    - [CreateGroupResponse](#pidgr-v1-CreateGroupResponse)
    - [DeleteGroupRequest](#pidgr-v1-DeleteGroupRequest)
    - [DeleteGroupResponse](#pidgr-v1-DeleteGroupResponse)
    - [GetGroupRequest](#pidgr-v1-GetGroupRequest)
    - [GetGroupResponse](#pidgr-v1-GetGroupResponse)
    - [GetUserGroupMembershipsRequest](#pidgr-v1-GetUserGroupMembershipsRequest)
    - [GetUserGroupMembershipsResponse](#pidgr-v1-GetUserGroupMembershipsResponse)
    - [Group](#pidgr-v1-Group)
    - [ListGroupMembersRequest](#pidgr-v1-ListGroupMembersRequest)
    - [ListGroupMembersResponse](#pidgr-v1-ListGroupMembersResponse)
    - [ListGroupsRequest](#pidgr-v1-ListGroupsRequest)
    - [ListGroupsResponse](#pidgr-v1-ListGroupsResponse)
    - [RemoveGroupMembersRequest](#pidgr-v1-RemoveGroupMembersRequest)
    - [RemoveGroupMembersResponse](#pidgr-v1-RemoveGroupMembersResponse)
    - [UpdateGroupRequest](#pidgr-v1-UpdateGroupRequest)
    - [UpdateGroupResponse](#pidgr-v1-UpdateGroupResponse)
    - [UserGroupMembership](#pidgr-v1-UserGroupMembership)
  
    - [GroupService](#pidgr-v1-GroupService)
  
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
    - [UploadScreenshotRequest](#pidgr-v1-UploadScreenshotRequest)
    - [UploadScreenshotResponse](#pidgr-v1-UploadScreenshotResponse)
  
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
  
- [pidgr/v1/invite_link.proto](#pidgr_v1_invite_link-proto)
    - [CreateInviteLinkRequest](#pidgr-v1-CreateInviteLinkRequest)
    - [CreateInviteLinkResponse](#pidgr-v1-CreateInviteLinkResponse)
    - [InviteLink](#pidgr-v1-InviteLink)
    - [ListInviteLinksRequest](#pidgr-v1-ListInviteLinksRequest)
    - [ListInviteLinksResponse](#pidgr-v1-ListInviteLinksResponse)
    - [RedeemInviteLinkRequest](#pidgr-v1-RedeemInviteLinkRequest)
    - [RedeemInviteLinkResponse](#pidgr-v1-RedeemInviteLinkResponse)
    - [RevokeInviteLinkRequest](#pidgr-v1-RevokeInviteLinkRequest)
    - [RevokeInviteLinkResponse](#pidgr-v1-RevokeInviteLinkResponse)
    - [ValidateInviteLinkRequest](#pidgr-v1-ValidateInviteLinkRequest)
    - [ValidateInviteLinkResponse](#pidgr-v1-ValidateInviteLinkResponse)
  
    - [InviteLinkService](#pidgr-v1-InviteLinkService)
  
- [pidgr/v1/member.proto](#pidgr_v1_member-proto)
    - [BulkInviteResult](#pidgr-v1-BulkInviteResult)
    - [BulkInviteUsersRequest](#pidgr-v1-BulkInviteUsersRequest)
    - [BulkInviteUsersResponse](#pidgr-v1-BulkInviteUsersResponse)
    - [ConfirmPasskeyEnrollmentRequest](#pidgr-v1-ConfirmPasskeyEnrollmentRequest)
    - [ConfirmPasskeyEnrollmentResponse](#pidgr-v1-ConfirmPasskeyEnrollmentResponse)
    - [DeactivateUserRequest](#pidgr-v1-DeactivateUserRequest)
    - [DeactivateUserResponse](#pidgr-v1-DeactivateUserResponse)
    - [GetUserRequest](#pidgr-v1-GetUserRequest)
    - [GetUserResponse](#pidgr-v1-GetUserResponse)
    - [GetUserSettingsRequest](#pidgr-v1-GetUserSettingsRequest)
    - [GetUserSettingsResponse](#pidgr-v1-GetUserSettingsResponse)
    - [InviteUserRequest](#pidgr-v1-InviteUserRequest)
    - [InviteUserResponse](#pidgr-v1-InviteUserResponse)
    - [ListUsersRequest](#pidgr-v1-ListUsersRequest)
    - [ListUsersResponse](#pidgr-v1-ListUsersResponse)
    - [ReactivateUserRequest](#pidgr-v1-ReactivateUserRequest)
    - [ReactivateUserResponse](#pidgr-v1-ReactivateUserResponse)
    - [RevokeInviteRequest](#pidgr-v1-RevokeInviteRequest)
    - [RevokeInviteResponse](#pidgr-v1-RevokeInviteResponse)
    - [UpdateUserProfileRequest](#pidgr-v1-UpdateUserProfileRequest)
    - [UpdateUserProfileResponse](#pidgr-v1-UpdateUserProfileResponse)
    - [UpdateUserRoleRequest](#pidgr-v1-UpdateUserRoleRequest)
    - [UpdateUserRoleResponse](#pidgr-v1-UpdateUserRoleResponse)
    - [UpdateUserSettingsRequest](#pidgr-v1-UpdateUserSettingsRequest)
    - [UpdateUserSettingsResponse](#pidgr-v1-UpdateUserSettingsResponse)
  
    - [MemberService](#pidgr-v1-MemberService)
  
- [pidgr/v1/organization.proto](#pidgr_v1_organization-proto)
    - [CreateOrganizationRequest](#pidgr-v1-CreateOrganizationRequest)
    - [CreateOrganizationResponse](#pidgr-v1-CreateOrganizationResponse)
    - [GetOrganizationRequest](#pidgr-v1-GetOrganizationRequest)
    - [GetOrganizationResponse](#pidgr-v1-GetOrganizationResponse)
    - [Organization](#pidgr-v1-Organization)
    - [RotateAnalyticsSaltRequest](#pidgr-v1-RotateAnalyticsSaltRequest)
    - [RotateAnalyticsSaltResponse](#pidgr-v1-RotateAnalyticsSaltResponse)
    - [SsoAttributeMapping](#pidgr-v1-SsoAttributeMapping)
    - [UpdateAnalyticsEpsilonRequest](#pidgr-v1-UpdateAnalyticsEpsilonRequest)
    - [UpdateAnalyticsEpsilonResponse](#pidgr-v1-UpdateAnalyticsEpsilonResponse)
    - [UpdateOrganizationRequest](#pidgr-v1-UpdateOrganizationRequest)
    - [UpdateOrganizationResponse](#pidgr-v1-UpdateOrganizationResponse)
    - [UpdateSsoAttributeMappingsRequest](#pidgr-v1-UpdateSsoAttributeMappingsRequest)
    - [UpdateSsoAttributeMappingsResponse](#pidgr-v1-UpdateSsoAttributeMappingsResponse)
  
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
    - [SSOProvider](#pidgr-v1-SSOProvider)
    - [SamlAttributeNames](#pidgr-v1-SamlAttributeNames)
  
    - [SSOProviderType](#pidgr-v1-SSOProviderType)
  
    - [SSOService](#pidgr-v1-SSOService)
  
- [pidgr/v1/team.proto](#pidgr_v1_team-proto)
    - [AddTeamMembersRequest](#pidgr-v1-AddTeamMembersRequest)
    - [AddTeamMembersResponse](#pidgr-v1-AddTeamMembersResponse)
    - [CreateTeamRequest](#pidgr-v1-CreateTeamRequest)
    - [CreateTeamResponse](#pidgr-v1-CreateTeamResponse)
    - [DeleteTeamRequest](#pidgr-v1-DeleteTeamRequest)
    - [DeleteTeamResponse](#pidgr-v1-DeleteTeamResponse)
    - [GetTeamRequest](#pidgr-v1-GetTeamRequest)
    - [GetTeamResponse](#pidgr-v1-GetTeamResponse)
    - [ListTeamMembersRequest](#pidgr-v1-ListTeamMembersRequest)
    - [ListTeamMembersResponse](#pidgr-v1-ListTeamMembersResponse)
    - [ListTeamsRequest](#pidgr-v1-ListTeamsRequest)
    - [ListTeamsResponse](#pidgr-v1-ListTeamsResponse)
    - [RemoveTeamMembersRequest](#pidgr-v1-RemoveTeamMembersRequest)
    - [RemoveTeamMembersResponse](#pidgr-v1-RemoveTeamMembersResponse)
    - [Team](#pidgr-v1-Team)
    - [UpdateTeamRequest](#pidgr-v1-UpdateTeamRequest)
    - [UpdateTeamResponse](#pidgr-v1-UpdateTeamResponse)
  
    - [TeamService](#pidgr-v1-TeamService)
  
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
  
    - [TemplateType](#pidgr-v1-TemplateType)
    - [TemplateVariableSource](#pidgr-v1-TemplateVariableSource)
  
    - [TemplateService](#pidgr-v1-TemplateService)
  
- [Scalar Value Types](#scalar-value-types)



<a name="pidgr_v1_access_code-proto"></a>
<p align="right"><a href="#top">Top</a></p>

## pidgr/v1/access_code.proto



<a name="pidgr-v1-AccessCode"></a>

### AccessCode
A pre-generated access code for early access gating.
Codes are single-use: once redeemed during organization creation, they cannot be reused.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| id | [string](#string) |  | Unique identifier for the access code. |
| code | [string](#string) |  | The access code value (e.g. &#34;PIDGR-A3BF7K2N&#34;). Format: PIDGR- followed by 8 alphanumeric characters (excludes 0, O, 1, I for readability). |
| label | [string](#string) |  | Optional human-friendly label for tracking (e.g. &#34;Batch Feb 2026&#34;, &#34;Demo for Acme&#34;). Constraints: Max length 200 characters. |
| created_at | [google.protobuf.Timestamp](#google-protobuf-Timestamp) |  | When the code was generated. |
| redeemed_at | [google.protobuf.Timestamp](#google-protobuf-Timestamp) |  | When the code was redeemed. Empty if not yet redeemed. |
| redeemed_by | [string](#string) |  | Email of the user who redeemed the code. Empty if not yet redeemed. |
| revoked_at | [google.protobuf.Timestamp](#google-protobuf-Timestamp) |  | When the code was revoked. Empty if not revoked. |






<a name="pidgr-v1-GenerateAccessCodesRequest"></a>

### GenerateAccessCodesRequest
Request to generate one or more access codes.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| count | [int32](#int32) |  | Number of codes to generate. Required, must be between 1 and 100. |
| label | [string](#string) |  | Optional label applied to all generated codes. Constraints: Max length 200 characters. |






<a name="pidgr-v1-GenerateAccessCodesResponse"></a>

### GenerateAccessCodesResponse
Response containing the newly generated access codes.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| access_codes | [AccessCode](#pidgr-v1-AccessCode) | repeated | The generated access codes. |






<a name="pidgr-v1-ListAccessCodesRequest"></a>

### ListAccessCodesRequest
Request to list all access codes.






<a name="pidgr-v1-ListAccessCodesResponse"></a>

### ListAccessCodesResponse
Response containing all access codes.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| access_codes | [AccessCode](#pidgr-v1-AccessCode) | repeated | All access codes (active, redeemed, and revoked). |






<a name="pidgr-v1-RevokeAccessCodeRequest"></a>

### RevokeAccessCodeRequest
Request to revoke an access code.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| access_code_id | [string](#string) |  | ID of the access code to revoke. Required. |






<a name="pidgr-v1-RevokeAccessCodeResponse"></a>

### RevokeAccessCodeResponse
Response after revoking an access code.





 

 

 


<a name="pidgr-v1-AccessCodeService"></a>

### AccessCodeService
Manages early access codes for organization creation gating.
All RPCs require API key authentication (platform-level, not org-scoped).

| Method Name | Request Type | Response Type | Description |
| ----------- | ------------ | ------------- | ------------|
| GenerateAccessCodes | [GenerateAccessCodesRequest](#pidgr-v1-GenerateAccessCodesRequest) | [GenerateAccessCodesResponse](#pidgr-v1-GenerateAccessCodesResponse) | Generate one or more access codes with an optional label. Authorization: Requires API key auth (service-level). |
| ListAccessCodes | [ListAccessCodesRequest](#pidgr-v1-ListAccessCodesRequest) | [ListAccessCodesResponse](#pidgr-v1-ListAccessCodesResponse) | List all access codes (active, redeemed, and revoked). Authorization: Requires API key auth (service-level). |
| RevokeAccessCode | [RevokeAccessCodeRequest](#pidgr-v1-RevokeAccessCodeRequest) | [RevokeAccessCodeResponse](#pidgr-v1-RevokeAccessCodeResponse) | Revoke an access code. Revoked codes cannot be used for organization creation. Authorization: Requires API key auth (service-level). |

 



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
Actions drive workflow progression (e.g. ACK completes a wait step).

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
| url | [string](#string) |  | URL to POST campaign context to. Constraints: Max length 2048 characters. Security: HTTPS required in production. Backend MUST reject private, loopback, and link-local addresses to prevent SSRF attacks. |
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
| delay | [string](#string) |  | Duration string for the deadline delay (e.g. &#34;120h&#34;, &#34;72h&#34;). Constraints: Valid range 1m to 8760h (1 year). |






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
| template_id | [string](#string) |  | ID of the template to use for this step&#39;s notification. Empty falls back to campaign-level template_id. Constraints: Max length 36 characters (UUID). |
| template_version | [int32](#int32) |  | Pinned template version for this step. 0 falls back to campaign-level template_version. |
| action_label | [string](#string) |  | Display label for the action button (e.g. &#34;Acknowledge&#34;, &#34;Got it&#34;). Constraints: Max length 50 characters. |
| action_type | [ActionType](#pidgr-v1-ActionType) |  | Action type for this step&#39;s message button. |
| custom_variables | [SendNotificationConfig.CustomVariablesEntry](#pidgr-v1-SendNotificationConfig-CustomVariablesEntry) | repeated | Values for custom-sourced template variables specific to this step. Constraints: Max 100 entries. Key max length 100 characters, value max length 10000 characters. |






<a name="pidgr-v1-SendNotificationConfig-CustomVariablesEntry"></a>

### SendNotificationConfig.CustomVariablesEntry



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| key | [string](#string) |  |  |
| value | [string](#string) |  |  |






<a name="pidgr-v1-SendReminderConfig"></a>

### SendReminderConfig
Configuration for a step that sends a one-time reminder to non-responsive recipients.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| type | [string](#string) |  | Reminder delivery type (e.g. &#34;push&#34;). Constraints: Accepted values: &#34;push&#34;. Max length 50 characters. |






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
| PERMISSION_INBOX_READ | 11 | View inbox messages and deliveries. |
| PERMISSION_INBOX_ACT | 12 | Submit actions on deliveries. |
| PERMISSION_GROUPS_ALL_READ | 13 | View all groups in the organization. |
| PERMISSION_GROUPS_WRITE | 14 | Create, edit, delete groups the caller created, manage own group membership. |
| PERMISSION_GROUPS_ALL_WRITE | 15 | Create, edit, delete any group in the organization, manage any group membership. |
| PERMISSION_TEAMS_ALL_READ | 16 | View all teams (organizational units) in the organization. |
| PERMISSION_TEAMS_WRITE | 17 | Create, edit, delete teams the caller created, manage own team membership. |
| PERMISSION_TEAMS_ALL_WRITE | 18 | Create, edit, delete any team in the organization, manage any team membership. |
| PERMISSION_PRIVACY_READ | 19 | View privacy requests (exports, deletions) for the organization. |
| PERMISSION_PRIVACY_WRITE | 20 | Schedule deletions, export user data, restrict processing. |
| PERMISSION_AUDIT_READ | 21 | View audit trail events for the organization. |



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


 

 

 



<a name="pidgr_v1_api_key-proto"></a>
<p align="right"><a href="#top">Top</a></p>

## pidgr/v1/api_key.proto



<a name="pidgr-v1-ApiKey"></a>

### ApiKey
A scoped API key for programmatic access (MCP agents, service integrations).


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| id | [string](#string) |  | Unique identifier. |
| name | [string](#string) |  | Human-friendly label (e.g. &#34;MCP Production&#34;, &#34;CI Pipeline&#34;). |
| key_prefix | [string](#string) |  | Displayable prefix of the key (e.g. &#34;pidgr_k_abc12345&#34;). Used for identification — the full key is only returned on creation. |
| permissions | [Permission](#pidgr-v1-Permission) | repeated | Permissions granted to this key. |
| created_at | [google.protobuf.Timestamp](#google-protobuf-Timestamp) |  | When the key was created. |
| last_used_at | [google.protobuf.Timestamp](#google-protobuf-Timestamp) |  | Last time the key was used to authenticate a request. Empty if never used. |
| expires_at | [google.protobuf.Timestamp](#google-protobuf-Timestamp) |  | When the key expires. Empty means no expiration. |






<a name="pidgr-v1-CreateApiKeyRequest"></a>

### CreateApiKeyRequest
Request to create a new API key.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| name | [string](#string) |  | Human-friendly label. Required, max 200 characters. |
| permissions | [Permission](#pidgr-v1-Permission) | repeated | Permissions to grant. Required, at least one. PERMISSION_UNSPECIFIED values are rejected. |
| expires_at | [google.protobuf.Timestamp](#google-protobuf-Timestamp) |  | Optional expiration time. If omitted, the key does not expire. |






<a name="pidgr-v1-CreateApiKeyResponse"></a>

### CreateApiKeyResponse
Response after creating an API key.
IMPORTANT: The full key is only returned here — it cannot be retrieved later.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| api_key | [ApiKey](#pidgr-v1-ApiKey) |  | The created API key metadata. |
| key | [string](#string) |  | The full secret key value (e.g. &#34;pidgr_k_abc12345...&#34;). Store this securely — it is not retrievable after this response. |






<a name="pidgr-v1-ListApiKeysRequest"></a>

### ListApiKeysRequest
Request to list all API keys in the caller&#39;s organization.






<a name="pidgr-v1-ListApiKeysResponse"></a>

### ListApiKeysResponse
Response containing the organization&#39;s API keys.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| api_keys | [ApiKey](#pidgr-v1-ApiKey) | repeated | All active (non-revoked) API keys. Full key values are not included. |






<a name="pidgr-v1-RevokeApiKeyRequest"></a>

### RevokeApiKeyRequest
Request to revoke an API key.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| api_key_id | [string](#string) |  | ID of the API key to revoke. Required. |






<a name="pidgr-v1-RevokeApiKeyResponse"></a>

### RevokeApiKeyResponse
Response after revoking an API key.





 

 

 


<a name="pidgr-v1-ApiKeyService"></a>

### ApiKeyService
Manages scoped API keys for programmatic access.
All RPCs operate within the caller&#39;s org (extracted from JWT).

| Method Name | Request Type | Response Type | Description |
| ----------- | ------------ | ------------- | ------------|
| CreateApiKey | [CreateApiKeyRequest](#pidgr-v1-CreateApiKeyRequest) | [CreateApiKeyResponse](#pidgr-v1-CreateApiKeyResponse) | Create a new scoped API key with specific permissions. The full secret key is only returned in the response — store it securely. Authorization: Requires PERMISSION_ORG_WRITE. |
| ListApiKeys | [ListApiKeysRequest](#pidgr-v1-ListApiKeysRequest) | [ListApiKeysResponse](#pidgr-v1-ListApiKeysResponse) | List all active API keys in the organization (metadata only, no secrets). Authorization: Requires PERMISSION_ORG_READ. |
| RevokeApiKey | [RevokeApiKeyRequest](#pidgr-v1-RevokeApiKeyRequest) | [RevokeApiKeyResponse](#pidgr-v1-RevokeApiKeyResponse) | Revoke an API key. The key becomes immediately unusable. Authorization: Requires PERMISSION_ORG_WRITE. |

 



<a name="pidgr_v1_privacy-proto"></a>
<p align="right"><a href="#top">Top</a></p>

## pidgr/v1/privacy.proto



<a name="pidgr-v1-CancelDeletionRequest"></a>

### CancelDeletionRequest
Request to cancel a pending deletion.
Auth: Requires JWT. Admin only.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| request_id | [string](#string) |  | The privacy request ID to cancel. |
| confirmation_email | [string](#string) |  | Admin must type the target user&#39;s email to confirm. |






<a name="pidgr-v1-CancelDeletionResponse"></a>

### CancelDeletionResponse
Response confirming the cancellation.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| status | [PrivacyRequestStatus](#pidgr-v1-PrivacyRequestStatus) |  | Updated status (should be FAILED with reason cancelled). |






<a name="pidgr-v1-DeleteUserDataRequest"></a>

### DeleteUserDataRequest
Request to delete or anonymize all personal data associated with a user.
Auth: Requires JWT. Admin only.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| user_id | [string](#string) |  | Internal user ID whose data is being deleted. Constraints: UUID format (36 characters). |
| anonymize | [bool](#bool) |  | When true, PII is replaced with placeholders instead of hard-deleted. This preserves audit trail integrity while removing personal data. |






<a name="pidgr-v1-DeleteUserDataResponse"></a>

### DeleteUserDataResponse
Response confirming the deletion request.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| status | [PrivacyRequestStatus](#pidgr-v1-PrivacyRequestStatus) |  | Current status of the deletion request. |
| deleted_at | [google.protobuf.Timestamp](#google-protobuf-Timestamp) |  | Timestamp when deletion was completed (or scheduled). Only populated when status is COMPLETED. |
| request_id | [string](#string) |  | Unique identifier for this deletion request. |






<a name="pidgr-v1-ExportUserDataRequest"></a>

### ExportUserDataRequest
Request to export all personal data associated with a user.
Auth: Requires JWT. Callable by the user themselves or an org admin.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| user_id | [string](#string) |  | Internal user ID whose data is being exported. Constraints: UUID format (36 characters). |






<a name="pidgr-v1-ExportUserDataResponse"></a>

### ExportUserDataResponse
Response containing the export status and download location.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| status | [PrivacyRequestStatus](#pidgr-v1-PrivacyRequestStatus) |  | Current status of the export request. |
| result_url | [string](#string) |  | Pre-signed S3 URL to download the exported data (ZIP format). Only populated when status is COMPLETED. |
| export_id | [string](#string) |  | Unique identifier for this export request. Constraints: UUID format (36 characters). |






<a name="pidgr-v1-GetDataExistenceConfirmationRequest"></a>

### GetDataExistenceConfirmationRequest
Request to confirm whether personal data exists for a user.
LGPD-specific: confirmação de existência (Art. 18, I).
Auth: Requires JWT. Admin only.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| user_id | [string](#string) |  | Internal user ID to check. Constraints: UUID format (36 characters). |






<a name="pidgr-v1-GetDataExistenceConfirmationResponse"></a>

### GetDataExistenceConfirmationResponse
Response confirming data existence and listing data categories.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| exists | [bool](#bool) |  | Whether any personal data exists for this user. |
| data_categories | [string](#string) | repeated | Categories of data stored (e.g., &#34;profile&#34;, &#34;deliveries&#34;, &#34;analytics&#34;). |






<a name="pidgr-v1-ImmediateDeleteRequest"></a>

### ImmediateDeleteRequest
Request to skip the grace period and delete immediately.
Auth: Requires JWT. Admin only.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| request_id | [string](#string) |  | The privacy request ID to expedite. |
| confirmation_email | [string](#string) |  | Admin must type the target user&#39;s email to confirm. |






<a name="pidgr-v1-ImmediateDeleteResponse"></a>

### ImmediateDeleteResponse
Response confirming the immediate deletion was triggered.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| status | [PrivacyRequestStatus](#pidgr-v1-PrivacyRequestStatus) |  | Updated status (should be PROCESSING). |






<a name="pidgr-v1-ListPrivacyRequestsRequest"></a>

### ListPrivacyRequestsRequest
Request to list privacy requests for the organization.
Auth: Requires JWT. Admin only.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| page_size | [int32](#int32) |  | Maximum number of results per page. Constraints: 1–100, default 25. |
| page_token | [string](#string) |  | Continuation token from a previous response. |
| request_type | [string](#string) |  | Filter by request type (export, delete, rectify, restrict). Empty = all. |
| status | [PrivacyRequestStatus](#pidgr-v1-PrivacyRequestStatus) |  | Filter by status. UNSPECIFIED = all. |






<a name="pidgr-v1-ListPrivacyRequestsResponse"></a>

### ListPrivacyRequestsResponse
Response containing privacy requests.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| requests | [PrivacyRequest](#pidgr-v1-PrivacyRequest) | repeated | The privacy requests matching the filters. |
| next_page_token | [string](#string) |  | Token for the next page. Empty if no more results. |






<a name="pidgr-v1-PrivacyRequest"></a>

### PrivacyRequest
A privacy request record.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| id | [string](#string) |  | Unique identifier. |
| user_id | [string](#string) |  | The user this request applies to. |
| user_email | [string](#string) |  | Email of the target user. |
| request_type | [string](#string) |  | Type of request (export, delete, rectify, restrict). |
| status | [PrivacyRequestStatus](#pidgr-v1-PrivacyRequestStatus) |  | Current status. |
| anonymize | [bool](#bool) |  | Whether to anonymize (true) or hard-delete (false). Only for delete requests. |
| requested_by_email | [string](#string) |  | Email of the admin who initiated this request. |
| created_at | [google.protobuf.Timestamp](#google-protobuf-Timestamp) |  | When the request was created. |
| completed_at | [google.protobuf.Timestamp](#google-protobuf-Timestamp) |  | When the request was completed (if applicable). |
| metadata | [PrivacyRequest.MetadataEntry](#pidgr-v1-PrivacyRequest-MetadataEntry) | repeated | Additional metadata (JSON). |






<a name="pidgr-v1-PrivacyRequest-MetadataEntry"></a>

### PrivacyRequest.MetadataEntry



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| key | [string](#string) |  |  |
| value | [string](#string) |  |  |






<a name="pidgr-v1-RectifyUserDataRequest"></a>

### RectifyUserDataRequest
Request to correct personal data for a user.
Auth: Requires JWT. Callable by the user themselves or an org admin.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| user_id | [string](#string) |  | Internal user ID whose data is being corrected. Constraints: UUID format (36 characters). |
| corrections | [RectifyUserDataRequest.CorrectionsEntry](#pidgr-v1-RectifyUserDataRequest-CorrectionsEntry) | repeated | Map of field names to corrected values. Corrections are propagated to all stored locations. Constraints: Max 50 corrections per request. |






<a name="pidgr-v1-RectifyUserDataRequest-CorrectionsEntry"></a>

### RectifyUserDataRequest.CorrectionsEntry



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| key | [string](#string) |  |  |
| value | [string](#string) |  |  |






<a name="pidgr-v1-RectifyUserDataResponse"></a>

### RectifyUserDataResponse
Response listing which fields were successfully corrected.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| rectified_fields | [string](#string) | repeated | Names of fields that were rectified. |






<a name="pidgr-v1-RestrictProcessingRequest"></a>

### RestrictProcessingRequest
Request to restrict or unrestrict processing for a user.
Auth: Requires JWT. Admin only.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| user_id | [string](#string) |  | Internal user ID whose processing is being restricted. Constraints: UUID format (36 characters). |
| restricted | [bool](#bool) |  | When true, processing is restricted. When false, restriction is lifted. |






<a name="pidgr-v1-RestrictProcessingResponse"></a>

### RestrictProcessingResponse
Response confirming the processing restriction status.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| restricted | [bool](#bool) |  | Current restriction status. |
| restricted_at | [google.protobuf.Timestamp](#google-protobuf-Timestamp) |  | Timestamp when the restriction was applied or removed. |





 


<a name="pidgr-v1-PrivacyRequestStatus"></a>

### PrivacyRequestStatus
Status of a privacy request (export, delete, rectify, restrict).

| Name | Number | Description |
| ---- | ------ | ----------- |
| PRIVACY_REQUEST_STATUS_UNSPECIFIED | 0 | Default value; should not be used explicitly. |
| PRIVACY_REQUEST_STATUS_PENDING | 1 | Request has been created but not yet started. |
| PRIVACY_REQUEST_STATUS_PROCESSING | 2 | Request is currently being processed. |
| PRIVACY_REQUEST_STATUS_COMPLETED | 3 | Request completed successfully. |
| PRIVACY_REQUEST_STATUS_FAILED | 4 | Request failed during processing. |


 

 


<a name="pidgr-v1-PrivacyService"></a>

### PrivacyService
PrivacyService handles GDPR/LGPD data subject rights.
All RPCs extract org_id from the JWT — it is never in request messages.

| Method Name | Request Type | Response Type | Description |
| ----------- | ------------ | ------------- | ------------|
| ExportUserData | [ExportUserDataRequest](#pidgr-v1-ExportUserDataRequest) | [ExportUserDataResponse](#pidgr-v1-ExportUserDataResponse) | Export all personal data associated with a user as a downloadable ZIP. Async operation — returns immediately with PENDING status, sends push notification when the export is ready. Auth: Requires JWT. Callable by the user themselves or an org admin. |
| DeleteUserData | [DeleteUserDataRequest](#pidgr-v1-DeleteUserDataRequest) | [DeleteUserDataResponse](#pidgr-v1-DeleteUserDataResponse) | Delete or anonymize all personal data associated with a user. Deletion has a 30-day grace period during which processing is restricted and the request can be cancelled. After 30 days, deletion is irreversible. Auth: Requires JWT. Admin only. |
| RectifyUserData | [RectifyUserDataRequest](#pidgr-v1-RectifyUserDataRequest) | [RectifyUserDataResponse](#pidgr-v1-RectifyUserDataResponse) | Correct personal data for a user. Propagates corrections to all stored locations (profile, delivery records, analytics metadata). Auth: Requires JWT. Callable by the user themselves or an org admin. |
| RestrictProcessing | [RestrictProcessingRequest](#pidgr-v1-RestrictProcessingRequest) | [RestrictProcessingResponse](#pidgr-v1-RestrictProcessingResponse) | Restrict or unrestrict processing for a user. When restricted, the API skips this user in campaigns, analytics, and session replay. Auth: Requires JWT. Admin only. |
| GetDataExistenceConfirmation | [GetDataExistenceConfirmationRequest](#pidgr-v1-GetDataExistenceConfirmationRequest) | [GetDataExistenceConfirmationResponse](#pidgr-v1-GetDataExistenceConfirmationResponse) | Confirm whether personal data exists for a user and list data categories. LGPD-specific: confirmação de existência (Art. 18, I). Auth: Requires JWT. Admin only. |
| ListPrivacyRequests | [ListPrivacyRequestsRequest](#pidgr-v1-ListPrivacyRequestsRequest) | [ListPrivacyRequestsResponse](#pidgr-v1-ListPrivacyRequestsResponse) | List privacy requests for the organization, with optional filters. Used by the admin UI to show scheduled deletions table. Auth: Requires JWT. Admin only. |
| CancelDeletion | [CancelDeletionRequest](#pidgr-v1-CancelDeletionRequest) | [CancelDeletionResponse](#pidgr-v1-CancelDeletionResponse) | Cancel a pending deletion request. Reactivates the user and aborts the deletion workflow. Only valid during the 30-day grace period. Auth: Requires JWT. Admin only. |
| ImmediateDelete | [ImmediateDeleteRequest](#pidgr-v1-ImmediateDeleteRequest) | [ImmediateDeleteResponse](#pidgr-v1-ImmediateDeleteResponse) | Skip the grace period and delete immediately. Signals the deletion workflow to proceed without waiting for the 30-day timer. Auth: Requires JWT. Admin only. |

 



<a name="pidgr_v1_audit-proto"></a>
<p align="right"><a href="#top">Top</a></p>

## pidgr/v1/audit.proto



<a name="pidgr-v1-AuditEvent"></a>

### AuditEvent
An immutable audit event capturing a significant platform action.
Audit events are append-only — they cannot be updated or deleted.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| id | [string](#string) |  | Unique identifier for this audit event. Constraints: UUID format (36 characters). |
| org_id | [string](#string) |  | Organization in which the event occurred. Constraints: UUID format (36 characters). |
| actor_id | [string](#string) |  | User who performed the action. Empty for system-initiated events. Constraints: UUID format (36 characters) when present. |
| event_type | [AuditEventType](#pidgr-v1-AuditEventType) |  | Type of action that was performed. |
| entity_type | [string](#string) |  | Type of entity affected (e.g., &#34;campaign&#34;, &#34;user&#34;, &#34;template&#34;). Constraints: Max length 50 characters. |
| entity_id | [string](#string) |  | Identifier of the entity affected. Constraints: UUID format (36 characters). |
| metadata | [AuditEvent.MetadataEntry](#pidgr-v1-AuditEvent-MetadataEntry) | repeated | Additional context about the event (e.g., old/new values for changes). Constraints: Max 20 key-value pairs, keys max 50 chars, values max 500 chars. |
| created_at | [google.protobuf.Timestamp](#google-protobuf-Timestamp) |  | Timestamp when the event was recorded. |






<a name="pidgr-v1-AuditEvent-MetadataEntry"></a>

### AuditEvent.MetadataEntry



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| key | [string](#string) |  |  |
| value | [string](#string) |  |  |






<a name="pidgr-v1-AuditExport"></a>

### AuditExport
A persistent record of an audit trail export request.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| id | [string](#string) |  | Unique identifier. |
| format | [string](#string) |  | Export format (csv, json). |
| status | [PrivacyRequestStatus](#pidgr-v1-PrivacyRequestStatus) |  | Current status. |
| result_url | [string](#string) |  | Pre-signed download URL. Only populated when status is COMPLETED. |
| error_message | [string](#string) |  | Error message if the export failed. |
| requested_by_email | [string](#string) |  | Email of the admin who requested the export. |
| created_at | [google.protobuf.Timestamp](#google-protobuf-Timestamp) |  | When the export was requested. |
| completed_at | [google.protobuf.Timestamp](#google-protobuf-Timestamp) |  | When the export completed (if applicable). |






<a name="pidgr-v1-ExportAuditTrailRequest"></a>

### ExportAuditTrailRequest
Request to export the audit trail to S3 in a specified format.
Auth: Requires JWT. Admin only.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| format | [AuditExportFormat](#pidgr-v1-AuditExportFormat) |  | Export format. |
| start_time | [google.protobuf.Timestamp](#google-protobuf-Timestamp) |  | Optional: export events after this timestamp. |
| end_time | [google.protobuf.Timestamp](#google-protobuf-Timestamp) |  | Optional: export events before this timestamp. |






<a name="pidgr-v1-ExportAuditTrailResponse"></a>

### ExportAuditTrailResponse
Response containing the export download URL.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| export_url | [string](#string) |  | Pre-signed S3 URL to download the exported audit trail. Only populated when status is COMPLETED. |
| status | [PrivacyRequestStatus](#pidgr-v1-PrivacyRequestStatus) |  | Current status of the export request. |






<a name="pidgr-v1-ListAuditEventsRequest"></a>

### ListAuditEventsRequest
Request to list audit events with optional filters.
Auth: Requires JWT. Admin only.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| page_token | [string](#string) |  | Pagination token from a previous response. |
| page_size | [int32](#int32) |  | Maximum number of events to return. Constraints: Min 1, max 100. Default 50. |
| event_type | [AuditEventType](#pidgr-v1-AuditEventType) |  | Optional filter: only return events of this type. |
| actor_id | [string](#string) |  | Optional filter: only return events by this actor. Constraints: UUID format (36 characters). |
| start_time | [google.protobuf.Timestamp](#google-protobuf-Timestamp) |  | Optional filter: events after this timestamp (inclusive). |
| end_time | [google.protobuf.Timestamp](#google-protobuf-Timestamp) |  | Optional filter: events before this timestamp (exclusive). |






<a name="pidgr-v1-ListAuditEventsResponse"></a>

### ListAuditEventsResponse
Response containing a paginated list of audit events.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| events | [AuditEvent](#pidgr-v1-AuditEvent) | repeated | Audit events matching the request filters. |
| next_page_token | [string](#string) |  | Token for fetching the next page. Empty when no more events. |






<a name="pidgr-v1-ListAuditExportsRequest"></a>

### ListAuditExportsRequest
Request to list audit export history.
Auth: Requires JWT. Admin only.






<a name="pidgr-v1-ListAuditExportsResponse"></a>

### ListAuditExportsResponse
Response containing the list of audit exports.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| exports | [AuditExport](#pidgr-v1-AuditExport) | repeated | Audit export records, newest first. |





 


<a name="pidgr-v1-AuditEventType"></a>

### AuditEventType
Type of auditable platform action.

| Name | Number | Description |
| ---- | ------ | ----------- |
| AUDIT_EVENT_TYPE_UNSPECIFIED | 0 | Default value; should not be used explicitly. |
| AUDIT_EVENT_TYPE_CAMPAIGN_CREATED | 1 | ── Campaign lifecycle ─────────────────────────────────────────────────── A campaign was created. |
| AUDIT_EVENT_TYPE_MESSAGE_SENT | 2 | A message was sent to a recipient. |
| AUDIT_EVENT_TYPE_MESSAGE_OPENED | 3 | A message was opened by a recipient. |
| AUDIT_EVENT_TYPE_ACK_REGISTERED | 4 | A recipient acknowledged a campaign. |
| AUDIT_EVENT_TYPE_ESCALATION_EXECUTED | 5 | An escalation was triggered by the workflow. |
| AUDIT_EVENT_TYPE_CAMPAIGN_STARTED | 12 | A campaign was started. |
| AUDIT_EVENT_TYPE_CAMPAIGN_CANCELLED | 13 | A campaign was cancelled. |
| AUDIT_EVENT_TYPE_CAMPAIGN_UPDATED | 14 | A campaign was updated. |
| AUDIT_EVENT_TYPE_USER_INVITED | 6 | ── User lifecycle ─────────────────────────────────────────────────────── A user was invited to the organization. |
| AUDIT_EVENT_TYPE_USER_DEACTIVATED | 7 | A user was deactivated. |
| AUDIT_EVENT_TYPE_USER_REACTIVATED | 15 | A user was reactivated. |
| AUDIT_EVENT_TYPE_ROLE_CHANGED | 10 | A user&#39;s role was changed (assigned to a different role). |
| AUDIT_EVENT_TYPE_INVITE_REVOKED | 16 | A user&#39;s invite was revoked. |
| AUDIT_EVENT_TYPE_PROFILE_UPDATED | 17 | A user&#39;s profile was updated. |
| AUDIT_EVENT_TYPE_SETTINGS_UPDATED | 18 | A user&#39;s settings were updated. |
| AUDIT_EVENT_TYPE_PASSKEY_ENROLLED | 19 | A user enrolled a passkey. |
| AUDIT_EVENT_TYPE_DATA_EXPORT_REQUESTED | 8 | ── GDPR / Privacy ────────────────────────────────────────────────────── A data export was requested (GDPR Art. 15). |
| AUDIT_EVENT_TYPE_DATA_DELETION_REQUESTED | 9 | A data deletion was requested (GDPR Art. 17). |
| AUDIT_EVENT_TYPE_DATA_RECTIFIED | 20 | User data was rectified (GDPR Art. 16). |
| AUDIT_EVENT_TYPE_PROCESSING_RESTRICTED | 21 | Data processing was restricted (GDPR Art. 18). |
| AUDIT_EVENT_TYPE_DELETION_CANCELLED | 22 | A scheduled deletion was cancelled. |
| AUDIT_EVENT_TYPE_DELETION_IMMEDIATE | 23 | An immediate deletion was executed. |
| AUDIT_EVENT_TYPE_SSO_CONFIGURED | 11 | ── Organization / SSO ─────────────────────────────────────────────────── An SSO provider was configured. |
| AUDIT_EVENT_TYPE_SSO_PROVIDER_CREATED | 24 | An SSO provider was created. |
| AUDIT_EVENT_TYPE_SSO_PROVIDER_DELETED | 25 | An SSO provider was deleted. |
| AUDIT_EVENT_TYPE_ORG_UPDATED | 26 | Organization settings were updated. |
| AUDIT_EVENT_TYPE_ROLE_CREATED | 27 | ── Roles ──────────────────────────────────────────────────────────────── A role was created. |
| AUDIT_EVENT_TYPE_ROLE_UPDATED | 28 | A role&#39;s name or permissions were updated. |
| AUDIT_EVENT_TYPE_ROLE_DELETED | 29 | A role was deleted. |
| AUDIT_EVENT_TYPE_TEMPLATE_CREATED | 30 | ── Templates ──────────────────────────────────────────────────────────── A template was created. |
| AUDIT_EVENT_TYPE_TEMPLATE_UPDATED | 31 | A template was updated. |
| AUDIT_EVENT_TYPE_API_KEY_CREATED | 32 | ── API Keys ───────────────────────────────────────────────────────────── An API key was created. |
| AUDIT_EVENT_TYPE_API_KEY_REVOKED | 33 | An API key was revoked. |
| AUDIT_EVENT_TYPE_INVITE_LINK_CREATED | 34 | ── Invite Links ───────────────────────────────────────────────────────── An invite link was created. |
| AUDIT_EVENT_TYPE_INVITE_LINK_REVOKED | 35 | An invite link was revoked. |
| AUDIT_EVENT_TYPE_GROUP_CREATED | 36 | ── Groups ─────────────────────────────────────────────────────────────── A group was created. |
| AUDIT_EVENT_TYPE_GROUP_UPDATED | 37 | A group was updated. |
| AUDIT_EVENT_TYPE_GROUP_DELETED | 38 | A group was deleted. |
| AUDIT_EVENT_TYPE_GROUP_MEMBERS_ADDED | 39 | Members were added to a group. |
| AUDIT_EVENT_TYPE_GROUP_MEMBERS_REMOVED | 40 | Members were removed from a group. |
| AUDIT_EVENT_TYPE_TEAM_CREATED | 41 | ── Teams ──────────────────────────────────────────────────────────────── A team was created. |
| AUDIT_EVENT_TYPE_TEAM_UPDATED | 42 | A team was updated. |
| AUDIT_EVENT_TYPE_TEAM_DELETED | 43 | A team was deleted. |
| AUDIT_EVENT_TYPE_TEAM_MEMBERS_ADDED | 44 | Members were added to a team. |
| AUDIT_EVENT_TYPE_TEAM_MEMBERS_REMOVED | 45 | Members were removed from a team. |



<a name="pidgr-v1-AuditExportFormat"></a>

### AuditExportFormat
Format for audit trail export.

| Name | Number | Description |
| ---- | ------ | ----------- |
| AUDIT_EXPORT_FORMAT_UNSPECIFIED | 0 | Default value; should not be used explicitly. |
| AUDIT_EXPORT_FORMAT_CSV | 1 | Comma-separated values. |
| AUDIT_EXPORT_FORMAT_JSON | 2 | JSON lines format. |
| AUDIT_EXPORT_FORMAT_PARQUET | 3 | Apache Parquet columnar format. |


 

 


<a name="pidgr-v1-AuditService"></a>

### AuditService
AuditService provides read access to the append-only audit trail.
All RPCs extract org_id from the JWT — it is never in request messages.

| Method Name | Request Type | Response Type | Description |
| ----------- | ------------ | ------------- | ------------|
| ListAuditEvents | [ListAuditEventsRequest](#pidgr-v1-ListAuditEventsRequest) | [ListAuditEventsResponse](#pidgr-v1-ListAuditEventsResponse) | List audit events with optional filtering by event type, actor, and date range. Results are ordered by created_at descending (newest first). Auth: Requires JWT. Admin only. |
| ExportAuditTrail | [ExportAuditTrailRequest](#pidgr-v1-ExportAuditTrailRequest) | [ExportAuditTrailResponse](#pidgr-v1-ExportAuditTrailResponse) | Export the audit trail to S3 in CSV, JSON, or Parquet format. Creates a persistent record and starts an async Temporal workflow. Auth: Requires JWT. Admin only. |
| ListAuditExports | [ListAuditExportsRequest](#pidgr-v1-ListAuditExportsRequest) | [ListAuditExportsResponse](#pidgr-v1-ListAuditExportsResponse) | List audit export history for the organization. Auth: Requires JWT. Admin only. |

 



<a name="pidgr_v1_campaign-proto"></a>
<p align="right"><a href="#top">Top</a></p>

## pidgr/v1/campaign.proto



<a name="pidgr-v1-AudienceMember"></a>

### AudienceMember
A single audience member with optional per-user template variables.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| user_id | [string](#string) |  | User ID (UUID). |
| variables | [AudienceMember.VariablesEntry](#pidgr-v1-AudienceMember-VariablesEntry) | repeated | Template variable values for this user (e.g. {&#34;name&#34;: &#34;Alice&#34;}). |






<a name="pidgr-v1-AudienceMember-VariablesEntry"></a>

### AudienceMember.VariablesEntry



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| key | [string](#string) |  |  |
| value | [string](#string) |  |  |






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
| audience_snapshot_ref | [string](#string) |  | Object storage reference to the audience snapshot taken at campaign creation. |
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
| critical | [bool](#bool) |  | Whether this campaign&#39;s notifications break through Do Not Disturb / Focus mode. |






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
| audience | [AudienceMember](#pidgr-v1-AudienceMember) | repeated | Rich audience with per-user template variables. When set, takes precedence over user_ids. Constraints: Max 100000 items. |
| include_restricted | [bool](#bool) |  | Whether to include users with processing_restricted=true in the audience. Default false: restricted users are excluded. Set true only with Art. 18(2) legal basis. |
| critical | [bool](#bool) |  | Whether this campaign&#39;s notifications break through Do Not Disturb / Focus mode. |






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
| recipient_email | [string](#string) |  | Email address of the recipient, populated from the users table on read. |






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






<a name="pidgr-v1-UpdateCampaignRequest"></a>

### UpdateCampaignRequest
Request to update a draft campaign (status must be CREATED).
Only non-empty/non-zero fields are updated; omitted fields remain unchanged.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| campaign_id | [string](#string) |  | ID of the campaign to update. Constraints: UUID format (36 characters). |
| name | [string](#string) |  | Updated campaign name. Empty string means no change. Constraints: Max length 200 characters. |
| sender_name | [string](#string) |  | Updated sender display name. Empty string means no change. Constraints: Max length 200 characters. |
| title | [string](#string) |  | Updated title override. Empty string means no change. Constraints: Max length 200 characters. |
| template_id | [string](#string) |  | Updated template ID. Empty string means no change. Constraints: UUID format (36 characters). |
| template_version | [int32](#int32) |  | Updated template version. Zero means no change. |
| workflow | [WorkflowDefinition](#pidgr-v1-WorkflowDefinition) |  | Updated workflow DAG. Null/omitted means no change. |






<a name="pidgr-v1-UpdateCampaignResponse"></a>

### UpdateCampaignResponse
Response after updating a campaign.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| campaign | [Campaign](#pidgr-v1-Campaign) |  | The campaign with updated fields. |





 

 

 


<a name="pidgr-v1-CampaignService"></a>

### CampaignService
Manages the full lifecycle of communication campaigns — creation,
execution, monitoring, and cancellation.

| Method Name | Request Type | Response Type | Description |
| ----------- | ------------ | ------------- | ------------|
| CreateCampaign | [CreateCampaignRequest](#pidgr-v1-CreateCampaignRequest) | [CreateCampaignResponse](#pidgr-v1-CreateCampaignResponse) | Create a new campaign with a template, audience, and workflow. Authorization: Requires MANAGER&#43; role. |
| StartCampaign | [StartCampaignRequest](#pidgr-v1-StartCampaignRequest) | [StartCampaignResponse](#pidgr-v1-StartCampaignResponse) | Start a created campaign, triggering its workflow execution via the orchestration engine. Authorization: Requires MANAGER&#43; role. |
| GetCampaign | [GetCampaignRequest](#pidgr-v1-GetCampaignRequest) | [GetCampaignResponse](#pidgr-v1-GetCampaignResponse) | Retrieve a single campaign by ID. Authorization: Authenticated user within the organization. |
| ListCampaigns | [ListCampaignsRequest](#pidgr-v1-ListCampaignsRequest) | [ListCampaignsResponse](#pidgr-v1-ListCampaignsResponse) | List campaigns for the organization with pagination. Authorization: Authenticated user within the organization. |
| UpdateCampaign | [UpdateCampaignRequest](#pidgr-v1-UpdateCampaignRequest) | [UpdateCampaignResponse](#pidgr-v1-UpdateCampaignResponse) | Update a draft campaign (CREATED status only). Non-empty fields overwrite existing values. Authorization: Requires MANAGER&#43; role. |
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
| push_token | [string](#string) |  | Push token used to send notifications to this device. |
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






<a name="pidgr-v1-ListMemberDevicesRequest"></a>

### ListMemberDevicesRequest
Request to list devices for a specific member (admin use).


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| user_id | [string](#string) |  | ID of the user whose devices to list. Constraints: UUID format (36 characters). |






<a name="pidgr-v1-ListMemberDevicesResponse"></a>

### ListMemberDevicesResponse
Response containing all devices for the specified member.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| devices | [DeviceSummary](#pidgr-v1-DeviceSummary) | repeated | List of devices registered to the specified user. |






<a name="pidgr-v1-RegisterRequest"></a>

### RegisterRequest
Request to register a device for push notifications.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| device_id | [string](#string) |  | Client-generated unique device identifier. Constraints: UUID format (36 characters). |
| platform | [Platform](#pidgr-v1-Platform) |  | Mobile platform of the device. |
| push_token | [string](#string) |  | Push token obtained from the push notification provider on the client. Constraints: Max length 4096 characters. |






<a name="pidgr-v1-RegisterResponse"></a>

### RegisterResponse
Response after registering a device.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| device | [DeviceSummary](#pidgr-v1-DeviceSummary) |  | The registered device summary (excludes push_token). |





 

 

 


<a name="pidgr-v1-DeviceService"></a>

### DeviceService
Manages push notification device registration.
Used by the mobile app to register push tokens and manage device lifecycle.

| Method Name | Request Type | Response Type | Description |
| ----------- | ------------ | ------------- | ------------|
| Register | [RegisterRequest](#pidgr-v1-RegisterRequest) | [RegisterResponse](#pidgr-v1-RegisterResponse) | Register a device with its push token for receiving notifications. Authorization: Authenticated user (own devices only). |
| Deactivate | [DeactivateRequest](#pidgr-v1-DeactivateRequest) | [DeactivateResponse](#pidgr-v1-DeactivateResponse) | Deactivate a device, preventing further push notifications. Authorization: Authenticated user (own devices only). |
| ListDevices | [ListDevicesRequest](#pidgr-v1-ListDevicesRequest) | [ListDevicesResponse](#pidgr-v1-ListDevicesResponse) | List all devices registered to the authenticated user. Authorization: Authenticated user (own devices only). |
| ListMemberDevices | [ListMemberDevicesRequest](#pidgr-v1-ListMemberDevicesRequest) | [ListMemberDevicesResponse](#pidgr-v1-ListMemberDevicesResponse) | List all devices for a specific organization member. Authorization: Requires MEMBERS_READ permission. |

 



<a name="pidgr_v1_user-proto"></a>
<p align="right"><a href="#top">Top</a></p>

## pidgr/v1/user.proto



<a name="pidgr-v1-User"></a>

### User
A user within an organization.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| id | [string](#string) |  | Unique identifier for the user (internal platform UUID, not identity provider subject ID). |
| email | [string](#string) |  | User&#39;s email address. Constraints: Max length 254 characters (RFC 5321). |
| name | [string](#string) |  | User&#39;s display name. Constraints: Max length 200 characters. |
| status | [UserStatus](#pidgr-v1-UserStatus) |  | Current account status. |
| created_at | [google.protobuf.Timestamp](#google-protobuf-Timestamp) |  | Timestamp when the user was created. |
| role | [Role](#pidgr-v1-Role) |  | The user&#39;s role with its permission set. |
| role_id | [string](#string) |  | ID of the user&#39;s role (for assignment operations). |
| profile | [UserProfile](#pidgr-v1-UserProfile) |  | Structured profile attributes (department, title, etc.). May be empty if the user has not completed their profile. |
| processing_restricted | [bool](#bool) |  | Whether data processing is restricted for this user (GDPR Art. 18). When true, the user is excluded from campaign audiences by default. |






<a name="pidgr-v1-UserProfile"></a>

### UserProfile
Structured profile attributes for a user within an organization.
Populated through admin invitation, mobile onboarding, or SSO attribute sync.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| first_name | [string](#string) |  | User&#39;s given name. Constraints: Max length 200 characters. |
| last_name | [string](#string) |  | User&#39;s family name. Constraints: Max length 200 characters. |
| department | [string](#string) |  | Department or team within the organization. Constraints: Max length 200 characters. |
| title | [string](#string) |  | Job title. Constraints: Max length 200 characters. |
| phone | [string](#string) |  | Phone number. Constraints: Max length 200 characters. |
| location | [string](#string) |  | Office or geographic location. Constraints: Max length 200 characters. |
| employee_id | [string](#string) |  | Organization-specific employee identifier. Constraints: Max length 200 characters. |
| manager_name | [string](#string) |  | Display name of the user&#39;s direct manager. Constraints: Max length 200 characters. |
| start_date | [string](#string) |  | Employment start date in ISO 8601 format (YYYY-MM-DD). Constraints: Max length 200 characters. |
| custom_attributes | [UserProfile.CustomAttributesEntry](#pidgr-v1-UserProfile-CustomAttributesEntry) | repeated | Organization-defined custom attributes for fields not covered by the fixed schema. Constraints: Max 50 entries. Key max length 100 characters, value max length 1000 characters. |






<a name="pidgr-v1-UserProfile-CustomAttributesEntry"></a>

### UserProfile.CustomAttributesEntry



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| key | [string](#string) |  |  |
| value | [string](#string) |  |  |






<a name="pidgr-v1-UserSettings"></a>

### UserSettings
User-configurable platform settings that apply across all clients.
All fields use their UNSPECIFIED/zero value to mean &#34;no change&#34; in updates.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| theme_preference | [ThemePreference](#pidgr-v1-ThemePreference) |  | Preferred color scheme for the UI. |
| preferred_locale | [string](#string) |  | User&#39;s preferred language for the UI and push notifications. Empty string means &#34;use organization default&#34; or &#34;auto-detect&#34;. Valid values: en, es, pt-BR, zh, ja. |





 


<a name="pidgr-v1-ThemePreference"></a>

### ThemePreference
User&#39;s preferred color scheme.

| Name | Number | Description |
| ---- | ------ | ----------- |
| THEME_PREFERENCE_UNSPECIFIED | 0 | Default value; treated as SYSTEM when reading, &#34;no change&#34; when updating. |
| THEME_PREFERENCE_LIGHT | 1 | Always use light mode regardless of system setting. |
| THEME_PREFERENCE_DARK | 2 | Always use dark mode regardless of system setting. |
| THEME_PREFERENCE_SYSTEM | 3 | Follow the operating system or browser preference. |



<a name="pidgr-v1-UserStatus"></a>

### UserStatus
Lifecycle status of a user account.

| Name | Number | Description |
| ---- | ------ | ----------- |
| USER_STATUS_UNSPECIFIED | 0 | Default value; not a valid status. |
| USER_STATUS_INVITED | 1 | User has been invited but has not completed onboarding. |
| USER_STATUS_ACTIVE | 2 | User is active and can receive messages. |
| USER_STATUS_DEACTIVATED | 3 | User has been deactivated and will not receive messages. |


 

 

 



<a name="pidgr_v1_group-proto"></a>
<p align="right"><a href="#top">Top</a></p>

## pidgr/v1/group.proto



<a name="pidgr-v1-AddGroupMembersRequest"></a>

### AddGroupMembersRequest
Request to add users to a group.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| group_id | [string](#string) |  | ID of the group to add members to. Required. |
| user_ids | [string](#string) | repeated | IDs of users to add. Must belong to the same organization. Adding an existing member is a no-op (idempotent). Constraints: Max 100 user IDs per request. |






<a name="pidgr-v1-AddGroupMembersResponse"></a>

### AddGroupMembersResponse
Response after adding group members.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| group | [Group](#pidgr-v1-Group) |  | The group with updated member_count. |






<a name="pidgr-v1-CreateGroupRequest"></a>

### CreateGroupRequest
Request to create a new group.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| name | [string](#string) |  | Display name for the group. Required. Constraints: Max length 200 characters. |
| description | [string](#string) |  | Optional description. Constraints: Max length 1000 characters. |






<a name="pidgr-v1-CreateGroupResponse"></a>

### CreateGroupResponse
Response after creating a group.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| group | [Group](#pidgr-v1-Group) |  | The newly created group. |






<a name="pidgr-v1-DeleteGroupRequest"></a>

### DeleteGroupRequest
Request to delete a group.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| group_id | [string](#string) |  | ID of the group to delete. Required. Default groups cannot be deleted. |






<a name="pidgr-v1-DeleteGroupResponse"></a>

### DeleteGroupResponse
Response after deleting a group.






<a name="pidgr-v1-GetGroupRequest"></a>

### GetGroupRequest
Request to retrieve a group by ID.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| group_id | [string](#string) |  | ID of the group to retrieve. Required. |






<a name="pidgr-v1-GetGroupResponse"></a>

### GetGroupResponse
Response containing the requested group.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| group | [Group](#pidgr-v1-Group) |  | The requested group. |






<a name="pidgr-v1-GetUserGroupMembershipsRequest"></a>

### GetUserGroupMembershipsRequest
Request to get group memberships for a batch of users.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| user_ids | [string](#string) | repeated | IDs of users to look up. Required. Constraints: Max 200 user IDs per request. |






<a name="pidgr-v1-GetUserGroupMembershipsResponse"></a>

### GetUserGroupMembershipsResponse
Response containing group memberships for the requested users.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| memberships | [UserGroupMembership](#pidgr-v1-UserGroupMembership) | repeated | Group memberships per user. Only users with at least one group are included. |






<a name="pidgr-v1-Group"></a>

### Group
A named collection of users within an organization, used for campaign
audience targeting (recipient groups).


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| id | [string](#string) |  | Unique identifier for the group. |
| name | [string](#string) |  | Human-readable display name (unique within the organization). Constraints: Max length 200 characters. |
| description | [string](#string) |  | Optional description of the group&#39;s purpose. Constraints: Max length 1000 characters. |
| member_count | [int32](#int32) |  | Number of users currently in the group. |
| created_at | [google.protobuf.Timestamp](#google-protobuf-Timestamp) |  | Timestamp when the group was created. |
| updated_at | [google.protobuf.Timestamp](#google-protobuf-Timestamp) |  | Timestamp when the group was last updated. |
| is_default | [bool](#bool) |  | Whether this is the organization&#39;s default group (cannot be deleted or renamed). |
| created_by | [string](#string) |  | ID of the user who created this group. Empty for system-seeded defaults. |






<a name="pidgr-v1-ListGroupMembersRequest"></a>

### ListGroupMembersRequest
Request to list members of a group with pagination.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| group_id | [string](#string) |  | ID of the group whose members to list. Required. |
| pagination | [Pagination](#pidgr-v1-Pagination) |  | Pagination parameters. |






<a name="pidgr-v1-ListGroupMembersResponse"></a>

### ListGroupMembersResponse
Response containing a page of group members.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| users | [User](#pidgr-v1-User) | repeated | Users in this page. |
| pagination_meta | [PaginationMeta](#pidgr-v1-PaginationMeta) |  | Pagination metadata for fetching subsequent pages. |






<a name="pidgr-v1-ListGroupsRequest"></a>

### ListGroupsRequest
Request to list groups in the organization with pagination.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| pagination | [Pagination](#pidgr-v1-Pagination) |  | Pagination parameters. |






<a name="pidgr-v1-ListGroupsResponse"></a>

### ListGroupsResponse
Response containing a page of groups.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| groups | [Group](#pidgr-v1-Group) | repeated | Groups in this page. |
| pagination_meta | [PaginationMeta](#pidgr-v1-PaginationMeta) |  | Pagination metadata for fetching subsequent pages. |






<a name="pidgr-v1-RemoveGroupMembersRequest"></a>

### RemoveGroupMembersRequest
Request to remove users from a group.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| group_id | [string](#string) |  | ID of the group to remove members from. Required. |
| user_ids | [string](#string) | repeated | IDs of users to remove. Removing a non-member is a no-op (idempotent). Constraints: Max 100 user IDs per request. |






<a name="pidgr-v1-RemoveGroupMembersResponse"></a>

### RemoveGroupMembersResponse
Response after removing group members.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| group | [Group](#pidgr-v1-Group) |  | The group with updated member_count. |






<a name="pidgr-v1-UpdateGroupRequest"></a>

### UpdateGroupRequest
Request to update a group&#39;s name and/or description.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| group_id | [string](#string) |  | ID of the group to update. Required. |
| name | [string](#string) |  | New display name. If empty, the name is not changed. Default groups cannot be renamed. Constraints: Max length 200 characters. |
| description | [string](#string) |  | New description. If empty, the description is not changed. Constraints: Max length 1000 characters. |






<a name="pidgr-v1-UpdateGroupResponse"></a>

### UpdateGroupResponse
Response after updating a group.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| group | [Group](#pidgr-v1-Group) |  | The updated group. |






<a name="pidgr-v1-UserGroupMembership"></a>

### UserGroupMembership
A group membership entry for batch lookups.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| user_id | [string](#string) |  | ID of the user. |
| groups | [Group](#pidgr-v1-Group) | repeated | Groups the user belongs to. |





 

 

 


<a name="pidgr-v1-GroupService"></a>

### GroupService
Manages groups and group membership within an organization.
Groups are recipient collections used for campaign audience targeting.
All RPCs operate within the caller&#39;s org (extracted from JWT).

| Method Name | Request Type | Response Type | Description |
| ----------- | ------------ | ------------- | ------------|
| CreateGroup | [CreateGroupRequest](#pidgr-v1-CreateGroupRequest) | [CreateGroupResponse](#pidgr-v1-CreateGroupResponse) | Create a new group in the organization. Authorization: Requires PERMISSION_GROUPS_WRITE or PERMISSION_GROUPS_ALL_WRITE. |
| GetGroup | [GetGroupRequest](#pidgr-v1-GetGroupRequest) | [GetGroupResponse](#pidgr-v1-GetGroupResponse) | Retrieve a group by ID. Authorization: Caller must be a member of the group, or have PERMISSION_GROUPS_ALL_READ or PERMISSION_GROUPS_ALL_WRITE. |
| ListGroups | [ListGroupsRequest](#pidgr-v1-ListGroupsRequest) | [ListGroupsResponse](#pidgr-v1-ListGroupsResponse) | List groups in the organization with pagination. Without PERMISSION_GROUPS_ALL_READ/ALL_WRITE, returns only groups the caller belongs to. |
| UpdateGroup | [UpdateGroupRequest](#pidgr-v1-UpdateGroupRequest) | [UpdateGroupResponse](#pidgr-v1-UpdateGroupResponse) | Update a group&#39;s name and/or description. Authorization: Requires PERMISSION_GROUPS_WRITE (own groups) or PERMISSION_GROUPS_ALL_WRITE (any). |
| DeleteGroup | [DeleteGroupRequest](#pidgr-v1-DeleteGroupRequest) | [DeleteGroupResponse](#pidgr-v1-DeleteGroupResponse) | Delete a group and all its membership records. Default groups cannot be deleted. Authorization: Requires PERMISSION_GROUPS_WRITE (own groups) or PERMISSION_GROUPS_ALL_WRITE (any). |
| AddGroupMembers | [AddGroupMembersRequest](#pidgr-v1-AddGroupMembersRequest) | [AddGroupMembersResponse](#pidgr-v1-AddGroupMembersResponse) | Add one or more users to a group (idempotent). Authorization: Requires PERMISSION_GROUPS_WRITE (own groups) or PERMISSION_GROUPS_ALL_WRITE (any). |
| RemoveGroupMembers | [RemoveGroupMembersRequest](#pidgr-v1-RemoveGroupMembersRequest) | [RemoveGroupMembersResponse](#pidgr-v1-RemoveGroupMembersResponse) | Remove one or more users from a group (idempotent). Authorization: Requires PERMISSION_GROUPS_WRITE (own groups) or PERMISSION_GROUPS_ALL_WRITE (any). |
| ListGroupMembers | [ListGroupMembersRequest](#pidgr-v1-ListGroupMembersRequest) | [ListGroupMembersResponse](#pidgr-v1-ListGroupMembersResponse) | List members of a group with pagination. Authorization: Caller must be a member of the group, or have PERMISSION_GROUPS_ALL_READ or PERMISSION_GROUPS_ALL_WRITE. |
| GetUserGroupMemberships | [GetUserGroupMembershipsRequest](#pidgr-v1-GetUserGroupMembershipsRequest) | [GetUserGroupMembershipsResponse](#pidgr-v1-GetUserGroupMembershipsResponse) | Get group memberships for a batch of users. Used by campaign audience to show group badges. Authorization: Requires PERMISSION_GROUPS_ALL_READ or PERMISSION_GROUPS_ALL_WRITE. |

 



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
| grid_resolution | [float](#float) |  | Grid resolution for coordinate rounding. Default: 0.02 (50×50 grid). Constraints: Range 0.005 to 0.1. |
| mode | [HeatmapMode](#pidgr-v1-HeatmapMode) |  | Aggregation mode (TOTAL or MEDIAN). |
| event_types | [TouchEventType](#pidgr-v1-TouchEventType) | repeated | Optional: filter by event types. Empty list means all types. |






<a name="pidgr-v1-QueryHeatmapDataResponse"></a>

### QueryHeatmapDataResponse
Response containing aggregated heatmap data.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| data_points | [HeatmapDataPoint](#pidgr-v1-HeatmapDataPoint) | repeated | Aggregated data points for heatmap rendering. |
| screenshot_url | [string](#string) |  | URL to a mobile-captured screenshot for this screen, if available. Empty string when no screenshot exists. |
| cohort_enabled | [bool](#bool) |  | Whether per-cohort bucket breakdowns are available (k &gt;= 5). |






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
| campaign_id | [string](#string) |  | Campaign ID if the touch occurred during a campaign message view. Empty string for organic (non-campaign) navigation. |






<a name="pidgr-v1-UploadScreenshotRequest"></a>

### UploadScreenshotRequest
Request to upload a screenshot captured from the mobile app.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| screen_name | [string](#string) |  | Screen name matching React Navigation route (e.g. &#34;MessageDetail::&lt;campaign_uuid&gt;&#34;). Constraints: Max length 200 characters. |
| app_version | [string](#string) |  | App version that captured the screenshot (e.g. &#34;1.15.0&#34;). |
| image_data | [bytes](#bytes) |  | PNG image data. Constraints: Max 512KB. |






<a name="pidgr-v1-UploadScreenshotResponse"></a>

### UploadScreenshotResponse
Response after uploading a screenshot.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| url | [string](#string) |  | S3 URL where the screenshot was stored. |





 


<a name="pidgr-v1-HeatmapMode"></a>

### HeatmapMode
Aggregation mode for heatmap data queries.

| Name | Number | Description |
| ---- | ------ | ----------- |
| HEATMAP_MODE_UNSPECIFIED | 0 | Default value; not a valid mode. |
| HEATMAP_MODE_TOTAL | 1 | Sum of all cohort buckets&#39; touches per grid cell (default). |
| HEATMAP_MODE_MEDIAN | 2 | Median touch count per grid cell across cohort buckets. |



<a name="pidgr-v1-TouchEventType"></a>

### TouchEventType
Type of touch event captured on the mobile app.

| Name | Number | Description |
| ---- | ------ | ----------- |
| TOUCH_EVENT_TYPE_UNSPECIFIED | 0 | Default value; not a valid event type. |
| TOUCH_EVENT_TYPE_TAP | 1 | A single tap on the screen. |
| TOUCH_EVENT_TYPE_LONG_PRESS | 2 | A long press (held for 500ms&#43;). |
| TOUCH_EVENT_TYPE_SCROLL | 3 | A periodic scroll position sample (viewport midpoint every 2s). |
| TOUCH_EVENT_TYPE_ACTION_CLICK | 4 | The user tapped an action button (e.g. &#34;Acknowledge&#34;). |


 

 


<a name="pidgr-v1-HeatmapService"></a>

### HeatmapService
Manages touch event ingestion, heatmap data aggregation, and screen screenshots
for mobile app interaction analytics.

| Method Name | Request Type | Response Type | Description |
| ----------- | ------------ | ------------- | ------------|
| IngestTouchEvents | [IngestTouchEventsRequest](#pidgr-v1-IngestTouchEventsRequest) | [IngestTouchEventsResponse](#pidgr-v1-IngestTouchEventsResponse) | Ingest a batch of touch events from the mobile app. Authorization: Authenticated mobile user. |
| QueryHeatmapData | [QueryHeatmapDataRequest](#pidgr-v1-QueryHeatmapDataRequest) | [QueryHeatmapDataResponse](#pidgr-v1-QueryHeatmapDataResponse) | Query aggregated touch data for heatmap rendering. Authorization: Requires CAMPAIGNS_READ permission. |
| ListScreenshots | [ListScreenshotsRequest](#pidgr-v1-ListScreenshotsRequest) | [ListScreenshotsResponse](#pidgr-v1-ListScreenshotsResponse) | List available screen screenshots for heatmap backgrounds. Authorization: Requires CAMPAIGNS_READ permission. |
| UploadScreenshot | [UploadScreenshotRequest](#pidgr-v1-UploadScreenshotRequest) | [UploadScreenshotResponse](#pidgr-v1-UploadScreenshotResponse) | Upload a screenshot captured from the mobile app for heatmap backdrops. Authorization: Authenticated mobile user. |

 



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

 



<a name="pidgr_v1_invite_link-proto"></a>
<p align="right"><a href="#top">Top</a></p>

## pidgr/v1/invite_link.proto



<a name="pidgr-v1-CreateInviteLinkRequest"></a>

### CreateInviteLinkRequest
Request to create a new invite link for the organization.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| role_id | [string](#string) |  | ID of the role to assign. Defaults to the organization&#39;s employee role if empty. |
| max_uses | [int32](#int32) |  | Maximum number of redemptions. 0 means unlimited. |
| expires_in_hours | [int32](#int32) |  | Number of hours until the link expires. 0 means no expiry. Constraints: Valid range 0 to 8760 (1 year). |






<a name="pidgr-v1-CreateInviteLinkResponse"></a>

### CreateInviteLinkResponse
Response after creating an invite link.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| invite_link | [InviteLink](#pidgr-v1-InviteLink) |  | The newly created invite link. |
| url | [string](#string) |  | Full URL for sharing (e.g. &#34;https://app.pidgr.com/join?token=&lt;TOKEN&gt;&#34;). |






<a name="pidgr-v1-InviteLink"></a>

### InviteLink
A shareable invite link that allows users to self-join an organization.
Links carry a role assignment and optional usage/expiry constraints.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| id | [string](#string) |  | Unique identifier for the invite link. |
| token | [string](#string) |  | Cryptographically random base64url-encoded token (43 characters). |
| role_id | [string](#string) |  | ID of the role assigned to users who redeem this link. |
| max_uses | [int32](#int32) |  | Maximum number of times this link can be redeemed. 0 means unlimited. |
| use_count | [int32](#int32) |  | Number of times this link has been redeemed. |
| expires_at | [google.protobuf.Timestamp](#google-protobuf-Timestamp) |  | When the link expires. Empty if no expiry. |
| revoked_at | [google.protobuf.Timestamp](#google-protobuf-Timestamp) |  | When the link was revoked. Empty if not revoked. |
| created_by | [string](#string) |  | ID of the admin who created the link. |
| created_at | [google.protobuf.Timestamp](#google-protobuf-Timestamp) |  | When the link was created. |






<a name="pidgr-v1-ListInviteLinksRequest"></a>

### ListInviteLinksRequest
Request to list all invite links for the organization.






<a name="pidgr-v1-ListInviteLinksResponse"></a>

### ListInviteLinksResponse
Response containing all invite links for the organization.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| invite_links | [InviteLink](#pidgr-v1-InviteLink) | repeated | All invite links (active, expired, maxed-out, and revoked), ordered by creation date descending. |






<a name="pidgr-v1-RedeemInviteLinkRequest"></a>

### RedeemInviteLinkRequest
Request to redeem an invite link (authenticated — email extracted from JWT).


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| token | [string](#string) |  | The invite link token from the URL query parameter. |






<a name="pidgr-v1-RedeemInviteLinkResponse"></a>

### RedeemInviteLinkResponse
Response after redeeming an invite link.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| organization_name | [string](#string) |  | Name of the organization the user was added to. |






<a name="pidgr-v1-RevokeInviteLinkRequest"></a>

### RevokeInviteLinkRequest
Request to revoke an invite link.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| invite_link_id | [string](#string) |  | ID of the invite link to revoke. Required. |






<a name="pidgr-v1-RevokeInviteLinkResponse"></a>

### RevokeInviteLinkResponse
Response after revoking an invite link.






<a name="pidgr-v1-ValidateInviteLinkRequest"></a>

### ValidateInviteLinkRequest
Request to validate an invite link and provision a user account if needed (unauthenticated).


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| token | [string](#string) |  | The invite link token from the URL query parameter. |
| email | [string](#string) |  | Email address of the user joining the organization. Constraints: Max length 254 characters (RFC 5321). |






<a name="pidgr-v1-ValidateInviteLinkResponse"></a>

### ValidateInviteLinkResponse
Response after validating an invite link.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| organization_name | [string](#string) |  | Name of the organization the invite link belongs to. |





 

 

 


<a name="pidgr-v1-InviteLinkService"></a>

### InviteLinkService
Manages shareable invite links for organization self-join.
Create, List, and Revoke require JWT &#43; PERMISSION_MEMBERS_INVITE.
ValidateInviteLink is unauthenticated — the token IS the authorization.
RedeemInviteLink requires a valid JWT.

| Method Name | Request Type | Response Type | Description |
| ----------- | ------------ | ------------- | ------------|
| CreateInviteLink | [CreateInviteLinkRequest](#pidgr-v1-CreateInviteLinkRequest) | [CreateInviteLinkResponse](#pidgr-v1-CreateInviteLinkResponse) | Create a new shareable invite link for the organization. Authorization: Requires PERMISSION_MEMBERS_INVITE. |
| ListInviteLinks | [ListInviteLinksRequest](#pidgr-v1-ListInviteLinksRequest) | [ListInviteLinksResponse](#pidgr-v1-ListInviteLinksResponse) | List all invite links for the organization. Authorization: Requires PERMISSION_MEMBERS_INVITE. |
| RevokeInviteLink | [RevokeInviteLinkRequest](#pidgr-v1-RevokeInviteLinkRequest) | [RevokeInviteLinkResponse](#pidgr-v1-RevokeInviteLinkResponse) | Revoke an invite link, making it immediately unusable. Idempotent. Authorization: Requires PERMISSION_MEMBERS_INVITE. |
| ValidateInviteLink | [ValidateInviteLinkRequest](#pidgr-v1-ValidateInviteLinkRequest) | [ValidateInviteLinkResponse](#pidgr-v1-ValidateInviteLinkResponse) | Validate an invite link and provision a user account if needed. This is a pre-authentication endpoint — no JWT required. Rate limited to 10 requests per minute per IP. Authorization: None (token-based). |
| RedeemInviteLink | [RedeemInviteLinkRequest](#pidgr-v1-RedeemInviteLinkRequest) | [RedeemInviteLinkResponse](#pidgr-v1-RedeemInviteLinkResponse) | Redeem an invite link to join an organization. Requires a valid JWT — the email is extracted from the token. Authorization: JWT required. |

 



<a name="pidgr_v1_member-proto"></a>
<p align="right"><a href="#top">Top</a></p>

## pidgr/v1/member.proto



<a name="pidgr-v1-BulkInviteResult"></a>

### BulkInviteResult
Per-email result within a bulk invite operation.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| email | [string](#string) |  | The email address that was processed. |
| success | [bool](#bool) |  | Whether the invitation succeeded. |
| error | [string](#string) |  | Error message if the invitation failed (e.g. &#34;user already exists&#34;). Empty on success. |
| user | [User](#pidgr-v1-User) |  | The created user. Only set on success. |






<a name="pidgr-v1-BulkInviteUsersRequest"></a>

### BulkInviteUsersRequest
Request to invite multiple users to the organization in a single call.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| emails | [string](#string) | repeated | Email addresses to invite. Constraints: Min 1, max 100 emails. Duplicates are deduplicated before processing. |
| role_id | [string](#string) |  | ID of the role to assign. Defaults to the organization&#39;s employee role if empty. |






<a name="pidgr-v1-BulkInviteUsersResponse"></a>

### BulkInviteUsersResponse
Response after bulk inviting users.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| results | [BulkInviteResult](#pidgr-v1-BulkInviteResult) | repeated | Per-email results in the same order as the deduplicated input. |
| invited_count | [int32](#int32) |  | Number of users successfully invited. |
| failed_count | [int32](#int32) |  | Number of emails that failed. |






<a name="pidgr-v1-ConfirmPasskeyEnrollmentRequest"></a>

### ConfirmPasskeyEnrollmentRequest
Request to confirm passkey enrollment after client-side WebAuthn registration.
The server verifies that the caller has at least one registered WebAuthn
credential before setting the enrollment attribute.






<a name="pidgr-v1-ConfirmPasskeyEnrollmentResponse"></a>

### ConfirmPasskeyEnrollmentResponse
Response after confirming passkey enrollment.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| confirmed | [bool](#bool) |  | Whether enrollment was confirmed and the user attribute was updated. |






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






<a name="pidgr-v1-GetUserSettingsRequest"></a>

### GetUserSettingsRequest
Request to retrieve the caller&#39;s platform settings.






<a name="pidgr-v1-GetUserSettingsResponse"></a>

### GetUserSettingsResponse
Response containing the caller&#39;s platform settings.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| settings | [UserSettings](#pidgr-v1-UserSettings) |  | Current settings. Fields at their default value indicate the platform default. |






<a name="pidgr-v1-InviteUserRequest"></a>

### InviteUserRequest
Request to invite a new user to the organization.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| email | [string](#string) |  | Email address to send the invitation to. Constraints: Max length 254 characters (RFC 5321). |
| name | [string](#string) |  | Display name for the invited user. Constraints: Max length 200 characters. |
| role_id | [string](#string) |  | ID of the role to assign. Defaults to the organization&#39;s employee role if empty. |
| profile | [UserProfile](#pidgr-v1-UserProfile) |  | Optional profile attributes to pre-fill at invitation time. |






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






<a name="pidgr-v1-ReactivateUserRequest"></a>

### ReactivateUserRequest
Request to reactivate a deactivated user.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| user_id | [string](#string) |  | ID of the user to reactivate. |






<a name="pidgr-v1-ReactivateUserResponse"></a>

### ReactivateUserResponse
Response after reactivating a user.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| user | [User](#pidgr-v1-User) |  | The reactivated user (status: INVITED). |






<a name="pidgr-v1-RevokeInviteRequest"></a>

### RevokeInviteRequest
Request to revoke an invitation for a user who has not yet registered.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| user_id | [string](#string) |  | ID of the invited user to remove. Constraints: UUID format (36 characters). |






<a name="pidgr-v1-RevokeInviteResponse"></a>

### RevokeInviteResponse
Response after revoking an invitation. Empty on success.






<a name="pidgr-v1-UpdateUserProfileRequest"></a>

### UpdateUserProfileRequest
Request to update a user&#39;s profile attributes.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| user_id | [string](#string) |  | ID of the user whose profile to update. Empty or matching the caller&#39;s own ID allows self-update without PERMISSION_MEMBERS_MANAGE. |
| profile | [UserProfile](#pidgr-v1-UserProfile) |  | Profile attributes to set. All provided fields overwrite existing values. |






<a name="pidgr-v1-UpdateUserProfileResponse"></a>

### UpdateUserProfileResponse
Response after updating a user&#39;s profile.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| user | [User](#pidgr-v1-User) |  | The updated user with the new profile. |






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






<a name="pidgr-v1-UpdateUserSettingsRequest"></a>

### UpdateUserSettingsRequest
Request to update the caller&#39;s platform settings.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| settings | [UserSettings](#pidgr-v1-UserSettings) |  | Settings to update. Only fields with non-default (non-UNSPECIFIED) values are applied; default-valued fields are left unchanged. |






<a name="pidgr-v1-UpdateUserSettingsResponse"></a>

### UpdateUserSettingsResponse
Response after updating the caller&#39;s platform settings.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| settings | [UserSettings](#pidgr-v1-UserSettings) |  | The full settings after the update. |





 

 

 


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
| ReactivateUser | [ReactivateUserRequest](#pidgr-v1-ReactivateUserRequest) | [ReactivateUserResponse](#pidgr-v1-ReactivateUserResponse) | Reactivate a deactivated user, restoring their status to INVITED. The user must complete the invite link flow again to become ACTIVE. Authorization: Requires PERMISSION_MEMBERS_MANAGE. |
| RevokeInvite | [RevokeInviteRequest](#pidgr-v1-RevokeInviteRequest) | [RevokeInviteResponse](#pidgr-v1-RevokeInviteResponse) | Revoke an invitation for a user who has not yet completed registration. Hard-deletes the user record (INVITED users have no data to preserve). Authorization: Requires PERMISSION_MEMBERS_MANAGE. |
| UpdateUserProfile | [UpdateUserProfileRequest](#pidgr-v1-UpdateUserProfileRequest) | [UpdateUserProfileResponse](#pidgr-v1-UpdateUserProfileResponse) | Update a user&#39;s profile attributes (department, title, etc.). Self-update (empty user_id or matching JWT sub) requires no special permission. Updating another user requires PERMISSION_MEMBERS_MANAGE. |
| GetUserSettings | [GetUserSettingsRequest](#pidgr-v1-GetUserSettingsRequest) | [GetUserSettingsResponse](#pidgr-v1-GetUserSettingsResponse) | Retrieve the caller&#39;s platform settings (theme, etc.). Authorization: Any authenticated user (self-only). |
| UpdateUserSettings | [UpdateUserSettingsRequest](#pidgr-v1-UpdateUserSettingsRequest) | [UpdateUserSettingsResponse](#pidgr-v1-UpdateUserSettingsResponse) | Update the caller&#39;s platform settings. Only fields with non-default values are applied; others are left unchanged. Authorization: Any authenticated user (self-only). |
| BulkInviteUsers | [BulkInviteUsersRequest](#pidgr-v1-BulkInviteUsersRequest) | [BulkInviteUsersResponse](#pidgr-v1-BulkInviteUsersResponse) | Invite multiple users to the organization in a single call. Emails are deduplicated. Each email is processed independently — individual failures do not abort the batch. Identity provider calls are parallelized (bounded concurrency). Authorization: Requires PERMISSION_MEMBERS_INVITE. |
| ConfirmPasskeyEnrollment | [ConfirmPasskeyEnrollmentRequest](#pidgr-v1-ConfirmPasskeyEnrollmentRequest) | [ConfirmPasskeyEnrollmentResponse](#pidgr-v1-ConfirmPasskeyEnrollmentResponse) | Confirm passkey enrollment after client-side WebAuthn registration. Verifies the caller has at least one registered credential server-side, then marks the user as passkey-enrolled in the identity provider. Authorization: Any authenticated user (self-only, no permission required). |

 



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
| admin_email | [string](#string) |  | Email address for the initial admin user. Only used with API key auth; ignored with JWT auth (email derived from identity provider subject). |
| industry | [Industry](#pidgr-v1-Industry) |  | Industry vertical for the organization. |
| company_size | [CompanySize](#pidgr-v1-CompanySize) |  | Employee headcount range. |
| access_code | [string](#string) |  | Access code required during early access (JWT auth only). Ignored with API key auth. Format: PIDGR-XXXXXXXX (8 alphanumeric characters). |






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
| sso_attribute_mappings | [SsoAttributeMapping](#pidgr-v1-SsoAttributeMapping) | repeated | SSO identity provider claim-to-profile mappings. Empty when the organization does not use SSO. |
| default_locale | [string](#string) |  | Default language for new users in this organization. Empty means no org default (users auto-detect from device/browser). Valid values: en, es, pt-BR, zh, ja. |






<a name="pidgr-v1-RotateAnalyticsSaltRequest"></a>

### RotateAnalyticsSaltRequest
Request to rotate the analytics salt and optionally increase the bucket count.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| new_bucket_count | [int32](#int32) |  | New bucket count. Must be &gt;= current bucket count. 0 means keep current. |






<a name="pidgr-v1-RotateAnalyticsSaltResponse"></a>

### RotateAnalyticsSaltResponse
Response after rotating the analytics salt.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| bucket_count | [int32](#int32) |  | The new bucket count after rotation. |






<a name="pidgr-v1-SsoAttributeMapping"></a>

### SsoAttributeMapping
Maps an identity provider claim to a user profile field.
Used for automatic profile population when users authenticate via SSO/SAML.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| idp_claim | [string](#string) |  | Claim name from the identity provider (e.g. &#34;urn:oid:2.5.4.11&#34;, &#34;given_name&#34;). Constraints: Max length 500 characters. |
| profile_field | [string](#string) |  | Target UserProfile field name (e.g. &#34;department&#34;, &#34;first_name&#34;). For custom attributes, use &#34;custom:&#34; prefix (e.g. &#34;custom:cost_center&#34;). Constraints: Max length 100 characters. |






<a name="pidgr-v1-UpdateAnalyticsEpsilonRequest"></a>

### UpdateAnalyticsEpsilonRequest
Request to update the analytics epsilon (differential privacy parameter).


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| epsilon | [float](#float) |  | New epsilon value. Must be in range [0.5, 5.0]. |






<a name="pidgr-v1-UpdateAnalyticsEpsilonResponse"></a>

### UpdateAnalyticsEpsilonResponse
Response after updating the analytics epsilon.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| epsilon | [float](#float) |  | The new epsilon value. |






<a name="pidgr-v1-UpdateOrganizationRequest"></a>

### UpdateOrganizationRequest
Request to update organization settings.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| name | [string](#string) |  | New organization name. Empty string leaves unchanged. Constraints: Max length 200 characters. |
| default_workflow | [WorkflowDefinition](#pidgr-v1-WorkflowDefinition) |  | New default workflow definition. Null leaves unchanged. |
| industry | [Industry](#pidgr-v1-Industry) |  | New industry vertical. UNSPECIFIED leaves unchanged. |
| company_size | [CompanySize](#pidgr-v1-CompanySize) |  | New employee headcount range. UNSPECIFIED leaves unchanged. |
| default_locale | [string](#string) |  | New default language for new users. Empty string leaves unchanged. Valid values: en, es, pt-BR, zh, ja. |






<a name="pidgr-v1-UpdateOrganizationResponse"></a>

### UpdateOrganizationResponse
Response after updating the organization.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| organization | [Organization](#pidgr-v1-Organization) |  | The updated organization. |






<a name="pidgr-v1-UpdateSsoAttributeMappingsRequest"></a>

### UpdateSsoAttributeMappingsRequest
Request to replace all SSO attribute mappings for the organization.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| sso_attribute_mappings | [SsoAttributeMapping](#pidgr-v1-SsoAttributeMapping) | repeated | Complete list of SSO mappings (replaces all existing mappings). |






<a name="pidgr-v1-UpdateSsoAttributeMappingsResponse"></a>

### UpdateSsoAttributeMappingsResponse
Response after updating SSO attribute mappings.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| organization | [Organization](#pidgr-v1-Organization) |  | The updated organization with the new SSO mappings. |





 


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
| UpdateSsoAttributeMappings | [UpdateSsoAttributeMappingsRequest](#pidgr-v1-UpdateSsoAttributeMappingsRequest) | [UpdateSsoAttributeMappingsResponse](#pidgr-v1-UpdateSsoAttributeMappingsResponse) | Replace all SSO attribute mappings for the organization. Authorization: Requires PERMISSION_ORG_WRITE. |
| RotateAnalyticsSalt | [RotateAnalyticsSaltRequest](#pidgr-v1-RotateAnalyticsSaltRequest) | [RotateAnalyticsSaltResponse](#pidgr-v1-RotateAnalyticsSaltResponse) | Rotate the analytics salt and optionally increase the bucket count for k-anonymization. Authorization: Requires PERMISSION_PRIVACY_WRITE. |
| UpdateAnalyticsEpsilon | [UpdateAnalyticsEpsilonRequest](#pidgr-v1-UpdateAnalyticsEpsilonRequest) | [UpdateAnalyticsEpsilonResponse](#pidgr-v1-UpdateAnalyticsEpsilonResponse) | Update the differential privacy epsilon parameter. Authorization: Requires PERMISSION_PRIVACY_WRITE. |

 



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
| recording_id | [string](#string) |  | Recording ID from the analytics provider. Constraints: Max length 200 characters. |






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
| campaign_id | [string](#string) |  | Optional: filter recordings by campaign ID (mapped to analytics property filter). Constraints: UUID format (36 characters). |
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
A session recording summary from the analytics provider.
Anonymous: no user identifiers are included.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| id | [string](#string) |  | Recording ID from the analytics provider. |
| start_time | [google.protobuf.Timestamp](#google-protobuf-Timestamp) |  | Timestamp when the recording started. |
| end_time | [google.protobuf.Timestamp](#google-protobuf-Timestamp) |  | Timestamp when the recording ended. |
| duration_seconds | [int32](#int32) |  | Duration of the recording in seconds. |
| activity_score | [float](#float) |  | Activity score (0.0–1.0). |





 

 

 


<a name="pidgr-v1-ReplayService"></a>

### ReplayService
Proxies the analytics provider&#39;s session recording API, keeping credentials server-side.
All data is fetched from the analytics provider on demand; no recording data is stored in pidgr.

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
| provider_name | [string](#string) |  | Identity provider name for signInWithRedirect. Empty if sso_enabled is false. |






<a name="pidgr-v1-CreateSSOProviderRequest"></a>

### CreateSSOProviderRequest
Request to create an SSO provider for the organization.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| domain | [string](#string) |  | Email domain to associate (e.g. &#34;acme.com&#34;). Constraints: Max length 253 characters (RFC 1035). |
| type | [SSOProviderType](#pidgr-v1-SSOProviderType) |  | Type of identity provider. |
| metadata_url | [string](#string) |  | SAML metadata URL or OIDC discovery URL. Constraints: Max length 2048 characters. HTTPS required. |
| attribute_mapping | [SamlAttributeNames](#pidgr-v1-SamlAttributeNames) |  | Optional custom SAML attribute name overrides. When omitted, attribute names are auto-detected from the metadata URL. |






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






<a name="pidgr-v1-SSOProvider"></a>

### SSOProvider
An SSO identity provider configured for an organization.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| id | [string](#string) |  | Unique identifier for the provider. |
| domain | [string](#string) |  | Email domain that triggers this SSO provider (e.g. &#34;acme.com&#34;). Constraints: Max length 253 characters (RFC 1035). |
| type | [SSOProviderType](#pidgr-v1-SSOProviderType) |  | Type of identity provider. |
| metadata_url | [string](#string) |  | SAML metadata URL or OIDC discovery URL. Constraints: Max length 2048 characters. HTTPS required. |
| idp_provider_name | [string](#string) |  | Name of the identity provider (used for signInWithRedirect). Set by the API when the IdP is created. |
| created_at | [google.protobuf.Timestamp](#google-protobuf-Timestamp) |  | Timestamp when the provider was created. |
| updated_at | [google.protobuf.Timestamp](#google-protobuf-Timestamp) |  | Timestamp when the provider was last updated. |
| attribute_mapping | [SamlAttributeNames](#pidgr-v1-SamlAttributeNames) |  | Optional custom SAML attribute name overrides. |






<a name="pidgr-v1-SamlAttributeNames"></a>

### SamlAttributeNames
Custom SAML attribute name overrides for identity providers that use
non-standard attribute names. When provided, these override the
auto-detected values from the metadata URL host.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| email | [string](#string) |  | SAML attribute name for the user&#39;s email address. |
| given_name | [string](#string) |  | SAML attribute name for the user&#39;s first name. |
| family_name | [string](#string) |  | SAML attribute name for the user&#39;s last name. |





 


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
| CreateSSOProvider | [CreateSSOProviderRequest](#pidgr-v1-CreateSSOProviderRequest) | [CreateSSOProviderResponse](#pidgr-v1-CreateSSOProviderResponse) | Create an SSO provider for the organization. Validates the metadata URL before saving. Creates the corresponding identity provider in the auth service. Authorization: Requires PERMISSION_ORG_WRITE. |
| GetSSOProvider | [GetSSOProviderRequest](#pidgr-v1-GetSSOProviderRequest) | [GetSSOProviderResponse](#pidgr-v1-GetSSOProviderResponse) | Get the organization&#39;s SSO provider configuration. Authorization: Requires PERMISSION_ORG_READ. |
| DeleteSSOProvider | [DeleteSSOProviderRequest](#pidgr-v1-DeleteSSOProviderRequest) | [DeleteSSOProviderResponse](#pidgr-v1-DeleteSSOProviderResponse) | Delete the organization&#39;s SSO provider. Deletes the corresponding identity provider from the auth service. Users with that domain fall back to passkey/OTP. Authorization: Requires PERMISSION_ORG_WRITE. |

 



<a name="pidgr_v1_team-proto"></a>
<p align="right"><a href="#top">Top</a></p>

## pidgr/v1/team.proto



<a name="pidgr-v1-AddTeamMembersRequest"></a>

### AddTeamMembersRequest
Request to add users to a team.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| team_id | [string](#string) |  | ID of the team to add members to. Required. |
| user_ids | [string](#string) | repeated | IDs of users to add. Must belong to the same organization. Adding an existing member is a no-op (idempotent). Constraints: Max 100 user IDs per request. |






<a name="pidgr-v1-AddTeamMembersResponse"></a>

### AddTeamMembersResponse
Response after adding team members.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| team | [Team](#pidgr-v1-Team) |  | The team with updated member_count. |






<a name="pidgr-v1-CreateTeamRequest"></a>

### CreateTeamRequest
Request to create a new team.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| name | [string](#string) |  | Display name for the team. Required. Constraints: Max length 200 characters. |
| description | [string](#string) |  | Optional description. Constraints: Max length 1000 characters. |






<a name="pidgr-v1-CreateTeamResponse"></a>

### CreateTeamResponse
Response after creating a team.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| team | [Team](#pidgr-v1-Team) |  | The newly created team. |






<a name="pidgr-v1-DeleteTeamRequest"></a>

### DeleteTeamRequest
Request to delete a team.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| team_id | [string](#string) |  | ID of the team to delete. Required. Default teams cannot be deleted. |






<a name="pidgr-v1-DeleteTeamResponse"></a>

### DeleteTeamResponse
Response after deleting a team.






<a name="pidgr-v1-GetTeamRequest"></a>

### GetTeamRequest
Request to retrieve a team by ID.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| team_id | [string](#string) |  | ID of the team to retrieve. Required. |






<a name="pidgr-v1-GetTeamResponse"></a>

### GetTeamResponse
Response containing the requested team.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| team | [Team](#pidgr-v1-Team) |  | The requested team. |






<a name="pidgr-v1-ListTeamMembersRequest"></a>

### ListTeamMembersRequest
Request to list members of a team with pagination.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| team_id | [string](#string) |  | ID of the team whose members to list. Required. |
| pagination | [Pagination](#pidgr-v1-Pagination) |  | Pagination parameters. |






<a name="pidgr-v1-ListTeamMembersResponse"></a>

### ListTeamMembersResponse
Response containing a page of team members.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| users | [User](#pidgr-v1-User) | repeated | Users in this page. |
| pagination_meta | [PaginationMeta](#pidgr-v1-PaginationMeta) |  | Pagination metadata for fetching subsequent pages. |






<a name="pidgr-v1-ListTeamsRequest"></a>

### ListTeamsRequest
Request to list teams in the organization with pagination.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| pagination | [Pagination](#pidgr-v1-Pagination) |  | Pagination parameters. |






<a name="pidgr-v1-ListTeamsResponse"></a>

### ListTeamsResponse
Response containing a page of teams.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| teams | [Team](#pidgr-v1-Team) | repeated | Teams in this page. |
| pagination_meta | [PaginationMeta](#pidgr-v1-PaginationMeta) |  | Pagination metadata for fetching subsequent pages. |






<a name="pidgr-v1-RemoveTeamMembersRequest"></a>

### RemoveTeamMembersRequest
Request to remove users from a team.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| team_id | [string](#string) |  | ID of the team to remove members from. Required. |
| user_ids | [string](#string) | repeated | IDs of users to remove. Removing a non-member is a no-op (idempotent). Constraints: Max 100 user IDs per request. |






<a name="pidgr-v1-RemoveTeamMembersResponse"></a>

### RemoveTeamMembersResponse
Response after removing team members.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| team | [Team](#pidgr-v1-Team) |  | The team with updated member_count. |






<a name="pidgr-v1-Team"></a>

### Team
An organizational unit within an organization (e.g. department, division).
Teams represent the organizational structure and can serve as sender identity
in campaigns.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| id | [string](#string) |  | Unique identifier for the team. |
| name | [string](#string) |  | Human-readable display name (unique within the organization). Constraints: Max length 200 characters. |
| description | [string](#string) |  | Optional description of the team&#39;s purpose. Constraints: Max length 1000 characters. |
| member_count | [int32](#int32) |  | Number of users currently in the team. |
| created_at | [google.protobuf.Timestamp](#google-protobuf-Timestamp) |  | Timestamp when the team was created. |
| updated_at | [google.protobuf.Timestamp](#google-protobuf-Timestamp) |  | Timestamp when the team was last updated. |
| is_default | [bool](#bool) |  | Whether this is the organization&#39;s default team (cannot be deleted or renamed). |
| created_by | [string](#string) |  | ID of the user who created this team. Empty for system-seeded defaults. |






<a name="pidgr-v1-UpdateTeamRequest"></a>

### UpdateTeamRequest
Request to update a team&#39;s name and/or description.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| team_id | [string](#string) |  | ID of the team to update. Required. |
| name | [string](#string) |  | New display name. If empty, the name is not changed. Default teams cannot be renamed. Constraints: Max length 200 characters. |
| description | [string](#string) |  | New description. If empty, the description is not changed. Constraints: Max length 1000 characters. |






<a name="pidgr-v1-UpdateTeamResponse"></a>

### UpdateTeamResponse
Response after updating a team.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| team | [Team](#pidgr-v1-Team) |  | The updated team. |





 

 

 


<a name="pidgr-v1-TeamService"></a>

### TeamService
Manages organizational teams (departments, divisions) within an organization.
Teams represent the organizational structure and can serve as sender identity.
All RPCs operate within the caller&#39;s org (extracted from JWT).

| Method Name | Request Type | Response Type | Description |
| ----------- | ------------ | ------------- | ------------|
| CreateTeam | [CreateTeamRequest](#pidgr-v1-CreateTeamRequest) | [CreateTeamResponse](#pidgr-v1-CreateTeamResponse) | Create a new team in the organization. Authorization: Requires PERMISSION_TEAMS_WRITE or PERMISSION_TEAMS_ALL_WRITE. |
| GetTeam | [GetTeamRequest](#pidgr-v1-GetTeamRequest) | [GetTeamResponse](#pidgr-v1-GetTeamResponse) | Retrieve a team by ID. Authorization: Caller must be a member of the team, or have PERMISSION_TEAMS_ALL_READ or PERMISSION_TEAMS_ALL_WRITE. |
| ListTeams | [ListTeamsRequest](#pidgr-v1-ListTeamsRequest) | [ListTeamsResponse](#pidgr-v1-ListTeamsResponse) | List teams in the organization with pagination. Without PERMISSION_TEAMS_ALL_READ/ALL_WRITE, returns only teams the caller belongs to. |
| UpdateTeam | [UpdateTeamRequest](#pidgr-v1-UpdateTeamRequest) | [UpdateTeamResponse](#pidgr-v1-UpdateTeamResponse) | Update a team&#39;s name and/or description. Authorization: Requires PERMISSION_TEAMS_WRITE (own teams) or PERMISSION_TEAMS_ALL_WRITE (any). |
| DeleteTeam | [DeleteTeamRequest](#pidgr-v1-DeleteTeamRequest) | [DeleteTeamResponse](#pidgr-v1-DeleteTeamResponse) | Delete a team and all its membership records. Default teams cannot be deleted. Authorization: Requires PERMISSION_TEAMS_WRITE (own teams) or PERMISSION_TEAMS_ALL_WRITE (any). |
| AddTeamMembers | [AddTeamMembersRequest](#pidgr-v1-AddTeamMembersRequest) | [AddTeamMembersResponse](#pidgr-v1-AddTeamMembersResponse) | Add one or more users to a team (idempotent). Authorization: Requires PERMISSION_TEAMS_WRITE (own teams) or PERMISSION_TEAMS_ALL_WRITE (any). |
| RemoveTeamMembers | [RemoveTeamMembersRequest](#pidgr-v1-RemoveTeamMembersRequest) | [RemoveTeamMembersResponse](#pidgr-v1-RemoveTeamMembersResponse) | Remove one or more users from a team (idempotent). Authorization: Requires PERMISSION_TEAMS_WRITE (own teams) or PERMISSION_TEAMS_ALL_WRITE (any). |
| ListTeamMembers | [ListTeamMembersRequest](#pidgr-v1-ListTeamMembersRequest) | [ListTeamMembersResponse](#pidgr-v1-ListTeamMembersResponse) | List members of a team with pagination. Authorization: Caller must be a member of the team, or have PERMISSION_TEAMS_ALL_READ or PERMISSION_TEAMS_ALL_WRITE. |

 



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
| type | [TemplateType](#pidgr-v1-TemplateType) |  | Content format of the template. Defaults to MARKDOWN if unspecified. |






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
| type | [TemplateType](#pidgr-v1-TemplateType) |  | Filter by template type. UNSPECIFIED returns all templates. |






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
| type | [TemplateType](#pidgr-v1-TemplateType) |  | Content format of this template (markdown, rich, HTML). UNSPECIFIED is treated as MARKDOWN for backward compatibility. |






<a name="pidgr-v1-TemplateVariable"></a>

### TemplateVariable
A variable placeholder within a template that gets substituted during rendering.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| name | [string](#string) |  | Variable name used in the template body (e.g. &#34;employee_name&#34;). Constraints: Max length 100 characters. |
| description | [string](#string) |  | Human-readable description of what this variable represents. Constraints: Max length 500 characters. |
| required | [bool](#bool) |  | Whether this variable must be provided during rendering. |
| source | [TemplateVariableSource](#pidgr-v1-TemplateVariableSource) |  | Where this variable&#39;s value comes from (profile attribute or campaign config). |
| default_value | [string](#string) |  | Fallback value used when the source does not provide a value. Constraints: Max length 1000 characters. |
| pii | [bool](#bool) |  | When true, this variable&#39;s rendered value is masked in session replay and heatmap screenshots. Org admin controls per variable. |






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





 


<a name="pidgr-v1-TemplateType"></a>

### TemplateType
Content format of a template, determining which editor and renderer to use.

| Name | Number | Description |
| ---- | ------ | ----------- |
| TEMPLATE_TYPE_UNSPECIFIED | 0 | Default value; treated as MARKDOWN for backward compatibility. |
| TEMPLATE_TYPE_MARKDOWN | 1 | Markdown with {{variable}} placeholders. |
| TEMPLATE_TYPE_RICH | 2 | Rich text format (reserved for future use). |
| TEMPLATE_TYPE_HTML | 3 | Raw HTML format (reserved for future use). |



<a name="pidgr-v1-TemplateVariableSource"></a>

### TemplateVariableSource
Source from which a template variable&#39;s value is resolved at render time.

| Name | Number | Description |
| ---- | ------ | ----------- |
| TEMPLATE_VARIABLE_SOURCE_UNSPECIFIED | 0 | Default value; treated as CUSTOM for backward compatibility. |
| TEMPLATE_VARIABLE_SOURCE_PROFILE | 1 | Auto-resolved from the target user&#39;s profile attributes. |
| TEMPLATE_VARIABLE_SOURCE_CUSTOM | 2 | Provided manually in the campaign or workflow step configuration. |


 

 


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

