
use crate::name::Name;

mod internal_control_function;
pub use internal_control_function::InternalControlFunction;

mod external_control_function;
pub use external_control_function::ExternalControlFunction;

pub enum ControlFunction {
    Internal(InternalControlFunction), //< The control function is part of our stack and can address claim.
    External(ExternalControlFunction), //< The control function is some other device on the bus.
    Partnered(ExternalControlFunction), //< An external control function that you explicitly want to talk to.
}

impl ControlFunction {
    pub fn address(&self) -> u8 {
        match self {
            ControlFunction::Internal(cf) => cf.address(),
            ControlFunction::External(cf) => cf.address(),
            ControlFunction::Partnered(cf) => cf.address(),
        }
    }
    
    pub fn is_address_valid(&self) -> bool {
        let address: u8 = match self {
            ControlFunction::Internal(cf) => cf.address(),
            ControlFunction::External(cf) => cf.address(),
            ControlFunction::Partnered(cf) => cf.address(),
        };

        (address != crate::BROADCAST_CAN_ADDRESS) && (address != crate::NULL_CAN_ADDRESS)
    }
    
    pub fn can_port(&self) -> u8 {
        match self {
            ControlFunction::Internal(cf) => cf.can_port(),
            ControlFunction::External(cf) => cf.can_port(),
            ControlFunction::Partnered(cf) => cf.can_port(),
        }
    }
    
    pub fn name(&self) -> Name {
        match self {
            ControlFunction::Internal(cf) => cf.name(),
            ControlFunction::External(cf) => cf.name(),
            ControlFunction::Partnered(cf) => cf.name(),
        }
    }
}
