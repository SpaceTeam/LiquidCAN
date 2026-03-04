use liquidcan_rust_macros::byte_codec::{ByteCodec, DeserializationError};
use modular_bitfield::Specifier;
use zerocopy_derive::{Immutable, IntoBytes, TryFromBytes};

#[derive(Specifier, Debug, Copy, Clone, PartialEq, Eq, Immutable, TryFromBytes, IntoBytes)]
#[repr(u8)]
pub enum CanDataType {
    Float32 = 0,
    Int32 = 1,
    Int16 = 2,
    Int8 = 3,
    UInt32 = 4,
    UInt16 = 5,
    UInt8 = 6,
    Boolean = 7,
}

impl CanDataType {
    pub fn get_size(&self) -> usize {
        match self {
            CanDataType::Float32 => 4,
            CanDataType::Int32 => 4,
            CanDataType::Int16 => 2,
            CanDataType::Int8 => 1,
            CanDataType::UInt32 => 4,
            CanDataType::UInt16 => 2,
            CanDataType::UInt8 => 1,
            CanDataType::Boolean => 1,
        }
    }
}

#[repr(u8)]
#[derive(Debug, Clone, PartialEq)]
pub enum CanDataValue {
    Float32(f32) = 0,
    Int32(i32) = 1,
    Int16(i16) = 2,
    Int8(i8) = 3,
    UInt32(u32) = 4,
    UInt16(u16) = 5,
    UInt8(u8) = 6,
    Boolean(bool) = 7,
    Raw(Vec<u8>) = u8::MAX,
}

impl CanDataValue {
    pub fn convert_from_slice(
        data: &[u8],
        data_type: CanDataType,
    ) -> Result<Self, DeserializationError> {
        if data.len() != data_type.get_size() {
            return Err(DeserializationError::InvalidData(format!(
                "Data length {} does not match expected length {} for type {:?}",
                data.len(),
                data_type.get_size(),
                data_type
            )));
        }

        match data_type {
            CanDataType::Float32 => {
                let mut arr = [0u8; 4];
                arr.copy_from_slice(data);
                Ok(CanDataValue::Float32(f32::from_le_bytes(arr)))
            }
            CanDataType::Int32 => {
                let mut arr = [0u8; 4];
                arr.copy_from_slice(data);
                Ok(CanDataValue::Int32(i32::from_le_bytes(arr)))
            }
            CanDataType::Int16 => {
                let mut arr = [0u8; 2];
                arr.copy_from_slice(data);
                Ok(CanDataValue::Int16(i16::from_le_bytes(arr)))
            }
            CanDataType::Int8 => Ok(CanDataValue::Int8(data[0] as i8)),
            CanDataType::UInt32 => {
                let mut arr = [0u8; 4];
                arr.copy_from_slice(data);
                Ok(CanDataValue::UInt32(u32::from_le_bytes(arr)))
            }
            CanDataType::UInt16 => {
                let mut arr = [0u8; 2];
                arr.copy_from_slice(data);
                Ok(CanDataValue::UInt16(u16::from_le_bytes(arr)))
            }
            CanDataType::UInt8 => Ok(CanDataValue::UInt8(data[0])),
            CanDataType::Boolean => Ok(CanDataValue::Boolean(data[0] != 0)),
        }
    }

    /// Convert a Raw CanDataValue into a strongly-typed CanDataValue based on the provided CanDataType.
    ///
    /// Since the data type is not known at the time of deserialization, we initially deserialize into a Raw variant containing the raw bytes.
    pub fn convert_from_raw(
        &self,
        data_type: CanDataType,
    ) -> Result<CanDataValue, DeserializationError> {
        let Self::Raw(raw_data) = self else {
            return Err(DeserializationError::InvalidData(
                "CanDataValue is not a Raw variant".to_string(),
            ));
        };

        CanDataValue::convert_from_slice(raw_data, data_type)
    }
}

impl ByteCodec for CanDataValue {
    fn serialize(&self, out: &mut Vec<u8>) {
        // don't include the tag in the serialized data, as message type must be known at deserialization time
        match self {
            CanDataValue::Float32(v) => {
                out.extend_from_slice(&v.to_le_bytes());
            }
            CanDataValue::Int32(v) => {
                out.extend_from_slice(&v.to_le_bytes());
            }
            CanDataValue::Int16(v) => {
                out.extend_from_slice(&v.to_le_bytes());
            }
            CanDataValue::Int8(v) => {
                out.push(*v as u8);
            }
            CanDataValue::UInt32(v) => {
                out.extend_from_slice(&v.to_le_bytes());
            }
            CanDataValue::UInt16(v) => {
                out.extend_from_slice(&v.to_le_bytes());
            }
            CanDataValue::UInt8(v) => {
                out.push(*v);
            }
            CanDataValue::Boolean(v) => {
                out.push(*v as u8);
            }
            CanDataValue::Raw(data) => {
                out.extend_from_slice(&data);
            }
        }
    }

    fn deserialize(input: &[u8]) -> Result<(Self, &[u8]), DeserializationError> {
        if input.is_empty() {
            return Err(DeserializationError::InvalidData("Empty input".to_string()));
        }

        // Deserialization of CanDataValue requires external knowledge of the expected data type,
        // so we can't determine the variant from the input data alone.
        // Return the raw bytes and let the caller interpret them based on the expected data type.
        Ok((CanDataValue::Raw(input.to_vec()), &[]))
    }
}

/// Custom string type for CAN messages, fixed size of N bytes,
/// null terminated (i.e. at most N-1 non-null bytes), ascii-only.
#[derive(Debug, Clone)]
pub struct CanString<const N: usize> {
    data: [u8; N],
}

impl<const N: usize> From<[u8; N]> for CanString<N> {
    fn from(data: [u8; N]) -> Self {
        // Ensure the string is null-terminated
        assert!(
            data.iter().position(|&b| b == 0).is_some(),
            "CanString must be null-terminated."
        );
        // Ensure all characters are ASCII
        assert!(
            data.iter().all(|&b| b.is_ascii()),
            "CanString must contain only ASCII characters."
        );
        CanString { data }
    }
}

impl<const N: usize> From<&str> for CanString<N> {
    fn from(s: &str) -> Self {
        let bytes = s.as_bytes();
        assert!(bytes.len() < N, "String too long for CanString<{}>", N);

        let mut data = [0u8; N];
        data[..bytes.len()].copy_from_slice(bytes);

        data.into()
    }
}

impl<const N: usize> PartialEq for CanString<N> {
    fn eq(&self, other: &Self) -> bool {
        let self_len = self.data.iter().position(|&b| b == 0).unwrap();
        let other_len = other.data.iter().position(|&b| b == 0).unwrap();
        self_len == other_len && self.data[..self_len] == other.data[..other_len]
    }
}

impl<const N: usize> ByteCodec for CanString<N> {
    fn serialize(&self, out: &mut Vec<u8>) {
        // Write bytes up to the null terminator
        let length = self
            .data
            .iter()
            .position(|&b| b == 0)
            .expect("CanString must be null-terminated.");
        out.extend_from_slice(&self.data[..length]);

        out.push(0); // Null terminator
    }

    fn deserialize(input: &[u8]) -> Result<(Self, &[u8]), DeserializationError> {
        if let Some(pos) = input.iter().position(|&b| b == 0) {
            if pos < N {
                let mut data = [0u8; N];
                data[..pos].copy_from_slice(&input[..pos]);

                Ok((data.into(), &input[pos + 1..]))
            } else {
                Err(DeserializationError::InvalidData(format!(
                    "CanString exceeds maximum length of {}",
                    N
                )))
            }
        } else {
            Err(DeserializationError::InvalidData(
                "CanString is not null-terminated".to_string(),
            ))
        }
    }
}

/// Represents a packed set of CAN data values.
/// The raw byte form must fit into N bytes.
#[derive(Clone, Debug, PartialEq)]
pub struct PackedCanDataValues<const N: usize> {
    data: Vec<u8>,
}

impl<const N: usize> PackedCanDataValues<N> {
    /// Unpack the raw byte data into a vector of CanDataValue based on the provided data types.
    /// The caller must ensure that the order and types of the data match what was originally packed.
    pub fn unpack(&self, data_types: &[CanDataType]) -> Result<Vec<CanDataValue>, DeserializationError> {
        let mut values = Vec::new();
        let mut offset = 0;

        for &data_type in data_types {
            let size = data_type.get_size();
            if offset + size > N {
                return Err(DeserializationError::InvalidData(format!(
                    "Not enough data to unpack CanDataValue of type {:?}",
                    data_type
                )));
            }

            let slice = &self.data[offset..offset + size];
            let value = CanDataValue::convert_from_slice(slice, data_type)?;
            values.push(value);
            offset += size;
        }

        Ok(values)
    }
}

impl<const N: usize> TryFrom<&[CanDataValue]> for PackedCanDataValues<N> {
    type Error = String;

    fn try_from(values: &[CanDataValue]) -> Result<Self, Self::Error> {
        let mut data = Vec::new();
        for value in values {
            value.serialize(&mut data);
        }
        if data.len() > N {
            return Err(format!(
                "Packed data length {} exceeds maximum of {}",
                data.len(),
                N
            ));
        }
        Ok(PackedCanDataValues { data })
    }
}

impl<const N: usize> ByteCodec for PackedCanDataValues<N> {
    fn serialize(&self, out: &mut Vec<u8>) {
        out.extend_from_slice(&self.data);
    }

    fn deserialize(input: &[u8]) -> Result<(Self, &[u8]), DeserializationError> {
        let len = input.len().min(N);
        let data = input[..len].to_vec();
        Ok((PackedCanDataValues { data }, &input[len..]))
    }
}



/// Up to N bytes that do not include a null byte.
#[derive(Debug, Clone)]
pub struct NonNullCanBytes<const N: usize> {
    data: [u8; N],
}

impl<const N: usize> TryFrom<&[u8]> for NonNullCanBytes<N> {
    type Error = String;

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        if value.len() > N {
            return Err(format!(
                "Input data length {} exceeds maximum of {}",
                value.len(),
                N
            ));
        }
        if value.iter().any(|&b| b == 0) {
            return Err("Input data contains null byte".to_string());
        }
        let mut data = [0u8; N];
        data[..value.len()].copy_from_slice(value);
        Ok(NonNullCanBytes { data })
    }
}

impl<'a, const N: usize> From<&'a NonNullCanBytes<N>> for &'a [u8] {
    fn from(value: &'a NonNullCanBytes<N>) -> Self {
        let len = value.data.iter().position(|&b| b == 0).unwrap_or(N);
        &value.data[..len]
    }
}

impl<const N: usize> ByteCodec for NonNullCanBytes<N> {
    fn serialize(&self, out: &mut Vec<u8>) {
        let len = self.data.iter().position(|&b| b == 0).unwrap_or(N);
        out.extend_from_slice(&self.data[..len]);
    }

    fn deserialize(input: &[u8]) -> Result<(Self, &[u8]), DeserializationError> {
        let len = input
            .iter()
            .position(|&b| b == 0)
            .unwrap_or(input.len())
            .min(N);
        let mut data = [0u8; N];
        data[..len].copy_from_slice(&input[..len]);
        Ok((NonNullCanBytes { data }, &input[len..]))
    }
}

impl<const N: usize> PartialEq for NonNullCanBytes<N> {
    fn eq(&self, other: &Self) -> bool {
        let self_len = self.data.iter().position(|&b| b == 0).unwrap_or(N);
        let other_len = other.data.iter().position(|&b| b == 0).unwrap_or(N);
        self_len == other_len && self.data[..self_len] == other.data[..other_len]
    }
}
