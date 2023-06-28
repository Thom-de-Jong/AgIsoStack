
use alloc::vec::Vec;

use crate::{
    control_function::{ControlFunction, InternalControlFunction},
    CanPriority, CanFrame, Id, ExtendedId, ParameterGroupNumber, can_message, CanMessage, CanMessageTrait,
    // transport_protocol_manager::TransportProtocolManager
};

use core::cell::RefCell;

const MAX_CAN_FRAMES_SEND_PER_PROCESS: u8 = 255;

const GLOBAL_PARAMETER_GROUP_NUMBER_CALLBACK_LIST_SIZE: usize = 4;

pub struct CanNetworkManager<'a> {
	// tp_manager: TransportProtocolManager, //< Instance of the transport protocol manager
    // etp_manager: ExtendedTransportProtocolManager, //< Instance of the extended transport protocol manager
	// fpp_manager: FastPacketProtocolManager, //< Instance of the fast packet protocol manager

    control_functions: RefCell<Vec<&'a InternalControlFunction>>,

    send_can_frame_buffer: Vec<CanFrame>,
    send_can_frame_callback: Option<&'a dyn Fn(CanFrame)>,

    can_message_to_send: Option<&'a dyn CanMessageTrait>,
    
	// global_parameter_group_number_callbacks: FnvIndexMap<u16, &'a dyn Fn(&'a CanMessage), GLOBAL_PARAMETER_GROUP_NUMBER_CALLBACK_LIST_SIZE>,
}

impl<'a> CanNetworkManager<'a> {
    pub fn new() -> CanNetworkManager<'static> {
        CanNetworkManager {
            // tp_manager: TransportProtocolManager::new(),

            control_functions: RefCell::new(Vec::new()),

            send_can_frame_buffer: Vec::new(),
            send_can_frame_callback: None,

            can_message_to_send: None,
			// global_parameter_group_number_callbacks: FnvIndexMap::new(),
        }
    }

    pub fn add_control_function(&self, cf: &'a InternalControlFunction) {
        let _ = self.control_functions.borrow_mut().push(cf);
    }

    // pub fn send_can_message(&self, pgn: ParameterGroupNumber, data: &[u8], src: &InternalControlFunction, dest: &PartneredControlFunction, priority: CanPriority) {
    pub fn send_can_message(&self, message: &'a dyn CanMessageTrait) {
            // TODO: Build id
        let id = ExtendedId::MAX;

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

    pub fn process_can_message(&self, frame: & dyn CanMessageTrait) {
        // Log all CAN traffic on the bus.
        #[cfg(feature = "log_all_can_read")]
        log::debug!("Read: {:?}", &frame);

        // Only listen to global messages and messages ment for us.
        let cfs = self.control_functions.borrow();
        if !frame.is_address_global() && !cfs.iter().any(|&c| { frame.is_address_specific(c.address()) }) {
            return;
        }

        // Log only CAN traffic ment for us.
        // #[cfg(feature = "log_can_read")]
        // log::debug!("read: {:?}", &frame);



        // check if global
        // for (pgn, callback) in self.global_parameter_group_number_callbacks {
        //     callback(frame);
        // }

            

		// while (0 != get_number_can_messages_in_rx_queue())
		// {
		// 	CANMessage currentMessage = get_next_can_message_from_rx_queue();

		// 	update_address_table(currentMessage);

		// 	// Update Special Callbacks, like protocols and non-cf specific ones
		// 	process_protocol_pgn_callbacks(currentMessage);
		// 	process_any_control_function_pgn_callbacks(currentMessage);

		// 	// Update Others
		// 	process_can_message_for_global_and_partner_callbacks(currentMessage);
		// }
	}

    pub fn process_can_frame<const N: usize>(&self, frame: CanFrame) -> Option<CanMessage<N>> {
        // Check if TP or ETP message
        // Give it to the handlers

        Some(CanMessage::new(frame.id(), frame.data()))
    }

    pub fn send_can_frame_callback(&mut self, callback: &'a dyn Fn(CanFrame)) {
        self.send_can_frame_callback = Some(callback);
    }

    
	// pub fn add_global_parameter_group_number_callback<const N: usize>(&mut self, pgn: ParameterGroupNumber, callback: &'a dyn Fn(&'a CanMessage<N>)) -> Result<(), ()> {
	//     match self.global_parameter_group_number_callbacks.insert(pgn as u16, callback) {
    //         Ok(_) => Ok(()),
    //         Err(_) => Err(()),
    //     }
	// }

    // pub fn remove_global_parameter_group_number_callback(&mut self, pgn: ParameterGroupNumber) {
	// 	let _ = self.global_parameter_group_number_callbacks.remove(&(pgn as u16));
	// }
}

