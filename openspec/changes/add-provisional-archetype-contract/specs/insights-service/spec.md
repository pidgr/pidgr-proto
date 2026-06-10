## ADDED Requirements

### Requirement: Archetype source metadata

`pidgr.v1.Archetype` SHALL carry an `ArchetypeSource source` field (= 11) distinguishing trained ML clustering output (`ARCHETYPE_SOURCE_ML`) from rule-based provisional output (`ARCHETYPE_SOURCE_PROVISIONAL`). Servers SHALL set the field on every archetype they return; clients SHALL treat `ARCHETYPE_SOURCE_UNSPECIFIED` (older servers) as ML output, because provisional output is always explicitly labelled.

#### Scenario: Provisional archetypes are labelled
- **GIVEN** a server returns provisional archetypes for a group
- **WHEN** a client reads `Archetype.source`
- **THEN** every provisional archetype carries `ARCHETYPE_SOURCE_PROVISIONAL`

#### Scenario: Pre-v0.81 server omits the field
- **GIVEN** a response serialized by a server built before this change
- **WHEN** a client reads `Archetype.source`
- **THEN** the field reads `ARCHETYPE_SOURCE_UNSPECIFIED` and the client renders the archetype as ML output without a provisional disclaimer

### Requirement: Response-level confidence metadata

`pidgr.v1.GetGroupArchetypesResponse` SHALL carry a `ConfidenceLevel confidence_level` field (= 4). The server SHALL set it to `CONFIDENCE_LEVEL_LOW` whenever provisional archetypes are returned, so clients can render a low-confidence disclaimer without inspecting each archetype.

#### Scenario: Provisional response is low confidence
- **GIVEN** `GetGroupArchetypes` returns one or more provisional archetypes
- **WHEN** a client reads `confidence_level`
- **THEN** it is `CONFIDENCE_LEVEL_LOW`
