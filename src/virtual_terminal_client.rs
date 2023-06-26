
use crate::control_function::*;

use heapless::FnvIndexMap;

const VT_SOFT_KEY_EVENT_CALLBACK_LIST_SIZE: usize = 8;
const VT_BUTTON_EVENT_CALLBACK_LIST_SIZE: usize = 8;
const VT_POINTING_EVENT_CALLBACK_LIST_SIZE: usize = 8;
const VT_SELECT_INPUT_OBJECT_CALLBACK_LIST_SIZE: usize = 8;
const VT_ESC_MESSAGE_CALLBACK_LIST_SIZE: usize = 8;
const VT_CHANGE_NUMERIC_VALUE_CALLBACK_LIST_SIZE: usize = 8;
const VT_CHANGE_ACTIVE_MASK_CALLBACK_LIST_SIZE: usize = 8;
const VT_CHANGE_SOFT_KEY_MASK_CALLBACK_LIST_SIZE: usize = 8;
const VT_CHANGE_STRING_VALUE_CALLBACK_LIST_SIZE: usize = 8;
const VT_USER_LAYOUT_HIDE_SHOW_CALLBACK_LIST_SIZE: usize = 8;
const VT_AUDIO_SIGNAL_TERMINATION_CALLBACK_LIST_SIZE: usize = 8;
const AUXILIARY_FUNCTION_CALLBACK_LIST_SIZE: usize = 8;

pub struct VirtualTerminalClient {
    partnered_control_function: PartneredControlFunction, //< The partner control function this client will send to
    internal_control_function: InternalControlFunction, //< The internal control function the client uses to send from

	soft_key_event_callbacks: FnvIndexMap<usize, fn(VTKeyEvent), VT_SOFT_KEY_EVENT_CALLBACK_LIST_SIZE>,
	button_event_callbacks: FnvIndexMap<usize, fn(VTKeyEvent), VT_BUTTON_EVENT_CALLBACK_LIST_SIZE>,
	pointing_event_callbacks: FnvIndexMap<usize, fn(VTPointingEvent), VT_POINTING_EVENT_CALLBACK_LIST_SIZE>,
	select_input_object_event_callbacks: FnvIndexMap<usize, fn(VTSelectInputObjectEvent), VT_SELECT_INPUT_OBJECT_CALLBACK_LIST_SIZE>,
	esc_message_event_callbacks: FnvIndexMap<usize, fn(VTESCMessageEvent), VT_ESC_MESSAGE_CALLBACK_LIST_SIZE>,
	change_numeric_value_event_callbacks: FnvIndexMap<usize, fn(VTChangeNumericValueEvent), VT_CHANGE_NUMERIC_VALUE_CALLBACK_LIST_SIZE>,
	change_active_mask_event_callbacks: FnvIndexMap<usize, fn(VTChangeActiveMaskEvent), VT_CHANGE_ACTIVE_MASK_CALLBACK_LIST_SIZE>,
	change_soft_key_mask_event_callbacks: FnvIndexMap<usize, fn(VTChangeSoftKeyMaskEvent), VT_CHANGE_SOFT_KEY_MASK_CALLBACK_LIST_SIZE>,
	change_string_value_event_callbacks: FnvIndexMap<usize, fn(VTChangeStringValueEvent), VT_CHANGE_STRING_VALUE_CALLBACK_LIST_SIZE>,
	user_layout_hide_show_event_callbacks: FnvIndexMap<usize, fn(VTUserLayoutHideShowEvent), VT_USER_LAYOUT_HIDE_SHOW_CALLBACK_LIST_SIZE>,
	audio_signal_termination_event_callbacks: FnvIndexMap<usize, fn(VTAudioSignalTerminationEvent), VT_AUDIO_SIGNAL_TERMINATION_CALLBACK_LIST_SIZE>,
	auxiliary_function_event_callbacks: FnvIndexMap<usize, fn(AuxiliaryFunctionEvent), AUXILIARY_FUNCTION_CALLBACK_LIST_SIZE>,
}

impl VirtualTerminalClient {
    pub fn new(partner: PartneredControlFunction, client: InternalControlFunction) -> VirtualTerminalClient {
        VirtualTerminalClient {
            partnered_control_function: partner,
            internal_control_function: client,
			soft_key_event_callbacks: FnvIndexMap::new(),
			button_event_callbacks: FnvIndexMap::new(),
            pointing_event_callbacks: FnvIndexMap::new(),
            select_input_object_event_callbacks: FnvIndexMap::new(),
            esc_message_event_callbacks: FnvIndexMap::new(),
            change_numeric_value_event_callbacks: FnvIndexMap::new(),
            change_active_mask_event_callbacks: FnvIndexMap::new(),
            change_soft_key_mask_event_callbacks: FnvIndexMap::new(),
            change_string_value_event_callbacks: FnvIndexMap::new(),
            user_layout_hide_show_event_callbacks: FnvIndexMap::new(),
            audio_signal_termination_event_callbacks: FnvIndexMap::new(),
            auxiliary_function_event_callbacks: FnvIndexMap::new(),
        }
    }

	pub fn add_vt_soft_key_event_listener(&mut self, callback: fn(VTKeyEvent)) -> Result<usize, ()> {
		// Generate a key based on raw address (extreamly unsafe)
		let key: usize = unsafe { core::mem::transmute(callback) };

		let val = self.soft_key_event_callbacks.insert(key, callback);
		if let Ok(_) = val {
			log::debug!("vt_soft_key_event_listener registered! key:{key}");
			Ok(key)
		} else {
			log::error!("VT_SOFT_KEY_EVENT_CALLBACK_LIST_SIZE to small!");
			Err(())
		}
	}

	pub fn remove_vt_soft_key_event_listener(&mut self, handle: usize) {
		let _ = self.soft_key_event_callbacks.remove(&handle);
	}

	pub fn add_vt_button_event_listener(&mut self, callback: fn(VTKeyEvent)) -> Result<usize, ()> {
		// Generate a key based on raw address (extreamly unsafe)
		let key: usize = unsafe { core::mem::transmute(callback) };

		let val = self.button_event_callbacks.insert(key, callback);
		if let Ok(_) = val {
			log::debug!("vt_button_event_listener registered! key:{key}");
			Ok(key)
		} else {
			log::error!("VT_BUTTON_EVENT_CALLBACK_LIST_SIZE to small!");
			Err(())
		}
	}

	pub fn remove_button_key_event_listener(&mut self, handle: usize) {
		let _ = self.button_event_callbacks.remove(&handle);
	}

    pub fn update(&mut self) {
        // Firt update the connected controll functions
        self.internal_control_function.update();
        // self.partnered_control_function.update();

		for (_,callback) in &mut self.soft_key_event_callbacks {
			callback(VTKeyEvent{ object_Id: 0, parent_object_Id: 0, key_number: 0 });
		}
		for (_,callback) in &mut self.button_event_callbacks {
			callback(VTKeyEvent{ object_Id: 0, parent_object_Id: 0, key_number: 0 });
		}

        // StateMachineState previousStateMachineState = state; // Save state to see if it changes this update

		// if (nullptr != partnerControlFunction)
		// {
		// 	switch (state)
		// 	{
		// 		case StateMachineState::Disconnected:
		// 		{
		// 			sendWorkingSetMaintenance = false;
		// 			sendAuxiliaryMaintenance = false;

		// 			if (partnerControlFunction->get_address_valid())
		// 			{
		// 				set_state(StateMachineState::WaitForPartnerVTStatusMessage);
		// 			}
		// 		}
		// 		break;

		// 		case StateMachineState::WaitForPartnerVTStatusMessage:
		// 		{
		// 			if (0 != lastVTStatusTimestamp_ms)
		// 			{
		// 				set_state(StateMachineState::SendWorkingSetMasterMessage);
		// 			}
		// 		}
		// 		break;

		// 		case StateMachineState::SendWorkingSetMasterMessage:
		// 		{
		// 			if (send_working_set_master())
		// 			{
		// 				set_state(StateMachineState::ReadyForObjectPool);
		// 			}
		// 		}
		// 		break;

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
		// }
		// else
		// {
		// 	set_state(StateMachineState::Disconnected);
		// }

		// if ((sendWorkingSetMaintenance) &&
		//     (SystemTiming::time_expired_ms(lastWorkingSetMaintenanceTimestamp_ms, WORKING_SET_MAINTENANCE_TIMEOUT_MS)))
		// {
		// 	txFlags.set_flag(static_cast<std::uint32_t>(TransmitFlags::SendWorkingSetMaintenance));
		// }
		// if ((sendAuxiliaryMaintenance) &&
		//     (!ourAuxiliaryInputs.empty()) &&
		//     (SystemTiming::time_expired_ms(lastAuxiliaryMaintenanceTimestamp_ms, AUXILIARY_MAINTENANCE_TIMEOUT_MS)))
		// {
		// 	/// @todo We should make sure that when we disconnect/reconnect atleast 500ms has passed since the last auxiliary maintenance message
		// 	txFlags.set_flag(static_cast<std::uint32_t>(TransmitFlags::SendAuxiliaryMaintenance));
		// }
		// txFlags.process_all_flags();

		// if (state == previousStateMachineState)
		// {
		// 	firstTimeInState = false;
		// }
    }
}

/// @brief A struct for storing information of a VT key input event
#[derive(Debug)]
pub struct VTKeyEvent {
	// VirtualTerminalClient *parentPointer; ///< A pointer to the parent VT client
	object_Id: u16, //< The object ID
	parent_object_Id: u16, //< The parent object ID
	key_number: u8, //< The key number
	// KeyActivationCode keyEvent; ///< The key event
}

/// @brief A struct for storing information of a VT pointing event
#[derive(Debug)]
struct VTPointingEvent
{
	// VirtualTerminalClient *parentPointer; ///< A pointer to the parent VT client
	x_pos: u16, //< The x position
	y_pos: u16, //< The y position
	parent_object_Id: u16, //< The parent object ID
	// KeyActivationCode keyEvent; ///< The key event
}

/// @brief A struct for storing information of a VT input object selection event
#[derive(Debug)]
struct VTSelectInputObjectEvent
{
	// VirtualTerminalClient *parentPointer; ///< A pointer to the parent VT client
	// std::uint16_t objectID; ///< The object ID
	// bool objectSelected; ///< Whether the object is selected
	// bool objectOpenForInput; ///< Whether the object is open for input
}

/// @brief A struct for storing information of a VT ESC message event
#[derive(Debug)]
struct VTESCMessageEvent
{
	// VirtualTerminalClient *parentPointer; ///< A pointer to the parent VT client
	// std::uint16_t objectID; ///< The object ID
	// ESCMessageErrorCode errorCode; ///< The error code
}

/// @brief A struct for storing information of a VT change numeric value event
#[derive(Debug)]
struct VTChangeNumericValueEvent
{
	// VirtualTerminalClient *parentPointer; ///< A pointer to the parent VT client
	// std::uint32_t value; ///< The value
	// std::uint16_t objectID; ///< The object ID
}

/// @brief A struct for storing information of a VT change active mask event
#[derive(Debug)]
struct VTChangeActiveMaskEvent
{
	// VirtualTerminalClient *parentPointer; ///< A pointer to the parent VT client
	// std::uint16_t maskObjectID; ///< The mask object ID
	// std::uint16_t errorObjectID; ///< The error object ID
	// std::uint16_t parentObjectID; ///< The parent object ID
	// bool missingObjects; ///< Whether there are missing objects
	// bool maskOrChildHasErrors; ///< Whether the mask or child has errors
	// bool anyOtherError; ///< Whether there are any other errors
	// bool poolDeleted; ///< Whether the pool has been deleted
}

/// @brief A struct for storing information of a VT change soft key mask event
#[derive(Debug)]
struct VTChangeSoftKeyMaskEvent
{
	// VirtualTerminalClient *parentPointer; ///< A pointer to the parent VT client
	// std::uint16_t dataOrAlarmMaskObjectID; ///< The data or alarm mask object ID
	// std::uint16_t softKeyMaskObjectID; ///< The soft key mask object ID
	// bool missingObjects; ///< Whether there are missing objects
	// bool maskOrChildHasErrors; ///< Whether the mask or child has errors
	// bool anyOtherError; ///< Whether there are any other errors
	// bool poolDeleted; ///< Whether the pool has been deleted
}

/// @brief A struct for storing information of a VT change string value event
#[derive(Debug)]
struct VTChangeStringValueEvent
{
// 	std::string value; ///< The value
// 	VirtualTerminalClient *parentPointer; ///< A pointer to the parent VT client
// 	std::uint16_t objectID; ///< The object ID
}

/// @brief A struct for storing information of a VT on user-layout hide/show event
#[derive(Debug)]
struct VTUserLayoutHideShowEvent
{
	// VirtualTerminalClient *parentPointer; ///< A pointer to the parent VT client
	// std::uint16_t objectID; ///< The object ID
	// bool isHidden; ///< Whether the object is hidden
}

/// @brief A struct for storing information of a VT control audio signal termination event
#[derive(Debug)]
struct VTAudioSignalTerminationEvent
{
	// VirtualTerminalClient *parentPointer; ///< A pointer to the parent VT client
	// bool isTerminated; ///< Whether the audio signal is terminated
}

/// @brief A struct for storing information of an auxilary function event
#[derive(Debug)]
struct AuxiliaryFunctionEvent
{
	// AssignedAuxiliaryFunction function; ///< The function
	// VirtualTerminalClient *parentPointer; ///< A pointer to the parent VT client
	// std::uint16_t value1; ///< The first value
	// std::uint16_t value2; ///< The second value
}