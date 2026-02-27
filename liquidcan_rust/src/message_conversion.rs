use crate::CanMessageFrame;
use crate::can_message::{CanMessage, CanMessagePadded};
use anyhow::anyhow;
use zerocopy::{FromZeros, IntoBytes, TryFromBytes};

impl TryFrom<CanMessageFrame> for CanMessage {
    type Error = anyhow::Error;

    fn try_from(frame: CanMessageFrame) -> Result<Self, Self::Error> {
        let frame_data = frame.as_bytes();
        let padded_msg = CanMessagePadded::try_read_from_bytes(frame_data)
            .map_err(|e| anyhow!("Failed to convert message: {}", e))?;
        let msg: CanMessage = padded_msg.into();
        Ok(msg)
    }
}

impl From<CanMessage> for CanMessageFrame {
    fn from(msg: CanMessage) -> Self {
        let mut msg_frame = CanMessageFrame::new_zeroed();
        let discriminant = msg.discriminant();
        let padded_msg: CanMessagePadded = msg.into();
        // The first byte is the discriminant, which is set separately.
        let bytes: &[u8] = &padded_msg.as_bytes()[1..];
        msg_frame.data[..bytes.len()].copy_from_slice(bytes);
        msg_frame.message_type = discriminant;
        msg_frame
    }
}

#[cfg(test)]
mod tests {
    use crate::CanMessageFrame;
    use crate::can_message::CanMessage;
    use crate::payloads;
    use zerocopy::FromZeros;
    use crate::payloads::FieldStatus;

    fn test_round_trip(msg: CanMessage) {
        let can_data: CanMessageFrame = msg.clone().into();
        let msg_back: CanMessage = can_data
            .try_into()
            .expect("Failed to convert back to Command");
        assert_eq!(msg, msg_back);
    }

    #[test]
    fn test_node_info_req() {
        let msg = CanMessage::NodeInfoReq;
        test_round_trip(msg);
    }

    #[test]
    fn test_node_info_announcement() {
        let payload = payloads::NodeInfoResPayload {
            tel_count: 7,
            par_count: 5,
            firmware_hash: 1234,
            liquid_hash: 5678,
            device_name: [0xAA; 53],
        };
        let msg = CanMessage::NodeInfoAnnouncement { payload };
        test_round_trip(msg);
    }

    #[test]
    fn test_info_status() {
        let payload = payloads::StatusPayload { msg: [0xBB; 63] };
        let msg = CanMessage::InfoStatus { payload };
        test_round_trip(msg);
    }

    #[test]
    fn test_warning_status() {
        let payload = payloads::StatusPayload { msg: [0xCC; 63] };
        let msg = CanMessage::WarningStatus { payload };
        test_round_trip(msg);
    }

    #[test]
    fn test_error_status() {
        let payload = payloads::StatusPayload { msg: [0xDD; 63] };
        let msg = CanMessage::ErrorStatus { payload };
        test_round_trip(msg);
    }

    #[test]
    fn test_telemetry_value_registration() {
        let payload = payloads::FieldRegistrationPayload {
            field_id: 5,
            field_type: payloads::CanDataType::UInt16,
            field_name: [0xEE; 61],
        };
        let msg = CanMessage::TelemetryValueRegistration { payload };
        test_round_trip(msg);
    }

    #[test]
    fn test_parameter_registration() {
        let payload = payloads::FieldRegistrationPayload {
            field_id: 7,
            field_type: payloads::CanDataType::Boolean,
            field_name: [0xFF; 61],
        };
        let msg = CanMessage::ParameterRegistration { payload };
        test_round_trip(msg);
    }

    #[test]
    fn test_telemetry_group_definition() {
        let payload = payloads::TelemetryGroupDefinitionPayload {
            group_id: 3,
            field_ids: [0xFA; 62],
        };
        let msg = CanMessage::TelemetryGroupDefinition { payload };
        test_round_trip(msg);
    }

    #[test]
    fn test_telemetry_group_update() {
        let payload = payloads::TelemetryGroupUpdatePayload {
            group_id: 4,
            values: [0xFB; 62],
        };
        let msg = CanMessage::TelemetryGroupUpdate { payload };
        test_round_trip(msg);
    }

    #[test]
    fn test_heartbeat_req() {
        let payload = payloads::HeartbeatPayload { counter: 17 };
        let msg = CanMessage::HeartbeatReq { payload };
        test_round_trip(msg);
    }

    #[test]
    fn test_heartbeat_res() {
        let payload = payloads::HeartbeatPayload { counter: 18 };
        let msg = CanMessage::HeartbeatRes { payload };
        test_round_trip(msg);
    }

    #[test]
    fn test_parameter_set_req() {
        let payload = payloads::ParameterSetReqPayload {
            parameter_id: 10,
            value: [0xAA; 61],
        };
        let msg = CanMessage::ParameterSetReq { payload };
        test_round_trip(msg);
    }

    #[test]
    fn test_parameter_set_confirmation() {
        let payload = payloads::ParameterSetConfirmationPayload {
            parameter_id: 11,
            status: payloads::ParameterSetStatus::Success,
            value: [0xBB; 61],
        };
        let msg = CanMessage::ParameterSetConfirmation { payload };
        test_round_trip(msg);
    }

    #[test]
    fn test_parameter_set_lock_req() {
        let payload = payloads::ParameterSetLockPayload {
            parameter_id: 12,
            parameter_lock: payloads::ParameterLockStatus::Locked,
        };
        let msg = CanMessage::ParameterSetLockReq { payload };
        test_round_trip(msg);
    }

    #[test]
    fn test_parameter_set_lock_confirmation() {
        let payload = payloads::ParameterSetLockConfirmationPayload {
            parameter_id: 13,
            parameter_lock: payloads::ParameterLockStatus::Unlocked,
            field_status: FieldStatus::Ok,
        };
        let msg = CanMessage::ParameterSetLockConfirmation { payload };
        test_round_trip(msg);
    }

    #[test]
    fn test_field_get_req() {
        let payload = payloads::FieldGetReqPayload { field_id: 20 };
        let msg = CanMessage::FieldGetReq { payload };
        test_round_trip(msg);
    }

    #[test]
    fn test_field_get_res() {
        let payload = payloads::FieldGetResPayload {
            field_id: 21,
            field_status:FieldStatus::Ok,
            value: [0xCC; 61],
        };
        let msg = CanMessage::FieldGetRes { payload };
        test_round_trip(msg);
    }

    #[test]
    fn test_field_id_lookup_req() {
        let payload = payloads::FieldIDLookupReqPayload {
            field_name: [0xDD; 61],
        };
        let msg = CanMessage::FieldIDLookupReq { payload };
        test_round_trip(msg);
    }

    #[test]
    fn test_field_id_lookup_res() {
        let payload = payloads::FieldIDLookupResPayload {
            field_id: 22,
            field_status:FieldStatus::Ok,
            field_type: payloads::CanDataType::Float32,
        };
        let msg = CanMessage::FieldIDLookupRes { payload };
        test_round_trip(msg);
    }

    #[test]
    fn test_invalid_message_type() {
        // Create a frame with an invalid message type (255 is not defined)
        let mut frame = CanMessageFrame::new_zeroed();
        frame.message_type = 255;

        let result: Result<CanMessage, _> = frame.try_into();
        assert!(result.is_err(), "Expected error for invalid message type");
        let err_msg = result.unwrap_err().to_string();
        assert!(
            err_msg.contains("Failed to convert message"),
            "Error message should mention conversion failure: {}",
            err_msg
        );
    }

    #[test]
    fn test_invalid_can_data_type() {
        // Create a FieldRegistration with invalid CanDataType (255 is out of range)
        let mut frame = CanMessageFrame::new_zeroed();
        frame.message_type = 20; // TelemetryValueRegistration
        frame.data[0] = 5; // field_id
        frame.data[1] = 255; // Invalid CanDataType
        // Rest is field_name

        let result: Result<CanMessage, _> = frame.try_into();
        assert!(result.is_err(), "Expected error for invalid CanDataType");
    }

    #[test]
    fn test_invalid_parameter_set_status() {
        // Create a ParameterSetConfirmation with invalid status
        let mut frame = CanMessageFrame::new_zeroed();
        frame.message_type = 51; // ParameterSetConfirmation
        frame.data[0] = 10; // parameter_id
        frame.data[1] = 255; // Invalid ParameterSetStatus
        // Rest is value

        let result: Result<CanMessage, _> = frame.try_into();
        assert!(
            result.is_err(),
            "Expected error for invalid ParameterSetStatus"
        );
    }

    #[test]
    fn test_invalid_parameter_lock_status() {
        // Create a ParameterSetLockReq with invalid lock status
        let mut frame = CanMessageFrame::new_zeroed();
        frame.message_type = 52; // ParameterSetLockReq
        frame.data[0] = 12; // parameter_id
        frame.data[1] = 255; // Invalid ParameterLockStatus

        let result: Result<CanMessage, _> = frame.try_into();
        assert!(
            result.is_err(),
            "Expected error for invalid ParameterLockStatus"
        );
    }
}
