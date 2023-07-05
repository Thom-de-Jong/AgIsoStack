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
#[cfg(feature = "mock_time_driver")]
mod mock_time_driver;
#[cfg(feature = "mock_time_driver")]
pub use mock_time_driver::MockTimeDriver as TimeDriver;
#[cfg(feature = "std_time_driver")]
mod std_time_driver;
#[cfg(feature = "std_time_driver")]
pub use std_time_driver::StdTimeDriver as TimeDriver;
