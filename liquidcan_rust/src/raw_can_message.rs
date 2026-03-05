use modular_bitfield::prelude::B5;
use modular_bitfield::{Specifier, bitfield};

#[derive(Specifier, Debug, PartialEq, Eq)]
pub enum CanMessagePriority {
    Low = 0,
    High = 1,
}

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
