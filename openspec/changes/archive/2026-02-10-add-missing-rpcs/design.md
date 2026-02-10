## Design Decisions

### D1: Request and response message design for CancelCampaign

**Decision:** `CancelCampaignRequest` contains `campaign_id` (string). `CancelCampaignResponse` returns the updated `Campaign`.

**Rationale:** Mirrors `StartCampaignRequest`/`StartCampaignResponse` pattern. Returning the campaign lets the caller confirm the status transition without a follow-up `GetCampaign` call.

### D2: Request and response message design for ListDeliveries

**Decision:** `ListDeliveriesRequest` contains `campaign_id` (string), `status_filter` (DeliveryStatus, optional), and `pagination` (Pagination). `ListDeliveriesResponse` returns `deliveries` (repeated Delivery) and `pagination_meta` (PaginationMeta). A new `Delivery` message is added with: `id`, `user_id`, `campaign_id`, `status`, `delivered_at`, `read_at`, `acted_at`.

**Rationale:** The pidgr-api spec requires status filtering and pagination. A dedicated `Delivery` message (separate from `InboxEntry`) is needed because `ListDeliveries` is an admin-facing view showing all users' deliveries for a campaign, while `InboxEntry` is a user-facing inbox item with embedded message content. Using `DeliveryStatus` enum (already in `common.proto`) for filtering keeps things consistent.

### D3: Request and response message design for GetMessage

**Decision:** `GetMessageRequest` contains `delivery_id` (string). `GetMessageResponse` returns an `InboxEntry`.

**Rationale:** Reuses the existing `InboxEntry` message which already contains the full message content, delivery status, and read status — exactly what the pidgr-api spec requires. No new message type needed.

### D4: Additive proto changes

**Decision:** Add new messages and RPCs to existing proto files. No changes to existing messages or RPCs.

**Rationale:** Purely additive — no breaking changes, no version bump required for existing consumers.
