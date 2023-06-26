
use crate::{Address, name::{Name, NameFilter}};

mod address_claim_state_machine;
use address_claim_state_machine::AddressClaimStateMachine;

mod internal_control_function;
pub use internal_control_function::InternalControlFunction;

mod external_control_function;
pub use external_control_function::ExternalControlFunction;

pub struct PartneredControlFunction(ExternalControlFunction);

pub enum ControlFunction {
    Internal(InternalControlFunction), //< The control function is part of our stack and can address claim.
    External(ExternalControlFunction), //< The control function is some other device on the bus.
    Partnered(PartneredControlFunction), //< An external control function that you explicitly want to talk to.
}

impl ControlFunction {
    pub fn new_internal_control_function(desired_name: Name, preferred_address: Address, can_port: u8) -> Option<InternalControlFunction> {
        let state_machine = AddressClaimStateMachine::new(desired_name, preferred_address, can_port);

        if let Some(state_machine) = state_machine {
            Some(InternalControlFunction {
                object_changed_address_since_last_update: false,
                state_machine,
            })
        } else {
            None
        }
    }
    // pub fn new_external_control_function(desired_name: Name, prefered_address: Address, can_port: u8) -> ControlFunction {
    //     ControlFunction::Partnered(PartneredControlFunction( {
    //         // address: todo!(),
    //         // can_port: todo!(),
    //         // name: todo!(),
    //         // object_changed_address_since_last_update: false,
    //     }))
    // }

    pub fn new_partnered_control_function(_: u8, _filters: &[NameFilter]) -> Option<PartneredControlFunction> {
        Some(PartneredControlFunction(
            ExternalControlFunction
            {
                address: Address::GLOBAL,
                can_port: 0,
                name: Name::default(),
                object_changed_address_since_last_update: false,
            }
        ))
    }

    pub fn address(&self) -> Address {
        match self {
            ControlFunction::Internal(cf) => cf.address(),
            ControlFunction::External(cf) => cf.address(),
            ControlFunction::Partnered(cf) => cf.0.address(),
        }
    }
    
    pub fn is_address_valid(&self) -> bool {
        let address: Address = self.address();
        (address != Address::GLOBAL) && (address != Address::NULL)
    }
    
    pub fn can_port(&self) -> u8 {
        match self {
            ControlFunction::Internal(cf) => cf.can_port(),
            ControlFunction::External(cf) => cf.can_port(),
            ControlFunction::Partnered(cf) => cf.0.can_port(),
        }
    }
    
    pub fn name(&self) -> Name {
        match self {
            ControlFunction::Internal(cf) => cf.name(),
            ControlFunction::External(cf) => cf.name(),
            ControlFunction::Partnered(cf) => cf.0.name(),
        }
    }
}
