// use core::time::Duration;

// use crate::hardware_integration::{TimeDriver, TimeDriverTrait};
// use crate::{Address, CanMessage, CanNetworkManager, ObjectId, ParameterGroupNumber, CanPriority};

// use super::{KeyActivationCode, VTFunction, VTKeyEvent, VTVersion};


// pub struct VirtualTerminalClientStateMachine {
    
// }

// impl VirtualTerminalClientStateMachine {
//     pub fn new() -> Self {
//         Self {
//             current_state: State::default(),
//             is_enabled: false,

//             last_vtstatus_timestamp_ms: Duration::default(),
//             active_working_set_master_address: Address::NULL,
//             active_working_set_data_mask_object_id: ObjectId::NULL,
//             active_working_set_soft_key_mask_object_id: ObjectId::NULL,
//             busy_codes_bitfield: u8::default(),
//             current_command_function_code: u8::default(),

//             connected_vt_version: VTVersion::default(),
//         }
//     }

//     pub fn is_connected(&self) -> bool {
        
//     }

//     pub fn process_can_message(&mut self, message: &CanMessage) -> bool {
//         if message.pgn() != ParameterGroupNumber::VirtualTerminalToECU {
//             return false;
//         }

//         let handled = false;
//         if let Ok(vt_function) = message.get_u8_at(0).try_into() {
//             match vt_function {
                
//                 _ => {}
//             }
//         }
//         handled
//     }

//     pub fn update(&mut self, network_manager: &mut CanNetworkManager) {
        
//     }


// }
