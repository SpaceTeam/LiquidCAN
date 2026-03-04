pub use liquidcan_rust_macros_derive::ByteCodec;
use thiserror::Error;
use zerocopy::{Immutable, IntoBytes, TryFromBytes};

pub trait ByteCodec {
    const MAX_SERIALIZED_SIZE: usize;
    fn serialize(&self, out: &mut Vec<u8>);
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
