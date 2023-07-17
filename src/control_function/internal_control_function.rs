use crate::{name::Name, Address, CanNetworkManager, hardware_integration::CanDriverTrait, protocol_managers::ExtendedTransportProtocolManager};

use super::AddressClaimStateMachine;

pub struct InternalControlFunction {
    state_machine: AddressClaimStateMachine,

    // tp_manager: TransportProtocolManager, //< Instance of the transport protocol manager
    etp_manager: ExtendedTransportProtocolManager, //< Instance of the extended transport protocol manager
    // fpp_manager: FastPacketProtocolManager, //< Instance of the fast packet protocol manager
}

impl InternalControlFunction {
    pub fn new(name: Name, preferred_address: Address) -> Option<InternalControlFunction> {
        if let Some(state_machine) = AddressClaimStateMachine::new(name, preferred_address) {
            Some(InternalControlFunction{
                state_machine,
                etp_manager: ExtendedTransportProtocolManager::new(),
            })
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

    pub fn terminate(&mut self) {
        self.state_machine.disable();
    }

    pub fn update<T: CanDriverTrait>(&mut self, network_manager: &mut CanNetworkManager<T>) {
        // Process received messages and update internal state.
        network_manager.handle_message(|message| self.state_machine.process_can_message(message));

        // Do stuff based on the current internal state.
        self.state_machine.update(network_manager);
    }
}
