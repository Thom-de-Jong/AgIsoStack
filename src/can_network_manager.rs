use core::cell::RefCell;

use alloc::{
    collections::{BTreeMap, VecDeque},
    vec::Vec, boxed::Box, rc::Rc,
};

use crate::{
    name::Name,
    Address,
    CanFrame,
    CanMessage,
    CanPriority,
    ParameterGroupNumber, hardware_integration::CanDriverTrait,
    control_function::{InternalControlFunction, ExternalControlFunction, ControlFunction, ControlFunctionHandle},
};

// const MAX_CAN_FRAMES_SEND_PER_PROCESS: u8 = 255;
const MAX_RECEIVED_CAN_MESSAGE_QUEUE_SIZE: usize = 32;

// const GLOBAL_PARAMETER_GROUP_NUMBER_CALLBACK_LIST_SIZE: usize = 4;

pub struct CanNetworkManager {
    can_driver: Box<dyn CanDriverTrait>,

    // can_message_processors: Vec<RefCell<&'a dyn CanMessageProcessor>>,
    // control_functions_on_the_network: BTreeMap<Name, (bool, Address)>,

    // external_control_functions: BTreeMap<ControlFunctionHandle, ExternalControlFunction>,
    // internal_control_functions: BTreeMap<ControlFunctionHandle, InternalControlFunction>,

    control_functions: BTreeMap<ControlFunctionHandle, ControlFunction>,

    // send_can_frame_buffer: Vec<CanFrame>,
    // send_can_frame_callback: Option<&'a dyn Fn(CanFrame)>,

    // can_message_to_send: Option<CanMessage<'a>>,
    received_can_message_queue: VecDeque<CanMessage>,
    // received_can_message_queue_iter_index: usize,
    // global_parameter_group_number_callbacks: BTreeMap<u16, &'a dyn Fn(&CanMessage)>,
}

impl CanNetworkManager {
    pub fn new(can_driver: impl CanDriverTrait) -> CanNetworkManager {
        CanNetworkManager {
            can_driver: Box::new(can_driver),

            // can_message_processors: Vec::new(),
            // external_control_functions: BTreeMap::new(),
            // internal_control_functions: BTreeMap::new(),
        
            control_functions: BTreeMap::new(),

            // send_can_frame_buffer: Vec::new(),
            // send_can_frame_callback: None,

            received_can_message_queue: VecDeque::new(),
            // received_can_message_queue_iter_index: usize::default(),
        }
    }

    pub fn new_internal_control_function(&self, name: Name, preferred_address: Address) -> ControlFunctionHandle {
        let handle: ControlFunctionHandle = InternalControlFunction::new(name, preferred_address);
        let _ = self.control_functions.insert(handle, ControlFunction::Internal(internal_control_function));
        handle
    }

    pub fn send_can_message(&mut self, message: CanMessage) {
        // Keep track of all the internal control functions on the network.
        if message.pgn() == ParameterGroupNumber::AddressClaim {
            self.update_control_functions_on_the_network(
                message.get_name(0),
                false,
                message.source_address(),
            );
        }

        // Log all outgoing can messages.
        #[cfg(feature = "log_can_write")]
        log::debug!("Send: {}", &message);

        match message.len() {
            0..=8 => {
                if let Ok(frame) = message.as_can_frame() {
                    self.send_can_frame(frame);
                }
            }
            9..=1785 => {
                // self.tp_manager.send(&mut self.can_driver, pdu, time);
            }
            1786..=117_440_505 => {
                // self.etp_manager.send(message);
            }
            _ => {
                log::error!("Can message to long; > 117.440.505 bytes!");
            }
        }
    }

    pub fn send_can_frame(&mut self, frame: CanFrame) {
        // Log all CAN traffic on the bus.
        #[cfg(feature = "log_can_write")]
        log::debug!("Send ->: {}", frame);

        // if let Some(callback) = self.send_can_frame_callback {
        //     callback(frame);
        // }
        let _ = self.can_driver.write(&frame);
    }

    pub fn process_can_message(&mut self, message: CanMessage) {
        // Log all CAN traffic on the bus.
        #[cfg(feature = "log_all_can_read")]
        log::debug!("Read <-: {}", message);

        // Pass on global messages to all the internal control functions
        if message.is_address_global() {
            for icf in self.internal_control_functions_mut() {
                icf.process_can_message(message);
            }
        }

        // Pass on destination specific messages to the specified control function
        else if let Some(&icf) = self.internal_control_functions().iter()
            .find(|&icf| message.is_address_specific(icf.address()))
        {
            icf.process_can_message(message)
        }
    }

    /* pub fn process_can_message(&mut self, message: CanMessage) {
        // Log all CAN traffic on the bus.
        #[cfg(feature = "log_all_can_read")]
        log::debug!("Read: {}", &message);

        // Only listen to global messages and messages ment for us.
        if !message.is_address_global()
            && !self.is_address_internaly_claimed(message.destination_address())
        {
            return;
        }

        // Log only CAN traffic ment for us.
        #[cfg(feature = "log_can_read")]
        log::debug!("Read: {}", &message);

        // Have we handled the message.
        let mut handled = false;

        // Keep track of all the external control functions on the network.
        // Respond to any request for claimed addresses.
        // And pass messages to the TP and ETP managers.
        match message.pgn() {
            ParameterGroupNumber::ParameterGroupNumberRequest => {
                if ParameterGroupNumber::AddressClaim == message.get_pgn_at(0) {
                    // We received a request for all claimed addresses.
                    // The network manager will handle this on behalve of the internal control functions.
                    for (name, address) in self.internal_control_functions() {
                        self.send_address_claim(name, address)
                    }

                    handled = true;
                }
            }
            ParameterGroupNumber::AddressClaim => {
                self.update_control_functions_on_the_network(
                    message.get_name(0),
                    true,
                    message.source_address(),
                );

                handled = true;
            }
            // ParameterGroupNumber::TransportProtocolConnectionManagement |
            // ParameterGroupNumber::TransportProtocolDataTransfer => {
                // if let Some(message) = self.tp_manager.process_can_message(message) {
                //     self.received_can_message_queue.push_back(message);
                // }

                // handled = true;
            // }
            ParameterGroupNumber::ExtendedTransportProtocolConnectionManagement |
            ParameterGroupNumber::ExtendedTransportProtocolDataTransfer => {
                if let Some(message) = self.etp_manager.process_can_message(&message) {
                    self.received_can_message_queue.push_back(message);
                }

                handled = true;
            }
            _ => {}
        }

        // Limit the size of the message queue, by removing the oldest messages.
        // In Rust, using the event queue is prefered over using callbacks.
        while self.received_can_message_queue.len() > MAX_RECEIVED_CAN_MESSAGE_QUEUE_SIZE {
            self.received_can_message_queue.pop_front();
        }

        // If we could not handle the message, store it in the buffer.
        if !handled {
            self.received_can_message_queue.push_back(message);
        }
    } */

    pub fn update(&mut self) {
        // Receive an process CanFrames
        while let Some(frame) = self.can_driver.read() {
            self.process_can_message(frame.into());
        }

        // Update all the internal control functions
        for ControlFunction::Internal(icf) in self.control_functions.values() {
            icf.update(self);
        }
    }

    // pub fn send_can_frame_callback(&mut self, callback: &'a dyn Fn(CanFrame)) {
    //     self.send_can_frame_callback = Some(callback);
    // }

    /// Iterates over all messages, removing handled messages using the predicate.
    ///
    /// In other words, remove all messages `m` for which `f(&m)` returns `true`.
    // pub fn handle_message<F: FnMut(&CanMessage) -> bool>(&mut self, mut f: F) {
    //     self.received_can_message_queue.retain(move |m| !f(m));
    // }

    pub fn next_free_address(&self, current_address: Address) -> Option<Address> {
        for i in (current_address.0..=247).chain(128..current_address.0) {
            let address = Address(i);
            if !self.is_address_claimed(address) {
                return Some(address);
            }
        }
        None
    }

    pub fn is_address_claimed(&self, address: Address) -> bool {
        self.control_functions.values()
            .any(|cf| cf.address() == address)

    }
    pub fn is_address_internaly_claimed(&self, address: Address) -> bool {
        self.internal_control_functions().iter()
            .any(|cf| cf.address() == address)
    }
    pub fn is_address_externaly_claimed(&self, address: Address) -> bool {
        self.external_control_functions().iter()
            .any(|cf| cf.address() == address)
    }

    pub fn handle_by_address(&self, address: Address) -> Option<ControlFunctionHandle> {
        self.control_functions.iter()
            .find_map(|(&handle, cf)| if cf.address() == address { Some(handle) } else { None })
    }

    // pub fn internal_address(&self, name: Name) -> Option<Address> {
    //     self.control_functions_on_the_network
    //         .get(&name)
    //         .filter(|(is_external, _)| !is_external)
    //         .map(|(_, address)| *address)
    // }
    // pub fn external_address(&self, name: Name) -> Option<Address> {
    //     self.control_functions_on_the_network
    //         .get(&name)
    //         .filter(|(is_external, _)| *is_external)
    //         .map(|(_, address)| *address)
    // }
    // pub fn all_internal_addresses(&self) -> Vec<Address> {
    //     self.internal_control_functions()
    //         .into_iter()
    //         .map(|(_, address)| address)
    //         .collect()
    // }
    // pub fn all_external_addresses(&self) -> Vec<Address> {
    //     self.external_control_functions()
    //         .into_iter()
    //         .map(|(_, address)| address)
    //         .collect()
    // }

    pub fn control_function(&self, handle: ControlFunctionHandle) -> Option<&ControlFunction> {
        self.control_functions.get(&handle)
    }
    pub fn internal_control_function(&self, handle: ControlFunctionHandle) -> Option<&InternalControlFunction> {
        self.control_function(handle).map(|ControlFunction::Internal(icf)| icf)
    }
    pub fn external_control_function(&self, handle: ControlFunctionHandle) -> Option<&ExternalControlFunction> {
        self.control_function(handle).map(|ControlFunction::External(ecf)| ecf)
    }

    pub fn control_function_mut(&self, handle: ControlFunctionHandle) -> Option<&mut ControlFunction> {
        self.control_functions.get_mut(&handle)
    }
    pub fn internal_control_function_mut(&self, handle: ControlFunctionHandle) -> Option<&mut InternalControlFunction> {
        self.control_function_mut(handle).map(|ControlFunction::Internal(icf)| icf)
    }
    pub fn external_control_function_mut(&self, handle: ControlFunctionHandle) -> Option<&mut ExternalControlFunction> {
        self.control_function_mut(handle).map(|ControlFunction::External(ecf)| ecf)
    }

    pub fn internal_control_functions(&self) -> Vec<&InternalControlFunction> {
        self.control_functions.values()
            .map(|ControlFunction::Internal(cf)| cf)
            .collect()
    }
    pub fn external_control_functions(&self) -> Vec<&ExternalControlFunction> {
        self.control_functions.values()
            .map(|ControlFunction::External(cf)| cf)
            .collect()
    }

    pub fn internal_control_functions_mut(&self) -> Vec<&mut InternalControlFunction> {
        self.control_functions.values()
            .map(|ControlFunction::Internal(cf)| cf)
            .collect()
    }
    pub fn external_control_functions_mut(&self) -> Vec<&mut ExternalControlFunction> {
        self.control_functions.values()
            .map(|ControlFunction::External(cf)| cf)
            .collect()
    }

    fn update_control_function(
        &mut self,
        handle: ControlFunctionHandle,
        address: Address,
    ) {
        if address == Address::NULL {
            self.control_functions.remove(&handle);
        } else {
            if let Some(cf) = self.control_function(handle) {
                cf.set_address(address);
            }
        }
        // log::debug!("{:?}", self.control_functions_on_the_network);
    }

    pub fn send_request_address_claim(&mut self) {
        let data: [u8; 3] = ParameterGroupNumber::AddressClaim.into();

        let message = CanMessage::new(
            CanPriority::PriorityDefault6,
            ParameterGroupNumber::ParameterGroupNumberRequest,
            Address::NULL,
            Address::GLOBAL,
            &data,
        );
        self.send_can_message(message);
    }

    pub fn send_address_claim(&mut self, handle: ControlFunctionHandle, address: Address) {
        if address == Address::GLOBAL || address == Address::NULL {
            return;
        }

        if let Some(icf) = self.internal_control_function_mut(handle) {
            let data: [u8; 8] = icf.name().into();

            let message = CanMessage::new(
                CanPriority::PriorityDefault6,
                ParameterGroupNumber::AddressClaim,
                address,
                Address::GLOBAL,
                &data,
            );
            self.send_can_message(message);

            icf.set_address(address);
            // self.update_control_functions_on_the_network(name, false, address);
        }
    }
}

// impl Iterator for CanNetworkManager<'_> {
//     type Item = CanMessage;

//     fn next(&mut self) -> Option<Self::Item> {
//         self.received_can_message_queue
//             .get(self.received_can_message_queue_iter_index)
//             .cloned()
//     }
// }

// impl Iterator for &CanNetworkManager<'_> {
//     type Item = CanMessage;

//     fn next(&mut self) -> Option<Self::Item> {
//         self.received_can_message_queue
//             .get(self.received_can_message_queue_iter_index)
//             .cloned()
//     }
// }
