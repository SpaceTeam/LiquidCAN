pub use liquidcan_rust_macros_derive::ByteCodec;
use thiserror::Error;
use zerocopy::{Immutable, IntoBytes, TryFromBytes};

/// A trait for types that can be serialized to and deserialized from bytes.
pub trait ByteCodec {

    /// The maximum number of bytes required to serialize this type.
    const MAX_SERIALIZED_SIZE: usize;

    /// Serializes `self` into the provided output buffer.
    /// 
    /// The caller is responsible for ensuring that the buffer has enough capacity to hold the serialized data.
    /// Implementations of this method must not write more than `MAX_SERIALIZED_SIZE` bytes to the output buffer.
    fn serialize(&self, out: &mut Vec<u8>);

    /// Deserializes an instance of `Self` from the provided input bytes.
    /// 
    /// Returns a tuple containing the deserialized instance and a slice of the remaining input bytes after the deserialized data.
    /// Thus, implementations must know how many bytes they consume from the input, in case the input contains more data than needed to deserialize an instance of `Self`.
    /// The caller is responsible for ensuring that the input bytes contain enough data to deserialize an instance of `Self`.
    fn deserialize(input: &[u8]) -> Result<(Self, &[u8]), DeserializationError>
    where
        Self: Sized;
}

impl<T> ByteCodec for T
where
    T: TryFromBytes + IntoBytes + Immutable + Sized,
{
    const MAX_SERIALIZED_SIZE: usize = core::mem::size_of::<T>();

    fn serialize(&self, out: &mut Vec<u8>) {
        out.extend_from_slice(self.as_bytes());
    }

    fn deserialize(input: &[u8]) -> Result<(Self, &[u8]), DeserializationError> {
        let size = Self::MAX_SERIALIZED_SIZE;
        if input.len() < size {
            return Err(DeserializationError::NotEnoughData);
        }

        T::try_read_from_prefix(input)
            .map_err(|err| DeserializationError::InvalidData(format!("{err:?}")))
    }
}


#[non_exhaustive]
#[derive(Debug, Error, PartialEq, Eq)]
pub enum DeserializationError {
    #[error("invalid enum discriminant: {0}")]
    InvalidDiscriminant(u8),
    #[error("not enough input data")]
    NotEnoughData,
    #[error("input bytes are not a valid value for target type")]
    InvalidData(String),
}
