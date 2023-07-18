
use alloc::vec::Vec;

use crate::{
    name::{Name, NameFilter},
    Address, CanNetworkManager, hardware_integration::CanDriverTrait,
};

mod address_claim_state_machine;
use address_claim_state_machine::AddressClaimStateMachine;

mod internal_control_function;
pub use internal_control_function::InternalControlFunction;
mod external_control_function;
pub use external_control_function::ExternalControlFunction;

pub struct PartneredControlFunction {
    external_control_function_cache: Option<ExternalControlFunction>,
    name_filters: Vec<NameFilter>,
}

impl PartneredControlFunction {
    pub fn new(filters: &[NameFilter]) -> PartneredControlFunction {
        PartneredControlFunction {
            external_control_function_cache: None,
            name_filters: filters.to_vec(),
        }
    }

    pub fn name(&self) -> Option<Name> {
        self.external_control_function_cache
            .map(|ecf| ecf.name())
    }
    pub fn address(&self) -> Option<Address> {
        self.external_control_function_cache
            .map(|ecf| ecf.address())
    }
    pub fn is_partnered(&self) -> bool {
        self.external_control_function_cache.is_some()
    }

    pub fn update<T: CanDriverTrait>(&mut self, network_manager: &mut CanNetworkManager<T>) {
        self.external_control_function_cache = network_manager.external_control_functions().into_iter()
            .find(|&ecf|{
                self.name_filters.iter()
                    .all(|filter| filter.check_name_matches_filter(ecf.name()))
            })
    }
}


pub enum ControlFunction {
    Internal(InternalControlFunction), //< The control function is part of our stack and can address claim.
    External(ExternalControlFunction), //< The control function is some other device on the bus.
}

impl ControlFunction {
    pub fn new_internal_control_function(name: Name, address: Address) -> Option<ControlFunction> {
        InternalControlFunction::new(name, address)
            .map(|icf|ControlFunction::Internal(icf))
    }
    pub fn new_external_control_function(name: Name, address: Address) -> ControlFunction {
        ControlFunction::External(ExternalControlFunction::new(name, address))
    }

    pub fn name(&self) -> Name {
        match self {
            ControlFunction::Internal(cf) => cf.name(),
            ControlFunction::External(cf) => cf.name(),
        }
    }

    pub fn address(&self) -> Address {
        match self {
            ControlFunction::Internal(cf) => cf.address(),
            ControlFunction::External(cf) => cf.address(),
        }
    }

    pub fn is_address_valid(&self) -> bool {
        self.address() < Address::NULL
    }
}
