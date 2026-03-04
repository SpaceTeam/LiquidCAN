use liquidcan_rust_macros::byte_codec::ByteCodec;

use crate::payloads;

#[derive(Debug, ByteCodec, PartialEq, Clone)]
#[repr(u8)]
pub enum CanMessage {
    // Node Discovery and Information
    NodeInfoReq = 0, // NO payload
    NodeInfoAnnouncement {
        payload: payloads::NodeInfoResPayload,
    } = 1,

    // Status Messages
    InfoStatus {
        payload: payloads::StatusPayload,
    } = 10,
    WarningStatus {
        payload: payloads::StatusPayload,
    } = 11,
    ErrorStatus {
        payload: payloads::StatusPayload,
    } = 12,

    // Field Registration
    TelemetryValueRegistration {
        payload: payloads::FieldRegistrationPayload,
    } = 20,
    ParameterRegistration {
        payload: payloads::FieldRegistrationPayload,
    } = 21,

    // Telemetry Group Management
    TelemetryGroupDefinition {
        payload: payloads::TelemetryGroupDefinitionPayload,
    } = 30,
    TelemetryGroupUpdate {
        payload: payloads::TelemetryGroupUpdatePayload,
    } = 31,

    // Heartbeat
    HeartbeatReq {
        payload: payloads::HeartbeatPayload,
    } = 40,
    HeartbeatRes {
        payload: payloads::HeartbeatPayload,
    } = 41,

    // Parameter Management
    ParameterSetReq {
        payload: payloads::ParameterSetReqPayload,
    } = 50,
    ParameterSetConfirmation {
        payload: payloads::ParameterSetConfirmationPayload,
    } = 51,
    ParameterSetLockReq {
        payload: payloads::ParameterSetLockPayload,
    } = 52,
    ParameterSetLockConfirmation {
        payload: payloads::ParameterSetLockPayload,
    } = 53,

    // Field Access
    FieldGetReq {
        payload: payloads::FieldGetReqPayload,
    } = 60,
    FieldGetRes {
        payload: payloads::FieldGetResPayload,
    } = 61,
    FieldIDLookupReq {
        payload: payloads::FieldIDLookupReqPayload,
    } = 62,
    FieldIDLookupRes {
        payload: payloads::FieldIDLookupResPayload,
    } = 63,
}

static_assertions::const_assert_eq!(CanMessage::MAX_SERIALIZED_SIZE, 64);
