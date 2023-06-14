#![cfg_attr(not(std), no_std)]

#[cfg(feature = "std")]
extern crate std;

// CAN addresses
const NULL_CAN_ADDRESS: u8 = 0xFF;
const BROADCAST_CAN_ADDRESS: u8 = 0xFF;

// // TODO: Temp allows dead code
// #[allow(dead_code)]
// mod drivers;
pub mod hardware_integration;

pub mod name;

// pub mod control_function;

// pub mod isobus;
// pub use isobus::Isobus;
// pub use isobus::IsobusAddress;

// pub mod iso_11783_3;
// pub mod iso_11783_5;
// pub mod iso_11783_6;
// pub mod iso_11783_7;
