
use alloc::vec::Vec;

use crate::{
    name::{Name, NameFilter},
    Address,
    CanNetworkManager,
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

    pub fn update(&mut self, network_manager: &mut CanNetworkManager) {
        // self.external_control_function_cache = network_manager.external_control_functions().into_iter()
        //     .find(|&ecf|{
        //         self.name_filters.iter()
        //             .all(|filter| filter.check_name_matches_filter(ecf.name()))
        //     })
    }
}


pub enum ControlFunction {
    Internal(InternalControlFunction), //< The control function is part of our stack and can address claim.
    External(ExternalControlFunction), //< The control function is some other device on the bus.
    Partnered(PartneredControlFunction), //< The control function is some other device on the bus.
}

impl ControlFunction {
    pub fn new_internal_control_function(name: Name, address: Address) -> ControlFunction {
        ControlFunction::Internal(InternalControlFunction::new(name, address))
    }
    pub fn new_external_control_function(name: Name, address: Address) -> ControlFunction {
        ControlFunction::External(ExternalControlFunction::new(name, address))
    }

    pub fn name(&self) -> Name {
        match self {
            ControlFunction::Internal(cf) => cf.name(),
            ControlFunction::External(cf) => cf.name(),
            ControlFunction::Partnered(cf) => cf.name().unwrap_or_default(),
        }
    }

    pub fn address(&self) -> Address {
        match self {
            ControlFunction::Internal(cf) => cf.address(),
            ControlFunction::External(cf) => cf.address(),
            ControlFunction::Partnered(cf) => cf.address().unwrap_or_default(),
        }
    }

    pub fn set_address(&self, address: Address) {
        match self {
            ControlFunction::Internal(cf) => cf.claim_address(address),
            ControlFunction::External(cf) => {},
            ControlFunction::Partnered(cf) => {},
        }
    }

    pub fn is_address_valid(&self) -> bool {
        self.address() < Address::NULL
    }
}


#[derive(Copy, Clone, Default, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct ControlFunctionHandle(Name);

impl From<Name> for ControlFunctionHandle {
    fn from(value: Name) -> Self {
        ControlFunctionHandle(value)
    }
}