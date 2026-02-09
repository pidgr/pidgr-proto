## ADDED Requirements

### Requirement: DeviceService proto definition
The system SHALL define a DeviceService in `pidgr/v1/device.proto` with RPCs: Register, Deactivate, ListDevices.

#### Scenario: Service definition exists
- **WHEN** the proto file is compiled
- **THEN** it SHALL produce a valid DeviceService with all three RPCs

### Requirement: Device message type
The system SHALL define a Device message with fields: device_id (string), user_id (string), platform (Platform), push_token (string), active (bool), last_seen (Timestamp), created_at (Timestamp).

#### Scenario: Active device
- **WHEN** a device is registered and has a valid push token
- **THEN** the Device SHALL have active=true

#### Scenario: Deactivated device
- **WHEN** a device token is found invalid during push delivery
- **THEN** the Device SHALL have active=false

### Requirement: Register RPC
The system SHALL define Register(RegisterDeviceRequest) returning RegisterDeviceResponse. RegisterDeviceRequest SHALL contain: device_id (string), platform (Platform), push_token (string). RegisterDeviceResponse SHALL contain the Device.

#### Scenario: Register a new device
- **WHEN** a client calls Register with a device_id not previously seen
- **THEN** the server SHALL create a new device record and return it with active=true

#### Scenario: Update an existing device token
- **WHEN** a client calls Register with a device_id that already exists but a new push_token
- **THEN** the server SHALL update the push_token and last_seen timestamp

#### Scenario: Register multiple devices for one user
- **WHEN** a user registers two different device_ids
- **THEN** both devices SHALL be stored and active, associated with the same user

### Requirement: Deactivate RPC
The system SHALL define Deactivate(DeactivateDeviceRequest) returning DeactivateDeviceResponse. DeactivateDeviceRequest SHALL contain: device_id (string). DeactivateDeviceResponse SHALL contain: success (bool).

#### Scenario: Deactivate an active device
- **WHEN** a client calls Deactivate with a valid device_id
- **THEN** the server SHALL set active=false on the device record

#### Scenario: Deactivate an already inactive device
- **WHEN** a client calls Deactivate on an already inactive device
- **THEN** the server SHALL return success=true (idempotent)

### Requirement: ListDevices RPC
The system SHALL define ListDevices(ListDevicesRequest) returning ListDevicesResponse. ListDevicesRequest SHALL contain no fields (user is identified via JWT). ListDevicesResponse SHALL contain: devices (repeated Device).

#### Scenario: User with multiple devices
- **WHEN** a user has 3 registered devices (2 active, 1 inactive)
- **THEN** the response SHALL contain all 3 devices with their respective active states

#### Scenario: User with no devices
- **WHEN** a user has no registered devices
- **THEN** the response SHALL contain an empty devices list
