use liquidcan_rust_macros::byte_codec::ByteCodec;
use modular_bitfield::Specifier;
use zerocopy_derive::{Immutable, IntoBytes, TryFromBytes};

pub use crate::can_data::{CanDataType, CanDataValue, CanString, NonNullCanBytes, PackedCanDataValues};


#[derive(Specifier, Debug, Copy, Clone, PartialEq, Eq, Immutable, TryFromBytes, IntoBytes)]
#[repr(u8)]
pub enum ParameterSetStatus {
    Success = 0,                // Parameter was successfully set
    InvalidParameterID = 1,     // The parameter ID does not exist
    ParameterLocked = 2,        // The parameter is locked and cannot be modified
    NodeToNodeModification = 3, // The parameter was modified by another node
}

#[derive(Specifier, Debug, Copy, Clone, PartialEq, Eq, Immutable, TryFromBytes, IntoBytes)]
#[repr(u8)]
pub enum ParameterLockStatus {
    Unlocked = 0,
    Locked = 1,
}

#[derive(Debug, Clone, ByteCodec, PartialEq)]
pub struct NodeInfoResPayload {
    pub tel_count: u8,              // Number of telemetryValues on this node
    pub par_count: u8,              // Number of parameters on this node
    pub firmware_hash: u32,         // Hash of the firmware version
    pub liquid_hash: u32,           // Hash of the LiquidCan protocol version
    pub device_name: CanString<53>, // Human-readable device name
}

#[derive(Debug, Clone, ByteCodec, PartialEq)]
pub struct StatusPayload {
    pub msg: CanString<63>, // Status message text
}

#[derive(Debug, Clone, ByteCodec, PartialEq)]
pub struct FieldRegistrationPayload {
    pub field_id: u8,              // Unique identifier for this field
    pub field_type: CanDataType,   // Data type
    pub field_name: CanString<61>, // Human-readable field name
}

#[derive(Debug, Clone, ByteCodec, PartialEq)]
pub struct TelemetryGroupDefinitionPayload {
    pub group_id: u8,        // Unique identifier for this group
    pub field_ids: NonNullCanBytes<62>, // Array of field IDs in this group
}

#[derive(Debug, Clone, ByteCodec, PartialEq)]
pub struct TelemetryGroupUpdatePayload {
    pub group_id: u8,     // Group identifier
    pub values: PackedCanDataValues<62>, // Packed values of all telemetry values in the group
}

#[derive(Debug, Clone, ByteCodec, PartialEq)]
pub struct HeartbeatPayload {
    pub counter: u32, // Incrementing counter value
}

#[derive(Debug, Clone, ByteCodec, PartialEq)]
pub struct ParameterSetReqPayload {
    pub parameter_id: u8, // Parameter identifier
    pub value: CanDataValue,  // New value (type depends on parameter)
}

// Important: only derives TryFromBytes because enum ParameterSetStatus doesn't cover all possible enum variants for u8
#[derive(Debug, Clone, ByteCodec, PartialEq)]
pub struct ParameterSetConfirmationPayload {
    pub parameter_id: u8,           // Parameter identifier
    pub status: ParameterSetStatus, // Status code
    pub value: CanDataValue,        // Confirmed value after set operation
}
#[derive(Specifier, Debug, Copy, Clone, PartialEq, Eq, Immutable, TryFromBytes, IntoBytes)]
#[repr(u8)]
pub enum FieldStatus {
    Ok = 0,
    NotFound = 1,
}

#[derive(Debug, Clone, ByteCodec, PartialEq)]
pub struct FieldGetReqPayload {
    pub field_id: u8, // Field identifier
}

#[derive(Debug, Clone, ByteCodec, PartialEq)]
pub struct FieldGetResPayload {
    pub field_id: u8,              // Field identifier
    pub field_status: FieldStatus, // Status of the get operation
    pub value: CanDataValue,           // Field value
}

#[derive(Debug, Clone, ByteCodec, PartialEq)]
pub struct FieldIDLookupReqPayload {
    pub field_name: CanString<61>, // Field name
}

// Important: only derives TryFromBytes because enum CanDataType doesn't cover all possible enum variants for u8
#[derive(Debug, Clone, ByteCodec, PartialEq)]
pub struct FieldIDLookupResPayload {
    pub field_id: u8,              // Field ID
    pub field_status: FieldStatus, // Status of the lookup operation
    pub field_type: CanDataType,   // Field Datatype
}

// Important: only derives TryFromBytes because bool doesn't derive FromBytes
#[derive(Debug, Clone, ByteCodec, PartialEq)]
pub struct ParameterSetLockPayload {
    pub parameter_id: u8,                    // Parameter identifier to lock
    pub parameter_lock: ParameterLockStatus, // Lock status (0=unlocked, 1=locked)
}
// Important: only derives TryFromBytes because bool doesn't derive FromBytes
#[derive(Debug, Clone, TryFromBytes, IntoBytes, Immutable, PartialEq)]
#[repr(C, packed)]
pub struct ParameterSetLockConfirmationPayload {
    pub parameter_id: u8,                    // Parameter identifier to lock
    pub parameter_lock: ParameterLockStatus, // Lock status (0=unlocked, 1=locked)
    pub field_status: FieldStatus,           // Status of the parameter
}


static_assertions::const_assert_eq!(NodeInfoResPayload::MAX_SERIALIZED_SIZE, 63);
static_assertions::const_assert_eq!(StatusPayload::MAX_SERIALIZED_SIZE, 63);
static_assertions::const_assert_eq!(FieldRegistrationPayload::MAX_SERIALIZED_SIZE, 63);
static_assertions::const_assert_eq!(TelemetryGroupDefinitionPayload::MAX_SERIALIZED_SIZE, 63);
static_assertions::const_assert_eq!(TelemetryGroupUpdatePayload::MAX_SERIALIZED_SIZE, 63);
static_assertions::const_assert_eq!(HeartbeatPayload::MAX_SERIALIZED_SIZE, 4);
static_assertions::const_assert_eq!(ParameterSetReqPayload::MAX_SERIALIZED_SIZE, 5);
static_assertions::const_assert_eq!(ParameterSetConfirmationPayload::MAX_SERIALIZED_SIZE, 6);
static_assertions::const_assert_eq!(FieldGetReqPayload::MAX_SERIALIZED_SIZE, 1);
static_assertions::const_assert_eq!(FieldGetResPayload::MAX_SERIALIZED_SIZE, 6);
static_assertions::const_assert_eq!(FieldIDLookupReqPayload::MAX_SERIALIZED_SIZE, 61);
static_assertions::const_assert_eq!(FieldIDLookupResPayload::MAX_SERIALIZED_SIZE, 3);
static_assertions::const_assert_eq!(ParameterSetLockPayload::MAX_SERIALIZED_SIZE, 2);
