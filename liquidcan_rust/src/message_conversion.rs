use crate::can_message::CanMessage;
use liquidcan_rust_macros::byte_codec::ByteCodec;
use socketcan::EmbeddedFrame;

impl TryFrom<socketcan::CanFdFrame> for CanMessage {
    type Error = anyhow::Error;

    fn try_from(frame: socketcan::CanFdFrame) -> Result<Self, Self::Error> {
        let frame_data = frame.data();
        let (message, _) = CanMessage::deserialize(frame_data)?;
        Ok(message)
    }
}

impl From<CanMessage> for socketcan::CanFdFrame {
    fn from(msg: CanMessage) -> Self {
        let mut buf = Vec::with_capacity(64);
        msg.serialize(&mut buf);

        // ID needs to be set at a later point
        let id = socketcan::StandardId::ZERO;

        socketcan::CanFdFrame::new(id, &buf).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use crate::can_message::CanMessage;
    use crate::payloads;
    use socketcan::EmbeddedFrame;

    fn test_round_trip(msg: CanMessage) {
        let can_data: socketcan::CanFdFrame = msg.clone().into();
        let msg_back: CanMessage = can_data
            .try_into()
            .expect("Failed to convert back to Command");
        assert_eq!(msg, msg_back);
    }

    fn test_round_trip_lossy(msg: CanMessage) {
        let can_data: socketcan::CanFdFrame = msg.into();
        let msg_back: CanMessage = can_data
            .try_into()
            .expect("Failed to convert back to Command");

        // For payloads where type metadata is absent, decode is intentionally lossy.
        // Assert canonical wire round-tripping instead of strict AST equality.
        let can_data_back: socketcan::CanFdFrame = msg_back.clone().into();
        assert_eq!(
            can_data.data(),
            can_data_back.data(),
            "encoded bytes must be stable after one decode/encode cycle"
        );

        let msg_back_again: CanMessage = can_data_back
            .try_into()
            .expect("Failed to convert canonical bytes back to Command");
        assert_eq!(msg_back, msg_back_again);
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
            device_name: "Test".into(),
        };
        let msg = CanMessage::NodeInfoAnnouncement { payload };
        test_round_trip(msg);
    }

    #[test]
    fn test_info_status() {
        let payload = payloads::StatusPayload {
            msg: "Info status message".into(),
        };
        let msg = CanMessage::InfoStatus { payload };
        test_round_trip(msg);
    }

    #[test]
    fn test_warning_status() {
        let payload = payloads::StatusPayload {
            msg: "Warning status message".into(),
        };
        let msg = CanMessage::WarningStatus { payload };
        test_round_trip(msg);
    }

    #[test]
    fn test_error_status() {
        let payload = payloads::StatusPayload {
            msg: "Error status message".into(),
        };
        let msg = CanMessage::ErrorStatus { payload };
        test_round_trip(msg);
    }

    #[test]
    fn test_telemetry_value_registration() {
        let payload = payloads::FieldRegistrationPayload {
            field_id: 5,
            field_type: payloads::CanDataType::UInt16,
            field_name: "Telemetry Value Field".into(),
        };
        let msg = CanMessage::TelemetryValueRegistration { payload };
        test_round_trip(msg);
    }

    #[test]
    fn test_parameter_registration() {
        let payload = payloads::FieldRegistrationPayload {
            field_id: 7,
            field_type: payloads::CanDataType::Boolean,
            field_name: "Parameter Field".into(),
        };
        let msg = CanMessage::ParameterRegistration { payload };
        test_round_trip(msg);
    }

    #[test]
    fn test_telemetry_group_definition() {
        let payload = payloads::TelemetryGroupDefinitionPayload {
            group_id: 3,
            field_ids: [0xFA; 62].as_slice().try_into().unwrap(),
        };
        let msg = CanMessage::TelemetryGroupDefinition { payload };
        test_round_trip(msg);
    }

    #[test]
    fn test_telemetry_group_update() {
        let data_values = [
            payloads::CanDataValue::Int32(42),
            payloads::CanDataValue::Float32(3.14),
            payloads::CanDataValue::Boolean(true),
        ];
        let payload = payloads::TelemetryGroupUpdatePayload {
            group_id: 4,
            values: data_values.as_slice().try_into().unwrap(),
        };
        let msg = CanMessage::TelemetryGroupUpdate { payload };
        test_round_trip_lossy(msg);
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
            value: payloads::CanDataValue::Int32(67),
        };
        let msg = CanMessage::ParameterSetReq { payload };
        test_round_trip_lossy(msg);
    }

    #[test]
    fn test_parameter_set_confirmation() {
        let payload = payloads::ParameterSetConfirmationPayload {
            parameter_id: 11,
            status: payloads::ParameterSetStatus::Success,
            value: payloads::CanDataValue::Float32(42.0),
        };
        let msg = CanMessage::ParameterSetConfirmation { payload };
        test_round_trip_lossy(msg);
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
        let payload = payloads::ParameterSetLockPayload {
            parameter_id: 13,
            parameter_lock: payloads::ParameterLockStatus::Unlocked,
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
            value: payloads::CanDataValue::Boolean(true),
        };
        let msg = CanMessage::FieldGetRes { payload };
        test_round_trip_lossy(msg);
    }

    #[test]
    fn test_field_id_lookup_req() {
        let payload = payloads::FieldIDLookupReqPayload {
            field_name: "Test Field Name".into(),
        };
        let msg = CanMessage::FieldIDLookupReq { payload };
        test_round_trip(msg);
    }

    #[test]
    fn test_field_id_lookup_res() {
        let payload = payloads::FieldIDLookupResPayload {
            field_id: 22,
            field_type: payloads::CanDataType::Float32,
        };
        let msg = CanMessage::FieldIDLookupRes { payload };
        test_round_trip(msg);
    }

    #[test]
    fn test_invalid_message_type() {
        // Create a frame with an invalid message type (255 is not defined)
        let frame = socketcan::CanFdFrame::new(socketcan::StandardId::ZERO, &[255]).unwrap();

        let result: Result<CanMessage, _> = frame.try_into();
        assert!(result.is_err(), "Expected error for invalid message type");
        let err_msg = result.unwrap_err().to_string();
        assert!(
            err_msg.contains("invalid enum discriminant"),
            "Error message should mention conversion failure: {}",
            err_msg
        );
    }

    #[test]
    fn test_invalid_can_data_type() {
        // Create a FieldRegistration with invalid CanDataType (255 is out of range)
        let frame = socketcan::CanFdFrame::new(
            socketcan::StandardId::ZERO,
            &[
                20, // TelemetryValueRegistration
                5,  // field_id
                255, // Invalid CanDataType
                    // Rest is field_name
            ],
        )
        .unwrap();

        let result: Result<CanMessage, _> = frame.try_into();
        assert!(result.is_err(), "Expected error for invalid CanDataType");
    }

    #[test]
    fn test_invalid_parameter_set_status() {
        // Create a ParameterSetConfirmation with invalid status
        let frame = socketcan::CanFdFrame::new(
            socketcan::StandardId::ZERO,
            &[
                51,  // ParameterSetConfirmation
                10,  // parameter_id
                255, // Invalid ParameterSetStatus
            ],
        )
        .unwrap();
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
        let frame = socketcan::CanFdFrame::new(
            socketcan::StandardId::ZERO,
            &[
                52,  // ParameterSetLockReq
                12,  // parameter_id
                255, // Invalid ParameterLockStatus
            ],
        )
        .unwrap();

        let result: Result<CanMessage, _> = frame.try_into();
        assert!(
            result.is_err(),
            "Expected error for invalid ParameterLockStatus"
        );
    }
}
