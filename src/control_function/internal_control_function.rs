use crate::{name::Name, Address, CanNetworkManager};

use super::AddressClaimStateMachine;

pub struct InternalControlFunction {
    state_machine: AddressClaimStateMachine,
}

impl InternalControlFunction {
    pub fn new(desired_name: Name, preferred_address: Address) -> Option<InternalControlFunction> {
        if let Some(state_machine) = AddressClaimStateMachine::new(desired_name, preferred_address)
        {
            Some(InternalControlFunction { state_machine })
        } else {
            None
        }
    }

    pub fn address(&self) -> Address {
        self.state_machine.claimed_address()
    }

    pub fn name(&self) -> Name {
        self.state_machine.name()
    }

    pub fn initialize(&mut self) {
        self.state_machine.enable();
    }

    pub fn update(&mut self, network_manager: &mut CanNetworkManager) {
        // Process received messages and update internal state.
        network_manager.handle_message(|message| self.state_machine.process_can_message(message));

        // Do stuff based on the current internal state.
        self.state_machine.update(network_manager);
    }
}
