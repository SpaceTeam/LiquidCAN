use modular_bitfield::prelude::B5;
use modular_bitfield::{Specifier, bitfield};

#[derive(Specifier, Debug, PartialEq, Eq)]
pub enum CanMessagePriority {
    Low = 0,
    High = 1,
}

/// Reserved Node IDs with special meaning.
pub const NODE_ID_INVALID: u8 = 0;
pub const NODE_ID_SERVER: u8 = 1;
pub const NODE_ID_BROADCAST: u8 = 31;

#[bitfield]
#[derive(Debug, Copy, Clone, PartialEq)]
#[repr(u16)]
pub struct CanMessageId {
    pub receiver_id: B5,
    pub sender_id: B5,
    pub priority: CanMessagePriority,
    #[skip]
    __: B5,
}
