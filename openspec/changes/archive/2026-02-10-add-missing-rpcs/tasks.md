## 1. Campaign Proto Changes

- [x] 1.1 Add `Delivery` message to `campaign.proto` with fields: `id` (string), `user_id` (string), `campaign_id` (string), `status` (DeliveryStatus), `delivered_at` (Timestamp), `read_at` (Timestamp), `acted_at` (Timestamp)
- [x] 1.2 Add `CancelCampaignRequest` message with field: `campaign_id` (string)
- [x] 1.3 Add `CancelCampaignResponse` message with field: `campaign` (Campaign)
- [x] 1.4 Add `ListDeliveriesRequest` message with fields: `campaign_id` (string), `status_filter` (DeliveryStatus), `pagination` (Pagination)
- [x] 1.5 Add `ListDeliveriesResponse` message with fields: `deliveries` (repeated Delivery), `pagination_meta` (PaginationMeta)
- [x] 1.6 Add `CancelCampaign` and `ListDeliveries` RPCs to `CampaignService`

## 2. Inbox Proto Changes

- [x] 2.1 Add `GetMessageRequest` message to `inbox.proto` with field: `delivery_id` (string)
- [x] 2.2 Add `GetMessageResponse` message with field: `entry` (InboxEntry)
- [x] 2.3 Add `GetMessage` RPC to `InboxService`

## 3. Validation & Code Generation

- [x] 3.1 Run `buf build` and `buf lint` to verify
- [x] 3.2 Run `buf generate` to regenerate Go, Rust, and TypeScript stubs
- [x] 3.3 Verify `buf breaking` reports no breaking changes against main
