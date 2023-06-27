
use crate::{
    control_function::{InternalControlFunction, PartneredControlFunction},
    CanPriority, CanFrame, Id, ExtendedId, ParameterGroupNumber, can_message, CanMessage,
    // transport_protocol_manager::TransportProtocolManager
};

use heapless::{FnvIndexMap, Vec};

const SEND_CAN_FRAMES_BUFFER_SIZE: usize = 64;
const MAX_CAN_FRAMES_SEND_PER_PROCESS: u8 = 255;

const GLOBAL_PARAMETER_GROUP_NUMBER_CALLBACK_LIST_SIZE: usize = 4;

pub struct CanNetworkManager<'a> {
	// tp_manager: TransportProtocolManager, //< Instance of the transport protocol manager
    // etp_manager: ExtendedTransportProtocolManager, //< Instance of the extended transport protocol manager
	// fpp_manager: FastPacketProtocolManager, //< Instance of the fast packet protocol manager

    send_can_frame_buffer: Vec<CanFrame, SEND_CAN_FRAMES_BUFFER_SIZE>,
    send_can_frame_callback: Option<fn(CanFrame)>,
    
	global_parameter_group_number_callbacks: FnvIndexMap<u16, &'a dyn Fn(&'a CanMessage), GLOBAL_PARAMETER_GROUP_NUMBER_CALLBACK_LIST_SIZE>,
}

impl<'a> CanNetworkManager<'a> {
    pub fn new() -> CanNetworkManager<'static> {
        CanNetworkManager {
            // tp_manager: TransportProtocolManager::new(),
            send_can_frame_buffer: Vec::new(),
            send_can_frame_callback: None,

			global_parameter_group_number_callbacks: FnvIndexMap::new(),
        }
    }

    // pub fn send_can_message(&self, pgn: ParameterGroupNumber, data: &[u8], src: &InternalControlFunction, dest: &PartneredControlFunction, priority: CanPriority) {
    pub fn send_can_message<const N: usize>(&self, message: &CanMessage<N>) {
            // TODO: Build id
        let id = ExtendedId::MAX;

        match message.data_len() {
            0..=8 => {
                if let Ok(frame) = message.try_into() {
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

    pub fn process_can_frame(&mut self, frame: CanFrame){
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

    pub fn send_can_frame_callback(&mut self, callback: fn(CanFrame)) {
        self.send_can_frame_callback = Some(callback);
    }

    
	pub fn add_global_parameter_group_number_callback<const N: usize>(&mut self, pgn: ParameterGroupNumber, callback: &'a dyn Fn(&'a CanMessage<N>)) -> Result<(), ()> {
	    match self.global_parameter_group_number_callbacks.insert(pgn as u16, callback) {
            Ok(_) => Ok(()),
            Err(_) => Err(()),
        }
	}

    pub fn remove_global_parameter_group_number_callback(&mut self, pgn: ParameterGroupNumber) {
		let _ = self.global_parameter_group_number_callbacks.remove(&(pgn as u16));
	}
}

