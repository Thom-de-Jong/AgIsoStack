use crate::{
    name::{Name, NameFilter},
    Address, CanNetworkManager, hardware_integration::CanDriverTrait,
};

mod address_claim_state_machine;
use address_claim_state_machine::AddressClaimStateMachine;

mod internal_control_function;
use alloc::vec::Vec;
pub use internal_control_function::InternalControlFunction;

mod external_control_function;
pub use external_control_function::ExternalControlFunction;

pub struct PartneredControlFunction {
    // external_control_function: ExternalControlFunction,
    connected_name: Option<Name>,
    claimed_address: Option<Address>,

    name_filters: Vec<NameFilter>,
}

impl PartneredControlFunction {
    pub fn new(filters: &[NameFilter]) -> PartneredControlFunction {
        PartneredControlFunction {
            // external_control_function: ExternalControlFunction {
            //     address: Address::GLOBAL,
            //     name: Name::default(),
            //     object_changed_address_since_last_update: false,
            // },
            connected_name: None,
            claimed_address: None,
            name_filters: filters.to_vec(),
        }
    }

    pub fn name(&self) -> Name {
        self.connected_name.unwrap_or_default()
    }
    pub fn address(&self) -> Address {
        self.claimed_address.unwrap_or_default()
    }
    pub fn is_address_valid(&self) -> bool {
        self.claimed_address.is_some()
    }

    pub fn update<T: CanDriverTrait>(&mut self, network_manager: &mut CanNetworkManager<T>) {
        // Process received messages and update internal state.
        // network_manager.handle_message(|message| self.state_machine.process_can_message(message));

        // Do stuff based on the current internal state.
        // self.state_machine.update(network_manager);

        self.connected_name = None;
        self.claimed_address = None;

        for (name, address) in network_manager.external_control_functions() {
            if self.name_filters.iter().all(|filter|{ filter.check_name_matches_filter(name) }) {
                self.connected_name = Some(name);
                self.claimed_address = Some(address);
            }
        }
    }
}

pub enum ControlFunction {
    Internal(InternalControlFunction), //< The control function is part of our stack and can address claim.
    External(ExternalControlFunction), //< The control function is some other device on the bus.
    Partnered(PartneredControlFunction), //< An external control function that you explicitly want to talk to.
}

impl ControlFunction {
    // pub fn new_internal_control_function(desired_name: Name, preferred_address: Address, ) -> Option<InternalControlFunction> {

    // }
    // // pub fn new_external_control_function(desired_name: Name, prefered_address: Address, can_port: u8) -> ControlFunction {
    // //     ControlFunction::Partnered(PartneredControlFunction( {
    // //         // address: todo!(),
    // //         // can_port: todo!(),
    // //         // name: todo!(),
    // //         // object_changed_address_since_last_update: false,
    // //     }))
    // // }

    // pub fn new_partnered_control_function(_: u8, _filters: &[NameFilter]) -> Option<PartneredControlFunction> {
    //     Some(PartneredControlFunction(
    //         ExternalControlFunction
    //         {
    //             address: Address::GLOBAL,
    //             can_port: 0,
    //             name: Name::default(),
    //             object_changed_address_since_last_update: false,
    //         }
    //     ))
    // }

    pub fn address(&self) -> Address {
        match self {
            ControlFunction::Internal(cf) => cf.address(),
            ControlFunction::External(cf) => cf.address(),
            ControlFunction::Partnered(cf) => cf.address(),
        }
    }

    pub fn is_address_valid(&self) -> bool {
        let address: Address = self.address();
        (address != Address::GLOBAL) && (address != Address::NULL)
    }

    pub fn name(&self) -> Name {
        match self {
            ControlFunction::Internal(cf) => cf.name(),
            ControlFunction::External(cf) => cf.name(),
            ControlFunction::Partnered(cf) => cf.name(),
        }
    }
}
