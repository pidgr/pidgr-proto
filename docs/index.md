# Protocol Documentation
<a name="top"></a>

## Table of Contents

- [pidgr/v1/action.proto](#pidgr_v1_action-proto)
    - [SubmitActionRequest](#pidgr-v1-SubmitActionRequest)
    - [SubmitActionResponse](#pidgr-v1-SubmitActionResponse)
  
    - [ActionService](#pidgr-v1-ActionService)
  
- [pidgr/v1/channel_events.proto](#pidgr_v1_channel_events-proto)
    - [ChannelEvent](#pidgr-v1-ChannelEvent)
    - [RecordChannelEventBatchRequest](#pidgr-v1-RecordChannelEventBatchRequest)
    - [RecordChannelEventBatchResponse](#pidgr-v1-RecordChannelEventBatchResponse)
    - [RecordChannelEventBatchResult](#pidgr-v1-RecordChannelEventBatchResult)
    - [RecordChannelEventRequest](#pidgr-v1-RecordChannelEventRequest)
    - [RecordChannelEventResponse](#pidgr-v1-RecordChannelEventResponse)
  
    - [ChannelEventStatus](#pidgr-v1-ChannelEventStatus)
    - [ChannelName](#pidgr-v1-ChannelName)
    - [ChannelSkipReason](#pidgr-v1-ChannelSkipReason)
    - [ChannelStepKind](#pidgr-v1-ChannelStepKind)
  
    - [ChannelEventsService](#pidgr-v1-ChannelEventsService)
  
- [pidgr/v1/common.proto](#pidgr_v1_common-proto)
    - [CallWebhookConfig](#pidgr-v1-CallWebhookConfig)
    - [CallWebhookConfig.HeadersEntry](#pidgr-v1-CallWebhookConfig-HeadersEntry)
    - [DeadlineCheckConfig](#pidgr-v1-DeadlineCheckConfig)
    - [EscalateConfig](#pidgr-v1-EscalateConfig)
    - [EscalationTarget](#pidgr-v1-EscalationTarget)
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
    - [EscalateMode](#pidgr-v1-EscalateMode)
    - [EscalationCondition](#pidgr-v1-EscalationCondition)
    - [EscalationTargetType](#pidgr-v1-EscalationTargetType)
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
  
    - [KeyType](#pidgr-v1-KeyType)
  
    - [ApiKeyService](#pidgr-v1-ApiKeyService)
  
- [pidgr/v1/privacy.proto](#pidgr_v1_privacy-proto)
    - [CancelDeletionRequest](#pidgr-v1-CancelDeletionRequest)
    - [CancelDeletionResponse](#pidgr-v1-CancelDeletionResponse)
    - [DeleteUserDataRequest](#pidgr-v1-DeleteUserDataRequest)
    - [DeleteUserDataResponse](#pidgr-v1-DeleteUserDataResponse)
    - [ExportOrgDataRequest](#pidgr-v1-ExportOrgDataRequest)
    - [ExportOrgDataResponse](#pidgr-v1-ExportOrgDataResponse)
    - [ExportUserDataRequest](#pidgr-v1-ExportUserDataRequest)
    - [ExportUserDataResponse](#pidgr-v1-ExportUserDataResponse)
    - [GetDataExistenceConfirmationRequest](#pidgr-v1-GetDataExistenceConfirmationRequest)
    - [GetDataExistenceConfirmationResponse](#pidgr-v1-GetDataExistenceConfirmationResponse)
    - [ImmediateDeleteRequest](#pidgr-v1-ImmediateDeleteRequest)
    - [ImmediateDeleteResponse](#pidgr-v1-ImmediateDeleteResponse)
    - [ListMyPrivacyRequestsRequest](#pidgr-v1-ListMyPrivacyRequestsRequest)
    - [ListMyPrivacyRequestsResponse](#pidgr-v1-ListMyPrivacyRequestsResponse)
    - [ListOrgSecurityIncidentsRequest](#pidgr-v1-ListOrgSecurityIncidentsRequest)
    - [ListOrgSecurityIncidentsResponse](#pidgr-v1-ListOrgSecurityIncidentsResponse)
    - [ListPrivacyRequestsRequest](#pidgr-v1-ListPrivacyRequestsRequest)
    - [ListPrivacyRequestsResponse](#pidgr-v1-ListPrivacyRequestsResponse)
    - [OrgSecurityIncident](#pidgr-v1-OrgSecurityIncident)
    - [PrivacyRequest](#pidgr-v1-PrivacyRequest)
    - [PrivacyRequest.MetadataEntry](#pidgr-v1-PrivacyRequest-MetadataEntry)
    - [RectifyUserDataRequest](#pidgr-v1-RectifyUserDataRequest)
    - [RectifyUserDataRequest.CorrectionsEntry](#pidgr-v1-RectifyUserDataRequest-CorrectionsEntry)
    - [RectifyUserDataResponse](#pidgr-v1-RectifyUserDataResponse)
    - [RestrictProcessingRequest](#pidgr-v1-RestrictProcessingRequest)
    - [RestrictProcessingResponse](#pidgr-v1-RestrictProcessingResponse)
  
    - [PrivacyRequestStatus](#pidgr-v1-PrivacyRequestStatus)
    - [SecurityIncidentClassification](#pidgr-v1-SecurityIncidentClassification)
    - [SecurityIncidentSeverity](#pidgr-v1-SecurityIncidentSeverity)
  
    - [PrivacyService](#pidgr-v1-PrivacyService)
  
- [pidgr/v1/audit.proto](#pidgr_v1_audit-proto)
    - [AppendRequest](#pidgr-v1-AppendRequest)
    - [AppendResponse](#pidgr-v1-AppendResponse)
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
  
- [pidgr/v1/authorization.proto](#pidgr_v1_authorization-proto)
    - [ResolvePrincipalPermissionsRequest](#pidgr-v1-ResolvePrincipalPermissionsRequest)
    - [ResolvePrincipalPermissionsResponse](#pidgr-v1-ResolvePrincipalPermissionsResponse)
  
    - [PrincipalType](#pidgr-v1-PrincipalType)
  
    - [AuthorizationService](#pidgr-v1-AuthorizationService)
  
- [pidgr/v1/campaign.proto](#pidgr_v1_campaign-proto)
    - [ArchetypeShareShift](#pidgr-v1-ArchetypeShareShift)
    - [AudienceMember](#pidgr-v1-AudienceMember)
    - [AudienceMember.VariablesEntry](#pidgr-v1-AudienceMember-VariablesEntry)
    - [Campaign](#pidgr-v1-Campaign)
    - [CampaignOriginatingArchetype](#pidgr-v1-CampaignOriginatingArchetype)
    - [CancelCampaignRequest](#pidgr-v1-CancelCampaignRequest)
    - [CancelCampaignResponse](#pidgr-v1-CancelCampaignResponse)
    - [CreateCampaignRequest](#pidgr-v1-CreateCampaignRequest)
    - [CreateCampaignResponse](#pidgr-v1-CreateCampaignResponse)
    - [Delivery](#pidgr-v1-Delivery)
    - [DeliveryMetadata](#pidgr-v1-DeliveryMetadata)
    - [GetCampaignArchetypeBreakdownRequest](#pidgr-v1-GetCampaignArchetypeBreakdownRequest)
    - [GetCampaignArchetypeBreakdownResponse](#pidgr-v1-GetCampaignArchetypeBreakdownResponse)
    - [GetCampaignByShortCodeRequest](#pidgr-v1-GetCampaignByShortCodeRequest)
    - [GetCampaignByShortCodeResponse](#pidgr-v1-GetCampaignByShortCodeResponse)
    - [GetCampaignRequest](#pidgr-v1-GetCampaignRequest)
    - [GetCampaignResponse](#pidgr-v1-GetCampaignResponse)
    - [ListCampaignsRequest](#pidgr-v1-ListCampaignsRequest)
    - [ListCampaignsResponse](#pidgr-v1-ListCampaignsResponse)
    - [ListDeliveriesRequest](#pidgr-v1-ListDeliveriesRequest)
    - [ListDeliveriesResponse](#pidgr-v1-ListDeliveriesResponse)
    - [ResolveOrCreateShortCodeRequest](#pidgr-v1-ResolveOrCreateShortCodeRequest)
    - [ResolveOrCreateShortCodeResponse](#pidgr-v1-ResolveOrCreateShortCodeResponse)
    - [StartCampaignRequest](#pidgr-v1-StartCampaignRequest)
    - [StartCampaignResponse](#pidgr-v1-StartCampaignResponse)
    - [UpdateCampaignRequest](#pidgr-v1-UpdateCampaignRequest)
    - [UpdateCampaignResponse](#pidgr-v1-UpdateCampaignResponse)
  
    - [Delivery.Kind](#pidgr-v1-Delivery-Kind)
  
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
  
- [pidgr/v1/insights.proto](#pidgr_v1_insights-proto)
    - [Archetype](#pidgr-v1-Archetype)
    - [Archetype.FeatureBreakdownEntry](#pidgr-v1-Archetype-FeatureBreakdownEntry)
    - [Archetype.FeatureCentroidEntry](#pidgr-v1-Archetype-FeatureCentroidEntry)
    - [ArchetypeForecast](#pidgr-v1-ArchetypeForecast)
    - [CampaignAdvisory](#pidgr-v1-CampaignAdvisory)
    - [CohortPrediction](#pidgr-v1-CohortPrediction)
    - [DimensionStats](#pidgr-v1-DimensionStats)
    - [ExemplarSession](#pidgr-v1-ExemplarSession)
    - [ForecastHorizon](#pidgr-v1-ForecastHorizon)
    - [GenerateCampaignBodyDraftRequest](#pidgr-v1-GenerateCampaignBodyDraftRequest)
    - [GenerateCampaignBodyDraftResponse](#pidgr-v1-GenerateCampaignBodyDraftResponse)
    - [GetCampaignAdvisoryRequest](#pidgr-v1-GetCampaignAdvisoryRequest)
    - [GetCampaignAdvisoryResponse](#pidgr-v1-GetCampaignAdvisoryResponse)
    - [GetGroupArchetypesRequest](#pidgr-v1-GetGroupArchetypesRequest)
    - [GetGroupArchetypesResponse](#pidgr-v1-GetGroupArchetypesResponse)
    - [GetInsightNarrativeRequest](#pidgr-v1-GetInsightNarrativeRequest)
    - [GetInsightNarrativeResponse](#pidgr-v1-GetInsightNarrativeResponse)
    - [LatencyPercentiles](#pidgr-v1-LatencyPercentiles)
    - [PredictCampaignACKRequest](#pidgr-v1-PredictCampaignACKRequest)
    - [PredictCampaignACKResponse](#pidgr-v1-PredictCampaignACKResponse)
    - [ResponseTimeline](#pidgr-v1-ResponseTimeline)
    - [ScreenDwell](#pidgr-v1-ScreenDwell)
    - [ScreenDwellEntry](#pidgr-v1-ScreenDwellEntry)
    - [TapHeatmap](#pidgr-v1-TapHeatmap)
    - [TapHeatmapLayer](#pidgr-v1-TapHeatmapLayer)
    - [TriggerArchetypeClusteringRequest](#pidgr-v1-TriggerArchetypeClusteringRequest)
    - [TriggerArchetypeClusteringResponse](#pidgr-v1-TriggerArchetypeClusteringResponse)
    - [TriggerMLPipelineRequest](#pidgr-v1-TriggerMLPipelineRequest)
    - [TriggerMLPipelineResponse](#pidgr-v1-TriggerMLPipelineResponse)
  
    - [ArchetypeSource](#pidgr-v1-ArchetypeSource)
    - [ConfidenceLevel](#pidgr-v1-ConfidenceLevel)
    - [PipelineState](#pidgr-v1-PipelineState)
  
    - [InsightsService](#pidgr-v1-InsightsService)
  
- [pidgr/v1/integrations.proto](#pidgr_v1_integrations-proto)
    - [Reachability](#pidgr-v1-Reachability)
    - [RegionPolicy](#pidgr-v1-RegionPolicy)
  
    - [DispatchStatus](#pidgr-v1-DispatchStatus)
  
- [pidgr/v1/integrations_service.proto](#pidgr_v1_integrations_service-proto)
    - [CreateChannelConnectLinkRequest](#pidgr-v1-CreateChannelConnectLinkRequest)
    - [CreateChannelConnectLinkResponse](#pidgr-v1-CreateChannelConnectLinkResponse)
    - [DispatchToChannelRequest](#pidgr-v1-DispatchToChannelRequest)
    - [DispatchToChannelRequest.TemplateVarsEntry](#pidgr-v1-DispatchToChannelRequest-TemplateVarsEntry)
    - [DispatchToChannelResponse](#pidgr-v1-DispatchToChannelResponse)
    - [GetCostCapPolicyRequest](#pidgr-v1-GetCostCapPolicyRequest)
    - [GetCostCapPolicyResponse](#pidgr-v1-GetCostCapPolicyResponse)
    - [GetOrgWebhookConfigRequest](#pidgr-v1-GetOrgWebhookConfigRequest)
    - [GetOrgWebhookConfigResponse](#pidgr-v1-GetOrgWebhookConfigResponse)
    - [GetReachabilityRequest](#pidgr-v1-GetReachabilityRequest)
    - [GetReachabilityResponse](#pidgr-v1-GetReachabilityResponse)
    - [GetRegionPolicyRequest](#pidgr-v1-GetRegionPolicyRequest)
    - [GetRegionPolicyResponse](#pidgr-v1-GetRegionPolicyResponse)
    - [ListReachabilityForUserRequest](#pidgr-v1-ListReachabilityForUserRequest)
    - [ListReachabilityForUserResponse](#pidgr-v1-ListReachabilityForUserResponse)
    - [RemoveReachabilityRequest](#pidgr-v1-RemoveReachabilityRequest)
    - [RemoveReachabilityResponse](#pidgr-v1-RemoveReachabilityResponse)
    - [SetCostCapPolicyRequest](#pidgr-v1-SetCostCapPolicyRequest)
    - [SetCostCapPolicyResponse](#pidgr-v1-SetCostCapPolicyResponse)
    - [SetOrgWebhookConfigRequest](#pidgr-v1-SetOrgWebhookConfigRequest)
    - [SetOrgWebhookConfigResponse](#pidgr-v1-SetOrgWebhookConfigResponse)
    - [SetRegionPolicyRequest](#pidgr-v1-SetRegionPolicyRequest)
    - [SetRegionPolicyResponse](#pidgr-v1-SetRegionPolicyResponse)
    - [UpsertReachabilityRequest](#pidgr-v1-UpsertReachabilityRequest)
    - [UpsertReachabilityResponse](#pidgr-v1-UpsertReachabilityResponse)
  
    - [IntegrationsService](#pidgr-v1-IntegrationsService)
  
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
    - [UpdateUserRegionRequest](#pidgr-v1-UpdateUserRegionRequest)
    - [UpdateUserRegionResponse](#pidgr-v1-UpdateUserRegionResponse)
    - [UpdateUserRoleRequest](#pidgr-v1-UpdateUserRoleRequest)
    - [UpdateUserRoleResponse](#pidgr-v1-UpdateUserRoleResponse)
    - [UpdateUserSettingsRequest](#pidgr-v1-UpdateUserSettingsRequest)
    - [UpdateUserSettingsResponse](#pidgr-v1-UpdateUserSettingsResponse)
  
    - [MemberService](#pidgr-v1-MemberService)
  
- [pidgr/v1/org_security_keys_service.proto](#pidgr_v1_org_security_keys_service-proto)
    - [GetPeppersRequest](#pidgr-v1-GetPeppersRequest)
    - [GetPeppersResponse](#pidgr-v1-GetPeppersResponse)
    - [Pepper](#pidgr-v1-Pepper)
  
    - [OrgSecurityKeysService](#pidgr-v1-OrgSecurityKeysService)
  
- [pidgr/v1/organization.proto](#pidgr_v1_organization-proto)
    - [CreateOrganizationRequest](#pidgr-v1-CreateOrganizationRequest)
    - [CreateOrganizationResponse](#pidgr-v1-CreateOrganizationResponse)
    - [CreateSandboxOrganizationRequest](#pidgr-v1-CreateSandboxOrganizationRequest)
    - [CreateSandboxOrganizationResponse](#pidgr-v1-CreateSandboxOrganizationResponse)
    - [DeleteSandboxOrganizationRequest](#pidgr-v1-DeleteSandboxOrganizationRequest)
    - [DeleteSandboxOrganizationResponse](#pidgr-v1-DeleteSandboxOrganizationResponse)
    - [GetOrgPrivacySettingsRequest](#pidgr-v1-GetOrgPrivacySettingsRequest)
    - [GetOrgPrivacySettingsResponse](#pidgr-v1-GetOrgPrivacySettingsResponse)
    - [GetOrganizationRequest](#pidgr-v1-GetOrganizationRequest)
    - [GetOrganizationResponse](#pidgr-v1-GetOrganizationResponse)
    - [ListSandboxFixturesRequest](#pidgr-v1-ListSandboxFixturesRequest)
    - [ListSandboxFixturesResponse](#pidgr-v1-ListSandboxFixturesResponse)
    - [ListUserOrganizationsRequest](#pidgr-v1-ListUserOrganizationsRequest)
    - [ListUserOrganizationsResponse](#pidgr-v1-ListUserOrganizationsResponse)
    - [ListUserSandboxesRequest](#pidgr-v1-ListUserSandboxesRequest)
    - [ListUserSandboxesResponse](#pidgr-v1-ListUserSandboxesResponse)
    - [OrgPrivacySettings](#pidgr-v1-OrgPrivacySettings)
    - [OrgPrivacyToggle](#pidgr-v1-OrgPrivacyToggle)
    - [Organization](#pidgr-v1-Organization)
    - [RotateAnalyticsSaltRequest](#pidgr-v1-RotateAnalyticsSaltRequest)
    - [RotateAnalyticsSaltResponse](#pidgr-v1-RotateAnalyticsSaltResponse)
    - [SandboxFixture](#pidgr-v1-SandboxFixture)
    - [SsoAttributeMapping](#pidgr-v1-SsoAttributeMapping)
    - [UpdateAnalyticsEpsilonRequest](#pidgr-v1-UpdateAnalyticsEpsilonRequest)
    - [UpdateAnalyticsEpsilonResponse](#pidgr-v1-UpdateAnalyticsEpsilonResponse)
    - [UpdateOrgPrivacySettingsRequest](#pidgr-v1-UpdateOrgPrivacySettingsRequest)
    - [UpdateOrgPrivacySettingsResponse](#pidgr-v1-UpdateOrgPrivacySettingsResponse)
    - [UpdateOrganizationRequest](#pidgr-v1-UpdateOrganizationRequest)
    - [UpdateOrganizationResponse](#pidgr-v1-UpdateOrganizationResponse)
    - [UpdateSsoAttributeMappingsRequest](#pidgr-v1-UpdateSsoAttributeMappingsRequest)
    - [UpdateSsoAttributeMappingsResponse](#pidgr-v1-UpdateSsoAttributeMappingsResponse)
  
    - [CompanySize](#pidgr-v1-CompanySize)
    - [Industry](#pidgr-v1-Industry)
    - [OrgType](#pidgr-v1-OrgType)
  
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
    - [ApproveTemplateTranslationRequest](#pidgr-v1-ApproveTemplateTranslationRequest)
    - [ApproveTemplateTranslationResponse](#pidgr-v1-ApproveTemplateTranslationResponse)
    - [CreateTemplateRequest](#pidgr-v1-CreateTemplateRequest)
    - [CreateTemplateResponse](#pidgr-v1-CreateTemplateResponse)
    - [CreateTemplateTranslationRequest](#pidgr-v1-CreateTemplateTranslationRequest)
    - [CreateTemplateTranslationResponse](#pidgr-v1-CreateTemplateTranslationResponse)
    - [GetTemplateRequest](#pidgr-v1-GetTemplateRequest)
    - [GetTemplateResponse](#pidgr-v1-GetTemplateResponse)
    - [ListTemplateTranslationsRequest](#pidgr-v1-ListTemplateTranslationsRequest)
    - [ListTemplateTranslationsResponse](#pidgr-v1-ListTemplateTranslationsResponse)
    - [ListTemplatesRequest](#pidgr-v1-ListTemplatesRequest)
    - [ListTemplatesResponse](#pidgr-v1-ListTemplatesResponse)
    - [Template](#pidgr-v1-Template)
    - [TemplateTranslation](#pidgr-v1-TemplateTranslation)
    - [TemplateVariable](#pidgr-v1-TemplateVariable)
    - [UpdateTemplateRequest](#pidgr-v1-UpdateTemplateRequest)
    - [UpdateTemplateResponse](#pidgr-v1-UpdateTemplateResponse)
    - [UpdateTemplateTranslationRequest](#pidgr-v1-UpdateTemplateTranslationRequest)
    - [UpdateTemplateTranslationResponse](#pidgr-v1-UpdateTemplateTranslationResponse)
  
    - [TemplateType](#pidgr-v1-TemplateType)
    - [TemplateVariableSource](#pidgr-v1-TemplateVariableSource)
    - [TranslationStatus](#pidgr-v1-TranslationStatus)
  
    - [TemplateService](#pidgr-v1-TemplateService)
  
- [pidgr/v1/token.proto](#pidgr_v1_token-proto)
    - [DeeplinkTokenPayload](#pidgr-v1-DeeplinkTokenPayload)
    - [SignDeeplinkTokenRequest](#pidgr-v1-SignDeeplinkTokenRequest)
    - [SignDeeplinkTokenResponse](#pidgr-v1-SignDeeplinkTokenResponse)
    - [ValidateDeeplinkTokenRequest](#pidgr-v1-ValidateDeeplinkTokenRequest)
    - [ValidateDeeplinkTokenResponse](#pidgr-v1-ValidateDeeplinkTokenResponse)
  
    - [ValidationFailureReason](#pidgr-v1-ValidationFailureReason)
  
    - [TokenService](#pidgr-v1-TokenService)
  
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
Actions drive workflow progression (e.g. ACK completes a wait step).

| Method Name | Request Type | Response Type | Description |
| ----------- | ------------ | ------------- | ------------|
| SubmitAction | [SubmitActionRequest](#pidgr-v1-SubmitActionRequest) | [SubmitActionResponse](#pidgr-v1-SubmitActionResponse) | Submit an action for a specific delivery, advancing the campaign workflow. Backend MUST verify the authenticated user is the delivery recipient. |

 



<a name="pidgr_v1_channel_events-proto"></a>
<p align="right"><a href="#top">Top</a></p>

## pidgr/v1/channel_events.proto



<a name="pidgr-v1-ChannelEvent"></a>

### ChannelEvent
A single channel dispatch event for the audit trail. Append-only; the
receiver enforces idempotency on terminal states via a partial unique index
on (campaign_id, recipient_user_id, channel, step_kind).


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| org_id | [string](#string) |  |  |
| campaign_id | [string](#string) |  |  |
| recipient_user_id | [string](#string) |  |  |
| channel | [ChannelName](#pidgr-v1-ChannelName) |  |  |
| step_kind | [ChannelStepKind](#pidgr-v1-ChannelStepKind) |  |  |
| status | [ChannelEventStatus](#pidgr-v1-ChannelEventStatus) |  |  |
| skip_reason | [ChannelSkipReason](#pidgr-v1-ChannelSkipReason) |  | Set only when status = SKIPPED. UNSPECIFIED in all other cases. |
| provider_message_id | [string](#string) |  | Provider&#39;s identifier for this dispatch. Empty for SKIPPED events. |
| cost_micros | [int64](#int64) |  | Cost in micros (1/1000000 of a USD). Zero for absorbed channels. Negative is invalid. |
| metadata_json | [string](#string) |  | Free-form provider error payload on FAILED. JSON-encoded; opaque to the platform. |
| occurred_at | [google.protobuf.Timestamp](#google-protobuf-Timestamp) |  |  |






<a name="pidgr-v1-RecordChannelEventBatchRequest"></a>

### RecordChannelEventBatchRequest



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| events | [ChannelEvent](#pidgr-v1-ChannelEvent) | repeated |  |






<a name="pidgr-v1-RecordChannelEventBatchResponse"></a>

### RecordChannelEventBatchResponse



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| results | [RecordChannelEventBatchResult](#pidgr-v1-RecordChannelEventBatchResult) | repeated |  |






<a name="pidgr-v1-RecordChannelEventBatchResult"></a>

### RecordChannelEventBatchResult
Per-event result inside a batch. Order matches the request&#39;s events list.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| accepted | [bool](#bool) |  |  |
| reason | [string](#string) |  |  |






<a name="pidgr-v1-RecordChannelEventRequest"></a>

### RecordChannelEventRequest



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| event | [ChannelEvent](#pidgr-v1-ChannelEvent) |  |  |






<a name="pidgr-v1-RecordChannelEventResponse"></a>

### RecordChannelEventResponse



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| accepted | [bool](#bool) |  | True if the row was inserted. False if rejected as a duplicate of an existing terminal-state row. |
| reason | [string](#string) |  | &#34;duplicate&#34; when accepted=false and the partial unique index rejected the insert. Empty when accepted=true. |





 


<a name="pidgr-v1-ChannelEventStatus"></a>

### ChannelEventStatus
Status of a channel dispatch attempt. The table is append-only — each state
transition (e.g. SENT → DELIVERED via provider webhook) is its own row keyed
off provider_message_id, not an UPDATE.

| Name | Number | Description |
| ---- | ------ | ----------- |
| CHANNEL_EVENT_STATUS_UNSPECIFIED | 0 |  |
| CHANNEL_EVENT_STATUS_SENT | 1 |  |
| CHANNEL_EVENT_STATUS_DELIVERED | 2 |  |
| CHANNEL_EVENT_STATUS_OPENED | 3 |  |
| CHANNEL_EVENT_STATUS_CLICKED | 4 |  |
| CHANNEL_EVENT_STATUS_BOUNCED | 5 |  |
| CHANNEL_EVENT_STATUS_FAILED | 6 |  |
| CHANNEL_EVENT_STATUS_SKIPPED | 7 |  |



<a name="pidgr-v1-ChannelName"></a>

### ChannelName
Third-party notification channel for reminder &#43; escalation dispatch.

Push is intentionally NOT in this enum. Push is the primary channel; it
always fires alongside any third-party channels. The third-party channels
here are additive. Channels carry only a deeplink notification — message
content stays in the platform.

| Name | Number | Description |
| ---- | ------ | ----------- |
| CHANNEL_NAME_UNSPECIFIED | 0 |  |
| CHANNEL_NAME_EMAIL | 1 |  |
| CHANNEL_NAME_WEBHOOK | 2 |  |
| CHANNEL_NAME_TELEGRAM | 3 |  |
| CHANNEL_NAME_SLACK | 4 |  |
| CHANNEL_NAME_SMS | 5 |  |
| CHANNEL_NAME_WHATSAPP | 6 |  |
| CHANNEL_NAME_MICROSOFT_TEAMS | 7 |  |
| CHANNEL_NAME_LINE | 8 |  |
| CHANNEL_NAME_GOOGLE_CHAT | 9 |  |



<a name="pidgr-v1-ChannelSkipReason"></a>

### ChannelSkipReason
Reason a dispatch was SKIPPED rather than attempted. Set when status is
CHANNEL_EVENT_STATUS_SKIPPED; UNSPECIFIED otherwise.

| Name | Number | Description |
| ---- | ------ | ----------- |
| CHANNEL_SKIP_REASON_UNSPECIFIED | 0 |  |
| CHANNEL_SKIP_REASON_OPTED_OUT | 1 |  |
| CHANNEL_SKIP_REASON_REGION_BLOCKED | 2 |  |
| CHANNEL_SKIP_REASON_COST_CAP_EXCEEDED | 3 |  |
| CHANNEL_SKIP_REASON_NO_IDENTIFIER | 4 |  |



<a name="pidgr-v1-ChannelStepKind"></a>

### ChannelStepKind
Workflow step kind that triggered the channel dispatch. Different step
kinds for the same (campaign, recipient, channel) tuple are treated as
distinct dispatch events for idempotency purposes.

| Name | Number | Description |
| ---- | ------ | ----------- |
| CHANNEL_STEP_KIND_UNSPECIFIED | 0 |  |
| CHANNEL_STEP_KIND_REMINDER | 1 |  |
| CHANNEL_STEP_KIND_ESCALATION | 2 |  |


 

 


<a name="pidgr-v1-ChannelEventsService"></a>

### ChannelEventsService
ChannelEventsService records third-party channel dispatch events into the
platform&#39;s append-only audit table. Intended for internal-service callers
behind a service-mesh / mTLS boundary; not exposed for end-user clients.

| Method Name | Request Type | Response Type | Description |
| ----------- | ------------ | ------------- | ------------|
| RecordChannelEvent | [RecordChannelEventRequest](#pidgr-v1-RecordChannelEventRequest) | [RecordChannelEventResponse](#pidgr-v1-RecordChannelEventResponse) | Record a single channel event. Returns accepted=false reason=&#34;duplicate&#34; when the partial unique index rejects (caller treats as already-recorded, not as an error). |
| RecordChannelEventBatch | [RecordChannelEventBatchRequest](#pidgr-v1-RecordChannelEventBatchRequest) | [RecordChannelEventBatchResponse](#pidgr-v1-RecordChannelEventBatchResponse) | Record N events in one round-trip. Used by webhook ingress when a provider delivers multiple state transitions in one payload (e.g. a batched delivery report). Best-effort per-row. |

 



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






<a name="pidgr-v1-EscalateConfig"></a>

### EscalateConfig
Configuration for an escalation step in the workflow DAG.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| condition | [EscalationCondition](#pidgr-v1-EscalationCondition) |  | Condition that triggers escalation. |
| targets | [EscalationTarget](#pidgr-v1-EscalationTarget) | repeated | Targets to notify when escalation fires. |
| repeat_count | [int32](#int32) |  | Number of times to repeat this escalation before moving to the next step. Constraints: Max 5. |
| repeat_interval_minutes | [int32](#int32) |  | Minutes between repeat attempts. |
| mode | [EscalateMode](#pidgr-v1-EscalateMode) |  | Behavior mode for this escalation. UNSPECIFIED is normalized to DELIVER. |
| third_party_channels | [ChannelName](#pidgr-v1-ChannelName) | repeated | Additional third-party channels to dispatch the escalation through alongside the primary push / delivery side effect. Empty = no third-party fan-out (existing behaviour). Each entry produces an independent dispatch attempt recorded in `channel_events`. ALERT_ONLY and DELIVER modes both support third-party fan-out — the channel adapters render the alert content from the campaign &#43; a mode-aware copy variant. |






<a name="pidgr-v1-EscalationTarget"></a>

### EscalationTarget
A target for escalation — who should be notified when escalation fires.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| type | [EscalationTargetType](#pidgr-v1-EscalationTargetType) |  | Type of target. |
| target_id | [string](#string) |  | ID of the target (user_id, group_id, or role_id). Empty for MANAGER type (resolved at runtime from recipient&#39;s manager_id). |






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
| third_party_channels | [ChannelName](#pidgr-v1-ChannelName) | repeated | Additional third-party channels to dispatch the reminder through alongside the primary push notification. Empty = push-only behaviour (the platform&#39;s historical default; no surprise for existing workflows). Each entry produces an independent dispatch attempt recorded in `channel_events`; per-org configuration in pidgr-integrations decides which channels are eligible at runtime. |
| notify_targets | [EscalationTarget](#pidgr-v1-EscalationTarget) | repeated | Third parties to loop in when this reminder fires. Each resolved target receives a passive inbox delivery (no action button) plus a fan-out via the same `third_party_channels` list as the employee reminder. The delivery auto-dismisses when the original recipient acknowledges the campaign.

Each entry reuses the existing `EscalationTarget` shape (USER / GROUP / MANAGER / ROLE). When `type` is MANAGER, `target_id` is empty and is resolved at runtime from the original recipient&#39;s `manager_id`. Self-targets (resolved user_id == original recipient) are dropped at dispatch time. Constraints: Max 5 entries. |






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
| escalate_config | [EscalateConfig](#pidgr-v1-EscalateConfig) |  | Configuration for STEP_TYPE_ESCALATE steps. |
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



<a name="pidgr-v1-EscalateMode"></a>

### EscalateMode
Behavior mode controlling what an escalation produces for its targets.

| Name | Number | Description |
| ---- | ------ | ----------- |
| ESCALATE_MODE_UNSPECIFIED | 0 | Default value; servers normalize this to ESCALATE_MODE_DELIVER. |
| ESCALATE_MODE_DELIVER | 1 | Targets receive a delivery for the campaign just like primary recipients. |
| ESCALATE_MODE_ALERT_ONLY | 2 | Targets receive an out-of-band alert only; no delivery is created. |



<a name="pidgr-v1-EscalationCondition"></a>

### EscalationCondition
Condition that must be met for an escalation to fire.

| Name | Number | Description |
| ---- | ------ | ----------- |
| ESCALATION_CONDITION_UNSPECIFIED | 0 |  |
| ESCALATION_CONDITION_IF_NOT_ACKED | 1 | Escalate if the delivery has not been acknowledged. |
| ESCALATION_CONDITION_IF_NOT_CLOSED | 2 | Escalate if the campaign is still open (even if some deliveries are acknowledged). |



<a name="pidgr-v1-EscalationTargetType"></a>

### EscalationTargetType
Type of escalation target.

| Name | Number | Description |
| ---- | ------ | ----------- |
| ESCALATION_TARGET_TYPE_UNSPECIFIED | 0 |  |
| ESCALATION_TARGET_TYPE_USER | 1 | Escalate to a specific user by ID. |
| ESCALATION_TARGET_TYPE_GROUP | 2 | Escalate to all members of a group. |
| ESCALATION_TARGET_TYPE_MANAGER | 3 | Escalate to the recipient&#39;s direct manager (resolved from manager_id at runtime). |
| ESCALATION_TARGET_TYPE_ROLE | 4 | Escalate to all users with a specific role in the org. |



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
| PERMISSION_TEMPLATES_REVIEW | 22 | Review and approve template translations. |
| PERMISSION_PLATFORM_SUPPORT | 23 | Cross-organization read access for platform-level support operations. Assignable only to roles within an ORG_TYPE_STAFF organization. |
| PERMISSION_PLATFORM_ACCESS_CODES | 24 | Manage platform access codes (generation, listing, revocation). Assignable only to roles within an ORG_TYPE_STAFF organization. |
| PERMISSION_PLATFORM_PROVISION | 25 | Provision and manage organizations at the platform level. Assignable only to roles within an ORG_TYPE_STAFF organization. |
| PERMISSION_PLATFORM_ABUSE_RESPONSE | 26 | Take abuse-response actions against organizations (suspend, revoke, quota overrides). Assignable only to roles within an ORG_TYPE_STAFF organization. |
| PERMISSION_PLATFORM_COMPLIANCE_WRITE | 27 | Write subprocessor and compliance records at the platform level. Assignable only to roles within an ORG_TYPE_STAFF organization. |
| PERMISSION_PLATFORM_SYNTHETIC | 28 | Create synthetic (flagged) data on any org: seed resources and simulate campaign outcomes. Assignable only to roles within an ORG_TYPE_STAFF organization. |
| PERMISSION_CHANNELS_DISPATCH | 29 | Dispatch notifications to third-party channels (Slack, Telegram, webhook, etc.). |
| PERMISSION_REACHABILITY_WRITE | 30 | Create, update, or remove a member&#39;s third-party channel reachability. |
| PERMISSION_PLATFORM_INCIDENTS | 31 | Triage security incidents (list, classify, mark-notified) at the platform level. Assignable only to roles within an ORG_TYPE_STAFF organization. |



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
| STEP_TYPE_ESCALATE | 6 | Escalate unacknowledged deliveries to configured targets. |


 

 

 



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
| key_type | [KeyType](#pidgr-v1-KeyType) |  | Type of this key (API key or SCIM token). Defaults to KEY_TYPE_API_KEY for existing keys. |






<a name="pidgr-v1-CreateApiKeyRequest"></a>

### CreateApiKeyRequest
Request to create a new API key.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| name | [string](#string) |  | Human-friendly label. Required, max 200 characters. |
| permissions | [Permission](#pidgr-v1-Permission) | repeated | Permissions to grant. Required, at least one. PERMISSION_UNSPECIFIED values are rejected. |
| expires_at | [google.protobuf.Timestamp](#google-protobuf-Timestamp) |  | Optional expiration time. If omitted, the key does not expire. |
| key_type | [KeyType](#pidgr-v1-KeyType) |  | Type of key to create. Defaults to KEY_TYPE_API_KEY. SCIM tokens use the &#34;pidgr_scim_&#34; prefix instead of &#34;pidgr_k_&#34;. |






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


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| key_type | [KeyType](#pidgr-v1-KeyType) |  | Optional filter by key type. Unspecified returns all keys. |






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





 


<a name="pidgr-v1-KeyType"></a>

### KeyType
Type of API key, distinguishing platform keys from SCIM provisioning tokens.

| Name | Number | Description |
| ---- | ------ | ----------- |
| KEY_TYPE_UNSPECIFIED | 0 |  |
| KEY_TYPE_API_KEY | 1 |  |
| KEY_TYPE_SCIM_TOKEN | 2 |  |


 

 


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






<a name="pidgr-v1-ExportOrgDataRequest"></a>

### ExportOrgDataRequest
Request to export all data associated with the calling organization
(GDPR Art. 20 data portability at the org level). The organization is
extracted from the JWT — it is never in the request message.
Auth: Requires JWT. Org admin only.






<a name="pidgr-v1-ExportOrgDataResponse"></a>

### ExportOrgDataResponse
Response containing the org export status and download location.
The export workflow assembles org configuration, users, campaigns,
deliveries, and audit events into an encrypted bundle delivered via a
pre-signed S3 URL.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| status | [PrivacyRequestStatus](#pidgr-v1-PrivacyRequestStatus) |  | Current status of the export request. |
| result_url | [string](#string) |  | Pre-signed S3 URL to download the exported bundle (encrypted ZIP). Only populated when status is COMPLETED. |
| export_id | [string](#string) |  | Unique identifier for this export request. Constraints: UUID format (36 characters). |






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






<a name="pidgr-v1-ListMyPrivacyRequestsRequest"></a>

### ListMyPrivacyRequestsRequest
Request to list the calling user&#39;s own privacy requests.
Auth: Requires JWT. No admin permission required — returns only the caller&#39;s requests.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| page_size | [int32](#int32) |  | Maximum number of results per page. Constraints: 1–100, default 25. |
| page_token | [string](#string) |  | Continuation token from a previous response. |
| request_type | [string](#string) |  | Filter by request type (export, rectify). Empty = all. |
| status | [PrivacyRequestStatus](#pidgr-v1-PrivacyRequestStatus) |  | Filter by status. UNSPECIFIED = all. |






<a name="pidgr-v1-ListMyPrivacyRequestsResponse"></a>

### ListMyPrivacyRequestsResponse
Response containing the calling user&#39;s privacy requests.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| requests | [PrivacyRequest](#pidgr-v1-PrivacyRequest) | repeated | The privacy requests belonging to the calling user. |
| next_page_token | [string](#string) |  | Token for the next page. Empty if no more results. |






<a name="pidgr-v1-ListOrgSecurityIncidentsRequest"></a>

### ListOrgSecurityIncidentsRequest
Request to list security incidents that touched the calling organization.
The organization is extracted from the JWT — it is never in the request.
Auth: Requires JWT. Admin only.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| page_size | [int32](#int32) |  | Maximum number of results per page. Constraints: 1–100, default 25. |
| page_token | [string](#string) |  | Continuation token from a previous response. |






<a name="pidgr-v1-ListOrgSecurityIncidentsResponse"></a>

### ListOrgSecurityIncidentsResponse
Response containing the organization&#39;s security incident feed.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| incidents | [OrgSecurityIncident](#pidgr-v1-OrgSecurityIncident) | repeated | Incidents that touched the organization, ordered by detected_at descending (newest first). |
| next_page_token | [string](#string) |  | Token for the next page. Empty if no more results. |






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






<a name="pidgr-v1-OrgSecurityIncident"></a>

### OrgSecurityIncident
A security incident that touched the calling organization. Org-facing
read-only subset of the staff-side incident record — internal triage
fields (detector signal, classifier identity, evidence pointers) are
intentionally not exposed.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| id | [string](#string) |  | Unique identifier for the incident. Constraints: UUID format (36 characters). |
| detected_at | [google.protobuf.Timestamp](#google-protobuf-Timestamp) |  | When the observability platform detected the incident. The canonical anchor for the 72-hour GDPR Art. 33 notification clock. |
| severity | [SecurityIncidentSeverity](#pidgr-v1-SecurityIncidentSeverity) |  | Detector-assigned severity. |
| classification | [SecurityIncidentClassification](#pidgr-v1-SecurityIncidentClassification) |  | Legal classification verdict. PENDING until staff triage completes. |
| notified_at | [google.protobuf.Timestamp](#google-protobuf-Timestamp) |  | When the regulator was notified. Empty if no notification was required or it has not happened yet. |
| resolved_at | [google.protobuf.Timestamp](#google-protobuf-Timestamp) |  | When the incident was resolved. Empty while still open. |






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



<a name="pidgr-v1-SecurityIncidentClassification"></a>

### SecurityIncidentClassification
Legal classification verdict recorded by platform staff during triage.
Mirrors the staff-side incident taxonomy; immutable once set.

| Name | Number | Description |
| ---- | ------ | ----------- |
| SECURITY_INCIDENT_CLASSIFICATION_UNSPECIFIED | 0 | Default value; should not be used explicitly. |
| SECURITY_INCIDENT_CLASSIFICATION_PENDING | 1 | Queued for triage; no verdict recorded yet. |
| SECURITY_INCIDENT_CLASSIFICATION_NOT_BREACH | 2 | Triage concluded the incident is not a breach. |
| SECURITY_INCIDENT_CLASSIFICATION_OPERATIONAL_ONLY | 10 | Operational incident with no personal data involved. |
| SECURITY_INCIDENT_CLASSIFICATION_PERSONAL_DATA_BREACH | 11 | Personal data breach (GDPR Art. 33 notification clock running). |
| SECURITY_INCIDENT_CLASSIFICATION_PERSONAL_DATA_BREACH_HIGH_RISK | 12 | Personal data breach with high risk to data subjects (GDPR Art. 34). |



<a name="pidgr-v1-SecurityIncidentSeverity"></a>

### SecurityIncidentSeverity
Detector-assigned severity of a security incident. Mirrors the staff-side
incident taxonomy; the org feed exposes the same values read-only.

| Name | Number | Description |
| ---- | ------ | ----------- |
| SECURITY_INCIDENT_SEVERITY_UNSPECIFIED | 0 | Default value; should not be used explicitly. |
| SECURITY_INCIDENT_SEVERITY_INFO | 1 | Informational signal; no action expected. |
| SECURITY_INCIDENT_SEVERITY_WARN | 2 | Anomalous signal under investigation. |
| SECURITY_INCIDENT_SEVERITY_BREACH | 3 | Confirmed or suspected breach-grade signal. |


 

 


<a name="pidgr-v1-PrivacyService"></a>

### PrivacyService
PrivacyService handles GDPR/LGPD data subject rights.
All RPCs extract org_id from the JWT — it is never in request messages.

| Method Name | Request Type | Response Type | Description |
| ----------- | ------------ | ------------- | ------------|
| ExportUserData | [ExportUserDataRequest](#pidgr-v1-ExportUserDataRequest) | [ExportUserDataResponse](#pidgr-v1-ExportUserDataResponse) | Export all personal data associated with a user as a downloadable ZIP. Async operation — returns immediately with PENDING status, sends push notification when the export is ready. Auth: Requires JWT. Callable by the user themselves or an org admin. |
| ExportOrgData | [ExportOrgDataRequest](#pidgr-v1-ExportOrgDataRequest) | [ExportOrgDataResponse](#pidgr-v1-ExportOrgDataResponse) | Export all data associated with the calling organization as an encrypted bundle (GDPR Art. 20). The workflow assembles org configuration, users, campaigns, deliveries, and audit events; the bundle is delivered via a pre-signed S3 URL. Async operation — returns immediately with PENDING status and an export_id to poll. Auth: Requires JWT. Org admin only. |
| DeleteUserData | [DeleteUserDataRequest](#pidgr-v1-DeleteUserDataRequest) | [DeleteUserDataResponse](#pidgr-v1-DeleteUserDataResponse) | Delete or anonymize all personal data associated with a user. Deletion has a 30-day grace period during which processing is restricted and the request can be cancelled. After 30 days, deletion is irreversible. Auth: Requires JWT. Admin only. |
| RectifyUserData | [RectifyUserDataRequest](#pidgr-v1-RectifyUserDataRequest) | [RectifyUserDataResponse](#pidgr-v1-RectifyUserDataResponse) | Correct personal data for a user. Propagates corrections to all stored locations (profile, delivery records, analytics metadata). Auth: Requires JWT. Callable by the user themselves or an org admin. |
| RestrictProcessing | [RestrictProcessingRequest](#pidgr-v1-RestrictProcessingRequest) | [RestrictProcessingResponse](#pidgr-v1-RestrictProcessingResponse) | Restrict or unrestrict processing for a user. When restricted, the API skips this user in campaigns, analytics, and session replay. Auth: Requires JWT. Admin only. |
| GetDataExistenceConfirmation | [GetDataExistenceConfirmationRequest](#pidgr-v1-GetDataExistenceConfirmationRequest) | [GetDataExistenceConfirmationResponse](#pidgr-v1-GetDataExistenceConfirmationResponse) | Confirm whether personal data exists for a user and list data categories. LGPD-specific: confirmação de existência (Art. 18, I). Auth: Requires JWT. Admin only. |
| ListPrivacyRequests | [ListPrivacyRequestsRequest](#pidgr-v1-ListPrivacyRequestsRequest) | [ListPrivacyRequestsResponse](#pidgr-v1-ListPrivacyRequestsResponse) | List privacy requests for the organization, with optional filters. Used by the admin UI to show scheduled deletions table. Auth: Requires JWT. Admin only. |
| CancelDeletion | [CancelDeletionRequest](#pidgr-v1-CancelDeletionRequest) | [CancelDeletionResponse](#pidgr-v1-CancelDeletionResponse) | Cancel a pending deletion request. Reactivates the user and aborts the deletion workflow. Only valid during the 30-day grace period. Auth: Requires JWT. Admin only. |
| ImmediateDelete | [ImmediateDeleteRequest](#pidgr-v1-ImmediateDeleteRequest) | [ImmediateDeleteResponse](#pidgr-v1-ImmediateDeleteResponse) | Skip the grace period and delete immediately. Signals the deletion workflow to proceed without waiting for the 30-day timer. Auth: Requires JWT. Admin only. |
| ListMyPrivacyRequests | [ListMyPrivacyRequestsRequest](#pidgr-v1-ListMyPrivacyRequestsRequest) | [ListMyPrivacyRequestsResponse](#pidgr-v1-ListMyPrivacyRequestsResponse) | List the calling user&#39;s own privacy requests (export, rectify). The server extracts user_id from the JWT — no admin permission required. Auth: Requires JWT. Any authenticated user. |
| ListOrgSecurityIncidents | [ListOrgSecurityIncidentsRequest](#pidgr-v1-ListOrgSecurityIncidentsRequest) | [ListOrgSecurityIncidentsResponse](#pidgr-v1-ListOrgSecurityIncidentsResponse) | List security incidents that touched the calling organization, newest first. Read-only org-facing subset of the staff incident queue — used by the admin breach feed (GDPR Art. 33/34 transparency). Auth: Requires JWT. Admin only. |

 



<a name="pidgr_v1_audit-proto"></a>
<p align="right"><a href="#top">Top</a></p>

## pidgr/v1/audit.proto



<a name="pidgr-v1-AppendRequest"></a>

### AppendRequest
Request to append a single audit event from an internal service.

Auth: INTERNAL-mTLS ONLY. Unlike the read-side RPCs which authenticate
via Cognito JWT and infer `org_id` from the caller&#39;s claim, this RPC is
invoked by sibling services (e.g. pidgr-integrations) over the internal
mTLS mesh and therefore carries `org_id` in the request payload. The
server MUST reject any caller presenting only a JWT.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| event_type | [string](#string) |  | String form of the event type. Sibling services use a stable string identifier (e.g. &#34;REACHABILITY_UPSERT&#34;, &#34;REACHABILITY_REMOVE&#34;) so a new event type does not require a coordinated proto release across every internal service before it can be recorded. The audit server is responsible for mapping the string into its internal taxonomy. |
| org_id | [string](#string) |  | Organization in which the event occurred. UUID. |
| subject_user_id | [string](#string) | optional | User the audit event is about, if applicable. UUID. Unset when the event is not subject-bound (e.g. an org-wide policy change). |
| actor_id | [string](#string) | optional | Actor who initiated the action, if any. UUID. Unset for system-initiated or sibling-service-initiated events. |
| details | [google.protobuf.Struct](#google-protobuf-Struct) |  | Structured event-specific payload. Used in lieu of the rigid `map&lt;string, string&gt; metadata` on `AuditEvent` so sibling services can record nested objects (e.g. a `prefetch_signals` block) without string-encoding every value. Servers SHOULD redact PII before persist and MUST NOT log this field at INFO or above. Sensitive cryptographic material (plaintext identifiers, envelope ciphertext, raw HMAC keys) MUST NOT be placed here. |






<a name="pidgr-v1-AppendResponse"></a>

### AppendResponse



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| event_id | [string](#string) |  | Server-assigned audit event identifier (UUID). |






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
| synthetic | [bool](#bool) |  | True when this event is synthetic (artificially injected) data — used for demos, sandbox testing, or issue reproduction — rather than the record of a real user action. |
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
| AUDIT_EVENT_TYPE_SCIM_USER_PROVISIONED | 46 | ── SCIM Provisioning ─────────────────────────────────────────────────── A user was provisioned via SCIM. |
| AUDIT_EVENT_TYPE_SCIM_USER_DEPROVISIONED | 47 | A user was deprovisioned via SCIM. |
| AUDIT_EVENT_TYPE_SCIM_USER_UPDATED | 48 | A user was updated via SCIM. |
| AUDIT_EVENT_TYPE_TRANSLATION_CREATED | 49 | ── Translations ──────────────────────────────────────────────────────── A template translation was created. |
| AUDIT_EVENT_TYPE_TRANSLATION_APPROVED | 50 | A template translation was approved. |
| AUDIT_EVENT_TYPE_SANDBOX_CREATED | 51 | ── Sandbox Orgs ──────────────────────────────────────────────────────── A sandbox organization was created. |
| AUDIT_EVENT_TYPE_SANDBOX_EXPIRED | 52 | A sandbox organization expired and was deleted. |
| AUDIT_EVENT_TYPE_AI_PREDICTION_LOGGED | 53 | ── AI/Insights ───────────────────────────────────────────────────────── An AI prediction was served and logged (EU AI Act Art. 12). |
| AUDIT_EVENT_TYPE_ML_PIPELINE_TRIGGERED | 54 | The ML pipeline (archetype clustering &#43; enrichment) was manually triggered. |
| AUDIT_EVENT_TYPE_ARCHETYPE_CLUSTERING_TRIGGERED | 55 | Per-group archetype clustering was manually triggered. |
| AUDIT_EVENT_TYPE_ORG_CREATED | 56 | ── Org lifecycle ─────────────────────────────────────────────────────── An organization was created. |
| AUDIT_EVENT_TYPE_ORG_DELETED | 57 | An organization was deleted (sandbox cleanup or manual deletion). |
| AUDIT_EVENT_TYPE_REACHABILITY_UPSERT | 58 | ── Reachability registry (pidgr-integrations) ────────────────────────── A reachability identifier (email, phone, Slack ID, etc.) was upserted. GDPR-relevant per Chikorita audit classification. |
| AUDIT_EVENT_TYPE_REACHABILITY_REMOVE | 59 | A reachability identifier was removed. GDPR Art. 17 &#34;right to erasure&#34; event; written BEFORE the registry row is deleted per Recital 30. |



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
AuditService provides read access to the append-only audit trail, plus an
internal-mTLS-only Append RPC for sibling services that need to record
their own audit events into the shared trail.

All read RPCs extract org_id from the JWT — it is never in request
messages. The Append RPC carries org_id explicitly because it is
invoked over the internal mTLS mesh, not by a JWT-authenticated user.

| Method Name | Request Type | Response Type | Description |
| ----------- | ------------ | ------------- | ------------|
| ListAuditEvents | [ListAuditEventsRequest](#pidgr-v1-ListAuditEventsRequest) | [ListAuditEventsResponse](#pidgr-v1-ListAuditEventsResponse) | List audit events with optional filtering by event type, actor, and date range. Results are ordered by created_at descending (newest first). Auth: Requires JWT. Admin only. |
| ExportAuditTrail | [ExportAuditTrailRequest](#pidgr-v1-ExportAuditTrailRequest) | [ExportAuditTrailResponse](#pidgr-v1-ExportAuditTrailResponse) | Export the audit trail to S3 in CSV, JSON, or Parquet format. Creates a persistent record and starts an async Temporal workflow. Auth: Requires JWT. Admin only. |
| ListAuditExports | [ListAuditExportsRequest](#pidgr-v1-ListAuditExportsRequest) | [ListAuditExportsResponse](#pidgr-v1-ListAuditExportsResponse) | List audit export history for the organization. Auth: Requires JWT. Admin only. |
| Append | [AppendRequest](#pidgr-v1-AppendRequest) | [AppendResponse](#pidgr-v1-AppendResponse) | Append one audit event to the trail. Used by sibling services (pidgr-integrations, future internal services) to record GDPR-relevant events that originate outside pidgr-api.

Auth: INTERNAL-mTLS ONLY. The server MUST reject any caller that presents only a JWT. The server MUST allowlist callers by their mTLS client-certificate subject DN. |

 



<a name="pidgr_v1_authorization-proto"></a>
<p align="right"><a href="#top">Top</a></p>

## pidgr/v1/authorization.proto



<a name="pidgr-v1-ResolvePrincipalPermissionsRequest"></a>

### ResolvePrincipalPermissionsRequest
Request to resolve the effective permission set for one principal.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| subject | [string](#string) |  | UUID of the subject whose permissions are being resolved (user or principal identifier). |
| org_id | [string](#string) |  | Organization the resolution is scoped to. |
| principal_type | [PrincipalType](#pidgr-v1-PrincipalType) |  | Kind of principal identified by `subject`. |






<a name="pidgr-v1-ResolvePrincipalPermissionsResponse"></a>

### ResolvePrincipalPermissionsResponse
Effective permissions resolved for the requested principal.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| permissions | [Permission](#pidgr-v1-Permission) | repeated | Flattened, deduplicated set of permissions granted to the principal in the requested organization. Empty when the principal has no grants. |





 


<a name="pidgr-v1-PrincipalType"></a>

### PrincipalType
Kind of principal whose permissions are being resolved.

| Name | Number | Description |
| ---- | ------ | ----------- |
| PRINCIPAL_TYPE_UNSPECIFIED | 0 |  |
| PRINCIPAL_TYPE_USER | 1 | An end user identified by their user UUID, scoped to one organization. |
| PRINCIPAL_TYPE_ORG | 2 | An organization acting as its own principal (e.g. a service identity operating on behalf of the whole org rather than a member). |
| PRINCIPAL_TYPE_STAFF | 3 | A platform staff principal whose permissions derive from a role within the ORG_TYPE_STAFF organization. |


 

 


<a name="pidgr-v1-AuthorizationService"></a>

### AuthorizationService
AuthorizationService resolves the effective permission set for a principal
so a resource server can make authorization decisions without owning the
role and permission data itself.

AUTH: INTERNAL service-to-service only. This service is served by the core
API and called by other backend services. It MUST NOT be exposed on the
public ingress to JWT-authenticated end-user clients.

| Method Name | Request Type | Response Type | Description |
| ----------- | ------------ | ------------- | ------------|
| ResolvePrincipalPermissions | [ResolvePrincipalPermissionsRequest](#pidgr-v1-ResolvePrincipalPermissionsRequest) | [ResolvePrincipalPermissionsResponse](#pidgr-v1-ResolvePrincipalPermissionsResponse) | Resolve the effective permissions for one (subject, org, principal type). |

 



<a name="pidgr_v1_campaign-proto"></a>
<p align="right"><a href="#top">Top</a></p>

## pidgr/v1/campaign.proto



<a name="pidgr-v1-ArchetypeShareShift"></a>

### ArchetypeShareShift
Movement in one archetype&#39;s share of the originating group between the
&#34;before&#34; and &#34;after&#34; archetype-clustering snapshots. Cohort-level only;
no joining to user identity. The `is_origin` row is the archetype the
campaign was authored for.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| label | [string](#string) |  | Stable archetype label, e.g. &#34;Swift Acknowledger&#34;. |
| share_before | [double](#double) |  | Archetype&#39;s share of the group at the snapshot closest to (but not after) the campaign&#39;s created_at. Range 0.0 – 1.0. |
| share_after | [double](#double) |  | Archetype&#39;s share of the group at the most recent snapshot. Range 0.0 – 1.0. Equals share_before when no clustering has run since. |
| is_origin | [bool](#bool) |  | True when this row&#39;s label matches the campaign&#39;s originating_archetype.archetype_label. |
| email_delivered_count | [uint64](#uint64) |  | Count of email DELIVERED events recorded for this archetype&#39;s members across the campaign window. Denominator for both open-rate fields. |
| email_open_rate_real | [double](#double) |  | Open rate excluding events flagged as Apple-MPP prefetches (prefetch_suspected=true). Range 0.0 – 1.0. |
| email_open_rate_raw | [double](#double) |  | Open rate including all OPENED events, prefetches included. Range 0.0 – 1.0. |






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
| default_locale | [string](#string) |  | Optional locale override for all recipients in this campaign. When set, all recipients receive the campaign in this locale regardless of their preferred_locale. Empty means per-recipient locale resolution. Valid values: en, es, pt-BR, zh, ja. |
| wait_for_enrollment | [bool](#bool) |  | Whether the campaign deadline waits for users without registered devices. When true, NO_DEVICE users remain in pending_count and can acknowledge via inbox after installing the app. Default false preserves current behavior. |
| originating_archetype | [CampaignOriginatingArchetype](#pidgr-v1-CampaignOriginatingArchetype) |  | Optional. Set when the campaign was created from a Compass archetype CTA. Drives post-campaign archetype-response analytics. |
| synthetic | [bool](#bool) |  | True when this campaign contains synthetic (artificially injected) data — created or populated for demos, sandbox testing, or issue reproduction. |
| audience_snapshot_size | [int32](#int32) |  | Number of recipients frozen in the audience snapshot at creation time. Unlike total_recipients (which counts deliveries and is 0 until the campaign starts), this is known as soon as the campaign exists. 0 when the campaign predates snapshot-size tracking. |
| current_audience_size | [int32](#int32) |  | Number of members currently eligible for this campaign&#39;s audience, computed at read time. Compare with audience_snapshot_size to see how far the frozen audience has drifted from the present membership. |
| audience_snapshot_stale | [bool](#bool) |  | True when the frozen audience no longer covers the current eligible membership (current_audience_size &gt; audience_snapshot_size). Clients should surface this before the campaign is started: recipients added after creation are NOT reached unless the campaign is recreated. |






<a name="pidgr-v1-CampaignOriginatingArchetype"></a>

### CampaignOriginatingArchetype
Identifies the archetype that motivated the creation of a campaign.
The audience is NOT filtered by archetype membership — this is metadata
about the campaign&#39;s authoring intent only. See OpenSpec change
archetype-targeted-campaign-cta.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| group_id | [string](#string) |  | UUID of the group whose archetype set the label belongs to. |
| archetype_label | [string](#string) |  | Stable archetype label (e.g., &#34;Swift Acknowledger&#34;). Labels are stable across clustering retrains; archetype IDs are not. |






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
| workflow | [WorkflowDefinition](#pidgr-v1-WorkflowDefinition) |  | Workflow DAG defining the campaign&#39;s automation steps. Required: CreateCampaign rejects a request with no workflow (INVALID_ARGUMENT) and does not substitute a default. The definition MUST validate as an acyclic graph of well-formed steps. |
| sender_name | [string](#string) |  | Display name of the sender shown to recipients (e.g. &#34;HR Team&#34;). Constraints: Max length 200 characters. |
| title | [string](#string) |  | Optional user-facing title override. If empty, the template title is used. Constraints: Max length 200 characters. |
| audience | [AudienceMember](#pidgr-v1-AudienceMember) | repeated | Rich audience with per-user template variables. When set, takes precedence over user_ids. Constraints: Max 100000 items. |
| include_restricted | [bool](#bool) |  | Whether to include users with processing_restricted=true in the audience. Default false: restricted users are excluded. Set true only with Art. 18(2) legal basis. |
| critical | [bool](#bool) |  | Whether this campaign&#39;s notifications break through Do Not Disturb / Focus mode. |
| default_locale | [string](#string) |  | Optional locale override for all recipients. |
| wait_for_enrollment | [bool](#bool) |  | Whether the campaign deadline should wait for users without registered devices. When true, NO_DEVICE users are not decremented from pending_count, allowing them to acknowledge via inbox after installing the app. |
| originating_archetype | [CampaignOriginatingArchetype](#pidgr-v1-CampaignOriginatingArchetype) |  | Optional. Set when the campaign is created from a Compass archetype CTA. The server validates the caller has access to group_id and that archetype_label exists in the group&#39;s current archetype set; cross-org group_id returns PERMISSION_DENIED, unknown label returns NOT_FOUND. |






<a name="pidgr-v1-CreateCampaignResponse"></a>

### CreateCampaignResponse
Response after creating a campaign.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| campaign | [Campaign](#pidgr-v1-Campaign) |  | The newly created campaign. |






<a name="pidgr-v1-Delivery"></a>

### Delivery



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
| kind | [Delivery.Kind](#pidgr-v1-Delivery-Kind) |  | Discriminator distinguishing primary recipient deliveries from deliveries generated by downstream workflow steps. |
| parent_delivery_id | [string](#string) |  | For non-primary deliveries, the UUID of the originating delivery this row was derived from. Empty for primary deliveries. Constraints: UUID format (36 characters) when set. |
| rendered_locale | [string](#string) |  | The locale this delivery&#39;s body was actually rendered in after fallback resolution (recipient preference, campaign override, template default). Valid values: en, es, pt-BR, zh, ja. |
| metadata | [DeliveryMetadata](#pidgr-v1-DeliveryMetadata) |  | Optional out-of-band context. See `DeliveryMetadata` for which delivery kinds populate which fields. Empty for legacy / PRIMARY deliveries. |
| synthetic | [bool](#bool) |  | True when this delivery&#39;s outcome is synthetic (artificially injected) data rather than the result of a real delivery and user response. |






<a name="pidgr-v1-DeliveryMetadata"></a>

### DeliveryMetadata
A single delivery record tracking message delivery to one recipient.
Out-of-band context attached to a delivery beyond its canonical
recipient &#43; status &#43; content payload. Optional; fields are populated
per delivery kind. Currently only REMINDER_FYI children carry values,
to snapshot context from the parent delivery so clients can render
without fetching additional resources.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| original_message | [Message](#pidgr-v1-Message) |  | REMINDER_FYI: the rendered Message payload from the parent delivery, used to render the blockquoted &#34;Original message&#34; panel on the notify-target&#39;s inbox card. |
| original_recipient_name | [string](#string) |  | REMINDER_FYI: display name of the original recipient (the employee who hasn&#39;t responded). Used to interpolate the FYI title and banner. |
| campaign_title | [string](#string) |  | REMINDER_FYI: campaign title, denormalized so the notify-target&#39;s client can render without a separate campaign lookup. |
| reminder_fired_at | [google.protobuf.Timestamp](#google-protobuf-Timestamp) |  | REMINDER_FYI: when the parent reminder step fired, used to render the &#34;fired X ago&#34; footer on the FYI card. |






<a name="pidgr-v1-GetCampaignArchetypeBreakdownRequest"></a>

### GetCampaignArchetypeBreakdownRequest
Request to compute the archetype-tendency-shift surface for a campaign:
how each archetype&#39;s share of the originating group has moved between
the snapshot closest to campaign-creation time and the most recent
snapshot. Only valid for campaigns whose originating_archetype is set.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| campaign_id | [string](#string) |  | ID of the campaign to break down. Constraints: UUID format (36 characters). |






<a name="pidgr-v1-GetCampaignArchetypeBreakdownResponse"></a>

### GetCampaignArchetypeBreakdownResponse
Response containing per-archetype share shifts. The admin renders
these as a comparison table — origin row marked, others as peers, so
the admin can tell campaign-coincident drift apart from background
drift across the rest of the group.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| shifts | [ArchetypeShareShift](#pidgr-v1-ArchetypeShareShift) | repeated | One entry per archetype in the originating group. Empty when insufficient_history is true. |
| before_snapshot_at | [google.protobuf.Timestamp](#google-protobuf-Timestamp) |  | When the &#34;before&#34; sample was taken (closest snapshot at or before campaign creation). |
| after_snapshot_at | [google.protobuf.Timestamp](#google-protobuf-Timestamp) |  | When the &#34;after&#34; sample was taken (most recent snapshot). |
| insufficient_history | [bool](#bool) |  | True when fewer than two clustering snapshots exist for the group, so no shift can be computed yet. Admin renders an &#34;awaiting next clustering cycle&#34; empty state. |






<a name="pidgr-v1-GetCampaignByShortCodeRequest"></a>

### GetCampaignByShortCodeRequest
Request to look up a campaign by its public short-code. Called by the
native app when the recipient taps a third-party-channel deeplink and
the URL handler needs to route to the right campaign card. Designed to
be safe to call without authentication — the response carries no PII
and only enough context for the app to route correctly and show org
branding before the auth gate.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| short_code | [string](#string) |  | The 8-character short-code from the deeplink path. Constraints: Required, exactly 8 base62 characters. |






<a name="pidgr-v1-GetCampaignByShortCodeResponse"></a>

### GetCampaignByShortCodeResponse
Response carrying the minimum metadata the native app needs to route
the deeplink. Subject is the campaign&#39;s title text (already visible
in the recipient&#39;s inbox after dispatch — no new PII exposure). Body
content, audience size, delivery status and any other operational
fields are NOT included; the app fetches those via authenticated
`GetCampaign` after the recipient signs in.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| campaign_id | [string](#string) |  | Campaign UUID — the app uses this for the authenticated `GetCampaign` follow-up after the deeplink token validates. |
| org_id | [string](#string) |  | Organization UUID owning the campaign — lets the app pick the correct SSO / sign-in flow when the recipient is logged out. |
| organization_name | [string](#string) |  | Display name of the organization for sign-in branding (&#34;Sign in to Acme Inc to view this campaign&#34;). Public information; the organization&#39;s profile already exposes it elsewhere. |
| subject | [string](#string) |  | Campaign subject (title). Same string the recipient already saw in their inbox; included so the deeplink interstitial can show &#34;Acme Inc — All-hands Q3&#34; before the auth gate. |






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






<a name="pidgr-v1-ResolveOrCreateShortCodeRequest"></a>

### ResolveOrCreateShortCodeRequest
Request to resolve a campaign&#39;s short-code, lazily generating one on
first call. Used by internal-service callers (the dispatch layer)
when assembling a third-party-channel deeplink:
`links.pidgr.com/c/{short_code}?t={token}`.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| campaign_id | [string](#string) |  | The campaign whose short-code is being resolved. Constraints: Required, must be a UUID and exist within the caller&#39;s organization. |






<a name="pidgr-v1-ResolveOrCreateShortCodeResponse"></a>

### ResolveOrCreateShortCodeResponse
Response carrying the resolved short-code. The same campaign always
resolves to the same code for its lifetime; the value is safe to
cache by the caller.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| short_code | [string](#string) |  | 8-character base62 short-code stable for the campaign&#39;s lifetime. |






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





 


<a name="pidgr-v1-Delivery-Kind"></a>

### Delivery.Kind
Discriminator describing what produced this delivery row.

| Name | Number | Description |
| ---- | ------ | ----------- |
| KIND_UNSPECIFIED | 0 | Default value; not a valid kind. |
| KIND_PRIMARY | 1 | Delivery generated for an audience recipient at campaign start. |
| KIND_ESCALATION | 2 | Delivery generated by an escalation step targeting a non-audience user. |
| KIND_REMINDER_FYI | 3 | Passive heads-up delivery generated when a reminder step fans out to its `notify_targets`. Carries no action button; auto-dismisses when the parent delivery is acknowledged. See `SendReminderConfig.notify_targets`. |


 

 


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
| GetCampaignArchetypeBreakdown | [GetCampaignArchetypeBreakdownRequest](#pidgr-v1-GetCampaignArchetypeBreakdownRequest) | [GetCampaignArchetypeBreakdownResponse](#pidgr-v1-GetCampaignArchetypeBreakdownResponse) | Break down a campaign&#39;s recipients by current archetype membership and return ack-rate per bucket, with k-anonymity gate applied. Only valid for campaigns whose originating_archetype is set. Authorization: Authenticated user within the organization. |
| ResolveOrCreateShortCode | [ResolveOrCreateShortCodeRequest](#pidgr-v1-ResolveOrCreateShortCodeRequest) | [ResolveOrCreateShortCodeResponse](#pidgr-v1-ResolveOrCreateShortCodeResponse) | Resolve a campaign&#39;s 8-character base62 short-code, lazily generating one on the first call. Stable for the campaign&#39;s lifetime; the same campaign always returns the same code. Authorization: Internal-service callers only (mTLS-gated). The dispatch layer calls this when assembling a third-party-channel deeplink. |
| GetCampaignByShortCode | [GetCampaignByShortCodeRequest](#pidgr-v1-GetCampaignByShortCodeRequest) | [GetCampaignByShortCodeResponse](#pidgr-v1-GetCampaignByShortCodeResponse) | Look up a campaign by its short-code. Returns minimal, non-PII metadata sufficient for the native app to route a tapped third-party-channel deeplink to the correct campaign card and show org branding before the auth gate. Authorization: Unauthenticated. The short-code IS the lookup key; returned fields are already visible to the recipient via their inbox or the org&#39;s public profile. |

 



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
| data_governance_region | [string](#string) |  | Data governance region override. Empty string means &#34;inherit from org default&#34;. Valid values: EU, LATAM, BR, APAC, US. |






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
| manager_id | [string](#string) |  | UUID of the user&#39;s direct manager within the same organization. Populated from SCIM enterprise extension (manager.value), manual admin assignment, or SSO attribute mapping. Empty if not set. |






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
| kind | [Delivery.Kind](#pidgr-v1-Delivery-Kind) |  | Discriminator: PRIMARY for normal deliveries, ESCALATION for delivery-grade escalations. Mirrors Delivery.kind so inbox-sync clients can branch on the same dimension as listDeliveries clients. |
| parent_delivery_id | [string](#string) |  | For ESCALATION entries, the UUID of the unacked delivery that triggered this entry. Empty for PRIMARY entries. |
| rendered_locale | [string](#string) |  | The locale the body actually rendered in after fallback resolution. Empty for legacy/PRIMARY entries. |
| metadata | [DeliveryMetadata](#pidgr-v1-DeliveryMetadata) |  | Optional out-of-band context mirrored from the underlying delivery. See `DeliveryMetadata` for which delivery kinds populate which fields. Empty for PRIMARY entries. |






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

 



<a name="pidgr_v1_insights-proto"></a>
<p align="right"><a href="#top">Top</a></p>

## pidgr/v1/insights.proto



<a name="pidgr-v1-Archetype"></a>

### Archetype
A behavioral archetype describing a cohort pattern (never an individual).
Derived from k-anonymized, DP-noised behavioral feature vectors.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| label | [string](#string) |  | Human-readable label (e.g., &#34;Swift Acknowledger&#34;, &#34;Thorough Reader&#34;). |
| description | [string](#string) |  | Description of the behavioral pattern this archetype represents. |
| percentage | [float](#float) |  | Proportion of the group that belongs to this archetype (0.0-1.0). |
| feature_centroid | [Archetype.FeatureCentroidEntry](#pidgr-v1-Archetype-FeatureCentroidEntry) | repeated | Centroid of the behavioral feature vector for this archetype. Keys are stable dimension names from the feature extractor vocabulary (e.g., &#34;tap_density&#34;, &#34;engagement_depth&#34;, &#34;scroll_velocity_p50&#34;, &#34;idle_gap_p75&#34;). Single-letter keys are reserved for backward compatibility with pre-v0.64 servers and SHALL be ignored by clients. |
| feature_breakdown | [Archetype.FeatureBreakdownEntry](#pidgr-v1-Archetype-FeatureBreakdownEntry) | repeated | Per-dimension distribution of the archetype&#39;s members. Lets the admin render percentile bands instead of single-point centroids. Absent until at least k members exist in the cluster. Keys mirror `feature_centroid` keys. |
| tap_heatmap | [TapHeatmap](#pidgr-v1-TapHeatmap) | optional | Tap density heatmap aggregated across sessions for this archetype. Cohort-level only — never per-session timing. Absent when fewer than k sessions have tap data. |
| forecast | [ArchetypeForecast](#pidgr-v1-ArchetypeForecast) | optional | Forecast of cluster share at fixed horizons (7/14/30/90 days). Absent during cold start before historical clustering runs exist to extrapolate from. |
| exemplar_sessions | [ExemplarSession](#pidgr-v1-ExemplarSession) | repeated | Sessions that sit at the median and quartiles of the archetype&#39;s centroid distance, ranked by distance. Bounded at three entries. Absent until at least 50 sessions have been scored. Sessions can come from any client that emits to ReplayService — mobile (iOS, Android) or desktop (macOS, Windows, Linux). |
| screen_dwell | [ScreenDwell](#pidgr-v1-ScreenDwell) | optional | Per-screen dwell time distribution, derived from session replay. Absent when fewer than k sessions per screen exist. |
| response_timeline | [ResponseTimeline](#pidgr-v1-ResponseTimeline) | optional | End-to-end response latencies (push delivered → read → ack) for members of this archetype, as percentiles. Absent until at least k campaign deliveries have been recorded for this archetype. |
| source | [ArchetypeSource](#pidgr-v1-ArchetypeSource) |  | Where this archetype came from. UNSPECIFIED on responses from pre-v0.81 servers; clients SHOULD treat UNSPECIFIED as ML for backward compatibility (provisional output is always labelled). |






<a name="pidgr-v1-Archetype-FeatureBreakdownEntry"></a>

### Archetype.FeatureBreakdownEntry



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| key | [string](#string) |  |  |
| value | [DimensionStats](#pidgr-v1-DimensionStats) |  |  |






<a name="pidgr-v1-Archetype-FeatureCentroidEntry"></a>

### Archetype.FeatureCentroidEntry



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| key | [string](#string) |  |  |
| value | [double](#double) |  |  |






<a name="pidgr-v1-ArchetypeForecast"></a>

### ArchetypeForecast
Predicted cluster share at fixed horizons with confidence bands.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| horizons | [ForecastHorizon](#pidgr-v1-ForecastHorizon) | repeated | Horizons in increasing days. Always one entry each for 7, 14, 30, and 90 days when the field is present. |






<a name="pidgr-v1-CampaignAdvisory"></a>

### CampaignAdvisory
Advisory information for campaign configuration, combining predictions and archetypes.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| predicted_ack | [CohortPrediction](#pidgr-v1-CohortPrediction) |  | Cohort-level ACK prediction for the target audience. |
| suggested_escalation_delay_minutes | [int32](#int32) |  | Suggested escalation delay in minutes based on historical cohort patterns. 0 if insufficient data. |
| archetypes | [Archetype](#pidgr-v1-Archetype) | repeated | Behavioral archetypes for the target audience. |






<a name="pidgr-v1-CohortPrediction"></a>

### CohortPrediction
A cohort-level prediction for campaign acknowledgment rate.
Never targets or scores individuals — always represents an audience aggregate.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| predicted_ack_rate | [float](#float) |  | Predicted ACK rate for the audience (0.0-1.0). |
| confidence_low | [float](#float) |  | Lower bound of the confidence interval. |
| confidence_high | [float](#float) |  | Upper bound of the confidence interval. |
| confidence_level | [ConfidenceLevel](#pidgr-v1-ConfidenceLevel) |  | Confidence level based on available data volume. |
| data_point_count | [int32](#int32) |  | Number of anonymous data points used for this prediction. |






<a name="pidgr-v1-DimensionStats"></a>

### DimensionStats
Per-dimension distribution stats for one feature dimension within
an archetype&#39;s cohort. All values are in the same units as
`Archetype.feature_centroid`. Used to render percentile bands on
the admin&#39;s behavioral profile panel.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| centroid | [double](#double) |  | Centroid value (same as Archetype.feature_centroid[key]). |
| p25 | [double](#double) |  | 25th percentile across the archetype&#39;s members. |
| p50 | [double](#double) |  | Median across the archetype&#39;s members. |
| p75 | [double](#double) |  | 75th percentile across the archetype&#39;s members. |
| group_p50 | [double](#double) |  | Median across the entire group (all archetypes), included so the admin can render &#34;this archetype is X% above group median&#34;. |






<a name="pidgr-v1-ExemplarSession"></a>

### ExemplarSession
Pointer to a representative session for one archetype, ranked by
distance to the archetype centroid.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| session_id | [string](#string) |  | Session recording ID retrievable via ReplayService for the same org. Linkable from the admin regardless of originating platform. |
| rank | [int32](#int32) |  | Quantile rank within the archetype: 25, 50, or 75. The writer emits at most one session per rank. |
| distance | [double](#double) |  | L2 distance from the session&#39;s feature vector to the centroid. |
| duration_seconds | [int32](#int32) |  | Optional duration metadata for quick admin labelling. |
| platform | [string](#string) |  | Optional platform identifier from the vocabulary {&#34;ios&#34;, &#34;android&#34;, &#34;macos&#34;, &#34;windows&#34;, &#34;linux&#34;}. The admin renders unknown values verbatim for forward compatibility. |






<a name="pidgr-v1-ForecastHorizon"></a>

### ForecastHorizon
Predicted share at one horizon with a 90% prediction interval.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| days | [int32](#int32) |  | Horizon length in days (one of: 7, 14, 30, 90). |
| predicted_share | [double](#double) |  | Predicted fraction of the group falling in this archetype at the horizon (0.0-1.0). |
| lower | [double](#double) |  | 5th-percentile lower bound of the prediction interval. |
| upper | [double](#double) |  | 95th-percentile upper bound of the prediction interval. |
| confidence | [ConfidenceLevel](#pidgr-v1-ConfidenceLevel) |  | Confidence in this horizon&#39;s prediction. |






<a name="pidgr-v1-GenerateCampaignBodyDraftRequest"></a>

### GenerateCampaignBodyDraftRequest
Request to draft a campaign body for a given archetype using Bedrock.
Used by the Compass &#34;Target this archetype in a new campaign&#34; CTA to
pre-fill the campaign creation wizard&#39;s body field.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| group_id | [string](#string) |  | UUID of the source group whose archetype set the label belongs to. |
| archetype_label | [string](#string) |  | Stable archetype label, e.g. &#34;Swift Acknowledger&#34;. |
| lane_action | [string](#string) |  | Lane-recommended action copy passed through from the admin (e.g. &#34;Simplify the call-to-action&#34;). Used as a tone hint for the prompt. |






<a name="pidgr-v1-GenerateCampaignBodyDraftResponse"></a>

### GenerateCampaignBodyDraftResponse
Response containing the generated draft body in Markdown.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| body_markdown | [string](#string) |  | Draft Markdown body, 3-5 sentences. Authored as if written for the recipient — does not mention the archetype name. |






<a name="pidgr-v1-GetCampaignAdvisoryRequest"></a>

### GetCampaignAdvisoryRequest
Request for campaign configuration advisory.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| group_id | [string](#string) |  | ID of the target audience group. Required. |
| template_id | [string](#string) |  | Template ID (optional, for advisory context). |
| template_version | [int32](#int32) |  | Template version (optional). |
| workflow_step_count | [int32](#int32) |  | Number of workflow steps (optional). |






<a name="pidgr-v1-GetCampaignAdvisoryResponse"></a>

### GetCampaignAdvisoryResponse
Response containing campaign advisory information.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| advisory | [CampaignAdvisory](#pidgr-v1-CampaignAdvisory) |  | Campaign advisory with prediction, suggested escalation, and archetypes. |






<a name="pidgr-v1-GetGroupArchetypesRequest"></a>

### GetGroupArchetypesRequest
Request to retrieve behavioral archetypes for a group.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| group_id | [string](#string) |  | ID of the group to query archetypes for. Required. |






<a name="pidgr-v1-GetGroupArchetypesResponse"></a>

### GetGroupArchetypesResponse
Response containing behavioral archetypes for a group.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| archetypes | [Archetype](#pidgr-v1-Archetype) | repeated | Behavioral archetypes for the group (empty if insufficient data). |
| data_point_count | [int32](#int32) |  | Number of anonymous feature vectors used for clustering. |
| pipeline_state | [PipelineState](#pidgr-v1-PipelineState) |  | Why `archetypes` looks the way it does. Lets the UI render a distinct empty-state affordance for &#34;never trained&#34; vs &#34;below threshold&#34; vs &#34;no clusters&#34; vs &#34;ready&#34;. See PipelineState. |
| confidence_level | [ConfidenceLevel](#pidgr-v1-ConfidenceLevel) |  | Confidence in the returned archetypes, derived from available data volume. Always CONFIDENCE_LEVEL_LOW when provisional archetypes are returned — clients use this plus `Archetype.source` to render the low-confidence disclaimer. |






<a name="pidgr-v1-GetInsightNarrativeRequest"></a>

### GetInsightNarrativeRequest
Request to generate an AI narrative for a group&#39;s insights.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| group_id | [string](#string) |  | ID of the group to generate a narrative for. Required. |
| prompt_name | [string](#string) |  | Name of the prompt template to use (e.g., &#34;campaign-advisory&#34;, &#34;archetype-explanation&#34;). |






<a name="pidgr-v1-GetInsightNarrativeResponse"></a>

### GetInsightNarrativeResponse
Response containing an AI-generated narrative.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| narrative | [string](#string) |  | AI-generated narrative text (Markdown formatted). |
| generated_at | [google.protobuf.Timestamp](#google-protobuf-Timestamp) |  | Timestamp when the narrative was generated. |
| model_id | [string](#string) |  | Model identifier used for generation. |






<a name="pidgr-v1-LatencyPercentiles"></a>

### LatencyPercentiles
Latency distribution stats. Values are in seconds.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| p50 | [double](#double) |  |  |
| p75 | [double](#double) |  |  |
| p95 | [double](#double) |  |  |






<a name="pidgr-v1-PredictCampaignACKRequest"></a>

### PredictCampaignACKRequest
Request to predict cohort-level ACK rate for a campaign configuration.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| group_id | [string](#string) |  | ID of the target audience group. Required. |
| template_type | [string](#string) |  | Template type (optional, for prediction refinement). |
| workflow_step_count | [int32](#int32) |  | Number of workflow steps (optional, for prediction refinement). |






<a name="pidgr-v1-PredictCampaignACKResponse"></a>

### PredictCampaignACKResponse
Response containing a cohort-level ACK prediction.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| prediction | [CohortPrediction](#pidgr-v1-CohortPrediction) |  | Cohort-level prediction. |






<a name="pidgr-v1-ResponseTimeline"></a>

### ResponseTimeline
End-to-end response latencies for members of one archetype, in
seconds. Each percentile is computed across all qualifying campaign
deliveries for the archetype&#39;s members within the rolling window.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| read_after_delivered | [LatencyPercentiles](#pidgr-v1-LatencyPercentiles) |  | Time from `delivered_at` to `read_at`, in seconds. |
| ack_after_read | [LatencyPercentiles](#pidgr-v1-LatencyPercentiles) |  | Time from `read_at` to `acknowledged_at`, in seconds. Only includes deliveries that were both read and acknowledged. |
| ack_after_delivered | [LatencyPercentiles](#pidgr-v1-LatencyPercentiles) |  | End-to-end time from `delivered_at` to `acknowledged_at`, in seconds. Only includes deliveries that were acknowledged. |
| delivery_count | [int32](#int32) |  | Number of deliveries the timeline is computed over. |






<a name="pidgr-v1-ScreenDwell"></a>

### ScreenDwell
Per-screen dwell distribution within an archetype. Lets the admin
surface &#34;this archetype lingers 8.2s on the Message Detail screen
vs 0.4s on the Inbox list&#34;.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| entries | [ScreenDwellEntry](#pidgr-v1-ScreenDwellEntry) | repeated | One entry per screen. Screens with fewer than k members in the archetype are dropped from the list (not marked as absent). |






<a name="pidgr-v1-ScreenDwellEntry"></a>

### ScreenDwellEntry



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| screen_name | [string](#string) |  | Stable screen identifier (e.g., &#34;MessageDetail&#34;, &#34;Inbox&#34;, &#34;ProfileSettings&#34;). Sourced from the same screen_name vocabulary used by heatmap_cells. |
| median_seconds | [double](#double) |  | Median dwell time in seconds for this archetype on this screen. |
| p75_seconds | [double](#double) |  | 75th-percentile dwell time in seconds. |
| session_count | [int32](#int32) |  | Number of distinct sessions aggregated for this screen. |






<a name="pidgr-v1-TapHeatmap"></a>

### TapHeatmap
A density grid of tap activity for one archetype, normalized to
[0.0, 1.0] where 1.0 is the hottest cell in the cohort. Cohort-
level only.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| width | [int32](#int32) |  | Width of the density grid in cells. |
| height | [int32](#int32) |  | Height of the density grid in cells. |
| values | [double](#double) | repeated | Row-major density values, length must equal width*height. All in [0.0, 1.0]. |
| session_count | [int32](#int32) |  | Number of sessions aggregated. Always &gt;= MinFeatureVectorsForClustering when the field is present. |
| layers | [TapHeatmapLayer](#pidgr-v1-TapHeatmapLayer) | repeated | Optional per-event-type breakdown. When present, the writer SHALL emit one entry for each event type in the source data (TAP, LONG_PRESS, SCROLL, ACTION_CLICK). |






<a name="pidgr-v1-TapHeatmapLayer"></a>

### TapHeatmapLayer
One per-event-type layer of a TapHeatmap.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| event_type | [string](#string) |  | Event type this layer represents (e.g., &#34;TAP&#34;, &#34;LONG_PRESS&#34;, &#34;SCROLL&#34;, &#34;ACTION_CLICK&#34;). |
| values | [double](#double) | repeated | Row-major density values, same dimensions as the parent TapHeatmap. Independently normalized to [0.0, 1.0]. |






<a name="pidgr-v1-TriggerArchetypeClusteringRequest"></a>

### TriggerArchetypeClusteringRequest
Request to manually retrigger archetype clustering for a single group
without rerunning the full SageMaker training pipeline. Reuses the
already-deployed clustering model.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| group_id | [string](#string) |  | Group to recluster. Org is extracted from the JWT. |






<a name="pidgr-v1-TriggerArchetypeClusteringResponse"></a>

### TriggerArchetypeClusteringResponse
Response after triggering archetype clustering for one group.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| workflow_id | [string](#string) |  | Temporal workflow id — useful for client-side dedupe &#43; operator debugging via the Temporal UI. |
| remaining_this_month | [int32](#int32) |  | Remaining manual retrains allowed this month. Shares the same monthly counter as TriggerMLPipeline (ml_manual_limit_monthly). |
| last_clustered_at | [google.protobuf.Timestamp](#google-protobuf-Timestamp) |  | Timestamp of the last successful archetype clustering for this (org, group), null if never clustered. |






<a name="pidgr-v1-TriggerMLPipelineRequest"></a>

### TriggerMLPipelineRequest
Request to manually trigger the ML training pipeline.
Empty — organization is extracted from the JWT.






<a name="pidgr-v1-TriggerMLPipelineResponse"></a>

### TriggerMLPipelineResponse
Response after triggering the ML pipeline.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| remaining_this_month | [int32](#int32) |  | Remaining manual retrains allowed this month. |
| last_trained_at | [google.protobuf.Timestamp](#google-protobuf-Timestamp) |  | Timestamp of the last successful training (null if never trained). |





 


<a name="pidgr-v1-ArchetypeSource"></a>

### ArchetypeSource
Where an archetype came from. Lets clients distinguish trained ML
clustering output from low-confidence provisional output generated
for sandboxes and opted-in organizations before enough engagement
data exists. Clients MUST render a low-confidence disclaimer for
PROVISIONAL archetypes.

| Name | Number | Description |
| ---- | ------ | ----------- |
| ARCHETYPE_SOURCE_UNSPECIFIED | 0 |  |
| ARCHETYPE_SOURCE_ML | 1 | Produced by the trained ML clustering pipeline (k-anonymized, DP-noised behavioral feature vectors). |
| ARCHETYPE_SOURCE_PROVISIONAL | 2 | Rule-based provisional output derived from coarse delivery/read/ ack activity (or a stable starter distribution for sandboxes with no activity). Low confidence, never written to the ML artifact path, and always superseded by ML output once available. |



<a name="pidgr-v1-ConfidenceLevel"></a>

### ConfidenceLevel
Confidence level for cohort-level predictions, based on available data volume.

| Name | Number | Description |
| ---- | ------ | ----------- |
| CONFIDENCE_LEVEL_UNSPECIFIED | 0 |  |
| CONFIDENCE_LEVEL_LOW | 1 | Fewer than 50 campaigns — predictions based on heuristics/industry benchmarks. |
| CONFIDENCE_LEVEL_MEDIUM | 2 | 50-200 campaigns — basic clustering available, wide confidence intervals. |
| CONFIDENCE_LEVEL_HIGH | 3 | 200&#43; campaigns — full ML pipeline, narrow confidence intervals. |



<a name="pidgr-v1-PipelineState"></a>

### PipelineState
Pipeline state for a group&#39;s archetypes. Lets the admin UI render
distinct empty-state affordances (&#34;run clustering&#34; vs &#34;need N more
sessions&#34; vs &#34;pipeline ran but audience was too homogeneous&#34;) instead
of treating every empty archetype list the same. Populated by
InsightsService.GetGroupArchetypes.

| Name | Number | Description |
| ---- | ------ | ----------- |
| PIPELINE_STATE_UNSPECIFIED | 0 |  |
| PIPELINE_STATE_NEVER_RUN | 1 | The ML pipeline has never fired for this org. Archetypes are empty because nothing ran, not because of data shape. |
| PIPELINE_STATE_BELOW_THRESHOLD | 2 | The pipeline ran but the group had fewer than the k-anonymization minimum feature vectors (50), so clustering was skipped. UI renders &#34;keep running campaigns&#34; affordance. |
| PIPELINE_STATE_NO_CLUSTERS | 3 | The pipeline ran with enough vectors but the clustering provider returned zero clusters — typically means the audience is too homogeneous to separate into distinct archetypes. |
| PIPELINE_STATE_READY | 4 | Archetypes are populated and ready to render. |


 

 


<a name="pidgr-v1-InsightsService"></a>

### InsightsService
Provides cohort-level AI insights for campaign planning and optimization.
All predictions are aggregate — no individual-level profiling.
Data is derived from k-anonymized, differential-privacy-noised behavioral features.
All RPCs operate within the caller&#39;s org (extracted from JWT).

| Method Name | Request Type | Response Type | Description |
| ----------- | ------------ | ------------- | ------------|
| GetGroupArchetypes | [GetGroupArchetypesRequest](#pidgr-v1-GetGroupArchetypesRequest) | [GetGroupArchetypesResponse](#pidgr-v1-GetGroupArchetypesResponse) | Retrieve behavioral archetypes for a group based on anonymous feature vectors. Returns empty archetypes if insufficient data (cold start). Authorization: Requires PERMISSION_CAMPAIGNS_READ. |
| PredictCampaignACK | [PredictCampaignACKRequest](#pidgr-v1-PredictCampaignACKRequest) | [PredictCampaignACKResponse](#pidgr-v1-PredictCampaignACKResponse) | Predict cohort-level ACK rate for a campaign targeting a specific group. Returns a confidence interval that narrows as more campaign data accumulates. Authorization: Requires PERMISSION_CAMPAIGNS_READ. |
| GetCampaignAdvisory | [GetCampaignAdvisoryRequest](#pidgr-v1-GetCampaignAdvisoryRequest) | [GetCampaignAdvisoryResponse](#pidgr-v1-GetCampaignAdvisoryResponse) | Get campaign configuration advisory (prediction &#43; suggested escalation &#43; archetypes). Advisory is informational only — never drives automated decisions. Authorization: Requires PERMISSION_CAMPAIGNS_READ. |
| GetInsightNarrative | [GetInsightNarrativeRequest](#pidgr-v1-GetInsightNarrativeRequest) | [GetInsightNarrativeResponse](#pidgr-v1-GetInsightNarrativeResponse) | Generate an AI-powered narrative summary of a group&#39;s insights. Combines archetype, prediction, and campaign data into human-readable analysis. Authorization: Requires PERMISSION_CAMPAIGNS_READ. |
| TriggerMLPipeline | [TriggerMLPipelineRequest](#pidgr-v1-TriggerMLPipelineRequest) | [TriggerMLPipelineResponse](#pidgr-v1-TriggerMLPipelineResponse) | Manually trigger the ML training pipeline for the caller&#39;s organization. Rate-limited by ml_manual_limit_monthly (default 3 per month, auto-resets). Authorization: Requires PERMISSION_ORGANIZATION_WRITE. |
| TriggerArchetypeClustering | [TriggerArchetypeClusteringRequest](#pidgr-v1-TriggerArchetypeClusteringRequest) | [TriggerArchetypeClusteringResponse](#pidgr-v1-TriggerArchetypeClusteringResponse) | Manually retrigger archetype clustering for a single group, reusing the already-deployed SageMaker clustering model. Cheaper than a full TriggerMLPipeline run because no training happens. Shares the same ml_manual_limit_monthly quota as TriggerMLPipeline — callers get N manual retrains per month across both RPCs. Authorization: Requires PERMISSION_ORGANIZATION_WRITE. |
| GenerateCampaignBodyDraft | [GenerateCampaignBodyDraftRequest](#pidgr-v1-GenerateCampaignBodyDraftRequest) | [GenerateCampaignBodyDraftResponse](#pidgr-v1-GenerateCampaignBodyDraftResponse) | Draft a campaign body for the given archetype using Bedrock with the campaign-for-archetype prompt template. Used by the Compass &#34;Target this archetype&#34; CTA to pre-fill the wizard&#39;s body field. Cross-org group_id returns PERMISSION_DENIED, unknown archetype_label returns NOT_FOUND. Authorization: Requires PERMISSION_CAMPAIGNS_WRITE. |

 



<a name="pidgr_v1_integrations-proto"></a>
<p align="right"><a href="#top">Top</a></p>

## pidgr/v1/integrations.proto



<a name="pidgr-v1-Reachability"></a>

### Reachability
A single reachability registry row, returned by `GetReachability` and
`ListReachabilityForUser`. The plaintext identifier and envelope ciphertext
are NEVER returned over the wire — only metadata. The dispatch worker reads
the plaintext directly from the database and decrypts via KMS.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| id | [string](#string) |  | Server-assigned row identifier (UUID). |
| org_id | [string](#string) |  | Organization that owns this reachability entry. |
| user_id | [string](#string) |  | User this reachability entry is for. |
| channel | [ChannelName](#pidgr-v1-ChannelName) |  | Channel for which this entry stores a contact identifier. |
| created_at | [google.protobuf.Timestamp](#google-protobuf-Timestamp) |  | When the row was first written. |
| updated_at | [google.protobuf.Timestamp](#google-protobuf-Timestamp) |  | When the row was last upserted. |
| region_constraint | [string](#string) | optional | Optional AWS region identifier (e.g. &#34;eu-west-1&#34;) this user&#39;s data must remain in for GDPR/residency reasons. Unset means &#34;no constraint.&#34; Enforcement happens at dispatch time, not write time. |






<a name="pidgr-v1-RegionPolicy"></a>

### RegionPolicy
Per-(org, channel) region allowlist used by the dispatch worker to enforce
data-residency policy. An empty `allowed_regions` list means &#34;no policy
configured&#34; — NOT &#34;no regions allowed.&#34;


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| org_id | [string](#string) |  |  |
| channel | [ChannelName](#pidgr-v1-ChannelName) |  |  |
| allowed_regions | [string](#string) | repeated | AWS region identifiers (e.g. &#34;eu-west-1&#34;, &#34;us-east-1&#34;). Empty list == &#34;no policy configured&#34; — the dispatch worker SHALL NOT block on empty. |
| updated_at | [google.protobuf.Timestamp](#google-protobuf-Timestamp) |  |  |





 


<a name="pidgr-v1-DispatchStatus"></a>

### DispatchStatus
Terminal status of a single dispatch attempt as returned by the worker-mode
`DispatchToChannel` RPC. Distinct from the richer `ChannelEventStatus` in
`channel_events.proto`, which models the audit-trail row for every state
transition (SENT → DELIVERED → OPENED → …). DispatchStatus is the immediate
outcome of one worker call.

| Name | Number | Description |
| ---- | ------ | ----------- |
| DISPATCH_STATUS_UNSPECIFIED | 0 | Default value; should not be used explicitly. |
| DISPATCH_STATUS_SENT | 1 | The adapter accepted the message for delivery (provider returned success). |
| DISPATCH_STATUS_FAILED | 2 | The adapter returned a terminal error (e.g. recipient blocked, domain not verified). Retries SHALL NOT be attempted; consult `failure_reason`. |
| DISPATCH_STATUS_DEDUPED | 3 | An existing `(dispatch_id, SENT)` row was found by the idempotency guard before the adapter was called; the prior receipt was returned without a second provider call. |


 

 

 



<a name="pidgr_v1_integrations_service-proto"></a>
<p align="right"><a href="#top">Top</a></p>

## pidgr/v1/integrations_service.proto



<a name="pidgr-v1-CreateChannelConnectLinkRequest"></a>

### CreateChannelConnectLinkRequest
Mints a short-lived, HMAC-signed opt-in link a user follows to bind a
third-party channel to their (org, user). Only follow-style channels are
accepted: CHANNEL_NAME_TELEGRAM (bot-follow), CHANNEL_NAME_SLACK (OAuth),
CHANNEL_NAME_LINE (follow-code). Any other channel is rejected server-side
with `invalid_argument`. Wraps the pidgr-api `internal/linktoken` minter.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| org_id | [string](#string) |  |  |
| user_id | [string](#string) |  | Internal user UUID; resolved via UserResolver on the server. The minted token binds the resulting channel identifier to this (org, user). |
| channel | [ChannelName](#pidgr-v1-ChannelName) |  | Channel to connect. Constraints: must be one of CHANNEL_NAME_TELEGRAM, CHANNEL_NAME_SLACK, CHANNEL_NAME_LINE. Other values return `invalid_argument`. |






<a name="pidgr-v1-CreateChannelConnectLinkResponse"></a>

### CreateChannelConnectLinkResponse



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| connect_url | [string](#string) |  | The deep link the client renders for the user to follow (e.g. a Telegram bot-follow URL, Slack OAuth authorize URL, or LINE follow URL). |
| token | [string](#string) |  | The raw 64-char base64url opt-in token embedded in `connect_url`, surfaced separately so clients can render it as a QR code or copy button. Implementation detail — clients SHOULD NOT parse or mutate it. |
| expires_at | [google.protobuf.Timestamp](#google-protobuf-Timestamp) |  | When the minted token expires. After this time the link no longer binds and the user must request a fresh one. |






<a name="pidgr-v1-DispatchToChannelRequest"></a>

### DispatchToChannelRequest
Worker-mode entry point invoked by the Temporal worker for one recipient.
Idempotent on `dispatch_id`: if a `(dispatch_id, SENT)` row already exists
in `channel_dispatches`, the worker SHALL return DISPATCH_STATUS_DEDUPED
without re-invoking the channel adapter.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| dispatch_id | [string](#string) |  | Idempotency key. Must be stable across retries from pidgr-api side. |
| org_id | [string](#string) |  |  |
| user_id | [string](#string) |  |  |
| channel | [ChannelName](#pidgr-v1-ChannelName) |  | Which channel adapter to invoke (EMAIL is the Wave 1 implementation). |
| template_id | [string](#string) |  | Template to render before dispatch. |
| template_vars | [DispatchToChannelRequest.TemplateVarsEntry](#pidgr-v1-DispatchToChannelRequest-TemplateVarsEntry) | repeated | Per-recipient template variables. |
| locale | [string](#string) |  | BCP-47 locale used to select the template translation. |
| region_constraint | [string](#string) | optional | Optional AWS region the worker MUST dispatch from (typically copied from the recipient&#39;s reachability row). Unset means &#34;no constraint.&#34; |






<a name="pidgr-v1-DispatchToChannelRequest-TemplateVarsEntry"></a>

### DispatchToChannelRequest.TemplateVarsEntry



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| key | [string](#string) |  |  |
| value | [string](#string) |  |  |






<a name="pidgr-v1-DispatchToChannelResponse"></a>

### DispatchToChannelResponse



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| dispatch_id | [string](#string) |  | Echoes back the request&#39;s `dispatch_id`. |
| status | [DispatchStatus](#pidgr-v1-DispatchStatus) |  | Terminal outcome of this call. |
| failure_reason | [string](#string) | optional | Human-readable failure reason; set only when `status` is DISPATCH_STATUS_FAILED. |






<a name="pidgr-v1-GetCostCapPolicyRequest"></a>

### GetCostCapPolicyRequest
Get the cost-cap state for the current calendar-month period (UTC). When
no row exists for `(org_id, channel, period_yyyymm)`, the server returns
the channel default cap from server config
(`COST_CAP_DEFAULT_${CHANNEL}_MICROS`) with `used_micros = 0`.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| org_id | [string](#string) |  |  |
| channel | [ChannelName](#pidgr-v1-ChannelName) |  |  |






<a name="pidgr-v1-GetCostCapPolicyResponse"></a>

### GetCostCapPolicyResponse



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| org_id | [string](#string) |  |  |
| channel | [ChannelName](#pidgr-v1-ChannelName) |  |  |
| cap_micros | [int64](#int64) |  | Current period&#39;s cap in micros (1/1_000_000 of a USD). |
| used_micros | [int64](#int64) |  | Current period&#39;s accumulated spend in micros. |
| period_yyyymm | [int32](#int32) |  | Calendar-month period in integer YYYYMM form (e.g. 202605 for May 2026). |






<a name="pidgr-v1-GetOrgWebhookConfigRequest"></a>

### GetOrgWebhookConfigRequest
Get the org&#39;s generic-webhook channel configuration. The shared secret is
write-only and never returned — `has_secret` reports whether one is set.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| org_id | [string](#string) |  |  |






<a name="pidgr-v1-GetOrgWebhookConfigResponse"></a>

### GetOrgWebhookConfigResponse



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| org_id | [string](#string) |  |  |
| url | [string](#string) |  | Destination URL Pidgr POSTs notification events to. Empty when no configuration exists. |
| enabled | [bool](#bool) |  | Whether dispatch via the WEBHOOK channel is enabled for the org. |
| has_secret | [bool](#bool) |  | Whether a signing secret is currently configured. The secret itself is never returned. |
| created_at | [google.protobuf.Timestamp](#google-protobuf-Timestamp) |  |  |
| updated_at | [google.protobuf.Timestamp](#google-protobuf-Timestamp) |  |  |






<a name="pidgr-v1-GetReachabilityRequest"></a>

### GetReachabilityRequest
Returns the reachability metadata for a single (user, channel) tuple.
Returns NOT_FOUND if no row exists.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| org_id | [string](#string) |  |  |
| user_id | [string](#string) |  |  |
| channel | [ChannelName](#pidgr-v1-ChannelName) |  |  |






<a name="pidgr-v1-GetReachabilityResponse"></a>

### GetReachabilityResponse



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| reachability | [Reachability](#pidgr-v1-Reachability) |  | Plaintext identifier and envelope ciphertext are intentionally absent. |






<a name="pidgr-v1-GetRegionPolicyRequest"></a>

### GetRegionPolicyRequest



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| org_id | [string](#string) |  |  |
| channel | [ChannelName](#pidgr-v1-ChannelName) |  |  |






<a name="pidgr-v1-GetRegionPolicyResponse"></a>

### GetRegionPolicyResponse



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| policy | [RegionPolicy](#pidgr-v1-RegionPolicy) |  | Always populated. Empty `allowed_regions` means &#34;no policy configured&#34; — NOT &#34;no regions allowed.&#34; |






<a name="pidgr-v1-ListReachabilityForUserRequest"></a>

### ListReachabilityForUserRequest
Returns one Reachability entry per channel configured for a (org, user)
pair. Used by the admin-side per-user matrix view. Plaintext identifiers
and envelope ciphertext are intentionally absent — the admin UI only needs
to know which channels are configured.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| org_id | [string](#string) |  |  |
| user_id | [string](#string) |  |  |






<a name="pidgr-v1-ListReachabilityForUserResponse"></a>

### ListReachabilityForUserResponse



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| reachabilities | [Reachability](#pidgr-v1-Reachability) | repeated | One entry per channel that has a row for the (org_id, user_id) pair. |






<a name="pidgr-v1-RemoveReachabilityRequest"></a>

### RemoveReachabilityRequest
Idempotent removal. GDPR Recital 30 audit row is appended via internal-mTLS
BEFORE the registry row is deleted (see AuditService.Append). If no row
existed, `removed = false` and no audit row is emitted.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| org_id | [string](#string) |  |  |
| user_id | [string](#string) |  |  |
| channel | [ChannelName](#pidgr-v1-ChannelName) |  |  |






<a name="pidgr-v1-RemoveReachabilityResponse"></a>

### RemoveReachabilityResponse



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| removed | [bool](#bool) |  | True if a row was deleted. False if no row existed for the tuple (idempotent success). |






<a name="pidgr-v1-SetCostCapPolicyRequest"></a>

### SetCostCapPolicyRequest
Admin-only upsert of the cap for the current calendar-month period. Future
periods inherit the most recent SetCostCapPolicy value until the next call.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| org_id | [string](#string) |  |  |
| channel | [ChannelName](#pidgr-v1-ChannelName) |  |  |
| cap_micros | [int64](#int64) |  |  |






<a name="pidgr-v1-SetCostCapPolicyResponse"></a>

### SetCostCapPolicyResponse



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| org_id | [string](#string) |  |  |
| channel | [ChannelName](#pidgr-v1-ChannelName) |  |  |
| cap_micros | [int64](#int64) |  |  |
| used_micros | [int64](#int64) |  |  |
| period_yyyymm | [int32](#int32) |  |  |






<a name="pidgr-v1-SetOrgWebhookConfigRequest"></a>

### SetOrgWebhookConfigRequest
Admin-only upsert of the org&#39;s generic-webhook configuration. The server
validates the URL (https-only, public addresses only) before persisting,
and envelope-encrypts the secret at rest. Setting a new `secret` rotates
it; leaving `secret` unset keeps the existing one.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| org_id | [string](#string) |  |  |
| url | [string](#string) |  | Destination URL. Constraints: https scheme; non-private, non-loopback host. Validation failures return `invalid_argument`. |
| enabled | [bool](#bool) |  |  |
| secret | [string](#string) | optional | Shared secret used for the `X-Pidgr-Signature` HMAC-SHA256 header. Write-only. Unset keeps the current secret; set rotates it. Constraints: 16–256 bytes when set. |






<a name="pidgr-v1-SetOrgWebhookConfigResponse"></a>

### SetOrgWebhookConfigResponse



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| org_id | [string](#string) |  |  |
| url | [string](#string) |  |  |
| enabled | [bool](#bool) |  |  |
| has_secret | [bool](#bool) |  |  |
| created_at | [google.protobuf.Timestamp](#google-protobuf-Timestamp) |  |  |
| updated_at | [google.protobuf.Timestamp](#google-protobuf-Timestamp) |  |  |






<a name="pidgr-v1-SetRegionPolicyRequest"></a>

### SetRegionPolicyRequest
Admin-only upsert. Empty `allowed_regions` clears the policy.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| org_id | [string](#string) |  |  |
| channel | [ChannelName](#pidgr-v1-ChannelName) |  |  |
| allowed_regions | [string](#string) | repeated | AWS region identifiers (e.g. &#34;eu-west-1&#34;). Empty list == &#34;no policy.&#34; |






<a name="pidgr-v1-SetRegionPolicyResponse"></a>

### SetRegionPolicyResponse



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| policy | [RegionPolicy](#pidgr-v1-RegionPolicy) |  |  |






<a name="pidgr-v1-UpsertReachabilityRequest"></a>

### UpsertReachabilityRequest
Records a recipient identifier for a (user, channel) tuple. The plaintext
identifier is column-level KMS-encrypted on insert and never logged or
returned. The server computes the org-scoped HMAC lookup hash so opt-out
webhooks can find the row without decrypt.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| org_id | [string](#string) |  |  |
| user_id | [string](#string) |  |  |
| channel | [ChannelName](#pidgr-v1-ChannelName) |  |  |
| identifier_plaintext | [string](#string) |  | The plaintext identifier (email address, phone number, Slack user ID, Telegram chat ID, etc.). Encrypted at rest server-side. Servers MUST NOT log this field. Clients SHOULD treat this message as sensitive. |
| region_constraint | [string](#string) | optional | Optional AWS region this user&#39;s data must remain in (e.g. &#34;eu-west-1&#34;). Recorded but NOT enforced at write time; enforcement is at dispatch. |






<a name="pidgr-v1-UpsertReachabilityResponse"></a>

### UpsertReachabilityResponse



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| reachability | [Reachability](#pidgr-v1-Reachability) |  | The metadata for the upserted row. Plaintext identifier and envelope ciphertext are intentionally absent. |





 

 

 


<a name="pidgr-v1-IntegrationsService"></a>

### IntegrationsService
IntegrationsService is the gRPC surface of the pidgr-integrations service.

Auth model:
  - DispatchToChannel: internal-mTLS only. Called by the Temporal worker
    on behalf of pidgr-api dispatch activities. Never exposed publicly.
  - UpsertReachability / RemoveReachability / GetReachability /
    ListReachabilityForUser: Cognito JWT (admin RPCs, org-scoped on the
    caller&#39;s `custom:org_id` claim).
  - GetRegionPolicy / SetRegionPolicy / GetCostCapPolicy /
    SetCostCapPolicy / GetOrgWebhookConfig / SetOrgWebhookConfig /
    CreateChannelConnectLink: Cognito JWT (admin only, org-scoped).

Cross-org access is denied with `permission_denied`.

| Method Name | Request Type | Response Type | Description |
| ----------- | ------------ | ------------- | ------------|
| DispatchToChannel | [DispatchToChannelRequest](#pidgr-v1-DispatchToChannelRequest) | [DispatchToChannelResponse](#pidgr-v1-DispatchToChannelResponse) | Dispatch a single rendered message to one channel. Idempotent on `dispatch_id`. Internal-mTLS only. |
| UpsertReachability | [UpsertReachabilityRequest](#pidgr-v1-UpsertReachabilityRequest) | [UpsertReachabilityResponse](#pidgr-v1-UpsertReachabilityResponse) | Upsert a reachability identifier for a (user, channel) tuple. Encrypts and HMAC-hashes server-side before insert; emits a `REACHABILITY_UPSERT` audit row BEFORE the registry commit. |
| RemoveReachability | [RemoveReachabilityRequest](#pidgr-v1-RemoveReachabilityRequest) | [RemoveReachabilityResponse](#pidgr-v1-RemoveReachabilityResponse) | Remove a reachability identifier. Idempotent. Emits a `REACHABILITY_REMOVE` audit row BEFORE the registry delete. |
| GetReachability | [GetReachabilityRequest](#pidgr-v1-GetReachabilityRequest) | [GetReachabilityResponse](#pidgr-v1-GetReachabilityResponse) | Read a single reachability row&#39;s metadata. Returns NOT_FOUND if missing. Does not return plaintext or envelope ciphertext. |
| ListReachabilityForUser | [ListReachabilityForUserRequest](#pidgr-v1-ListReachabilityForUserRequest) | [ListReachabilityForUserResponse](#pidgr-v1-ListReachabilityForUserResponse) | List per-channel reachability metadata for a single user. Used by the admin per-user matrix view. Does not return plaintext or ciphertext. |
| GetRegionPolicy | [GetRegionPolicyRequest](#pidgr-v1-GetRegionPolicyRequest) | [GetRegionPolicyResponse](#pidgr-v1-GetRegionPolicyResponse) | Read the per-(org, channel) region allowlist. Returns an empty list when no policy is configured — NOT a NOT_FOUND. |
| SetRegionPolicy | [SetRegionPolicyRequest](#pidgr-v1-SetRegionPolicyRequest) | [SetRegionPolicyResponse](#pidgr-v1-SetRegionPolicyResponse) | Admin-only upsert of the per-(org, channel) region allowlist. |
| GetCostCapPolicy | [GetCostCapPolicyRequest](#pidgr-v1-GetCostCapPolicyRequest) | [GetCostCapPolicyResponse](#pidgr-v1-GetCostCapPolicyResponse) | Read the current calendar-month cost-cap state. Returns the channel default cap when no row exists; never NOT_FOUND. |
| SetCostCapPolicy | [SetCostCapPolicyRequest](#pidgr-v1-SetCostCapPolicyRequest) | [SetCostCapPolicyResponse](#pidgr-v1-SetCostCapPolicyResponse) | Admin-only upsert of the current calendar-month cost cap. |
| GetOrgWebhookConfig | [GetOrgWebhookConfigRequest](#pidgr-v1-GetOrgWebhookConfigRequest) | [GetOrgWebhookConfigResponse](#pidgr-v1-GetOrgWebhookConfigResponse) | Read the org&#39;s generic-webhook channel configuration. The signing secret is never returned. Returns an empty-url config when none exists — NOT a NOT_FOUND. |
| SetOrgWebhookConfig | [SetOrgWebhookConfigRequest](#pidgr-v1-SetOrgWebhookConfigRequest) | [SetOrgWebhookConfigResponse](#pidgr-v1-SetOrgWebhookConfigResponse) | Admin-only upsert of the org&#39;s generic-webhook configuration. Validates the destination URL (https-only, public hosts) and envelope-encrypts the secret at rest. |
| CreateChannelConnectLink | [CreateChannelConnectLinkRequest](#pidgr-v1-CreateChannelConnectLinkRequest) | [CreateChannelConnectLinkResponse](#pidgr-v1-CreateChannelConnectLinkResponse) | CreateChannelConnectLink mints a short-lived, HMAC-signed opt-in link a user follows to bind a third-party channel (Telegram bot-follow, Slack OAuth, LINE follow-code) to their (org, user). Wraps the api-side internal/linktoken minter. Channels other than TELEGRAM, SLACK, and LINE are rejected with `invalid_argument`. |

 



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
| data_governance_region | [string](#string) |  | Optional data governance region. Users who redeem this link inherit this region. Empty means inherit from org default. Valid values: EU, LATAM, BR, APAC, US. |






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
| data_governance_region | [string](#string) |  | Data governance region assigned to users who redeem this link. Empty means inherit from org default. Valid values: EU, LATAM, BR, APAC, US. |






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
| data_governance_region | [string](#string) |  | Optional data governance region for the invited user. Empty means inherit from org default. Valid values: EU, LATAM, BR, APAC, US. |






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






<a name="pidgr-v1-UpdateUserRegionRequest"></a>

### UpdateUserRegionRequest
Request to update a user&#39;s data governance region.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| user_id | [string](#string) |  | ID of the user whose region to update. Required. |
| data_governance_region | [string](#string) |  | New governance region, or empty to inherit from org default. Valid values: EU, LATAM, BR, APAC, US. |






<a name="pidgr-v1-UpdateUserRegionResponse"></a>

### UpdateUserRegionResponse
Response after updating a user&#39;s governance region.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| user | [User](#pidgr-v1-User) |  | The updated user. |
| migration_workflow_id | [string](#string) |  | Temporal workflow ID for the region migration, if a migration was triggered. Empty if the region didn&#39;t actually change. |






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
| UpdateUserRegion | [UpdateUserRegionRequest](#pidgr-v1-UpdateUserRegionRequest) | [UpdateUserRegionResponse](#pidgr-v1-UpdateUserRegionResponse) | Update the data governance region for a user. Triggers a data migration workflow if the region changed. Admin-only operation. Authorization: Requires PERMISSION_MEMBERS_MANAGE. |

 



<a name="pidgr_v1_org_security_keys_service-proto"></a>
<p align="right"><a href="#top">Top</a></p>

## pidgr/v1/org_security_keys_service.proto



<a name="pidgr-v1-GetPeppersRequest"></a>

### GetPeppersRequest
Request to fetch the active (non-retired) peppers for one org/purpose.

Auth: internal-mTLS only. This RPC exposes raw cryptographic key material
and MUST NOT be reachable from the public ingress or from JWT-authenticated
clients. The server SHALL reject any caller whose mTLS identity is not on
the configured allowlist.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| org_id | [string](#string) |  | Organization whose peppers are requested. |
| purpose | [string](#string) |  | Purpose identifier scoping which key family to return. Use `&#34;reachability_lookup&#34;` for the pidgr-integrations registry lookup hash. |






<a name="pidgr-v1-GetPeppersResponse"></a>

### GetPeppersResponse



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| peppers | [Pepper](#pidgr-v1-Pepper) | repeated | All non-retired pepper versions for the (org_id, purpose) pair, in ascending version order. Typically exactly one entry; two during a rotation overlap window; zero only when no pepper has ever been generated for this (org, purpose). |






<a name="pidgr-v1-Pepper"></a>

### Pepper
A single non-retired pepper version. Returned by GetPeppers.

During a rotation overlap, multiple versions are returned — callers
(e.g. pidgr-integrations) compute lookup hashes under EVERY returned
version to write or match against `identifier_lookup_hash_v1` and
`identifier_lookup_hash_v2` on the reachability registry.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| version | [int32](#int32) |  | Monotonically-increasing version number. Lower versions retire first. |
| key_material | [bytes](#bytes) |  | Raw HMAC key material. Sensitive — callers MUST NOT log or persist this value to disk. In-memory caching keyed on (org_id, version) with a short TTL is permitted and expected. |





 

 

 


<a name="pidgr-v1-OrgSecurityKeysService"></a>

### OrgSecurityKeysService
OrgSecurityKeysService exposes per-org secret key material to internal
services that need it for HMAC computation, signature verification, or
envelope-key derivation.

AUTH: INTERNAL-mTLS ONLY. Every RPC on this service returns raw key
material. The server MUST require a client certificate from a known
internal service (allowlisted by subject DN) and MUST reject any request
presenting only a JWT. The service MUST NOT be exposed on the public ALB.

Callers (today: pidgr-integrations) cache returned peppers in-process
keyed on (org_id, purpose, version) with a short TTL (≤ 5 minutes) to
keep dispatch hot-path latency bounded.

| Method Name | Request Type | Response Type | Description |
| ----------- | ------------ | ------------- | ------------|
| GetPeppers | [GetPeppersRequest](#pidgr-v1-GetPeppersRequest) | [GetPeppersResponse](#pidgr-v1-GetPeppersResponse) | Return all non-retired pepper versions for one (org_id, purpose). Used by pidgr-integrations to compute `identifier_lookup_hash_v1` / `identifier_lookup_hash_v2` during pepper-rotation overlap windows. |

 



<a name="pidgr_v1_organization-proto"></a>
<p align="right"><a href="#top">Top</a></p>

## pidgr/v1/organization.proto



<a name="pidgr-v1-CreateOrganizationRequest"></a>

### CreateOrganizationRequest
Request to create a new organization.
JWT auth only — the authenticated caller becomes the initial admin. Additional
admins are added via CreateInviteLink after the org exists.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| name | [string](#string) |  | Name for the new organization. Constraints: Max length 200 characters. |
| industry | [Industry](#pidgr-v1-Industry) |  | Industry vertical for the organization. |
| company_size | [CompanySize](#pidgr-v1-CompanySize) |  | Employee headcount range. |
| access_code | [string](#string) |  | Access code required during early access. Format: PIDGR-XXXXXXXX (8 alphanumeric characters). |
| data_governance_region | [string](#string) |  | Data governance framework. Defaults to &#34;US&#34; if omitted. Valid values: EU, LATAM, BR, APAC, US. |
| fixture_id | [string](#string) |  | Optional bootstrap fixture to seed the organization with starter data. Empty string means the default fixture. |






<a name="pidgr-v1-CreateOrganizationResponse"></a>

### CreateOrganizationResponse
Response after creating an organization.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| organization | [Organization](#pidgr-v1-Organization) |  | The newly created organization. |
| admin_user | [User](#pidgr-v1-User) |  | The admin user created for the organization. |






<a name="pidgr-v1-CreateSandboxOrganizationRequest"></a>

### CreateSandboxOrganizationRequest
Request to create a sandbox organization for testing.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| name | [string](#string) |  | Name for the sandbox organization. Constraints: Max length 200 characters. |
| expires_at | [google.protobuf.Timestamp](#google-protobuf-Timestamp) |  | Required expiration time. Max 30 days from now for interactive callers; API-key callers may set shorter TTLs for ephemeral test sandboxes. |
| data_governance_region | [string](#string) |  | Data governance framework. Defaults to &#34;US&#34; if omitted. Valid values: EU, LATAM, BR, APAC, US. |
| fixture_id | [string](#string) |  | Optional bootstrap fixture to seed the sandbox with starter data. Empty string means the default fixture. Must match an id returned by ListSandboxFixtures. |






<a name="pidgr-v1-CreateSandboxOrganizationResponse"></a>

### CreateSandboxOrganizationResponse
Response after creating a sandbox organization.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| organization | [Organization](#pidgr-v1-Organization) |  | The newly created sandbox organization (org_type: SANDBOX). |
| admin_user | [User](#pidgr-v1-User) |  | The admin user created for the sandbox. |






<a name="pidgr-v1-DeleteSandboxOrganizationRequest"></a>

### DeleteSandboxOrganizationRequest
Request to delete a sandbox organization. Only callable for orgs with
org_type=SANDBOX. Allowed for super admins of the sandbox or the creator.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| org_id | [string](#string) |  | ID of the sandbox organization to delete. |






<a name="pidgr-v1-DeleteSandboxOrganizationResponse"></a>

### DeleteSandboxOrganizationResponse
Response after requesting deletion. Deletion runs asynchronously via
the DeleteOrgWorkflow; a success response means the workflow started.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| workflow_id | [string](#string) |  | ID of the Temporal workflow handling the deletion. |






<a name="pidgr-v1-GetOrgPrivacySettingsRequest"></a>

### GetOrgPrivacySettingsRequest
Request to retrieve the org-level privacy settings.
The organization is extracted from the JWT.






<a name="pidgr-v1-GetOrgPrivacySettingsResponse"></a>

### GetOrgPrivacySettingsResponse
Response containing the org-level privacy settings with consent-trace
metadata for each toggle.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| settings | [OrgPrivacySettings](#pidgr-v1-OrgPrivacySettings) |  | The organization&#39;s current privacy settings. |






<a name="pidgr-v1-GetOrganizationRequest"></a>

### GetOrganizationRequest
Request to retrieve the organization for the authenticated user.






<a name="pidgr-v1-GetOrganizationResponse"></a>

### GetOrganizationResponse
Response containing the organization.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| organization | [Organization](#pidgr-v1-Organization) |  | The organization the authenticated user belongs to. |






<a name="pidgr-v1-ListSandboxFixturesRequest"></a>

### ListSandboxFixturesRequest
Request to list all bootstrap fixtures available for seeding.
No parameters — catalog is the same for all callers.






<a name="pidgr-v1-ListSandboxFixturesResponse"></a>

### ListSandboxFixturesResponse
Response containing the bootstrap fixture catalog.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| fixtures | [SandboxFixture](#pidgr-v1-SandboxFixture) | repeated | All registered fixtures, ordered by name. |






<a name="pidgr-v1-ListUserOrganizationsRequest"></a>

### ListUserOrganizationsRequest
Request to list all organizations the authenticated user belongs to.
No parameters — user identity is extracted from the JWT sub claim.






<a name="pidgr-v1-ListUserOrganizationsResponse"></a>

### ListUserOrganizationsResponse
Response containing all organizations the authenticated user belongs to.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| organizations | [Organization](#pidgr-v1-Organization) | repeated | Organizations the user belongs to, ordered by created_at ascending. Excludes expired sandbox organizations. |






<a name="pidgr-v1-ListUserSandboxesRequest"></a>

### ListUserSandboxesRequest
Request to list only the sandbox organizations the authenticated user
belongs to (i.e. orgs where org_type = SANDBOX, filtered from the full
membership set). No parameters — user identity is extracted from the JWT
sub claim.






<a name="pidgr-v1-ListUserSandboxesResponse"></a>

### ListUserSandboxesResponse
Response containing the user&#39;s sandbox organizations.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| sandboxes | [Organization](#pidgr-v1-Organization) | repeated | Sandbox organizations the user belongs to, ordered by expires_at ascending (soonest-expiring first — matches the admin UI /organization/sandboxes ordering). Excludes already-expired sandboxes (those are pending cleanup by SandboxCleanupWorkflow). |






<a name="pidgr-v1-OrgPrivacySettings"></a>

### OrgPrivacySettings
Org-level data-processing settings (compliance consent surface).
Each toggle gates an entire category of processing for every user in
the organization.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| ai_clustering | [OrgPrivacyToggle](#pidgr-v1-OrgPrivacyToggle) |  | Gates ML archetype clustering and ACK predictions. |
| behavioral_analytics | [OrgPrivacyToggle](#pidgr-v1-OrgPrivacyToggle) |  | Gates behavioral analytics (session replay, heatmaps, dwell metrics). |
| third_party_channels | [OrgPrivacyToggle](#pidgr-v1-OrgPrivacyToggle) |  | Gates third-party notification channel dispatch (email, Slack, SMS, …). |






<a name="pidgr-v1-OrgPrivacyToggle"></a>

### OrgPrivacyToggle
A single org-level data-processing toggle with consent-trace metadata.
The metadata records who flipped the toggle last and when, so the admin
consent-trace UI can show a verifiable change trail.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| enabled | [bool](#bool) |  | Whether this category of processing is enabled for the organization. |
| last_changed_by_email | [string](#string) |  | Email of the admin who last changed this toggle. Empty if the toggle has never been changed from its default. |
| last_changed_at | [google.protobuf.Timestamp](#google-protobuf-Timestamp) |  | When this toggle was last changed. Empty if the toggle has never been changed from its default. |






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
| org_type | [OrgType](#pidgr-v1-OrgType) |  | Organization lifecycle type. |
| expires_at | [google.protobuf.Timestamp](#google-protobuf-Timestamp) |  | Expiration time for sandbox organizations. Empty for standard orgs. |
| data_governance_region | [string](#string) |  | Data governance framework (EU, LATAM, BR, APAC, US). Determines legal framework, DPA template, and Bedrock endpoint routing. |
| data_content_region | [string](#string) |  | AWS region for content storage (resolved from data_governance_region). e.g., &#34;eu-west-1&#34;, &#34;us-east-1&#34;. |
| ml_retrain_cold_threshold | [int32](#int32) |  | ─── ML pipeline settings ────────────────────────────────────────────────── Cold-start threshold: completed campaigns below this count trigger immediate retraining. At or above, the org is flagged for the weekly cron. Default 10, range 1-100. |
| ml_cancelled_counts | [bool](#bool) |  | Whether cancelled campaigns count toward the training counter. Default true. |
| ml_manual_limit_monthly | [int32](#int32) |  | Monthly limit on manual retrain triggers. Default 3, range 0-10. |
| ml_manual_retrains_used | [int32](#int32) |  | Number of manual retrains used in the current month (resets monthly). |
| ml_needs_retrain | [bool](#bool) |  | Whether the org is flagged for the next weekly cron run. |
| campaigns_since_last_training | [int32](#int32) |  | Campaigns completed since the last ML training run. |
| total_completed_campaigns | [int32](#int32) |  | Total campaigns completed across the organization lifetime. |
| last_ml_training_at | [google.protobuf.Timestamp](#google-protobuf-Timestamp) |  | Timestamp of the most recent successful ML training. Empty if never trained. |
| include_synthetic_in_aggregates | [bool](#bool) | optional | Controls whether aggregate stats (campaign recipient/ack/missed counts) include synthetic data. Unset = default by org type: sandbox orgs include, standard orgs exclude. Derived intelligence (ML, analytics, attestation evidence) always excludes synthetic regardless of this setting. |
| provisional_archetypes_enabled | [bool](#bool) |  | Whether the organization has opted into provisional (rule-based, low-confidence) archetypes for groups that don&#39;t yet have trained ML archetypes. Only meaningful for ORG_TYPE_STANDARD — sandbox organizations are always eligible regardless of this setting. Default false: production analytics stay conservative. |






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






<a name="pidgr-v1-SandboxFixture"></a>

### SandboxFixture
A bootstrap fixture that can be applied when creating a new organization.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| id | [string](#string) |  | Stable slug for referencing this fixture (e.g. &#34;starter&#34;, &#34;empty&#34;, &#34;fintech&#34;, &#34;sales&#34;). Pass it back as the fixture_id on create. |
| name | [string](#string) |  | Display name for admin UI (e.g. &#34;Starter&#34;). |
| description | [string](#string) |  | Description shown alongside the fixture option in the UI. |
| is_default | [bool](#bool) |  | Exactly one fixture has is_default=true. Clients that show a simple &#34;seed initial data&#34; control select this fixture&#39;s id by default. |






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






<a name="pidgr-v1-UpdateOrgPrivacySettingsRequest"></a>

### UpdateOrgPrivacySettingsRequest
Request to update org-level privacy settings. Only the provided fields
are changed; unset fields leave the corresponding toggle unchanged.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| ai_clustering_enabled | [bool](#bool) | optional | Enable or disable ML archetype clustering and ACK predictions. Unset leaves unchanged. |
| behavioral_analytics_enabled | [bool](#bool) | optional | Enable or disable behavioral analytics. Unset leaves unchanged. |
| third_party_channels_enabled | [bool](#bool) | optional | Enable or disable third-party notification channels. Unset leaves unchanged. |






<a name="pidgr-v1-UpdateOrgPrivacySettingsResponse"></a>

### UpdateOrgPrivacySettingsResponse
Response after updating org-level privacy settings.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| settings | [OrgPrivacySettings](#pidgr-v1-OrgPrivacySettings) |  | The organization&#39;s privacy settings after the update, with refreshed consent-trace metadata. |






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
| ml_retrain_cold_threshold | [int32](#int32) |  | New ML cold-start threshold. 0 leaves unchanged, otherwise must be in [1, 100]. |
| ml_cancelled_counts | [bool](#bool) | optional | New ML cancelled-counts flag. Uses google.protobuf.BoolValue-style semantics via optional to distinguish &#34;not provided&#34; from &#34;set to false&#34;. |
| ml_manual_limit_monthly | [int32](#int32) |  | New ML monthly manual limit. Negative leaves unchanged, otherwise must be in [0, 10]. Encoded as int32 with -1 meaning &#34;leave unchanged&#34;. |
| include_synthetic_in_aggregates | [bool](#bool) | optional | Set the synthetic-aggregates override; unset leaves it unchanged. |
| provisional_archetypes_enabled | [bool](#bool) | optional | New provisional-archetypes opt-in for standard organizations. Unset leaves unchanged. Rejected for sandbox organizations, which are always eligible automatically. |






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



<a name="pidgr-v1-OrgType"></a>

### OrgType
Classification of an organization&#39;s lifecycle type.

| Name | Number | Description |
| ---- | ------ | ----------- |
| ORG_TYPE_UNSPECIFIED | 0 |  |
| ORG_TYPE_STANDARD | 1 |  |
| ORG_TYPE_SANDBOX | 2 |  |
| ORG_TYPE_STAFF | 3 | Reserved for platform operations. At most one per deployment, seeded by migration. Cannot be created via CreateOrganization. |


 

 


<a name="pidgr-v1-OrganizationService"></a>

### OrganizationService
Manages organizations (tenants) in the Pidgr platform.
Most RPCs operate within the caller&#39;s org (extracted from JWT).
CreateOrganization supports API key auth or JWT auth (self-service onboarding).

| Method Name | Request Type | Response Type | Description |
| ----------- | ------------ | ------------- | ------------|
| CreateOrganization | [CreateOrganizationRequest](#pidgr-v1-CreateOrganizationRequest) | [CreateOrganizationResponse](#pidgr-v1-CreateOrganizationResponse) | Create a new organization. JWT auth only — the caller becomes the initial admin. Add further admins via CreateInviteLink. Authorization: Authenticated user (no specific permission required). |
| GetOrganization | [GetOrganizationRequest](#pidgr-v1-GetOrganizationRequest) | [GetOrganizationResponse](#pidgr-v1-GetOrganizationResponse) | Retrieve the organization for the authenticated user. Authorization: Requires PERMISSION_ORG_READ. |
| UpdateOrganization | [UpdateOrganizationRequest](#pidgr-v1-UpdateOrganizationRequest) | [UpdateOrganizationResponse](#pidgr-v1-UpdateOrganizationResponse) | Update organization settings (name, default workflow, industry, company size). Authorization: Requires PERMISSION_ORG_WRITE. |
| UpdateSsoAttributeMappings | [UpdateSsoAttributeMappingsRequest](#pidgr-v1-UpdateSsoAttributeMappingsRequest) | [UpdateSsoAttributeMappingsResponse](#pidgr-v1-UpdateSsoAttributeMappingsResponse) | Replace all SSO attribute mappings for the organization. Authorization: Requires PERMISSION_ORG_WRITE. |
| RotateAnalyticsSalt | [RotateAnalyticsSaltRequest](#pidgr-v1-RotateAnalyticsSaltRequest) | [RotateAnalyticsSaltResponse](#pidgr-v1-RotateAnalyticsSaltResponse) | Rotate the analytics salt and optionally increase the bucket count for k-anonymization. Authorization: Requires PERMISSION_PRIVACY_WRITE. |
| UpdateAnalyticsEpsilon | [UpdateAnalyticsEpsilonRequest](#pidgr-v1-UpdateAnalyticsEpsilonRequest) | [UpdateAnalyticsEpsilonResponse](#pidgr-v1-UpdateAnalyticsEpsilonResponse) | Update the differential privacy epsilon parameter. Authorization: Requires PERMISSION_PRIVACY_WRITE. |
| GetOrgPrivacySettings | [GetOrgPrivacySettingsRequest](#pidgr-v1-GetOrgPrivacySettingsRequest) | [GetOrgPrivacySettingsResponse](#pidgr-v1-GetOrgPrivacySettingsResponse) | Retrieve org-level data-processing toggles (AI clustering, behavioral analytics, third-party channels) with last-changed-by/at metadata for the consent-trace UI. Authorization: Requires PERMISSION_PRIVACY_READ. |
| UpdateOrgPrivacySettings | [UpdateOrgPrivacySettingsRequest](#pidgr-v1-UpdateOrgPrivacySettingsRequest) | [UpdateOrgPrivacySettingsResponse](#pidgr-v1-UpdateOrgPrivacySettingsResponse) | Update org-level data-processing toggles. Only provided fields change; each change is recorded with the acting admin and timestamp. Authorization: Requires PERMISSION_PRIVACY_WRITE. |
| CreateSandboxOrganization | [CreateSandboxOrganizationRequest](#pidgr-v1-CreateSandboxOrganizationRequest) | [CreateSandboxOrganizationResponse](#pidgr-v1-CreateSandboxOrganizationResponse) | Create a sandbox organization for testing configurations. Sandbox orgs auto-delete after expires_at. The caller becomes super admin. Authorization: Any authenticated user. Limited to 3 concurrent sandboxes per user to prevent abuse. |
| DeleteSandboxOrganization | [DeleteSandboxOrganizationRequest](#pidgr-v1-DeleteSandboxOrganizationRequest) | [DeleteSandboxOrganizationResponse](#pidgr-v1-DeleteSandboxOrganizationResponse) | Delete a sandbox organization immediately. Starts the DeleteOrgWorkflow which handles cleanup across DB, Cognito, S3, Temporal, and regional content stores. Authorization: Super admin of the target sandbox OR its creator. |
| ListSandboxFixtures | [ListSandboxFixturesRequest](#pidgr-v1-ListSandboxFixturesRequest) | [ListSandboxFixturesResponse](#pidgr-v1-ListSandboxFixturesResponse) | List bootstrap fixtures available for seeding new organizations. The catalog is backend-owned; admin UI populates the &#34;seed initial data&#34; control or dropdown from this response. Authorization: Any authenticated user. |
| ListUserOrganizations | [ListUserOrganizationsRequest](#pidgr-v1-ListUserOrganizationsRequest) | [ListUserOrganizationsResponse](#pidgr-v1-ListUserOrganizationsResponse) | List all organizations the authenticated user belongs to. Org-exempt: callable without org context (only requires valid JWT). Used by the admin org switcher to discover available orgs. Excludes expired sandbox organizations. Authorization: Authenticated user (no specific permission required). |
| ListUserSandboxes | [ListUserSandboxesRequest](#pidgr-v1-ListUserSandboxesRequest) | [ListUserSandboxesResponse](#pidgr-v1-ListUserSandboxesResponse) | List only the sandbox organizations the authenticated user belongs to. Org-exempt: callable without org context. The admin UI&#39;s /sandboxes management page is the primary consumer — it&#39;s a user-level surface rather than org-scoped, so this RPC is user-scoped too. Excludes already-expired sandboxes (pending cleanup). Authorization: Authenticated user (no specific permission required). |

 



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



<a name="pidgr-v1-ApproveTemplateTranslationRequest"></a>

### ApproveTemplateTranslationRequest
Request to approve a template translation.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| translation_id | [string](#string) |  | ID of the translation to approve. |






<a name="pidgr-v1-ApproveTemplateTranslationResponse"></a>

### ApproveTemplateTranslationResponse
Response after approving a template translation.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| translation | [TemplateTranslation](#pidgr-v1-TemplateTranslation) |  | The approved translation (status: APPROVED, reviewed_by and reviewed_at set). |






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
| source_locale | [string](#string) |  | Language of the template body content. Defaults to org&#39;s default_locale. Valid values: en, es, pt-BR, zh, ja. |






<a name="pidgr-v1-CreateTemplateResponse"></a>

### CreateTemplateResponse
Response after creating a template.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| template | [Template](#pidgr-v1-Template) |  | The newly created template (version 1). |






<a name="pidgr-v1-CreateTemplateTranslationRequest"></a>

### CreateTemplateTranslationRequest
Request to create a translation for a template.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| template_id | [string](#string) |  | ID of the template to translate. |
| version | [int32](#int32) |  | Version of the template to translate. |
| locale | [string](#string) |  | Target locale. |
| title | [string](#string) |  | Translated title. |
| body | [string](#string) |  | Translated body content. |
| translated_by | [string](#string) |  | Who created this translation (&#34;ai:bedrock&#34; or user UUID). |
| status | [TranslationStatus](#pidgr-v1-TranslationStatus) |  | Initial status (typically DRAFT or AI_TRANSLATED). |






<a name="pidgr-v1-CreateTemplateTranslationResponse"></a>

### CreateTemplateTranslationResponse
Response after creating a template translation.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| translation | [TemplateTranslation](#pidgr-v1-TemplateTranslation) |  | The created translation. |






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






<a name="pidgr-v1-ListTemplateTranslationsRequest"></a>

### ListTemplateTranslationsRequest
Request to list translations for a template version.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| template_id | [string](#string) |  | ID of the template. |
| version | [int32](#int32) |  | Version of the template. 0 returns translations for the latest version. |






<a name="pidgr-v1-ListTemplateTranslationsResponse"></a>

### ListTemplateTranslationsResponse
Response containing all translations for a template version.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| translations | [TemplateTranslation](#pidgr-v1-TemplateTranslation) | repeated | Translations for the requested template version. |






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
| source_locale | [string](#string) |  | Language of the template body content (e.g., &#34;en&#34;, &#34;es&#34;, &#34;ja&#34;). Defaults to the org&#39;s default_locale, falling back to &#34;en&#34;. Translations are created as locale variants of this source. |






<a name="pidgr-v1-TemplateTranslation"></a>

### TemplateTranslation
A locale-specific translation of a template&#39;s title and body.
Translations are created per template version and go through a review workflow.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| id | [string](#string) |  | Unique identifier for this translation. |
| template_id | [string](#string) |  | ID of the source template. |
| version | [int32](#int32) |  | Version of the source template this translation is for. |
| locale | [string](#string) |  | Target locale (e.g., &#34;es&#34;, &#34;pt-BR&#34;, &#34;zh&#34;, &#34;ja&#34;). |
| title | [string](#string) |  | Translated title. Constraints: Max length 200 characters. |
| body | [string](#string) |  | Translated body content with {{variable}} placeholders preserved. Constraints: Max length 50000 characters. |
| status | [TranslationStatus](#pidgr-v1-TranslationStatus) |  | Current review status. |
| translated_by | [string](#string) |  | Who created this translation (&#34;ai:bedrock&#34;, &#34;ai:deepl&#34;, or user UUID). |
| reviewed_by | [string](#string) |  | User who approved the translation. Empty until approved. |
| reviewed_at | [google.protobuf.Timestamp](#google-protobuf-Timestamp) |  | When the translation was approved. |
| created_at | [google.protobuf.Timestamp](#google-protobuf-Timestamp) |  | When the translation was created. |






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






<a name="pidgr-v1-UpdateTemplateTranslationRequest"></a>

### UpdateTemplateTranslationRequest
Request to update an existing template translation.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| translation_id | [string](#string) |  | ID of the translation to update. |
| title | [string](#string) |  | Updated title. Empty leaves unchanged. |
| body | [string](#string) |  | Updated body. Empty leaves unchanged. |
| status | [TranslationStatus](#pidgr-v1-TranslationStatus) |  | Updated status. |






<a name="pidgr-v1-UpdateTemplateTranslationResponse"></a>

### UpdateTemplateTranslationResponse
Response after updating a template translation.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| translation | [TemplateTranslation](#pidgr-v1-TemplateTranslation) |  | The updated translation. |





 


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



<a name="pidgr-v1-TranslationStatus"></a>

### TranslationStatus
Review status of a template translation.

| Name | Number | Description |
| ---- | ------ | ----------- |
| TRANSLATION_STATUS_UNSPECIFIED | 0 |  |
| TRANSLATION_STATUS_DRAFT | 1 | Translation draft, not yet reviewed. |
| TRANSLATION_STATUS_AI_TRANSLATED | 2 | Translation generated by AI, pending human review. |
| TRANSLATION_STATUS_IN_REVIEW | 3 | Translation is being reviewed by a human. |
| TRANSLATION_STATUS_APPROVED | 4 | Translation has been approved for use. |


 

 


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
| CreateTemplateTranslation | [CreateTemplateTranslationRequest](#pidgr-v1-CreateTemplateTranslationRequest) | [CreateTemplateTranslationResponse](#pidgr-v1-CreateTemplateTranslationResponse) | Create a locale-specific translation of a template. Authorization: Requires PERMISSION_TEMPLATES_WRITE. |
| UpdateTemplateTranslation | [UpdateTemplateTranslationRequest](#pidgr-v1-UpdateTemplateTranslationRequest) | [UpdateTemplateTranslationResponse](#pidgr-v1-UpdateTemplateTranslationResponse) | Update an existing template translation. Authorization: Requires PERMISSION_TEMPLATES_WRITE. |
| ListTemplateTranslations | [ListTemplateTranslationsRequest](#pidgr-v1-ListTemplateTranslationsRequest) | [ListTemplateTranslationsResponse](#pidgr-v1-ListTemplateTranslationsResponse) | List all translations for a template version. Authorization: Requires PERMISSION_TEMPLATES_READ. |
| ApproveTemplateTranslation | [ApproveTemplateTranslationRequest](#pidgr-v1-ApproveTemplateTranslationRequest) | [ApproveTemplateTranslationResponse](#pidgr-v1-ApproveTemplateTranslationResponse) | Approve a template translation for use in campaigns. Authorization: Requires PERMISSION_TEMPLATES_REVIEW. |

 



<a name="pidgr_v1_token-proto"></a>
<p align="right"><a href="#top">Top</a></p>

## pidgr/v1/token.proto



<a name="pidgr-v1-DeeplinkTokenPayload"></a>

### DeeplinkTokenPayload
Decoded deeplink-token payload. Populated by ValidateDeeplinkToken
only when validation succeeds.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| campaign_id | [string](#string) |  | Campaign UUID the deeplink targets. The native app uses this for the authenticated GetCampaign follow-up post-recipient-auth. |
| recipient_user_id | [string](#string) |  | Recipient UUID the token authorizes. The token does not authenticate the recipient (that&#39;s the auth flow&#39;s job); it authorizes &#34;this deeplink path is for this recipient&#34; so the native app can refuse to render a token whose embedded recipient mismatches the signed-in user. |
| step_kind | [ChannelStepKind](#pidgr-v1-ChannelStepKind) |  | Step kind the deeplink targets — REMINDER vs ESCALATION. Lets the native app pick the right campaign-card variant before the auth gate. |
| expires_at | [google.protobuf.Timestamp](#google-protobuf-Timestamp) |  | Expiry the token carries. Validation rejects tokens past this time even if the signature checks out. |






<a name="pidgr-v1-SignDeeplinkTokenRequest"></a>

### SignDeeplinkTokenRequest



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| campaign_id | [string](#string) |  | Campaign whose deeplink this token authorizes. Constraints: required, must be a UUID and exist within the caller&#39;s organization. |
| recipient_user_id | [string](#string) |  | Recipient the token authorizes. Constraints: required, must be a UUID and a member of the campaign&#39;s audience. |
| step_kind | [ChannelStepKind](#pidgr-v1-ChannelStepKind) |  | Step kind the deeplink targets. Required. |
| ttl_seconds | [int64](#int64) |  | Token lifetime in seconds from now. Constraints: required, must be in (0, 30 * 24 * 3600] (1 second to 30 days). 30 days matches the platform&#39;s outer bound on actionable campaign lifetimes; longer tokens are not signed. |






<a name="pidgr-v1-SignDeeplinkTokenResponse"></a>

### SignDeeplinkTokenResponse



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| token | [string](#string) |  | The signed token, ready to URL-embed in links.pidgr.com/c/{short_code}?t={token}. Format: base64url-encoded payload (JSON) &#43; base64url-encoded HMAC-SHA256 trailer, joined by a single dot. Implementation detail — clients SHOULD NOT parse or mutate the token; they pass it back to ValidateDeeplinkToken. |
| expires_at | [google.protobuf.Timestamp](#google-protobuf-Timestamp) |  | The expiry the token carries. Echoed back so clients don&#39;t need to redo the time-math the caller passed in via ttl_seconds. |
| key_version | [int32](#int32) |  | The platform key version used to sign. Clients MAY record for telemetry but SHOULD NOT branch logic on it — the platform manages overlap windows during rotation transparently. |






<a name="pidgr-v1-ValidateDeeplinkTokenRequest"></a>

### ValidateDeeplinkTokenRequest



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| token | [string](#string) |  | The token bytes from the deeplink URL&#39;s `t` query parameter. Constraints: required, non-empty. |
| campaign_id | [string](#string) |  | Campaign UUID embedded in the URL path (translated from the short-code by the native app via CampaignService.GetCampaignByShortCode). Validation rejects when the token&#39;s embedded campaign_id does not match — defense against replay attacks that swap the short-code path component while reusing a signed token from a different campaign. |






<a name="pidgr-v1-ValidateDeeplinkTokenResponse"></a>

### ValidateDeeplinkTokenResponse



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| valid | [bool](#bool) |  | True when signature &#43; expiry both check out under any active or overlap-window key version. |
| failure_reason | [ValidationFailureReason](#pidgr-v1-ValidationFailureReason) |  | Reason validation failed. Set only when valid=false; UNSPECIFIED when valid=true. The native app uses this to drive UX (silent retry vs. &#34;this link expired&#34; message vs. &#34;this link looks tampered&#34;). |
| payload | [DeeplinkTokenPayload](#pidgr-v1-DeeplinkTokenPayload) |  | Decoded payload. Populated only when valid=true. The native app SHOULD compare payload.recipient_user_id against the signed-in user and refuse to render the campaign card on mismatch. |





 


<a name="pidgr-v1-ValidationFailureReason"></a>

### ValidationFailureReason
Reason a deeplink-token validation failed. Empty when valid=true.

| Name | Number | Description |
| ---- | ------ | ----------- |
| VALIDATION_FAILURE_REASON_UNSPECIFIED | 0 |  |
| VALIDATION_FAILURE_REASON_INVALID_SIGNATURE | 1 | Token bytes parsed but the HMAC signature did not verify under any active or overlap-window key version. |
| VALIDATION_FAILURE_REASON_EXPIRED | 2 | Token signature verified but its embedded expiry has passed. |
| VALIDATION_FAILURE_REASON_KEY_RETIRED | 3 | Signature would have verified, but the key version that signed the token is past the rotation overlap window and has been hard-deleted. This means the token is older than the platform&#39;s retention bound (rotation cadence &#43; overlap window) — operationally equivalent to EXPIRED but distinguishable for telemetry. |
| VALIDATION_FAILURE_REASON_MALFORMED | 4 | Token bytes could not be parsed at all (not base64url, wrong length, missing payload separator, etc.). Indicates a tampered or truncated URL. |


 

 


<a name="pidgr-v1-TokenService"></a>

### TokenService
HMAC-signed deeplink-token signing and validation.

Deeplink tokens carry the (campaign, recipient, step) tuple that
authorizes a third-party-channel deeplink
(`links.pidgr.com/c/{short_code}?t={token}`). The token is signed by
pidgr-api with a per-org HMAC key managed by the platform&#39;s key store;
rotation is automatic with a 7-day overlap window so callers do not
have to coordinate.

| Method Name | Request Type | Response Type | Description |
| ----------- | ------------ | ------------- | ------------|
| SignDeeplinkToken | [SignDeeplinkTokenRequest](#pidgr-v1-SignDeeplinkTokenRequest) | [SignDeeplinkTokenResponse](#pidgr-v1-SignDeeplinkTokenResponse) | Sign a deeplink token. Authorization: internal-mTLS-only (caller is the dispatch layer assembling a deeplink during message composition). |
| ValidateDeeplinkToken | [ValidateDeeplinkTokenRequest](#pidgr-v1-ValidateDeeplinkTokenRequest) | [ValidateDeeplinkTokenResponse](#pidgr-v1-ValidateDeeplinkTokenResponse) | Validate a deeplink token. Authorization: unauthenticated (caller is the native app validating before showing the auth gate). The token IS the lookup key; valid responses carry the decoded payload. |

 



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

