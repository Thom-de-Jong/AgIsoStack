
use core::default;
use core::time::Duration;

use alloc::collections::{BTreeMap, VecDeque};

use crate::hardware_integration::{TimeDriver, TimeDriverTrait};
use crate::{
    control_function::*, Address, CanMessage, CanNetworkManager, CanPriority, ObjectId,
    ParameterGroupNumber, ObjectPool, object_pool,
};

use super::*;

const MAX_EVENT_QUEUE_SIZE: usize = 32;
const WORKING_SET_MAINTENANCE_TIMEOUT: Duration = Duration::from_secs(1);

pub struct VirtualTerminalClient<'a> {
    partnered_control_function: PartneredControlFunction, //< The partner control function this client will send to
    internal_control_function: InternalControlFunction, //< The internal control function the client uses to send from

    object_pools: BTreeMap<usize, ObjectPool>,

    current_state: State,
    is_initialized: bool,
    is_enabled: bool,

    // TODO: VT status variables, make PartneredControlFunction hold this in tuple struct VirtualTerminalServer?
    last_vtstatus_timestamp_ms: Duration,
    active_working_set_master_address: Address,
    active_working_set_data_mask_object_id: ObjectId,
    active_working_set_soft_key_mask_object_id: ObjectId,
    busy_codes_bitfield: u8,
    current_command_function_code: u8,
    connected_vt_version: VTVersion,

    first_time_in_state: bool,

    last_working_set_maintenance_timestamp: Duration,

    send_working_set_maintenance: bool,
    send_auxiliary_maintenance: bool,

    // Event queue and callbacks
    event_queue: VecDeque<Event>,
    soft_key_event_callbacks: BTreeMap<usize, &'a dyn Fn(VTKeyEvent)>,
    button_event_callbacks: BTreeMap<usize, &'a dyn Fn(VTKeyEvent)>,
    pointing_event_callbacks: BTreeMap<usize, &'a dyn Fn(VTPointingEvent)>,
    select_input_object_event_callbacks: BTreeMap<usize, &'a dyn Fn(VTSelectInputObjectEvent)>,
    esc_message_event_callbacks: BTreeMap<usize, &'a dyn Fn(VTESCMessageEvent)>,
    change_numeric_value_event_callbacks: BTreeMap<usize, &'a dyn Fn(VTChangeNumericValueEvent)>,
    change_active_mask_event_callbacks: BTreeMap<usize, &'a dyn Fn(VTChangeActiveMaskEvent)>,
    change_soft_key_mask_event_callbacks: BTreeMap<usize, &'a dyn Fn(VTChangeSoftKeyMaskEvent)>,
    change_string_value_event_callbacks: BTreeMap<usize, &'a dyn Fn(VTChangeStringValueEvent)>,
    user_layout_hide_show_event_callbacks: BTreeMap<usize, &'a dyn Fn(VTUserLayoutHideShowEvent)>,
    audio_signal_termination_event_callbacks:
        BTreeMap<usize, &'a dyn Fn(VTAudioSignalTerminationEvent)>,
    auxiliary_function_event_callbacks: BTreeMap<usize, &'a dyn Fn(AuxiliaryFunctionEvent)>,
}

impl<'a> VirtualTerminalClient<'a> {
    pub fn new(
        partner: PartneredControlFunction,
        client: InternalControlFunction,
    ) -> VirtualTerminalClient<'a> {
        VirtualTerminalClient {
            partnered_control_function: partner,
            internal_control_function: client,

            object_pools: BTreeMap::new(),

            current_state: State::default(),
            is_initialized: false,
            is_enabled: false,
        
            // TODO: VT status variables, make PartneredControlFunction hold this in tuple struct VirtualTerminalServer?
            last_vtstatus_timestamp_ms: Duration::default(),
            active_working_set_master_address: Address::default(),
            active_working_set_data_mask_object_id: ObjectId::default(),
            active_working_set_soft_key_mask_object_id: ObjectId::default(),
            busy_codes_bitfield: u8::default(),
            current_command_function_code: u8::default(),
            connected_vt_version: VTVersion::default(),

            first_time_in_state: false,

            last_working_set_maintenance_timestamp: Duration::default(),

            send_working_set_maintenance: false,
            send_auxiliary_maintenance: false,

            event_queue: VecDeque::new(),
            soft_key_event_callbacks: BTreeMap::new(),
            button_event_callbacks: BTreeMap::new(),
            pointing_event_callbacks: BTreeMap::new(),
            select_input_object_event_callbacks: BTreeMap::new(),
            esc_message_event_callbacks: BTreeMap::new(),
            change_numeric_value_event_callbacks: BTreeMap::new(),
            change_active_mask_event_callbacks: BTreeMap::new(),
            change_soft_key_mask_event_callbacks: BTreeMap::new(),
            change_string_value_event_callbacks: BTreeMap::new(),
            user_layout_hide_show_event_callbacks: BTreeMap::new(),
            audio_signal_termination_event_callbacks: BTreeMap::new(),
            auxiliary_function_event_callbacks: BTreeMap::new(),
        }
    }

    pub fn initialize(&mut self) {
        // // Bind the callbacks to CAN messages used by the Virtual Termainal Client
        // self.partnerControlFunction->add_parameter_group_number_callback(static_cast<std::uint32_t>(CANLibParameterGroupNumber::VirtualTerminalToECU), process_rx_message, this);
        // self.partnerControlFunction->add_parameter_group_number_callback(static_cast<std::uint32_t>(CANLibParameterGroupNumber::Acknowledge), process_rx_message, this);
        // CANNetworkManager::CANNetwork.add_global_parameter_group_number_callback(static_cast<std::uint32_t>(CANLibParameterGroupNumber::VirtualTerminalToECU), process_rx_message, this);
        // CANNetworkManager::CANNetwork.add_global_parameter_group_number_callback(static_cast<std::uint32_t>(CANLibParameterGroupNumber::ECUtoVirtualTerminal), process_rx_message, this);
        // network_manager.add_global_parameter_group_number_callback(ParameterGroupNumber::VirtualTerminalToECU, & |m| { self.process_can_message(m) });
        // network_manager.add_global_parameter_group_number_callback(ParameterGroupNumber::ECUtoVirtualTerminal, & |m| { self.process_can_message(m) });

        // if (!languageCommandInterface.get_initialized()) {
        // 	languageCommandInterface.initialize();
        // }

        self.internal_control_function.initialize();
        // self.partnered_control_function.initialize();

        self.is_initialized = true;
    }

    pub fn terminate(&mut self) {
        if !self.is_initialized { return; }

        // if ((StateMachineState::Connected == state) && (send_delete_object_pool())) {
        //		CANStackLogger::debug("[VT]: Requested object pool deletion from volatile VT memory.");
        // }

        // // Remove the callbacks to CAN messages used by the Virtual Termainal Client
        // partnerControlFunction->remove_parameter_group_number_callback(static_cast<std::uint32_t>(CANLibParameterGroupNumber::VirtualTerminalToECU), process_rx_message, this);
        // partnerControlFunction->remove_parameter_group_number_callback(static_cast<std::uint32_t>(CANLibParameterGroupNumber::Acknowledge), process_rx_message, this);
        // CANNetworkManager::CANNetwork.remove_global_parameter_group_number_callback(static_cast<std::uint32_t>(CANLibParameterGroupNumber::VirtualTerminalToECU), process_rx_message, this);
        // CANNetworkManager::CANNetwork.remove_global_parameter_group_number_callback(static_cast<std::uint32_t>(CANLibParameterGroupNumber::ECUtoVirtualTerminal), process_rx_message, this);

        // shouldTerminate = true;
        self.current_state = State::Disconnected;

        self.internal_control_function.terminate();
        // self.partnered_control_function.terminate()

        self.is_initialized = false;
        log::info!("[VT]: VT Client connection has been terminated.");
    }

    pub fn restart_communication(&mut self) {
        log::info!("[VT]: VT Client connection restart requested. Client will now terminate and reinitialize.");
        self.terminate();
        self.initialize();
    }


    pub fn is_connected(&self) -> bool {
        self.current_state == State::Connected
    }


    pub fn set_object_pool(
        &mut self,
        pool_index: usize,
        object_pool: ObjectPool,
    ) {
        // if ((nullptr != pool) &&
        //     (0 != size))
        // {
        // 	ObjectPoolDataStruct tempData;

        // 	tempData.objectPoolDataPointer = pool;
        // 	tempData.objectPoolVectorPointer = nullptr;
        // 	tempData.dataCallback = nullptr;
        // 	tempData.objectPoolSize = size;
        // 	tempData.autoScaleDataMaskOriginalDimension = 0;
        // 	tempData.autoScaleSoftKeyDesignatorOriginalHeight = 0;
        // 	tempData.version = poolSupportedVTVersion;
        // 	tempData.useDataCallback = false;
        // 	tempData.uploaded = false;
        // 	tempData.versionLabel = version;

        let _ = self.object_pools.insert(pool_index, object_pool);
    }

    pub fn next_event(&mut self) -> Option<Event> {
        self.event_queue.pop_front()
    }


    pub fn update(&mut self, network_manager: &mut CanNetworkManager) {
        // Firt update the connected control functions.
        self.internal_control_function.update(network_manager);
        self.partnered_control_function.update(network_manager);

        // Process received messages and update internal state.
        network_manager.handle_message(|message| self.process_can_message(message));

        // Limit the size of the event queue, by removing the oldest events.
        // In Rust, using the event queue is prefered over using callbacks.
        while self.event_queue.len() > MAX_EVENT_QUEUE_SIZE {
            self.event_queue.pop_front();
        }

        // 
        let previous_state = self.current_state;

        // Do stuff based on the current internal state.
        match self.current_state {
            State::Disconnected => {
                self.send_working_set_maintenance = false;
                self.send_auxiliary_maintenance = false;
    
                if self.partnered_control_function.is_address_valid() {
                    self.current_state = State::WaitForPartnerVTStatusMessage
                }
            }
            State::WaitForPartnerVTStatusMessage => {
                if self.last_vtstatus_timestamp_ms != Duration::default() {
                    self.current_state = State::SendWorkingSetMasterMessage;
                }
            }
            State::SendWorkingSetMasterMessage => {
                self.send_working_set_maintenance_message(network_manager);
                self.current_state = State::ReadyForObjectPool;
            }
    
        // 		case StateMachineState::ReadyForObjectPool:
        // 		{
        // 			// If we're in this state, we are ready to upload the
        // 			// object pool but no pool has been set to this class
        // 			// so the state machine cannot progress.
        // 			if (SystemTiming::time_expired_ms(lastVTStatusTimestamp_ms, VT_STATUS_TIMEOUT_MS))
        // 			{
        // 				CANStackLogger::CAN_stack_log(CANStackLogger::LoggingLevel::Error, "[VT]: Ready to upload pool, but VT server has timed out. Disconnecting.");
        // 				set_state(StateMachineState::Disconnected);
        // 			}
    
        // 			if (0 != objectPools.size())
        // 			{
        // 				set_state(StateMachineState::SendGetMemory);
        // 				send_working_set_maintenance(true, objectPools[0].version);
        // 				lastWorkingSetMaintenanceTimestamp_ms = SystemTiming::get_timestamp_ms();
        // 				sendWorkingSetMaintenance = true;
        // 				sendAuxiliaryMaintenance = true;
        // 			}
        // 		}
        // 		break;
    
        // 		case StateMachineState::SendGetMemory:
        // 		{
        // 			std::uint32_t totalPoolSize = 0;
    
        // 			for (auto &pool : objectPools)
        // 			{
        // 				totalPoolSize += pool.objectPoolSize;
        // 			}
    
        // 			if (send_get_memory(totalPoolSize))
        // 			{
        // 				set_state(StateMachineState::WaitForGetMemoryResponse);
        // 			}
        // 		}
        // 		break;
    
        // 		case StateMachineState::WaitForGetMemoryResponse:
        // 		{
        // 			if (SystemTiming::time_expired_ms(stateMachineTimestamp_ms, VT_STATUS_TIMEOUT_MS))
        // 			{
        // 				set_state(StateMachineState::Failed);
        // 				CANStackLogger::CAN_stack_log(CANStackLogger::LoggingLevel::Error, "[VT]: Get Memory Response Timeout");
        // 			}
        // 		}
        // 		break;
    
        // 		case StateMachineState::SendGetNumberSoftkeys:
        // 		{
        // 			if (send_get_number_of_softkeys())
        // 			{
        // 				set_state(StateMachineState::WaitForGetNumberSoftKeysResponse);
        // 			}
        // 		}
        // 		break;
    
        // 		case StateMachineState::WaitForGetNumberSoftKeysResponse:
        // 		{
        // 			if (SystemTiming::time_expired_ms(stateMachineTimestamp_ms, VT_STATUS_TIMEOUT_MS))
        // 			{
        // 				set_state(StateMachineState::Failed);
        // 				CANStackLogger::CAN_stack_log(CANStackLogger::LoggingLevel::Error, "[VT]: Get Number Softkeys Response Timeout");
        // 			}
        // 		}
        // 		break;
    
        // 		case StateMachineState::SendGetTextFontData:
        // 		{
        // 			if (send_get_text_font_data())
        // 			{
        // 				set_state(StateMachineState::WaitForGetTextFontDataResponse);
        // 			}
        // 		}
        // 		break;
    
        // 		case StateMachineState::WaitForGetTextFontDataResponse:
        // 		{
        // 			if (SystemTiming::time_expired_ms(stateMachineTimestamp_ms, VT_STATUS_TIMEOUT_MS))
        // 			{
        // 				set_state(StateMachineState::Failed);
        // 				CANStackLogger::CAN_stack_log(CANStackLogger::LoggingLevel::Error, "[VT]: Get Text Font Data Response Timeout");
        // 			}
        // 		}
        // 		break;
    
        // 		case StateMachineState::SendGetHardware:
        // 		{
        // 			if (send_get_hardware())
        // 			{
        // 				set_state(StateMachineState::WaitForGetHardwareResponse);
        // 			}
        // 		}
        // 		break;
    
        // 		case StateMachineState::WaitForGetHardwareResponse:
        // 		{
        // 			if (SystemTiming::time_expired_ms(stateMachineTimestamp_ms, VT_STATUS_TIMEOUT_MS))
        // 			{
        // 				set_state(StateMachineState::Failed);
        // 				CANStackLogger::CAN_stack_log(CANStackLogger::LoggingLevel::Error, "[VT]: Get Hardware Response Timeout");
        // 			}
        // 		}
        // 		break;
    
        // 		case StateMachineState::SendGetVersions:
        // 		{
        // 			if (SystemTiming::time_expired_ms(stateMachineTimestamp_ms, VT_STATUS_TIMEOUT_MS))
        // 			{
        // 				set_state(StateMachineState::Failed);
        // 				CANStackLogger::CAN_stack_log(CANStackLogger::LoggingLevel::Error, "[VT]: Get Versions Timeout");
        // 			}
        // 			else if ((!objectPools.empty()) &&
        // 			         (!objectPools[0].versionLabel.empty()) &&
        // 			         (send_get_versions()))
        // 			{
        // 				set_state(StateMachineState::WaitForGetVersionsResponse);
        // 			}
        // 		}
        // 		break;
    
        // 		case StateMachineState::WaitForGetVersionsResponse:
        // 		{
        // 			if (SystemTiming::time_expired_ms(stateMachineTimestamp_ms, VT_STATUS_TIMEOUT_MS))
        // 			{
        // 				set_state(StateMachineState::Failed);
        // 				CANStackLogger::CAN_stack_log(CANStackLogger::LoggingLevel::Error, "[VT]: Get Versions Response Timeout");
        // 			}
        // 		}
        // 		break;
    
        // 		case StateMachineState::SendLoadVersion:
        // 		{
        // 			if (SystemTiming::time_expired_ms(stateMachineTimestamp_ms, VT_STATUS_TIMEOUT_MS))
        // 			{
        // 				set_state(StateMachineState::Failed);
        // 				CANStackLogger::CAN_stack_log(CANStackLogger::LoggingLevel::Error, "[VT]: Send Load Version Timeout");
        // 			}
        // 			else
        // 			{
        // 				constexpr std::uint8_t VERSION_LABEL_LENGTH = 7;
        // 				std::array<std::uint8_t, VERSION_LABEL_LENGTH> tempVersionBuffer;
    
        // 				// Unused bytes filled with spaces
        // 				tempVersionBuffer[0] = ' ';
        // 				tempVersionBuffer[1] = ' ';
        // 				tempVersionBuffer[2] = ' ';
        // 				tempVersionBuffer[3] = ' ';
        // 				tempVersionBuffer[4] = ' ';
        // 				tempVersionBuffer[5] = ' ';
        // 				tempVersionBuffer[6] = ' ';
    
        // 				for (std::size_t i = 0; ((i < VERSION_LABEL_LENGTH) && (i < objectPools[0].versionLabel.size())); i++)
        // 				{
        // 					tempVersionBuffer[i] = objectPools[0].versionLabel[i];
        // 				}
    
        // 				if (send_load_version(tempVersionBuffer))
        // 				{
        // 					set_state(StateMachineState::WaitForLoadVersionResponse);
        // 				}
        // 			}
        // 		}
        // 		break;
    
        // 		case StateMachineState::WaitForLoadVersionResponse:
        // 		{
        // 			if (SystemTiming::time_expired_ms(stateMachineTimestamp_ms, VT_STATUS_TIMEOUT_MS))
        // 			{
        // 				set_state(StateMachineState::Failed);
        // 				CANStackLogger::CAN_stack_log(CANStackLogger::LoggingLevel::Error, "[VT]: Load Version Response Timeout");
        // 			}
        // 		}
        // 		break;
    
        // 		case StateMachineState::SendStoreVersion:
        // 		{
        // 			if (SystemTiming::time_expired_ms(stateMachineTimestamp_ms, VT_STATUS_TIMEOUT_MS))
        // 			{
        // 				set_state(StateMachineState::Failed);
        // 				CANStackLogger::CAN_stack_log(CANStackLogger::LoggingLevel::Error, "[VT]: Send Store Version Timeout");
        // 			}
        // 			else
        // 			{
        // 				constexpr std::uint8_t VERSION_LABEL_LENGTH = 7;
        // 				std::array<std::uint8_t, VERSION_LABEL_LENGTH> tempVersionBuffer;
    
        // 				// Unused bytes filled with spaces
        // 				tempVersionBuffer[0] = ' ';
        // 				tempVersionBuffer[1] = ' ';
        // 				tempVersionBuffer[2] = ' ';
        // 				tempVersionBuffer[3] = ' ';
        // 				tempVersionBuffer[4] = ' ';
        // 				tempVersionBuffer[5] = ' ';
        // 				tempVersionBuffer[6] = ' ';
    
        // 				for (std::size_t i = 0; ((i < VERSION_LABEL_LENGTH) && (i < objectPools[0].versionLabel.size())); i++)
        // 				{
        // 					tempVersionBuffer[i] = objectPools[0].versionLabel[i];
        // 				}
    
        // 				if (send_store_version(tempVersionBuffer))
        // 				{
        // 					set_state(StateMachineState::WaitForStoreVersionResponse);
        // 				}
        // 			}
        // 		}
        // 		break;
    
        // 		case StateMachineState::WaitForStoreVersionResponse:
        // 		{
        // 			if (SystemTiming::time_expired_ms(stateMachineTimestamp_ms, VT_STATUS_TIMEOUT_MS))
        // 			{
        // 				set_state(StateMachineState::Failed);
        // 				CANStackLogger::CAN_stack_log(CANStackLogger::LoggingLevel::Error, "[VT]: Store Version Response Timeout");
        // 			}
        // 		}
        // 		break;
    
        // 		case StateMachineState::UploadObjectPool:
        // 		{
        // 			bool allPoolsProcessed = true;
    
        // 			if (firstTimeInState)
        // 			{
        // 				if (get_any_pool_needs_scaling())
        // 				{
        // 					// Scale object pools before upload.
        // 					if (!scale_object_pools())
        // 					{
        // 						set_state(StateMachineState::Failed);
        // 					}
        // 				}
        // 			}
    
        // 			for (std::uint32_t i = 0; i < objectPools.size(); i++)
        // 			{
        // 				if (((nullptr != objectPools[i].objectPoolDataPointer) ||
        // 				     (nullptr != objectPools[i].dataCallback)) &&
        // 				    (objectPools[i].objectPoolSize > 0))
        // 				{
        // 					if (!objectPools[i].uploaded)
        // 					{
        // 						allPoolsProcessed = false;
        // 					}
    
        // 					if (CurrentObjectPoolUploadState::Uninitialized == currentObjectPoolState)
        // 					{
        // 						if (!objectPools[i].uploaded)
        // 						{
        // 							bool transmitSuccessful = CANNetworkManager::CANNetwork.send_can_message(static_cast<std::uint32_t>(CANLibParameterGroupNumber::ECUtoVirtualTerminal),
        // 							                                                                         nullptr,
        // 							                                                                         objectPools[i].objectPoolSize + 1, // Account for Mux byte
        // 							                                                                         myControlFunction.get(),
        // 							                                                                         partnerControlFunction.get(),
        // 							                                                                         CANIdentifier::CANPriority::PriorityLowest7,
        // 							                                                                         process_callback,
        // 							                                                                         this,
        // 							                                                                         process_internal_object_pool_upload_callback);
    
        // 							if (transmitSuccessful)
        // 							{
        // 								currentObjectPoolState = CurrentObjectPoolUploadState::InProgress;
        // 							}
        // 						}
        // 						else
        // 						{
        // 							// Pool already uploaded, move on to the next one
        // 						}
        // 					}
        // 					else if (CurrentObjectPoolUploadState::Success == currentObjectPoolState)
        // 					{
        // 						objectPools[i].uploaded = true;
        // 						currentObjectPoolState = CurrentObjectPoolUploadState::Uninitialized;
        // 					}
        // 					else if (CurrentObjectPoolUploadState::Failed == currentObjectPoolState)
        // 					{
        // 						currentObjectPoolState = CurrentObjectPoolUploadState::Uninitialized;
        // 						CANStackLogger::CAN_stack_log(CANStackLogger::LoggingLevel::Error, "[VT]: An object pool failed to upload. Resetting connection to VT.");
        // 						set_state(StateMachineState::Disconnected);
        // 					}
        // 					else
        // 					{
        // 						// Transfer is in progress. Nothing to do now.
        // 						break;
        // 					}
        // 				}
        // 				else
        // 				{
        // 					CANStackLogger::CAN_stack_log(CANStackLogger::LoggingLevel::Warning, "[VT]: An object pool was supplied with an invalid size or pointer. Ignoring it.");
        // 					objectPools[i].uploaded = true;
        // 				}
        // 			}
    
        // 			if (allPoolsProcessed)
        // 			{
        // 				set_state(StateMachineState::SendEndOfObjectPool);
        // 			}
        // 		}
        // 		break;
    
        // 		case StateMachineState::SendEndOfObjectPool:
        // 		{
        // 			if (send_end_of_object_pool())
        // 			{
        // 				set_state(StateMachineState::WaitForEndOfObjectPoolResponse);
        // 			}
        // 		}
        // 		break;
    
        // 		case StateMachineState::WaitForEndOfObjectPoolResponse:
        // 		{
        // 			if (SystemTiming::time_expired_ms(stateMachineTimestamp_ms, VT_STATUS_TIMEOUT_MS))
        // 			{
        // 				set_state(StateMachineState::Failed);
        // 				CANStackLogger::CAN_stack_log(CANStackLogger::LoggingLevel::Error, "[VT]: Get End of Object Pool Response Timeout");
        // 			}
        // 		}
        // 		break;
    
        // 		case StateMachineState::Connected:
        // 		{
        // 			// Check for timeouts
        // 			if (SystemTiming::time_expired_ms(lastVTStatusTimestamp_ms, VT_STATUS_TIMEOUT_MS))
        // 			{
        // 				set_state(StateMachineState::Disconnected);
        // 				CANStackLogger::CAN_stack_log(CANStackLogger::LoggingLevel::Error, "[VT]: Status Timeout");
        // 			}
        // 			update_auxiliary_input_status();
        // 		}
        // 		break;
    
        // 		case StateMachineState::Failed:
        // 		{
        // 			constexpr std::uint32_t VT_STATE_MACHINE_RETRY_TIMEOUT_MS = 5000;
        // 			sendWorkingSetMaintenance = false;
        // 			sendAuxiliaryMaintenance = false;
    
        // 			// Retry connecting after a while
        // 			if (SystemTiming::time_expired_ms(stateMachineTimestamp_ms, VT_STATE_MACHINE_RETRY_TIMEOUT_MS))
        // 			{
        // 				CANStackLogger::CAN_stack_log(CANStackLogger::LoggingLevel::Info, "[VT]: Resetting Failed VT Connection");
        // 				set_state(StateMachineState::Disconnected);
        // 			}
        // 		}
        // 		break;
    
        // 		default:
        // 		{
        // 		}
        // 		break;
        // 	}
            _ => {}
        }
        // else
        // {
        // 	set_state(StateMachineState::Disconnected);
        // }
    

        // Send a Workingset maintenance message every second.
        if self.send_working_set_maintenance &&
            TimeDriver::time_elapsed() >= self.last_working_set_maintenance_timestamp + WORKING_SET_MAINTENANCE_TIMEOUT
        {
            self.send_working_set_maintenance_message(network_manager);
        }

        // if ((sendAuxiliaryMaintenance) &&
        //     (!ourAuxiliaryInputs.empty()) &&
        //     (SystemTiming::time_expired_ms(lastAuxiliaryMaintenanceTimestamp_ms, AUXILIARY_MAINTENANCE_TIMEOUT_MS)))
        // {
        // 	/// TODO: We should make sure that when we disconnect/reconnect atleast 500ms has passed since the last auxiliary maintenance message
        // 	txFlags.set_flag(static_cast<std::uint32_t>(TransmitFlags::SendAuxiliaryMaintenance));
        // }
        // txFlags.process_all_flags();
    
        if self.current_state == previous_state {
        	self.first_time_in_state = false;
        } else {
            self.first_time_in_state = true;
        }
    }

    pub fn process_can_message(&mut self, message: &CanMessage) -> bool {
        let handled = false;

        match message.pgn() {
            ParameterGroupNumber::Acknowledge => {
                // if AcknowledgementType::Negative as u8 == message.data()[0] {
                // 	if ParameterGroupNumber::ECUtoVirtualTerminal == message.data()[5..8].into() {
                // 		log::error!("[VT]: The VT Server is NACK-ing our VT messages. Disconnecting.");
                // 		self.set_state(StateMachineState::Disconnected);
                // 	}
                // }
            }
            ParameterGroupNumber::VirtualTerminalToECU => {
                if let Ok(vt_function) = message.get_u8_at(0).try_into() {
                    match vt_function {
                        VTFunction::SoftKeyActivationMessage
                        | VTFunction::ButtonActivationMessage => {
                            let key_event: KeyActivationCode = message
                                .get_u8_at(1)
                                .try_into()
                                .unwrap_or_else(|_| KeyActivationCode::ButtonPressAborted);
                            let object_id: u16 = message.get_u16_at(2);
                            let parent_object_id: u16 = message.get_u16_at(4);
                            let key_number: u8 = message.get_u8_at(6);
                            // if self.partnered_control_function.get_vt_version_supported(VTVersion::Version6) {
                            // 	// TODO: process TAN
                            // }
                            let event = VTKeyEvent {
                                object_id,
                                parent_object_id,
                                key_number,
                                key_event,
                            };

                            // Call all of the callbacks, passing in a copy of the event.
                            for (_, callback) in &self.soft_key_event_callbacks {
                                callback(event);
                            }

                            // Push a copy of the event to the event queue.
                            self.event_queue.push_back(Event::VTKeyEvent(event));
                        }
                        // case static_cast<std::uint8_t>(Function::PointingEventMessage):
                        // {
                        // 	std::uint16_t xPosition = message.get_uint16_at(1);
                        // 	std::uint16_t yPosition = message.get_uint16_at(3);
                        // 	std::uint8_t touchState = static_cast<std::uint8_t>(KeyActivationCode::ButtonPressedOrLatched);
                        // 	std::uint16_t parentMaskObjectID = NULL_OBJECT_ID;
                        // 	if (parentVT->get_vt_version_supported(VTVersion::Version6))
                        // 	{
                        // 		// VT version is at least 6
                        // 		touchState = message.get_uint8_at(5) & 0x0F;
                        // 		parentMaskObjectID = message.get_uint16_at(6);
                        // 		//! @todo process TAN
                        // 	}
                        // 	else if (parentVT->get_vt_version_supported(VTVersion::Version4))
                        // 	{
                        // 		// VT version is either 4 or 5
                        // 		touchState = message.get_uint8_at(5);
                        // 	}
                        // 	if (touchState <= static_cast<std::uint8_t>(KeyActivationCode::ButtonPressAborted))
                        // 	{
                        // 		parentVT->pointingEventDispatcher.invoke({ parentVT, xPosition, yPosition, parentMaskObjectID, static_cast<KeyActivationCode>(touchState) });
                        // 	}
                        // }
                        // break;
                        // case static_cast<std::uint8_t>(Function::VTSelectInputObjectMessage):
                        // {
                        // 	std::uint16_t objectID = message.get_uint16_at(1);
                        // 	bool objectSelected = (0x01 == message.get_uint8_at(3));
                        // 	bool objectOpenForInput = true;
                        // 	if (parentVT->get_vt_version_supported(VTVersion::Version4))
                        // 	{
                        // 		objectOpenForInput = message.get_bool_at(4, 0);
                        // 	}
                        // 	if (parentVT->get_vt_version_supported(VTVersion::Version6))
                        // 	{
                        // 		//! @todo process TAN
                        // 	}
                        // 	parentVT->selectInputObjectEventDispatcher.invoke({ parentVT, objectID, objectSelected, objectOpenForInput });
                        // }
                        // break;
                        // case static_cast<std::uint8_t>(Function::VTESCMessage):
                        // {
                        // 	std::uint16_t objectID = message.get_uint16_at(1);
                        // 	std::uint8_t errorCode = message.get_uint8_at(3) & 0x1F;
                        // 	if ((errorCode == static_cast<std::uint8_t>(ESCMessageErrorCode::OtherError)) ||
                        // 	    (errorCode <= static_cast<std::uint8_t>(ESCMessageErrorCode::NoInputFieldOpen)))
                        // 	{
                        // 		if (parentVT->get_vt_version_supported(VTVersion::Version6))
                        // 		{
                        // 			//! @todo process TAN
                        // 		}
                        // 		parentVT->escMessageEventDispatcher.invoke({ parentVT, objectID, static_cast<ESCMessageErrorCode>(errorCode) });
                        // 	}
                        // }
                        // break;
                        // case static_cast<std::uint8_t>(Function::VTChangeNumericValueMessage):
                        // {
                        // 	std::uint16_t objectID = message.get_uint16_at(1);
                        // 	std::uint32_t value = message.get_uint32_at(4);
                        // 	if (parentVT->get_vt_version_supported(VTVersion::Version6))
                        // 	{
                        // 		//! @todo process TAN
                        // 	}
                        // 	parentVT->changeNumericValueEventDispatcher.invoke({ parentVT, value, objectID });
                        // }
                        // break;
                        // case static_cast<std::uint8_t>(Function::VTChangeActiveMaskMessage):
                        // {
                        // 	std::uint16_t maskObjectID = message.get_uint16_at(1);
                        // 	bool missingObjects = message.get_bool_at(3, 2);
                        // 	bool maskOrChildHasErrors = message.get_bool_at(3, 3);
                        // 	bool anyOtherError = message.get_bool_at(3, 4);
                        // 	bool poolDeleted = message.get_bool_at(3, 5);
                        // 	std::uint16_t errorObjectID = message.get_uint16_at(4);
                        // 	std::uint16_t parentObjectID = message.get_uint16_at(6);
                        // 	parentVT->changeActiveMaskEventDispatcher.invoke({ parentVT,
                        // 	                                                   maskObjectID,
                        // 	                                                   errorObjectID,
                        // 	                                                   parentObjectID,
                        // 	                                                   missingObjects,
                        // 	                                                   maskOrChildHasErrors,
                        // 	                                                   anyOtherError,
                        // 	                                                   poolDeleted });
                        // }
                        // break;
                        // case static_cast<std::uint8_t>(Function::VTChangeSoftKeyMaskMessage):
                        // {
                        // 	std::uint16_t dataOrAlarmMaskID = message.get_uint16_at(1);
                        // 	std::uint16_t softKeyMaskID = message.get_uint16_at(3);
                        // 	bool missingObjects = message.get_bool_at(5, 2);
                        // 	bool maskOrChildHasErrors = message.get_bool_at(5, 3);
                        // 	bool anyOtherError = message.get_bool_at(5, 4);
                        // 	bool poolDeleted = message.get_bool_at(5, 5);
                        // 	parentVT->changeSoftKeyMaskEventDispatcher.invoke({ parentVT,
                        // 	                                                    dataOrAlarmMaskID,
                        // 	                                                    softKeyMaskID,
                        // 	                                                    missingObjects,
                        // 	                                                    maskOrChildHasErrors,
                        // 	                                                    anyOtherError,
                        // 	                                                    poolDeleted });
                        // }
                        // break;
                        // case static_cast<std::uint8_t>(Function::VTChangeStringValueMessage):
                        // {
                        // 	std::uint16_t objectID = message.get_uint16_at(1);
                        // 	std::uint8_t stringLength = message.get_uint8_at(3);
                        // 	std::string value = std::string(message.get_data().begin() + 4, message.get_data().begin() + 4 + stringLength);
                        // 	parentVT->changeStringValueEventDispatcher.invoke({ value, parentVT, objectID });
                        // }
                        // break;
                        // case static_cast<std::uint8_t>(Function::VTOnUserLayoutHideShowMessage):
                        // {
                        // 	std::uint16_t objectID = message.get_uint16_at(1);
                        // 	bool hidden = !message.get_bool_at(3, 0);
                        // 	parentVT->userLayoutHideShowEventDispatcher.invoke({ parentVT, objectID, hidden });
                        // 	// There could be two layout messages in one packet
                        // 	objectID = message.get_uint16_at(4);
                        // 	if (objectID != NULL_OBJECT_ID)
                        // 	{
                        // 		hidden = !message.get_bool_at(6, 0);
                        // 		parentVT->userLayoutHideShowEventDispatcher.invoke({ parentVT, objectID, hidden });
                        // 	}
                        // 	if (parentVT->get_vt_version_supported(VTVersion::Version6))
                        // 	{
                        // 		//! @todo process TAN
                        // 	}
                        // }
                        // break;
                        // case static_cast<std::uint8_t>(Function::VTControlAudioSignalTerminationMessage):
                        // {
                        // 	bool terminated = message.get_bool_at(1, 0);
                        // 	parentVT->audioSignalTerminationEventDispatcher.invoke({ parentVT, terminated });
                        // 	if (parentVT->get_vt_version_supported(VTVersion::Version6))
                        // 	{
                        // 		//! @todo process TAN
                        // 	}
                        // }
                        // break;
                        VTFunction::PreferredAssignmentCommand => {
                            if message.get_bool_at(1, 0) {
                                log::error!("[AUX-N]: Preferred Assignment Error - Auxiliary Input Unit(s) (NAME or Model Identification Code) not valid");
                            }
                            if message.get_bool_at(1, 1) {
                                log::error!("[AUX-N]: Preferred Assignment Error - Function Object ID(S) not valid");
                            }
                            if message.get_bool_at(1, 2) {
                                log::error!("[AUX-N]: Preferred Assignment Error - Input Object ID(s) not valid");
                            }
                            if message.get_bool_at(1, 3) {
                                log::error!("[AUX-N]: Preferred Assignment Error - Duplicate Object ID of Auxiliary Function");
                            }
                            if message.get_bool_at(1, 4) {
                                log::error!("[AUX-N]: Preferred Assignment Error - Other");
                            }
                            if message.get_u8_at(1) != 0 {
                                log::error!("[AUX-N]: Auxiliary Function Object ID of faulty assignment: {}", message.get_u16_at(2));
                            } else {
                                // log::debug!("[AUX-N]: Preferred Assignment OK");
                                // TODO: load the preferred assignment into parentVT->assignedAuxiliaryInputDevices
                            }
                        } // 		break;
                          // 		case static_cast<std::uint8_t>(Function::AuxiliaryAssignmentTypeTwoCommand):
                          // 		{
                          // 			if (14 == message.get_data_length())
                          // 			{
                          // 				std::uint64_t isoName = message.get_uint64_at(1);
                          // 				bool storeAsPreferred = message.get_bool_at(9, 7);
                          // 				std::uint8_t functionType = (message.get_uint8_at(9) & 0x1F);
                          // 				std::uint16_t inputObjectID = message.get_uint16_at(10);
                          // 				std::uint16_t functionObjectID = message.get_uint16_at(12);
                          // 				bool hasError = false;
                          // 				bool isAlreadyAssigned = false;
                          // 				if (DEFAULT_NAME == isoName && 0x1F == functionType)
                          // 				{
                          // 					if (NULL_OBJECT_ID == functionObjectID)
                          // 					{
                          // 						for (AssignedAuxiliaryInputDevice &aux : parentVT->assignedAuxiliaryInputDevices)
                          // 						{
                          // 							aux.functions.clear();
                          // 						}
                          // 						CANStackLogger::CAN_stack_log(CANStackLogger::LoggingLevel::Info, "[AUX-N] Unassigned all functions");
                          // 					}
                          // 					else if (NULL_OBJECT_ID == inputObjectID)
                          // 					{
                          // 						for (AssignedAuxiliaryInputDevice &aux : parentVT->assignedAuxiliaryInputDevices)
                          // 						{
                          // 							for (auto iter = aux.functions.begin(); iter != aux.functions.end();)
                          // 							{
                          // 								if (iter->functionObjectID == functionObjectID)
                          // 								{
                          // 									aux.functions.erase(iter);
                          // 									if (storeAsPreferred)
                          // 									{
                          // 										//! @todo save preferred assignment to persistent configuration
                          // 									}
                          // 									CANStackLogger::CAN_stack_log(CANStackLogger::LoggingLevel::Info, "[AUX-N] Unassigned function " + isobus::to_string(static_cast<int>(functionObjectID)) + " from input " + isobus::to_string(static_cast<int>(inputObjectID)));
                          // 								}
                          // 								else
                          // 								{
                          // 									++iter;
                          // 								}
                          // 							}
                          // 						}
                          // 					}
                          // 				}
                          // 				else
                          // 				{
                          // 					auto result = std::find_if(parentVT->assignedAuxiliaryInputDevices.begin()>,assignedAuxiliaryInputDevices.end(), [&isoName](const AssignedAuxiliaryInputDevice &aux) {
                          // 						return aux.name == isoName;
                          // 					});
                          // 					if (result != std::end(parentVT->assignedAuxiliaryInputDevices))
                          // 					{
                          // 						if (static_cast<std::uint8_t>(AuxiliaryTypeTwoFunctionType::QuadratureBooleanMomentary) >= functionType)
                          // 						{
                          // 							AssignedAuxiliaryFunction assignment(functionObjectID, inputObjectID, static_cast<AuxiliaryTypeTwoFunctionType>(functionType));
                          // 							auto location = std::find(result->functions.begin()>,functions.end(), assignment);
                          // 							if (location == std::end(result->functions))
                          // 							{
                          // 								result->functions.push_back(assignment);
                          // 								if (storeAsPreferred)
                          // 								{
                          // 									//! @todo save preferred assignment to persistent configuration
                          // 								}
                          // 								CANStackLogger::CAN_stack_log(CANStackLogger::LoggingLevel::Info, "[AUX-N]: Assigned function " + isobus::to_string(static_cast<int>(functionObjectID)) + " to input " + isobus::to_string(static_cast<int>(inputObjectID)));
                          // 							}
                          // 							else
                          // 							{
                          // 								hasError = true;
                          // 								isAlreadyAssigned = true;
                          // 								CANStackLogger::CAN_stack_log(CANStackLogger::LoggingLevel::Warning, "[AUX-N]: Unable to store preferred assignment due to missing auxiliary input device with name: " + isobus::to_string(isoName));
                          // 							}
                          // 						}
                          // 						else
                          // 						{
                          // 							hasError = true;
                          // 							CANStackLogger::CAN_stack_log(CANStackLogger::LoggingLevel::Warning, "[AUX-N]: Unable to store preferred assignment due to unsupported function type: " + isobus::to_string(functionType));
                          // 						}
                          // 					}
                          // 					else
                          // 					{
                          // 						hasError = true;
                          // 						CANStackLogger::CAN_stack_log(CANStackLogger::LoggingLevel::Warning, "[AUX-N]: Unable to store preferred assignment due to missing auxiliary input device with name: " + isobus::to_string(isoName));
                          // 					}
                          // 				}
                          // 				parentVT->send_auxiliary_function_assignment_response(functionObjectID, hasError, isAlreadyAssigned);
                          // 			}
                          // 			else
                          // 			{
                          // 				CANStackLogger::CAN_stack_log(CANStackLogger::LoggingLevel::Warning, "[AUX-N]: Received AuxiliaryAssignmentTypeTwoCommand with wrong data length: " + isobus::to_string(message.get_data_length()) + " but expected 14.");
                          // 			}
                          // 		}
                          // 		break;
                          // 		case static_cast<std::uint8_t>(Function::AuxiliaryInputTypeTwoStatusMessage):
                          // 		{
                          // 			std::uint16_t inputObjectID = message.get_uint16_at(1);
                          // 			std::uint16_t value1 = message.get_uint16_at(3);
                          // 			std::uint16_t value2 = message.get_uint16_at(5);
                          // 			/// @todo figure out how to best pass other status properties below to application
                          // 			/// @todo The standard requires us to not perform any auxiliary function when learn mode is active, so we probably want to let the application know about that somehow
                          // 			// bool learnModeActive = message.get_bool_at(7, 0);
                          // 			// bool inputActive = message.get_bool_at(7, 1); // Only in learn mode?
                          // 			// bool controlIsLocked = false;
                          // 			// bool interactionWhileLocked = false;
                          // 			if (parentVT->get_vt_version_supported(VTVersion::Version6))
                          // 			{
                          // 				// controlIsLocked = message.get_bool_at(7, 2);
                          // 				// interactionWhileLocked = message.get_bool_at(7, 3);
                          // 			}
                          // 			for (AssignedAuxiliaryInputDevice &aux : parentVT->assignedAuxiliaryInputDevices)
                          // 			{
                          // 				auto result = std::find_if(aux.functions.begin(), aux.functions.end(), [&inputObjectID](const AssignedAuxiliaryFunction &assignment) {
                          // 					return assignment.inputObjectID == inputObjectID;
                          // 				});
                          // 				if (aux.functions.end() != result)
                          // 				{
                          // 					parentVT->auxiliaryFunctionEventDispatcher.invoke({ *result, parentVT, value1, value2 });
                          // 				}
                          // 			}
                          // 		}
                          // 		break;
                          // 		case static_cast<std::uint8_t>(Function::AuxiliaryInputStatusTypeTwoEnableCommand):
                          // 		{
                          // 			std::uint16_t inputObjectID = message.get_uint16_at(1);
                          // 			bool shouldEnable = message.get_bool_at(3, 0);
                          // 			auto result = std::find_if(parentVT->ourAuxiliaryInputs.begin()>,ourAuxiliaryInputs.end()>, &input) {
                          // 				return input.first == inputObjectID;
                          // 			});
                          // 			bool isInvalidObjectID = (result == std::end(parentVT->ourAuxiliaryInputs));
                          // 			if (!isInvalidObjectID)
                          // 			{
                          // 				result->second.enabled = shouldEnable;
                          // 			}
                          // 			parentVT->send_auxiliary_input_status_enable_response(inputObjectID, isInvalidObjectID ? false : shouldEnable, isInvalidObjectID);
                          // 		}
                          // 		break;
                          VTFunction::VTStatusMessage => {
                            self.last_vtstatus_timestamp_ms = TimeDriver::time_elapsed();
                            self.active_working_set_master_address = message.get_u8_at(1).into();
                            self.active_working_set_data_mask_object_id = message.get_u16_at(2).into();
                            self.active_working_set_soft_key_mask_object_id = message.get_u16_at(4).into();
                            self.busy_codes_bitfield = message.get_u8_at(6);
                            self.current_command_function_code = message.get_u8_at(7);
        
                            if (self.active_working_set_master_address == Address::GLOBAL
                                || self.active_working_set_master_address == Address::NULL
                                || self.active_working_set_master_address == message.source_address())
                                && self.current_state == State::WaitForPartnerVTStatusMessage
                            {
                                log::info!("[VT] Start connecting to VT: {}", message.source_address());
                                self.current_state = State::SendWorkingSetMasterMessage;
                            }
                        }
                        VTFunction::GetMemoryMessage => {
                            if State::WaitForGetMemoryResponse == self.current_state {
                                self.connected_vt_version = message.get_u8_at(1).into();
                                if 0 == message.get_u8_at(2) {
                                    // There IS enough memory
                                    self.current_state = State::SendGetNumberSoftkeys;
                                } else {
                                    self.current_state = State::Failed;
                                    log::error!("[VT]: Connection Failed, Not Enough Memory");
                                }
                            }
                        }
                        VTFunction::GetNumberOfSoftKeysMessage => {
                            // 			if (StateMachineState::WaitForGetNumberSoftKeysResponse == parentVT->state)
                            // 			{
                            // 				parentVT->softKeyXAxisPixels = message.get_uint8_at(4);
                            // 				parentVT->softKeyYAxisPixels = message.get_uint8_at(5);
                            // 				parentVT->numberVirtualSoftkeysPerSoftkeyMask = message.get_uint8_at(6);
                            // 				parentVT->numberPhysicalSoftkeys = message.get_uint8_at(7);
                            // 				parentVT->set_state(StateMachineState::SendGetTextFontData);
                            // 			}
                        }
                        VTFunction::GetTextFontDataMessage => {
                            // 			if (StateMachineState::WaitForGetTextFontDataResponse == parentVT->state)
                            // 			{
                            // 				parentVT->smallFontSizesBitfield = message.get_uint8_at(5);
                            // 				parentVT->largeFontSizesBitfield = message.get_uint8_at(6);
                            // 				parentVT->fontStylesBitfield = message.get_uint8_at(7);
                            // 				parentVT->set_state(StateMachineState::SendGetHardware);
                            // 			}
                        }
                        VTFunction::GetHardwareMessage => {
                            // 			if (StateMachineState::WaitForGetHardwareResponse == parentVT->state)
                            // 			{
                            // 				if (message.get_uint8_at(2) <= static_cast<std::uint8_t>(GraphicMode::TwoHundredFiftySixColour))
                            // 				{
                            // 					parentVT->supportedGraphicsMode = static_cast<GraphicMode>(message.get_uint8_at(2));
                            // 				}
                            // 				parentVT->hardwareFeaturesBitfield = message.get_uint8_at(3);
                            // 				parentVT->xPixels = message.get_uint16_at(4);
                            // 				parentVT->yPixels = message.get_uint16_at(6);
                            // 				parentVT->lastObjectPoolIndex = 0;
                            // 				// Check if we need to ask for pool versions
                            // 				// Ony check the first pool, all pools are labeled the same per working set.
                            // 				if ((!parentVT->objectPools.empty()) &&
                            // 				    (!parentVT->objectPools[0].versionLabel.empty()))
                            // 				{
                            // 					parentVT->set_state(StateMachineState::SendGetVersions);
                            // 				}
                            // 				else
                            // 				{
                            // 					parentVT->set_state(StateMachineState::UploadObjectPool);
                            // 				}
                            // 			}
                        }
                        VTFunction::GetVersionsResponse => {
                            // 			if (StateMachineState::WaitForGetVersionsResponse == parentVT->state)
                            // 			{
                            // 				// See if the server returned any labels
                            // 				const std::uint8_t numberOfLabels = message.get_uint8_at(1);
                            // 				constexpr std::size_t LABEL_LENGTH = 7;
                            // 				if (numberOfLabels > 0)
                            // 				{
                            // 					// Check for label match
                            // 					bool labelMatched = false;
                            // 					const std::size_t remainingLength = (2 + (LABEL_LENGTH * numberOfLabels));
                            // 					if (message.get_data_length() >= remainingLength)
                            // 					{
                            // 						for (std::uint_fast8_t i = 0; i < numberOfLabels; i++)
                            // 						{
                            // 							char tempStringLabel[8] = { 0 };
                            // 							tempStringLabel[0] = message.get_uint8_at(2 + (LABEL_LENGTH * i));
                            // 							tempStringLabel[1] = message.get_uint8_at(3 + (LABEL_LENGTH * i));
                            // 							tempStringLabel[2] = message.get_uint8_at(4 + (LABEL_LENGTH * i));
                            // 							tempStringLabel[3] = message.get_uint8_at(5 + (LABEL_LENGTH * i));
                            // 							tempStringLabel[4] = message.get_uint8_at(6 + (LABEL_LENGTH * i));
                            // 							tempStringLabel[5] = message.get_uint8_at(7 + (LABEL_LENGTH * i));
                            // 							tempStringLabel[6] = message.get_uint8_at(8 + (LABEL_LENGTH * i));
                            // 							tempStringLabel[7] = '\0';
                            // 							std::string labelDecoded(tempStringLabel);
                            // 							std::string tempActualLabel(parentVT->objectPools[0].versionLabel);
                            // 							// Check if we need to manipulate the passed in label by padding with spaces
                            // 							while (tempActualLabel.size() < LABEL_LENGTH)
                            // 							{
                            // 								tempActualLabel.push_back(' ');
                            // 							}
                            // 							if (tempActualLabel.size() > LABEL_LENGTH)
                            // 							{
                            // 								tempActualLabel.resize(LABEL_LENGTH);
                            // 							}
                            // 							if (tempActualLabel == labelDecoded)
                            // 							{
                            // 								labelMatched = true;
                            // 								parentVT->set_state(StateMachineState::SendLoadVersion);
                            // 								CANStackLogger::CAN_stack_log(CANStackLogger::LoggingLevel::Info, "[VT]: VT Server has a matching label for " + isobus::to_string(labelDecoded) + ". It will be loaded and upload will be skipped.");
                            // 								break;
                            // 							}
                            // 							else
                            // 							{
                            // 								CANStackLogger::CAN_stack_log(CANStackLogger::LoggingLevel::Info, "[VT]: VT Server has a label for " + isobus::to_string(labelDecoded) + ". This version will be deleted.");
                            // 								const std::array<std::uint8_t, 7> deleteBuffer = {
                            // 									static_cast<std::uint8_t>(labelDecoded[0]),
                            // 									static_cast<std::uint8_t>(labelDecoded[1]),
                            // 									static_cast<std::uint8_t>(labelDecoded[2]),
                            // 									static_cast<std::uint8_t>(labelDecoded[3]),
                            // 									static_cast<std::uint8_t>(labelDecoded[4]),
                            // 									static_cast<std::uint8_t>(labelDecoded[5]),
                            // 									static_cast<std::uint8_t>(labelDecoded[6])
                            // 								};
                            // 								if (!parentVT->send_delete_version(deleteBuffer))
                            // 								{
                            // 									CANStackLogger::CAN_stack_log(CANStackLogger::LoggingLevel::Warning, "[VT]: Failed to send the delete version message for label " + isobus::to_string(labelDecoded));
                            // 								}
                            // 							}
                            // 						}
                            // 						if (!labelMatched)
                            // 						{
                            // 							CANStackLogger::CAN_stack_log(CANStackLogger::LoggingLevel::Info, "[VT]: No version label from the VT matched. Client will upload the pool and store it instead.");
                            // 							parentVT->set_state(StateMachineState::UploadObjectPool);
                            // 						}
                            // 					}
                            // 					else
                            // 					{
                            // 						CANStackLogger::CAN_stack_log(CANStackLogger::LoggingLevel::Warning, "[VT]: Get Versions Response length is not long enough. Message ignored.");
                            // 					}
                            // 				}
                            // 				else
                            // 				{
                            // 					CANStackLogger::CAN_stack_log(CANStackLogger::LoggingLevel::Info, "[VT]: No version label from the VT matched. Client will upload the pool and store it instead.");
                            // 					parentVT->set_state(StateMachineState::UploadObjectPool);
                            // 				}
                            // 			}
                            // 			else
                            // 			{
                            // 				CANStackLogger::CAN_stack_log(CANStackLogger::LoggingLevel::Warning, "[VT]: Get Versions Response ignored!");
                            // 			}
                        }
                        VTFunction::LoadVersionCommand => {
                            // 			if (StateMachineState::WaitForLoadVersionResponse == parentVT->state)
                            // 			{
                            // 				if (0 == message.get_uint8_at(5))
                            // 				{
                            // 					CANStackLogger::CAN_stack_log(CANStackLogger::LoggingLevel::Info, "[VT]: Loaded object pool version from VT non-volatile memory with no errors.");
                            // 					parentVT->set_state(StateMachineState::Connected);
                            // 					//! @todo maybe a better way available than relying on aux function callbacks registered?
                            // 					if (parentVT->auxiliaryFunctionEventDispatcher.get_listener_count() > 0)
                            // 					{
                            // 						if (parentVT->send_auxiliary_functions_preferred_assignment())
                            // 						{
                            // 							CANStackLogger::CAN_stack_log(CANStackLogger::LoggingLevel::Debug, "[AUX-N]: Sent preferred assignments after LoadVersionCommand.");
                            // 						}
                            // 						else
                            // 						{
                            // 							CANStackLogger::CAN_stack_log(CANStackLogger::LoggingLevel::Warning, "[AUX-N]: Failed to send preferred assignments after LoadVersionCommand.");
                            // 						}
                            // 					}
                            // 				}
                            // 				else
                            // 				{
                            // 					// At least one error is set
                            // 					if (message.get_bool_at(5, 0))
                            // 					{
                            // 						CANStackLogger::CAN_stack_log(CANStackLogger::LoggingLevel::Warning, "[VT]: Load Versions Response error: File system error or corruption.");
                            // 					}
                            // 					if (message.get_bool_at(5, 1))
                            // 					{
                            // 						CANStackLogger::CAN_stack_log(CANStackLogger::LoggingLevel::Warning, "[VT]: Load Versions Response error: Insufficient memory.");
                            // 					}
                            // 					if (message.get_bool_at(5, 2))
                            // 					{
                            // 						CANStackLogger::CAN_stack_log(CANStackLogger::LoggingLevel::Warning, "[VT]: Load Versions Response error: Any other error.");
                            // 					}
                            // 					// Not sure what happened here... should be mostly impossible. Try to upload instead.
                            // 					CANStackLogger::CAN_stack_log(CANStackLogger::LoggingLevel::Warning, "[VT]: Switching to pool upload instead.");
                            // 					parentVT->set_state(StateMachineState::UploadObjectPool);
                            // 				}
                            // 			}
                            // 			else
                            // 			{
                            // 				CANStackLogger::CAN_stack_log(CANStackLogger::LoggingLevel::Warning, "[VT]: Load Versions Response ignored!");
                            // 			}
                        }
                        VTFunction::StoreVersionCommand => {
                            // 			if (StateMachineState::WaitForStoreVersionResponse == parentVT->state)
                            // 			{
                            // 				if (0 == message.get_uint8_at(5))
                            // 				{
                            // 					// Stored with no error
                            // 					parentVT->set_state(StateMachineState::Connected);
                            // 					CANStackLogger::CAN_stack_log(CANStackLogger::LoggingLevel::Info, "[VT]: Stored object pool with no error.");
                            // 				}
                            // 				else
                            // 				{
                            // 					// At least one error is set
                            // 					if (message.get_bool_at(5, 0))
                            // 					{
                            // 						CANStackLogger::CAN_stack_log(CANStackLogger::LoggingLevel::Warning, "[VT]: Store Versions Response error: Version label is not correct.");
                            // 					}
                            // 					if (message.get_bool_at(5, 1))
                            // 					{
                            // 						CANStackLogger::CAN_stack_log(CANStackLogger::LoggingLevel::Warning, "[VT]: Store Versions Response error: Insufficient memory.");
                            // 					}
                            // 					if (message.get_bool_at(5, 2))
                            // 					{
                            // 						CANStackLogger::CAN_stack_log(CANStackLogger::LoggingLevel::Warning, "[VT]: Store Versions Response error: Any other error.");
                            // 					}
                            // 				}
                            // 			}
                            // 			else
                            // 			{
                            // 				CANStackLogger::CAN_stack_log(CANStackLogger::LoggingLevel::Warning, "[VT]: Store Versions Response ignored!");
                            // 			}
                        }
                        VTFunction::DeleteVersionCommand => {
                            if message.get_u8_at(5) != 0 {
                                log::info!("[VT]: Delete Version Response OK!");
                            } else {
                                if message.get_bool_at(5, 1) {
                                    log::warn!("[VT]: Delete Version Response error: Version label is not correct, or unknown.");
                                }
                                if message.get_bool_at(5, 3) {
                                    log::warn!("[VT]: Delete Version Response error: Any other error.");
                                }
                            }
                        }
                        VTFunction::EndOfObjectPoolMessage => {
                            // 			if (StateMachineState::WaitForEndOfObjectPoolResponse == parentVT->state)
                            // 			{
                            // 				bool anyErrorInPool = message.get_bool_at(1, 0);
                            // 				bool vtRanOutOfMemory = message.get_bool_at(1, 1);
                            // 				bool otherErrors = message.get_bool_at(1, 3);
                            // 				std::uint16_t parentObjectIDOfFaultyObject = message.get_uint16_at(2);
                            // 				std::uint16_t objectIDOfFaultyObject = message.get_uint16_at(4);
                            // 				std::uint8_t objectPoolErrorBitmask = message.get_uint8_at(6);
                            // 				if ((!anyErrorInPool) &&
                            // 				    (0 == objectPoolErrorBitmask))
                            // 				{
                            // 					// Clear scaling buffers
                            // 					for (auto &objectPool : parentVT->objectPools)
                            // 					{
                            // 						objectPool.scaledObjectPool.clear();
                            // 					}
                            // 					// Check if we need to store this pool
                            // 					if (!parentVT->objectPools[0].versionLabel.empty())
                            // 					{
                            // 						parentVT->set_state(StateMachineState::SendStoreVersion);
                            // 					}
                            // 					else
                            // 					{
                            // 						parentVT->set_state(StateMachineState::Connected);
                            // 					}
                            // 					//! @todo maybe a better way available than relying on aux function callbacks registered?
                            // 					if (parentVT->auxiliaryFunctionEventDispatcher.get_listener_count() > 0)
                            // 					{
                            // 						if (parentVT->send_auxiliary_functions_preferred_assignment())
                            // 						{
                            // 							CANStackLogger::CAN_stack_log(CANStackLogger::LoggingLevel::Debug, "[AUX-N]: Sent preferred assignments after EndOfObjectPoolMessage.");
                            // 						}
                            // 						else
                            // 						{
                            // 							CANStackLogger::CAN_stack_log(CANStackLogger::LoggingLevel::Warning, "[AUX-N]: Failed to send preferred assignments after EndOfObjectPoolMessage.");
                            // 						}
                            // 					}
                            // 				}
                            // 				else
                            // 				{
                            // 					parentVT->set_state(StateMachineState::Failed);
                            // 					CANStackLogger::CAN_stack_log(CANStackLogger::LoggingLevel::Error, "[VT]: Error in end of object pool message." + std::string("Faulty Object ") + isobus::to_string(static_cast<int>(objectIDOfFaultyObject)) + std::string(" Faulty Object Parent ") + isobus::to_string(static_cast<int>(parentObjectIDOfFaultyObject)) + std::string(" Pool error bitmask value ") + isobus::to_string(static_cast<int>(objectPoolErrorBitmask)));
                            // 					if (vtRanOutOfMemory)
                            // 					{
                            // 						CANStackLogger::CAN_stack_log(CANStackLogger::LoggingLevel::Error, "[VT]: Ran out of memory");
                            // 					}
                            // 					if (otherErrors)
                            // 					{
                            // 						CANStackLogger::CAN_stack_log(CANStackLogger::LoggingLevel::Error, "[VT]: Reported other errors in EOM response");
                            // 					}
                            // 				}
                            // 			}
                        }
                        _ => {}
                    }
                }
            }
            _ => {}
        }

        handled
    }


    pub fn send_working_set_maintenance_message(&mut self, network_manager: &mut CanNetworkManager) {
        let mut data: [u8; 8] = [0xFF; 8];
        data[0] = VTFunction::WorkingSetMaintenanceMessage as u8;
        data[1] = if self.connected_vt_version <= VTVersion::Version2OrOlder {
            0xFF
        } else {
            self.first_time_in_state as u8
        };
        data[2] = self.object_pools.values().min_by(|f|{});

        self.send_to_virtual_terminal(network_manager, &data);
    }
    
    pub fn send_change_numeric_value(
        &mut self,
        network_manager: &mut CanNetworkManager,
        object_id: u16,
        value: u32,
    ) {
        let data = [
            VTFunction::ChangeNumericValueCommand as u8,
            object_id as u8,
            (object_id >> 8) as u8,
            0xFF,
            value as u8,
            ((value >> 8) & 0xFF) as u8,
            ((value >> 16) & 0xFF) as u8,
            ((value >> 24) & 0xFF) as u8,
        ];

        self.send_to_virtual_terminal(network_manager, &data);
    }

    pub fn send_to_virtual_terminal(&self, network_manager: &mut CanNetworkManager, data: &[u8]) {
        let message = CanMessage::new(
            if self.connected_vt_version <= VTVersion::Version5 {
                CanPriority::PriorityLowest7
            } else {
                CanPriority::Priority5
            },
            ParameterGroupNumber::ECUtoVirtualTerminal,
            self.internal_control_function.address(),
            self.partnered_control_function.address(),
            &data,
        );
        network_manager.send_can_message(message);
    }
}

impl Drop for VirtualTerminalClient<'_> {
    fn drop(&mut self) {
        self.terminate();
    }
}


// Impl callback functionallity
impl<'a> VirtualTerminalClient<'a> {
    pub fn add_vt_soft_key_event_listener(
        &mut self,
        callback: &'a dyn Fn(VTKeyEvent),
    ) -> Result<usize, ()> {
        // Generate a key based on raw address (extreamly unsafe)
        let key: usize = unsafe { core::mem::transmute(&callback) };

        let val = self.soft_key_event_callbacks.insert(key, callback);
        if val.is_none() {
            // log::debug!("vt_soft_key_event_listener registered! key:{key}");
            Ok(key)
        } else {
            log::error!("VT_SOFT_KEY_EVENT_CALLBACK_LIST_SIZE to small!");
            Err(())
        }
    }

    pub fn add_vt_button_event_listener(
        &mut self,
        callback: &'a dyn Fn(VTKeyEvent),
    ) -> Result<usize, ()> {
        // Generate a key based on raw address (extreamly unsafe)
        let key: usize = unsafe { core::mem::transmute(&callback) };

        let val = self.button_event_callbacks.insert(key, callback);
        if val.is_none() {
            // log::debug!("vt_button_event_listener registered! key:{key}");
            Ok(key)
        } else {
            log::error!("VT_BUTTON_EVENT_CALLBACK_LIST_SIZE to small!");
            Err(())
        }
    }

    pub fn add_vt_pointing_event_listener(
        &mut self,
        callback: &'a dyn Fn(VTPointingEvent),
    ) -> Result<usize, ()> {
        // Generate a key based on raw address (extreamly unsafe)
        let key: usize = unsafe { core::mem::transmute(&callback) };

        let val = self.pointing_event_callbacks.insert(key, callback);
        if val.is_none() {
            // log::debug!("vt_pointing_event_listener registered! key:{key}");
            Ok(key)
        } else {
            log::error!("VT_POINTING_EVENT_CALLBACK_LIST_SIZE to small!");
            Err(())
        }
    }

    pub fn add_vt_select_input_object_event_listener(
        &mut self,
        callback: &'a dyn Fn(VTSelectInputObjectEvent),
    ) -> Result<usize, ()> {
        // Generate a key based on raw address (extreamly unsafe)
        let key: usize = unsafe { core::mem::transmute(&callback) };

        let val = self
            .select_input_object_event_callbacks
            .insert(key, callback);
        if val.is_none() {
            // log::debug!("vt_select_input_object_event_listener registered! key:{key}");
            Ok(key)
        } else {
            log::error!("VT_SELECT_INPUT_OBJECT_EVENT_CALLBACK_LIST_SIZE to small!");
            Err(())
        }
    }

    pub fn add_vt_esc_message_event_listener(
        &mut self,
        callback: &'a dyn Fn(VTESCMessageEvent),
    ) -> Result<usize, ()> {
        // Generate a key based on raw address (extreamly unsafe)
        let key: usize = unsafe { core::mem::transmute(&callback) };

        let val = self.esc_message_event_callbacks.insert(key, callback);
        if val.is_none() {
            // log::debug!("vt_esc_message_event_listener registered! key:{key}");
            Ok(key)
        } else {
            log::error!("VT_ESC_MESSAGE_EVENT_CALLBACK_LIST_SIZE to small!");
            Err(())
        }
    }

    pub fn add_vt_change_numeric_value_event_listener<
        F: Fn(VTChangeNumericValueEvent) + 'static,
    >(
        &mut self,
        callback: &'a dyn Fn(VTChangeNumericValueEvent),
    ) -> Result<usize, ()> {
        // Generate a key based on raw address (extreamly unsafe)
        let key: usize = unsafe { core::mem::transmute(&callback) };

        let val = self
            .change_numeric_value_event_callbacks
            .insert(key, callback);
        if val.is_none() {
            // log::debug!("vt_change_numeric_value_event_listener registered! key:{key}");
            Ok(key)
        } else {
            log::error!("VT_CHANGE_NUMERIC_VALUE_EVENT_CALLBACK_LIST_SIZE to small!");
            Err(())
        }
    }

    pub fn add_vt_change_active_mask_event_listener(
        &mut self,
        callback: &'a dyn Fn(VTChangeActiveMaskEvent),
    ) -> Result<usize, ()> {
        // Generate a key based on raw address (extreamly unsafe)
        let key: usize = unsafe { core::mem::transmute(&callback) };

        let val = self
            .change_active_mask_event_callbacks
            .insert(key, callback);
        if val.is_none() {
            // log::debug!("vt_change_active_mask_event_listener registered! key:{key}");
            Ok(key)
        } else {
            log::error!("VT_CHANGE_ACTIVE_MASK_EVENT_CALLBACK_LIST_SIZE to small!");
            Err(())
        }
    }

    pub fn add_vt_change_soft_key_mask_event_listener(
        &mut self,
        callback: &'a dyn Fn(VTChangeSoftKeyMaskEvent),
    ) -> Result<usize, ()> {
        // Generate a key based on raw address (extreamly unsafe)
        let key: usize = unsafe { core::mem::transmute(&callback) };

        let val = self
            .change_soft_key_mask_event_callbacks
            .insert(key, callback);
        if val.is_none() {
            // log::debug!("vt_change_soft_key_mask_event_listener registered! key:{key}");
            Ok(key)
        } else {
            log::error!("VT_CHANGE_SOFT_KEY_MASK_CALLBACK_LIST_SIZE to small!");
            Err(())
        }
    }

    pub fn add_vt_change_string_value_event_listener(
        &mut self,
        callback: &'a dyn Fn(VTChangeStringValueEvent),
    ) -> Result<usize, ()> {
        // Generate a key based on raw address (extreamly unsafe)
        let key: usize = unsafe { core::mem::transmute(&callback) };

        let val = self
            .change_string_value_event_callbacks
            .insert(key, callback);
        if val.is_none() {
            // log::debug!("vt_change_string_value_event_listener registered! key:{key}");
            Ok(key)
        } else {
            log::error!("VT_CHANGE_STRING_VALUE_CALLBACK_LIST_SIZE to small!");
            Err(())
        }
    }

    pub fn add_vt_user_layout_hide_show_event_listener<
        F: Fn(VTUserLayoutHideShowEvent) + 'static,
    >(
        &mut self,
        callback: &'a dyn Fn(VTUserLayoutHideShowEvent),
    ) -> Result<usize, ()> {
        // Generate a key based on raw address (extreamly unsafe)
        let key: usize = unsafe { core::mem::transmute(&callback) };

        let val = self
            .user_layout_hide_show_event_callbacks
            .insert(key, callback);
        if val.is_none() {
            // log::debug!("vt_user_layout_hide_show_event_listener registered! key:{key}");
            Ok(key)
        } else {
            log::error!("VT_USER_LAYOUT_HIDE_SHOW_CALLBACK_LIST_SIZE to small!");
            Err(())
        }
    }

    pub fn add_vt_audio_signal_termination_event_listener<
        F: Fn(VTAudioSignalTerminationEvent) + 'static,
    >(
        &mut self,
        callback: &'a dyn Fn(VTAudioSignalTerminationEvent),
    ) -> Result<usize, ()> {
        // Generate a key based on raw address (extreamly unsafe)
        let key: usize = unsafe { core::mem::transmute(&callback) };

        let val = self
            .audio_signal_termination_event_callbacks
            .insert(key, callback);
        if val.is_none() {
            // log::debug!("vt_audio_signal_termination_event_listener registered! key:{key}");
            Ok(key)
        } else {
            log::error!("VT_AUDIO_SIGNAL_TERMINATION_CALLBACK_LIST_SIZE to small!");
            Err(())
        }
    }

    pub fn add_auxiliary_function_event_listener(
        &mut self,
        callback: &'a dyn Fn(AuxiliaryFunctionEvent),
    ) -> Result<usize, ()> {
        // Generate a key based on raw address (extreamly unsafe)
        let key: usize = unsafe { core::mem::transmute(&callback) };

        let val = self
            .auxiliary_function_event_callbacks
            .insert(key, callback);
        if val.is_none() {
            // log::debug!("auxiliary_function_event_listener registered! key:{key}");
            Ok(key)
        } else {
            log::error!("AUXILIARY_FUNCTION_CALLBACK_LIST_SIZE to small!");
            Err(())
        }
    }

    pub fn remove_vt_soft_key_event_listener(&mut self, handle: usize) {
        let _ = self.soft_key_event_callbacks.remove(&handle);
    }

    pub fn remove_vt_button_key_event_listener(&mut self, handle: usize) {
        let _ = self.button_event_callbacks.remove(&handle);
    }

    pub fn remove_vt_pointing_event_listener(&mut self, handle: usize) {
        let _ = self.pointing_event_callbacks.remove(&handle);
    }

    pub fn remove_vt_select_input_object_event_listener(&mut self, handle: usize) {
        let _ = self.select_input_object_event_callbacks.remove(&handle);
    }

    pub fn remove_vt_esc_message_event_listener(&mut self, handle: usize) {
        let _ = self.esc_message_event_callbacks.remove(&handle);
    }

    pub fn remove_vt_change_numeric_value_event_listener(&mut self, handle: usize) {
        let _ = self.change_numeric_value_event_callbacks.remove(&handle);
    }

    pub fn remove_vt_change_active_mask_event_listener(&mut self, handle: usize) {
        let _ = self.change_active_mask_event_callbacks.remove(&handle);
    }

    pub fn remove_vt_change_soft_key_mask_event_listener(&mut self, handle: usize) {
        let _ = self.change_soft_key_mask_event_callbacks.remove(&handle);
    }

    pub fn remove_vt_change_string_value_event_listener(&mut self, handle: usize) {
        let _ = self.change_string_value_event_callbacks.remove(&handle);
    }

    pub fn remove_vt_user_layout_hide_show_event_listener(&mut self, handle: usize) {
        let _ = self.user_layout_hide_show_event_callbacks.remove(&handle);
    }

    pub fn remove_vt_audio_signal_termination_event_listener(&mut self, handle: usize) {
        let _ = self
            .audio_signal_termination_event_callbacks
            .remove(&handle);
    }

    pub fn remove_auxiliary_function_event_listener(&mut self, handle: usize) {
        let _ = self.auxiliary_function_event_callbacks.remove(&handle);
    }
}


/// The internal state machine state of the VT client, mostly just public so tests can access it
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
enum State {
    Disconnected,                  //< VT is not connected, and is not trying to connect yet
    WaitForPartnerVTStatusMessage, //< VT client is initialized, waiting for a VT server to come online
    SendWorkingSetMasterMessage,   //< Client is sending the working state master message
    ReadyForObjectPool,            //< Client needs an object pool before connection can continue
    SendGetMemory, //< Client is sending the "get memory" message to see if VT has enough memory available
    WaitForGetMemoryResponse, //< Client is waiting for a response to the "get memory" message
    SendGetNumberSoftkeys, //< Client is sending the "get number of soft keys" message
    WaitForGetNumberSoftKeysResponse, //< Client is waiting for a response to the "get number of soft keys" message
    SendGetTextFontData,              //< Client is sending the "get text font data" message
    WaitForGetTextFontDataResponse, //< Client is waiting for a response to the "get text font data" message
    SendGetHardware,                //< Client is sending the "get hardware" message
    WaitForGetHardwareResponse, //< Client is waiting for a response to the "get hardware" message
    SendGetVersions, //< If a version label was specified, check to see if the VT has that version already
    WaitForGetVersionsResponse, //< Client is waiting for a response to the "get versions" message
    SendStoreVersion, //< Sending the store version command
    WaitForStoreVersionResponse, //< Client is waiting for a response to the store version command
    SendLoadVersion, //< Sending the load version command
    WaitForLoadVersionResponse, //< Client is waiting for the VT to respond to the "Load Version" command
    UploadObjectPool,           //< Client is uploading the object pool
    SendEndOfObjectPool,        //< Client is sending the end of object pool message
    WaitForEndOfObjectPoolResponse, //< Client is waiting for the end of object pool response message
    Connected, //< Client is connected to the VT server and the application layer is in control
    Failed,    //< Client could not connect to the VT due to an error
}

impl Default for State {
    fn default() -> Self {
        Self::Disconnected
    }
}