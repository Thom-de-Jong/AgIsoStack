
use core::cell::RefCell;

use alloc::rc::Rc;
use heapless::HistoryBuffer;

use crate::{
    name::Name, Address, CanNetworkManager,
    protocol_managers::{ExtendedTransportProtocolManager, TransportProtocolManager}, ParameterGroupNumber, CanMessage,
};

use super::{AddressClaimStateMachine, ControlFunction, ControlFunctionHandle};

pub struct InternalControlFunction {
    state_machine: AddressClaimStateMachine,
    name: Name,

    tp_manager: TransportProtocolManager,           //< Instance of the transport protocol manager
    etp_manager: ExtendedTransportProtocolManager,  //< Instance of the extended transport protocol manager

    pub received_can_message_queue: HistoryBuffer<CanMessage, 32>,
}

impl InternalControlFunction {
    pub fn new(name: Name, address: Address) -> ControlFunctionHandle {
        ControlFunctionHandle(
            Rc::new_cyclic(|&handle| {
                RefCell::new(ControlFunction::Internal(InternalControlFunction {
                    state_machine: AddressClaimStateMachine::new(handle, address),
                    name,
                    tp_manager: TransportProtocolManager::new(),
                    etp_manager: ExtendedTransportProtocolManager::new(),
                    received_can_message_queue: HistoryBuffer::new(),
                }))
            })
        )
    }

    pub fn name(&self) -> Name {
        self.name
    }

    pub fn address(&self) -> Address {
        self.state_machine.claimed_address()
    }
    pub fn claim_address(&self, address: Address) {
        self.state_machine.claim_address(address);
    }

    pub fn initialize(&mut self) {
        self.state_machine.enable();
    }

    pub fn terminate(&mut self) {
        self.state_machine.disable();
    }

    pub fn update(&mut self, network_manager: &mut CanNetworkManager) {
        // Process received messages and update internal state.
        // network_manager.handle_message(|message| self.state_machine.process_can_message(message));

        // Do stuff based on the current internal state.
        self.state_machine.update(self, network_manager);
    }

    pub fn process_can_message(&mut self, message: CanMessage) {
        // Log only CAN traffic ment for us.
        #[cfg(feature = "log_can_read")]
        log::debug!("Read <-: {}", message);

        // Pass messages to the TP and ETP managers.
        match message.pgn() {
            // ParameterGroupNumber::TransportProtocolConnectionManagement |
            // ParameterGroupNumber::TransportProtocolDataTransfer => {
                // if let Some(message) = self.tp_manager.process_can_message(message) {
                //     self.received_can_message_queue.write(message);
                // }
            // }
            ParameterGroupNumber::ExtendedTransportProtocolConnectionManagement |
            ParameterGroupNumber::ExtendedTransportProtocolDataTransfer => {
                if let Some(message) = self.etp_manager.process_can_message(&message) {
                    self.received_can_message_queue.write(message);
                }
            }
            _ => {}
        }
    }
}
