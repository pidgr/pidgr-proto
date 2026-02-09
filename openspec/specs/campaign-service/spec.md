## ADDED Requirements

### Requirement: CampaignService proto definition
The system SHALL define a CampaignService in `pidgr/v1/campaign.proto` with RPCs: CreateCampaign, StartCampaign, GetCampaign, ListCampaigns.

#### Scenario: Service definition exists
- **WHEN** the proto file is compiled
- **THEN** it SHALL produce a valid CampaignService with all four RPCs

### Requirement: Campaign message type
The system SHALL define a Campaign message with fields: id (string), name (string), template_id (string), template_version (int32), audience_snapshot_ref (string), status (CampaignStatus), workflow (WorkflowDefinition), total_recipients (int32), action_completed_count (int32), missed_count (int32), created_at (Timestamp), started_at (Timestamp), completed_at (Timestamp).

#### Scenario: Campaign with pinned template version
- **WHEN** a campaign is created with template_version=3
- **THEN** the Campaign message SHALL store template_version as 3

#### Scenario: Campaign with workflow definition
- **WHEN** a campaign is created with a custom WorkflowDefinition
- **THEN** the Campaign message SHALL store the full workflow DAG

### Requirement: CreateCampaign RPC
The system SHALL define CreateCampaign(CreateCampaignRequest) returning CreateCampaignResponse. CreateCampaignRequest SHALL contain: name (string), template_id (string), template_version (int32, 0 means latest), user_ids (repeated string), workflow (WorkflowDefinition, optional — uses org default if omitted). CreateCampaignResponse SHALL contain the created Campaign.

#### Scenario: Create campaign with explicit workflow
- **WHEN** a client calls CreateCampaign with a WorkflowDefinition
- **THEN** the response SHALL contain a Campaign with the provided workflow

#### Scenario: Create campaign without workflow
- **WHEN** a client calls CreateCampaign without a WorkflowDefinition
- **THEN** the server SHALL use the organization's default_workflow

#### Scenario: Create campaign with template_version 0
- **WHEN** a client calls CreateCampaign with template_version=0
- **THEN** the server SHALL resolve and pin the latest version of the template

### Requirement: StartCampaign RPC
The system SHALL define StartCampaign(StartCampaignRequest) returning StartCampaignResponse. StartCampaignRequest SHALL contain: campaign_id (string). StartCampaignResponse SHALL contain the updated Campaign.

#### Scenario: Start a created campaign
- **WHEN** a client calls StartCampaign on a campaign with status CREATED
- **THEN** the server SHALL freeze the audience snapshot in S3, transition status to RUNNING, and trigger the Temporal workflow

#### Scenario: Start an already running campaign
- **WHEN** a client calls StartCampaign on a campaign with status RUNNING
- **THEN** the server SHALL return a FAILED_PRECONDITION error

### Requirement: GetCampaign RPC
The system SHALL define GetCampaign(GetCampaignRequest) returning GetCampaignResponse. GetCampaignRequest SHALL contain: campaign_id (string). GetCampaignResponse SHALL contain the Campaign with current aggregated metrics.

#### Scenario: Get existing campaign
- **WHEN** a client calls GetCampaign with a valid campaign_id
- **THEN** the response SHALL contain the Campaign with up-to-date total_recipients, action_completed_count, and missed_count

#### Scenario: Get non-existent campaign
- **WHEN** a client calls GetCampaign with an invalid campaign_id
- **THEN** the server SHALL return a NOT_FOUND error

### Requirement: ListCampaigns RPC
The system SHALL define ListCampaigns(ListCampaignsRequest) returning ListCampaignsResponse. ListCampaignsRequest SHALL contain: pagination (Pagination). ListCampaignsResponse SHALL contain: campaigns (repeated Campaign), pagination_meta (PaginationMeta).

#### Scenario: List campaigns with pagination
- **WHEN** a client calls ListCampaigns with page_size=10
- **THEN** the response SHALL contain at most 10 campaigns and a next_page_token if more exist
