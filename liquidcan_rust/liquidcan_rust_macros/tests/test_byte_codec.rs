use liquidcan_rust_macros::byte_codec::{ByteCodec, DeserializationError};
use zerocopy::{Immutable, IntoBytes, TryFromBytes};

#[repr(C)]
#[derive(TryFromBytes, IntoBytes, Immutable, Clone, Copy, Debug, PartialEq, Eq)]
struct CanFramePayload {
	arbitration_id: u16,
	data: u16,
}

#[repr(u8)]
#[derive(ByteCodec, Debug, PartialEq, Eq)]
enum CanMessage {
	Heartbeat = 0x01,
	Frame(CanFramePayload) = 0x02,
	Command { cmd: u16, value: u16 } = 0x03,
	Empty = 0x04,
}

#[repr(u8)]
#[derive(TryFromBytes, IntoBytes, Immutable, Clone, Copy, Debug, PartialEq, Eq)]
enum CheckedByte {
	A = 0x11,
	B = 0x22,
}

#[derive(ByteCodec, Debug, PartialEq, Eq)]
struct NamedPacket {
	port: u16,
	payload: CanFramePayload,
}

#[derive(ByteCodec, Debug, PartialEq, Eq)]
struct TuplePacket(u16, CanFramePayload);

#[derive(ByteCodec, Debug, PartialEq, Eq)]
struct Marker;

#[test]
fn serializes_to_variable_lengths() {
	let mut heartbeat = Vec::new();
	CanMessage::Heartbeat.serialize(&mut heartbeat);

	let mut frame = Vec::new();
	CanMessage::Frame(CanFramePayload {
		arbitration_id: 0x1234,
		data: 0xABCD,
	})
	.serialize(&mut frame);

	let mut command = Vec::new();
	CanMessage::Command {
		cmd: 0x0102,
		value: 0x0304,
	}
	.serialize(&mut command);

	assert_eq!(heartbeat.len(), 1);
	assert_eq!(frame.len(), 1 + core::mem::size_of::<CanFramePayload>());
	assert_eq!(command.len(), 1 + core::mem::size_of::<u16>() * 2);
}

#[test]
fn roundtrips_all_variant_shapes() {
	let original = [
		CanMessage::Heartbeat,
		CanMessage::Frame(CanFramePayload {
			arbitration_id: 0x0001,
			data: 0xCAFE,
		}),
		CanMessage::Command {
			cmd: 0xBEEF,
			value: 0x2222,
		},
		CanMessage::Empty,
	];

	for value in original {
		let mut bytes = Vec::new();
		value.serialize(&mut bytes);

		let (decoded, rest) = CanMessage::deserialize(&bytes).expect("deserialize should succeed");
		assert_eq!(decoded, value);
		assert!(rest.is_empty());
	}
}

#[test]
fn fails_for_invalid_discriminant() {
	let err = CanMessage::deserialize(&[0xFF]).expect_err("expected invalid discriminant");
	assert!(matches!(
		err,
		DeserializationError::InvalidDiscriminant(0xFF)
	));
}

#[test]
fn fails_on_empty_input() {
	let err = CanMessage::deserialize(&[]).expect_err("expected not enough data");
	assert!(matches!(err, DeserializationError::NotEnoughData));
}

#[test]
fn fails_when_variant_payload_is_truncated() {
	let err = CanMessage::deserialize(&[0x03, 0xAA]).expect_err("expected not enough data");
	assert!(matches!(err, DeserializationError::NotEnoughData));
}

#[test]
fn zerocopy_base_case_roundtrip() {
	let payload = CanFramePayload {
		arbitration_id: 0x1357,
		data: 0x2468,
	};

	let mut bytes = Vec::new();
	payload.serialize(&mut bytes);
	assert_eq!(bytes.len(), core::mem::size_of::<CanFramePayload>());

	let (decoded, rest) = CanFramePayload::deserialize(&bytes).expect("deserialize should succeed");
	assert_eq!(decoded, payload);
	assert!(rest.is_empty());
}

#[test]
fn zerocopy_base_case_reports_invalid_data() {
	let err = CheckedByte::deserialize(&[0xFF]).expect_err("expected invalid data");
	assert!(matches!(err, DeserializationError::InvalidData(_)));
}

#[test]
fn zerocopy_checked_enum_roundtrip() {
	let mut bytes = Vec::new();
	CheckedByte::A.serialize(&mut bytes);

	let (decoded, rest) = CheckedByte::deserialize(&bytes).expect("deserialize should succeed");
	assert_eq!(decoded, CheckedByte::A);
	assert!(rest.is_empty());

	let mut bytes_b = Vec::new();
	CheckedByte::B.serialize(&mut bytes_b);
	let (decoded_b, rest_b) = CheckedByte::deserialize(&bytes_b).expect("deserialize should succeed");
	assert_eq!(decoded_b, CheckedByte::B);
	assert!(rest_b.is_empty());
}

#[test]
fn struct_named_roundtrip() {
	let packet = NamedPacket {
		port: 0x55AA,
		payload: CanFramePayload {
			arbitration_id: 0x1234,
			data: 0x5678,
		},
	};

	let mut bytes = Vec::new();
	packet.serialize(&mut bytes);

	let (decoded, rest) = NamedPacket::deserialize(&bytes).expect("deserialize should succeed");
	assert_eq!(decoded, packet);
	assert!(rest.is_empty());
}

#[test]
fn struct_tuple_roundtrip() {
	let packet = TuplePacket(
		0x0A0B,
		CanFramePayload {
			arbitration_id: 0x0C0D,
			data: 0x0E0F,
		},
	);

	let mut bytes = Vec::new();
	packet.serialize(&mut bytes);

	let (decoded, rest) = TuplePacket::deserialize(&bytes).expect("deserialize should succeed");
	assert_eq!(decoded, packet);
	assert!(rest.is_empty());
}

#[test]
fn struct_unit_roundtrip_preserves_remaining_bytes() {
	let marker = Marker;
	let mut bytes = Vec::new();
	marker.serialize(&mut bytes);
	assert!(bytes.is_empty());

	let input = [0xAA, 0xBB];
	let (decoded, rest) = Marker::deserialize(&input).expect("deserialize should succeed");
	assert_eq!(decoded, Marker);
	assert_eq!(rest, &input);
}

#[test]
fn struct_deserialize_truncated_field_fails() {
	let err = NamedPacket::deserialize(&[0x01]).expect_err("expected not enough data");
	assert!(matches!(err, DeserializationError::NotEnoughData));
}
