#![cfg_attr(not(std), no_std)]

#[cfg(feature = "std")]
extern crate std;

extern crate alloc;

// Re-export can data types
mod can_id;
pub use can_id::{Id, StandardId, ExtendedId};
mod can_frame;
pub use can_frame::CanFrame;

// Re-export low level Isobus types
mod can_message;
pub use can_message::CanMessage;
pub use can_message::CanMessageTrait;
pub mod name;
pub mod control_function;
mod parameter_group_numbers;
pub use parameter_group_numbers::ParameterGroupNumber;


pub mod hardware_integration;

// TODO: Decide if object pool manipulation is needed in de base library
// Should it work in no_std?
mod objects;
pub mod object_pool;
pub use objects::ObjectId;


pub mod virtual_terminal_client;
// pub use virtual_terminal_client::VirtualTerminalClient;


// mod transport_protocol_manager;
// mod extended_transport_protocol_manager;
mod can_network_manager;
pub use can_network_manager::CanNetworkManager;





/// Defines all the CAN frame priorities that can be encoded in a frame ID
#[repr(u8)]
#[derive(Debug, PartialEq)]
pub enum CanPriority {
	PriorityHighest0 = 0, //< Highest CAN priority
	Priority1 = 1, //< Priority highest - 1
	Priority2 = 2, //< Priority highest - 2
	Priority3 = 3, //< Priority highest - 3 (Control messages priority)
	Priority4 = 4, //< Priority highest - 4
	Priority5 = 5, //< Priority highest - 5
	PriorityDefault6 = 6, //< The default priority
	PriorityLowest7 = 7, //< The lowest priority
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct Address(pub u8);
impl Address {
    pub const NULL: Address = Address(0xFE);
    pub const GLOBAL: Address = Address(0xFF);
}
impl Default for Address {
    fn default() -> Self {
        Self::NULL
    }
}
impl From<u8> for Address {
    fn from(val: u8) -> Self {
        Address(val)
    }
}
impl From<Address> for u8 {
    fn from(val: Address) -> Self {
        val.0
    }
}
impl core::fmt::Display for Address {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_fmt(format_args!("{}", self.0))
    }
}
