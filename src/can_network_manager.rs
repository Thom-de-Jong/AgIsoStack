use alloc::{collections::{BTreeMap, VecDeque}, vec::Vec};

use crate::{
    name::Name,
    // transport_protocol_manager::TransportProtocolManager
    Address,
    CanFrame,
    CanMessage,
    ParameterGroupNumber, CanPriority,
};

// const MAX_CAN_FRAMES_SEND_PER_PROCESS: u8 = 255;
const MAX_RECEIVED_CAN_MESSAGE_QUEUE_SIZE: usize = 32;

// const GLOBAL_PARAMETER_GROUP_NUMBER_CALLBACK_LIST_SIZE: usize = 4;

pub struct CanNetworkManager<'a> {
    // tp_manager: TransportProtocolManager, //< Instance of the transport protocol manager
    // etp_manager: ExtendedTransportProtocolManager, //< Instance of the extended transport protocol manager
    // fpp_manager: FastPacketProtocolManager, //< Instance of the fast packet protocol manager

    // can_message_processors: Vec<RefCell<&'a dyn CanMessageProcessor>>,
    control_functions_on_the_network: BTreeMap<Name, (bool, Address)>,

    // send_can_frame_buffer: Vec<CanFrame>,
    send_can_frame_callback: Option<&'a dyn Fn(CanFrame)>,

    // can_message_to_send: Option<CanMessage<'a>>,
    received_can_message_queue: VecDeque<CanMessage>,
    received_can_message_queue_iter_index: usize,
    // global_parameter_group_number_callbacks: BTreeMap<u16, &'a dyn Fn(&CanMessage)>,
}

impl<'a> CanNetworkManager<'a> {
    pub fn new() -> CanNetworkManager<'a> {
        CanNetworkManager {
            // tp_manager: TransportProtocolManager::new(),

            // can_message_processors: Vec::new(),
            control_functions_on_the_network: BTreeMap::new(),

            // send_can_frame_buffer: Vec::new(),
            send_can_frame_callback: None,

            received_can_message_queue: VecDeque::new(),
            received_can_message_queue_iter_index: usize::default(),
        }
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
                // self.etp_manager.send(&mut self.can_driver, pdu, time);
            }
            _ => {
                log::error!("Can message to long; > 117.440.505 bytes!");
            }
        }
    }

    pub fn send_can_frame(&self, frame: CanFrame) {
        if let Some(callback) = self.send_can_frame_callback {
            callback(frame);
        }
    }

    pub fn process_can_message(&mut self, message: CanMessage) {
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
        log::debug!("read: {}", &message);

        // Limit the size of the message queue, by removing the oldest messages.
        // In Rust, using the event queue is prefered over using callbacks.
        while self.received_can_message_queue.len() > MAX_RECEIVED_CAN_MESSAGE_QUEUE_SIZE {
            self.received_can_message_queue.pop_front();
        }

        // Have we handled the message ourselves.
        let mut handled = false;

        // Keep track of all the external control functions on the network.
        // And respond to any request for claimed addresses.
        match message.pgn() {
            ParameterGroupNumber::ParameterGroupNumberRequest => {
                if ParameterGroupNumber::AddressClaim == message.get_pgn_at(0) {
                    // We received a request for all claimed addresses.
                    // The network manager will handle this on behave of the internal control functions.
                    for (name, address) in self.internal_control_functions() {
                        self.send_address_claim(name, address)
                    }

                    handled = true;
                }
            },
            ParameterGroupNumber::AddressClaim => {
                self.update_control_functions_on_the_network(
                    message.get_name(0),
                    true,
                    message.source_address(),
                );
            }
            _ => {}
        }

        // If we could not handle the message, store it in the buffer.
        if !handled {
            self.received_can_message_queue.push_back(message);
        }
    }

    pub fn process_can_frame(&mut self, frame: CanFrame) {
        // Check if TP or ETP message
        // Give it to the handlers

        let message: Option<CanMessage> = Some(frame.into());

        // If a message is complete
        if let Some(message) = message {
            self.process_can_message(message);
        }
    }

    pub fn send_can_frame_callback(&mut self, callback: &'a dyn Fn(CanFrame)) {
        self.send_can_frame_callback = Some(callback);
    }

    /// Iterates over all messages, removing handled messages using the predicate.
    ///
    /// In other words, remove all messages `m` for which `f(&m)` returns `true`.
    pub fn handle_message<F: FnMut(&CanMessage) -> bool>(&mut self, mut f: F) {
        self.received_can_message_queue.retain(move |m| !f(m));
    }


    pub fn next_free_address(&self) -> Option<Address> {
        let mut list = self.control_functions_on_the_network.clone();
        list.retain(|_, (_, a)| !Address::USER_ADDRESSES.contains(a));
        list.pop_first().map(|(_, (_, address))| address)
    }


    pub fn is_address_claimed(&self, address: Address) -> bool {
        self.control_functions_on_the_network.values().any(|(_, a)| *a == address)
    }
    pub fn is_address_internaly_claimed(&self, address: Address) -> bool {
        self.control_functions_on_the_network.values().any(|(is_external, a)| !*is_external && *a == address)
    }

    pub fn get_name_by_address(&self, address: Address) -> Option<Name> {
        self.control_functions_on_the_network
            .iter()
            .find_map(|(&name, (_, a))| if *a == address { Some(name) } else { None })
    }

    pub fn internal_address(&self, name: Name) -> Option<Address> {
        self.control_functions_on_the_network
            .get(&name)
            .filter(|(is_external, _)| !is_external)
            .map(|(_, address)| *address)
    }
    pub fn external_address(&self, name: Name) -> Option<Address> {
        self.control_functions_on_the_network
            .get(&name)
            .filter(|(is_external, _)| *is_external)
            .map(|(_, address)| *address)
    }
    pub fn all_internal_addresses(&self) -> Vec<Address> {
        self.internal_control_functions()
            .into_iter()
            .map(|(_, address)| address)
            .collect()
    }

    fn update_control_functions_on_the_network(
        &mut self,
        name: Name,
        is_external: bool,
        address: Address,
    ) {
        if address == Address::NULL {
            self.control_functions_on_the_network.remove(&name);
        } else {
            self.control_functions_on_the_network.insert(name, (is_external, address));
        }
        log::debug!("{:?}", self.control_functions_on_the_network);
    }

    pub fn internal_control_functions(&self) -> Vec<(Name, Address)> {
        self.control_functions_on_the_network
            .iter()
            .filter(|(_, (is_external, _))|  !is_external )
            .map(|(name, (_, address))| (*name, *address))
            .collect()
    }
    

    pub fn send_request_to_claim(&mut self) {
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

    pub fn send_address_claim(&mut self, name: Name, address: Address) {
        let data: [u8; 8] = name.into();

        let message = CanMessage::new(
            CanPriority::PriorityDefault6,
            ParameterGroupNumber::AddressClaim,
            address,
            Address::GLOBAL,
            &data,
        );
        self.send_can_message(message);
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
