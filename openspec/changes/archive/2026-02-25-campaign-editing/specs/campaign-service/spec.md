## MODIFIED Requirements

### Requirement: CampaignService proto definition
The system SHALL define a CampaignService in `pidgr/v1/campaign.proto` with RPCs: CreateCampaign, StartCampaign, GetCampaign, ListCampaigns, CancelCampaign, ListDeliveries, UpdateCampaign.

#### Scenario: Service definition exists
- **WHEN** the proto file is compiled
- **THEN** it SHALL produce a valid CampaignService with all seven RPCs

## ADDED Requirements

### Requirement: UpdateCampaign RPC
The system SHALL define UpdateCampaign(UpdateCampaignRequest) returning UpdateCampaignResponse. UpdateCampaignRequest SHALL contain: campaign_id (string, required), name (string, optional), sender_name (string, optional), title (string, optional), template_id (string, optional), template_version (int32, optional), workflow (WorkflowDefinition, optional). UpdateCampaignResponse SHALL contain the updated Campaign.

#### Scenario: Update campaign name
- **WHEN** a client calls UpdateCampaign with campaign_id and name="New Name"
- **THEN** the response SHALL contain a Campaign with name="New Name" and all other fields unchanged

#### Scenario: Update multiple fields
- **WHEN** a client calls UpdateCampaign with name, sender_name, and title
- **THEN** the response SHALL contain a Campaign with all three fields updated

#### Scenario: Update workflow
- **WHEN** a client calls UpdateCampaign with a new WorkflowDefinition
- **THEN** the response SHALL contain a Campaign with the new workflow

#### Scenario: Update campaign not in CREATED status
- **WHEN** a client calls UpdateCampaign on a campaign with status RUNNING
- **THEN** the server SHALL return a FAILED_PRECONDITION error

#### Scenario: Empty fields are not updated
- **WHEN** a client calls UpdateCampaign with only campaign_id and name (other fields empty/zero)
- **THEN** only the name field SHALL be updated; all other fields remain unchanged
