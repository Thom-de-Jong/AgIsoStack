
use alloc::{vec::Vec, collections::{BTreeMap, VecDeque}};

use crate::{
    control_function::{ControlFunction, InternalControlFunction},
    CanPriority, CanFrame, Id, ExtendedId, ParameterGroupNumber, CanMessage, Address,
    // transport_protocol_manager::TransportProtocolManager
};

use core::{cell::RefCell, iter::Peekable};

const MAX_CAN_FRAMES_SEND_PER_PROCESS: u8 = 255;

const GLOBAL_PARAMETER_GROUP_NUMBER_CALLBACK_LIST_SIZE: usize = 4;

pub struct CanNetworkManager<'a> {
	// tp_manager: TransportProtocolManager, //< Instance of the transport protocol manager
    // etp_manager: ExtendedTransportProtocolManager, //< Instance of the extended transport protocol manager
	// fpp_manager: FastPacketProtocolManager, //< Instance of the fast packet protocol manager

    // can_message_processors: Vec<RefCell<&'a dyn CanMessageProcessor>>,
    control_functions_on_the_network: BTreeMap<Name, Address>,

    // send_can_frame_buffer: Vec<CanFrame>,
    send_can_frame_callback: Option<&'a dyn Fn(CanFrame)>,

    // can_message_to_send: Option<CanMessage<'a>>,

    received_can_message_buffer: RefCell<VecDeque<CanMessage>>,
    received_can_message_buffer_iter_index: usize,
    
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


            // can_message_to_send: None,
            received_can_message_buffer: RefCell::new(VecDeque::new()),
            received_can_message_buffer_iter_index: usize::default(),

			// global_parameter_group_number_callbacks: BTreeMap::new(),
        }
    }

    // pub fn add_can_message_processor(&mut self, processor: &'a dyn CanMessageProcessor) {
    //     self.can_message_processors.push(RefCell::new(processor));
    // }

    pub fn send_can_message(&self, message: CanMessage) {
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

    pub fn process_can_message(&self, message: CanMessage) {
        // Log all CAN traffic on the bus.
        #[cfg(feature = "log_all_can_read")]
        log::debug!("Read: {:?}", &message);

        // Only listen to global messages and messages ment for us.
        // let cfs = self.control_functions.borrow();
        // if !message.is_address_global() && !cfs.iter().any(|&c| { message.is_address_specific(c.address()) }) {
        //     return;
        // }

        // Log only CAN traffic ment for us.
        #[cfg(feature = "log_can_read")]
        log::debug!("read: {:?}", &message);

        // Keep track of all the control functions on the network.
        match message.pgn() {
			ParameterGroupNumber::AddressClaim => {
                self.update_control_functions_on_the_network(
                    message.get_name(0),
                    message.source_address(),
                );
            }
            _ => {}
        }

        // Store message in buffer.
        self.received_can_message_buffer.borrow_mut().push_back(message);
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
    pub fn handle_message<F>(&self, f: F)
    where
        F: FnMut(&CanMessage) -> bool,
    {
        self.received_can_message_buffer.borrow_mut().retain(move |m| !f(m));
    }

    pub fn free_address(&self) -> Option<Address> {

        self.control_functions_on_the_network.retain(|k, v| !Address::USER_ADDRESSES.contains(V));

        // TODO, create an Address Iterator?
        for i in (128..=247) {
			
		}
    }

    fn update_control_functions_on_the_network(&mut self, name: Name, address: Address) {
        if address == Address::NULL {
            self.control_functions_on_the_network.remove(&name);
            return;
        }
        self.control_functions_on_the_network.insert(name, source_address);
    }
}

impl Iterator for CanNetworkManager<'_> {
    type Item = CanMessage;

    fn next(&mut self) -> Option<Self::Item> {
        self.received_can_message_buffer.borrow_mut().get(self.received_can_message_buffer_iter_index).cloned()
    }
}

impl Iterator for &CanNetworkManager<'_> {
    type Item = CanMessage;

    fn next(&mut self) -> Option<Self::Item> {
        self.received_can_message_buffer.borrow_mut().get(self.received_can_message_buffer_iter_index).cloned()
    }
}