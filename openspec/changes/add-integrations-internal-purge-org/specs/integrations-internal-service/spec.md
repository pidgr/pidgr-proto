## ADDED Requirements

### Requirement: IntegrationsInternalService proto file
The system SHALL define a new `integrations_internal.proto` file in
`proto/pidgr/v1/` with package `pidgr.v1` and the Go package option matching
existing conventions. The file SHALL declare an `IntegrationsInternalService`
that is kept separate from the customer-proxied `IntegrationsService` so that a
destructive platform operation is not mixed with the reachability/dispatch RPCs.

#### Scenario: Proto file compiles
- **WHEN** `buf build` is run
- **THEN** `integrations_internal.proto` compiles without errors

#### Scenario: Service is separate from IntegrationsService
- **WHEN** the proto sources are reviewed
- **THEN** `IntegrationsInternalService` SHALL be declared in `integrations_internal.proto`, NOT added to `IntegrationsService` in `integrations_service.proto`

### Requirement: Internal-mTLS-only auth documentation
The `IntegrationsInternalService` SHALL document that it is internal-mTLS only,
has no customer analog, is never exposed publicly, and is called only by
pidgr-api's staff `ForceDeleteOrg`.

#### Scenario: Auth documentation present
- **WHEN** the service doc comment is reviewed
- **THEN** it SHALL state the service is internal-mTLS only and called only by pidgr-api's staff `ForceDeleteOrg`

### Requirement: PurgeOrg RPC
The system SHALL define a `PurgeOrg(PurgeOrgRequest) returns (PurgeOrgResponse)`
RPC on `IntegrationsInternalService` that purges ALL of an org's integrations
data: the reachability registry, channel dispatches, cost-cap state, and region
policy.

#### Scenario: Request message
- **WHEN** `PurgeOrgRequest` is reviewed
- **THEN** it SHALL contain `string org_id = 1` and `string reason = 2`, where `reason` is a required, non-empty staff justification propagated for audit

#### Scenario: Response message reports per-table delete counts
- **WHEN** `PurgeOrgResponse` is reviewed
- **THEN** it SHALL contain `int64 reachabilities_deleted = 1`, `int64 dispatches_deleted = 2`, and `int64 policies_deleted = 3`, where `policies_deleted` is the combined count of `cost_cap_state` and `region_policy` rows

#### Scenario: Idempotency documented
- **WHEN** the `PurgeOrg` RPC doc comment is reviewed
- **THEN** it SHALL state the RPC is idempotent: purging an org with no remaining data succeeds and returns zero counts

#### Scenario: Auth documentation
- **WHEN** the `PurgeOrg` RPC doc comment is reviewed
- **THEN** it SHALL state the RPC is internal-mTLS only and invoked only by pidgr-api's staff `ForceDeleteOrg`
