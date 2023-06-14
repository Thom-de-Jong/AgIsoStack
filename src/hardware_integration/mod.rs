
use core::fmt::Display;

mod can_driver_trait;
pub use can_driver_trait::CanDriverTrait;
mod time_driver_trait;
pub use time_driver_trait::TimeDriverTrait;

// Selected CAN driver
#[cfg(feature = "mock_can_driver")]
mod mock_can_driver;
#[cfg(feature = "mock_can_driver")]
pub use mock_can_driver::MockCanDriver as CanDriver;

#[cfg(feature = "socket_can_driver")]
mod socket_can_driver;
#[cfg(feature = "socket_can_driver")]
pub use socket_can_driver::SocketCanDriver as CanDriver;

#[cfg(feature = "peak_can_driver")]
mod peak_can_driver;
#[cfg(feature = "peak_can_driver")]
pub use peak_can_driver::PeakCanDriver as CanDriver;

// Selected Time Driver
#[cfg(feature = "std_time_driver")]
mod std_time_driver;
pub use std_time_driver::StdTimeDriver as TimeDriver;


// TODO: Implement embedded-can

#[derive(Debug, PartialEq, Eq)]
pub struct CanFrame {
    id: Id,
    dlc: usize,
    data: [u8; 8],
}

impl CanFrame {
    pub fn new(id: impl Into<Id>, data: &[u8]) -> Self {
        let id = id.try_into().unwrap_or(Id::Extended(ExtendedId::MAX));
        let dlc = usize::min(data.len(), 8);
        let mut temp_data: [u8; 8] = [0x0; 8];

        temp_data[..dlc].clone_from_slice(data);

        Self {
            id,
            dlc,
            data: temp_data,
        }
    }

    pub fn is_extended(&self) -> bool {
        match self.id {
            Id::Standard(_) => false,
            Id::Extended(_) => true,
        }
    }

    pub fn id(&self) -> Id {
        self.id
    }

    pub fn dlc(&self) -> usize {
        self.dlc
    }

    pub fn data(&self) -> &[u8] {
        &self.data
    }
}

impl Display for CanFrame {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(
            f,
            "0x{:08X}, {}, {:02X?}",
            self.id().as_raw(),
            self.dlc(),
            &self.data
        )
    }
}

impl Default for CanFrame {
    fn default() -> Self {
        CanFrame::new(Id::Extended(ExtendedId::MAX), &[])
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
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
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
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
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Id {
    /// Standard 11-bit Identifier (`0..=0x7FF`).
    Standard(StandardId),

    /// Extended 29-bit Identifier (`0..=0x1FFF_FFFF`).
    Extended(ExtendedId),
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
