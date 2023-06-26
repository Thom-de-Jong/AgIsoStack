
use crate::control_function::*;


pub struct VirtualTerminalClient {
    partnered_control_function: PartneredControlFunction, //< The partner control function this client will send to
    internal_control_function: InternalControlFunction, //< The internal control function the client uses to send from
}

impl VirtualTerminalClient {
    pub fn new(partner: PartneredControlFunction, client: InternalControlFunction) -> VirtualTerminalClient {
        VirtualTerminalClient {
            partnered_control_function: partner,
            internal_control_function: client,
        }
    }

    pub fn update(&self) {
        // Firt update the connected controll functions
        self.internal_control_function.update();
        // self.partnered_control_function.update();

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