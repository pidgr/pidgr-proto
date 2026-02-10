## ADDED Requirements

### Requirement: CancelCampaign RPC
The system SHALL define CancelCampaign(CancelCampaignRequest) returning CancelCampaignResponse. CancelCampaignRequest SHALL contain: campaign_id (string). CancelCampaignResponse SHALL contain the updated Campaign.

#### Scenario: Cancel a running campaign
- **WHEN** a client calls CancelCampaign for a campaign in RUNNING status
- **THEN** the response SHALL contain the Campaign with status CAMPAIGN_STATUS_CANCELLED

#### Scenario: Cancel a created campaign
- **WHEN** a client calls CancelCampaign for a campaign in CREATED status
- **THEN** the response SHALL contain the Campaign with status CAMPAIGN_STATUS_CANCELLED

#### Scenario: Cancel a completed or already cancelled campaign
- **WHEN** a client calls CancelCampaign for a campaign in COMPLETED or CANCELLED status
- **THEN** the server SHALL return a FAILED_PRECONDITION error

### Requirement: Delivery message type
The system SHALL define a Delivery message with fields: id (string), user_id (string), campaign_id (string), status (DeliveryStatus), delivered_at (Timestamp), read_at (Timestamp), acted_at (Timestamp).

#### Scenario: Pending delivery
- **WHEN** a delivery has been created but not yet sent
- **THEN** the Delivery SHALL have status DELIVERY_STATUS_PENDING and empty timestamps for delivered_at, read_at, acted_at

#### Scenario: Completed delivery
- **WHEN** a user has acknowledged a delivery
- **THEN** the Delivery SHALL have status DELIVERY_STATUS_ACKED with all timestamps populated

### Requirement: ListDeliveries RPC
The system SHALL define ListDeliveries(ListDeliveriesRequest) returning ListDeliveriesResponse. ListDeliveriesRequest SHALL contain: campaign_id (string), status_filter (DeliveryStatus, optional — 0/UNSPECIFIED means no filter), pagination (Pagination). ListDeliveriesResponse SHALL contain: deliveries (repeated Delivery), pagination_meta (PaginationMeta).

#### Scenario: List all deliveries for a campaign
- **WHEN** a client calls ListDeliveries with a valid campaign_id and no status filter
- **THEN** the response SHALL contain all delivery records for that campaign with pagination

#### Scenario: Filter deliveries by status
- **WHEN** a client calls ListDeliveries with status_filter DELIVERY_STATUS_MISSED
- **THEN** the response SHALL contain only delivery records with status MISSED

#### Scenario: Campaign with no deliveries
- **WHEN** a client calls ListDeliveries for a campaign that hasn't been started
- **THEN** the response SHALL contain an empty deliveries list

## MODIFIED Requirements

### Requirement: CampaignService proto definition
The system SHALL define a CampaignService in `pidgr/v1/campaign.proto` with RPCs: CreateCampaign, StartCampaign, GetCampaign, ListCampaigns, CancelCampaign, ListDeliveries.

#### Scenario: Service definition exists
- **WHEN** the proto file is compiled
- **THEN** it SHALL produce a valid CampaignService with all six RPCs
