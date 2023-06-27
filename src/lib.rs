#![cfg_attr(not(std), no_std)]

#[cfg(feature = "std")]
extern crate std;

// Re-export can data types
mod can_frame;
pub use can_frame::CanFrame;

// Re-export low level Isobus types
mod can_message;
pub use can_message::CanMessage;
pub mod name;
pub mod control_function;
mod parameter_group_numbers;
pub use parameter_group_numbers::ParameterGroupNumber;


pub mod hardware_integration;

// TODO: Decide if object pool manipulation is needed in de base library
// Should it work in no_std?
// mod objects;
// pub mod object_pool;
// pub use objects::ObjectId;


pub mod virtual_terminal_client;
// pub use virtual_terminal_client::VirtualTerminalClient;


// mod transport_protocol_manager;
// mod extended_transport_protocol_manager;
mod can_network_manager;
pub use can_network_manager::CanNetworkManager;



#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct StandardId(u32);

impl StandardId {
    /// CAN ID `0`, the highest priority.
    pub const ZERO: Self = StandardId(0);

    /// CAN ID `0x7FF`, the lowest priority.
    pub const MAX: Self = StandardId(0x7FF);

    /// Tries to create a `StandardId` from a raw 32-bit integer.
    ///
    /// This will return `None` if `raw` is out of range of an 11-bit integer (`> 0x7FF`).
    #[inline]
    pub fn new(raw: u32) -> Option<Self> {
        if raw <= 0x7FF {
            Some(StandardId(raw))
        } else {
            None
        }
    }

    /// Creates a new `StandardId` without checking if it is inside the valid range.
    ///
    /// # Safety
    /// Using this method can create an invalid ID and is thus marked as unsafe.
    #[inline]
    pub const unsafe fn new_unchecked(raw: u32) -> Self {
        StandardId(raw)
    }

    /// Returns this CAN Identifier as a raw 32-bit integer.
    #[inline]
    pub fn as_raw(&self) -> u32 {
        self.0
    }
}

/// Extended 29-bit CAN Identifier (`0..=1FFF_FFFF`).
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct ExtendedId(u32);

impl ExtendedId {
    /// CAN ID `0`, the highest priority.
    pub const ZERO: Self = ExtendedId(0);

    /// CAN ID `0x1FFFFFFF`, the lowest priority.
    pub const MAX: Self = ExtendedId(0x1FFF_FFFF);

    /// Tries to create a `ExtendedId` from a raw 32-bit integer.
    ///
    /// This will return `None` if `raw` is out of range of an 29-bit integer (`> 0x1FFF_FFFF`).
    #[inline]
    pub fn new(raw: u32) -> Option<Self> {
        if raw <= 0x1FFF_FFFF {
            Some(ExtendedId(raw))
        } else {
            None
        }
    }

    /// Creates a new `ExtendedId` without checking if it is inside the valid range.
    ///
    /// # Safety
    /// Using this method can create an invalid ID and is thus marked as unsafe.
    #[inline]
    pub const unsafe fn new_unchecked(raw: u32) -> Self {
        ExtendedId(raw)
    }

    /// Returns this CAN Identifier as a raw 32-bit integer.
    #[inline]
    pub fn as_raw(&self) -> u32 {
        self.0
    }

    /// Returns the Base ID part of this extended identifier.
    pub fn standard_id(&self) -> StandardId {
        // ID-28 to ID-18
        StandardId(self.0 >> 18)
    }
}

/// A CAN Identifier (standard or extended).
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub enum Id {
    Standard(StandardId), //< Standard 11-bit Identifier (`0..=0x7FF`).
    Extended(ExtendedId), //< Extended 29-bit Identifier (`0..=0x1FFF_FFFF`).
}

impl Id {
    pub fn as_raw(&self) -> u32 {
        match self {
            Id::Standard(id) => id.as_raw(),
            Id::Extended(id) => id.as_raw(),
        }
    }
}

impl From<StandardId> for Id {
    fn from(id: StandardId) -> Self {
        Id::Standard(id)
    }
}

impl From<ExtendedId> for Id {
    fn from(id: ExtendedId) -> Self {
        Id::Extended(id)
    }
}

impl From<Id> for u32 {
    fn from(id: Id) -> Self {
        id.as_raw()
    }
}

impl From<u32> for Id {
    fn from(id: u32) -> Self {
        if id <= StandardId::MAX.as_raw() {
            return Id::Standard(StandardId(id));
        } else if id <= ExtendedId::MAX.as_raw() {
            return Id::Extended(ExtendedId(id));
        }

        Id::Extended(ExtendedId::MAX)
    }
}


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
