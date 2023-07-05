
use core::time::Duration;

use alloc::boxed::Box;
use alloc::collections::{VecDeque, BTreeMap};

use crate::{control_function::*, CanNetworkManager, CanPriority, ParameterGroupNumber, CanMessage, Address, ObjectId};
use crate::hardware_integration::{TimeDriver, TimeDriverTrait};

use super::*;

const MAX_EVENT_QUEUE_SIZE: usize = 32;

pub struct VirtualTerminalClient {
    state_machine: VirtualTerminalClientStateMachine,

    partnered_control_function: PartneredControlFunction, //< The partner control function this client will send to
    internal_control_function: InternalControlFunction, //< The internal control function the client uses to send from

	is_initialized: bool,

	// TODO: VT status variables, make PartneredControlFunction hold this in tuple struct VirtualTerminalServer?
	lastVTStatusTimestamp_ms: Duration,
	activeWorkingSetMasterAddress: Address,
	activeWorkingSetDataMaskObjectID: ObjectId,
	activeWorkingSetSoftKeyMaskObjectID: ObjectId,
	busyCodesBitfield: u8,
	currentCommandFunctionCode: u8,

	event_queue: VecDeque<Event>,

	soft_key_event_callbacks: BTreeMap<usize, Box<dyn Fn(VTKeyEvent)>>,
	button_event_callbacks: BTreeMap<usize, Box<dyn Fn(VTKeyEvent)>>,
	pointing_event_callbacks: BTreeMap<usize, Box<dyn Fn(VTPointingEvent)>>,
	select_input_object_event_callbacks: BTreeMap<usize, Box<dyn Fn(VTSelectInputObjectEvent)>>,
	esc_message_event_callbacks: BTreeMap<usize, Box<dyn Fn(VTESCMessageEvent)>>,
	change_numeric_value_event_callbacks: BTreeMap<usize, Box<dyn Fn(VTChangeNumericValueEvent)>>,
	change_active_mask_event_callbacks: BTreeMap<usize, Box<dyn Fn(VTChangeActiveMaskEvent)>>,
	change_soft_key_mask_event_callbacks: BTreeMap<usize, Box<dyn Fn(VTChangeSoftKeyMaskEvent)>>,
	change_string_value_event_callbacks: BTreeMap<usize, Box<dyn Fn(VTChangeStringValueEvent)>>,
	user_layout_hide_show_event_callbacks: BTreeMap<usize, Box<dyn Fn(VTUserLayoutHideShowEvent)>>,
	audio_signal_termination_event_callbacks: BTreeMap<usize, Box<dyn Fn(VTAudioSignalTerminationEvent)>>,
	auxiliary_function_event_callbacks: BTreeMap<usize, Box<dyn Fn(AuxiliaryFunctionEvent)>>,
}

impl VirtualTerminalClient {
    pub fn new(partner: PartneredControlFunction, client: InternalControlFunction) -> VirtualTerminalClient {
		let vtc = VirtualTerminalClient {
			state_machine: VirtualTerminalClientStateMachine::new(),

            partnered_control_function: partner,
            internal_control_function: client,

			is_initialized: false,

			lastVTStatusTimestamp_ms: Duration::default(),
			activeWorkingSetMasterAddress: Address::NULL,
			activeWorkingSetDataMaskObjectID: ObjectId::NULL,
			activeWorkingSetSoftKeyMaskObjectID: ObjectId::NULL,
			busyCodesBitfield: u8::default(),
			currentCommandFunctionCode: u8::default(),

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
        };
		vtc
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

		self.is_initialized = true;
	}

	pub fn is_initialized(&self) -> bool {
		self.is_initialized
	}
	
	pub fn is_connected(&self) -> bool {
		todo!()
	}

	pub fn terminate(&mut self) {
		if !self.is_initialized() { return }

		// if ((StateMachineState::Connected == state) && (send_delete_object_pool())) {
		//		CANStackLogger::debug("[VT]: Requested object pool deletion from volatile VT memory.");
		// }

		// // Remove the callbacks to CAN messages used by the Virtual Termainal Client
		// partnerControlFunction->remove_parameter_group_number_callback(static_cast<std::uint32_t>(CANLibParameterGroupNumber::VirtualTerminalToECU), process_rx_message, this);
		// partnerControlFunction->remove_parameter_group_number_callback(static_cast<std::uint32_t>(CANLibParameterGroupNumber::Acknowledge), process_rx_message, this);
		// CANNetworkManager::CANNetwork.remove_global_parameter_group_number_callback(static_cast<std::uint32_t>(CANLibParameterGroupNumber::VirtualTerminalToECU), process_rx_message, this);
		// CANNetworkManager::CANNetwork.remove_global_parameter_group_number_callback(static_cast<std::uint32_t>(CANLibParameterGroupNumber::ECUtoVirtualTerminal), process_rx_message, this);
		
		// shouldTerminate = true;
		// self.set_state(StateMachineState::Disconnected);
		self.is_initialized = false;
		log::info!("[VT]: VT Client connection has been terminated.");
	}

	pub fn restart_communication(&mut self) {
		log::info!("[VT]:VT Client connection restart requested. Client will now terminate and reinitialize.");
		self.terminate();
		self.initialize();
	}




	pub fn set_object_pool(&mut self, pool_index: u8, supported_vt_version: VTVersion, pool: &[u8]) {
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

		// 	if (poolIndex < objectPools.size())
		// 	{
		// 		objectPools[poolIndex] = tempData;
		// 	}
		// 	else
		// 	{
		// 		objectPools.resize(poolIndex + 1);
		// 		objectPools[poolIndex] = tempData;
		// 	}
		// }
	}


	pub fn next_event(&mut self) -> Option<Event> {
        self.event_queue.pop_front()
    }







	pub fn add_vt_soft_key_event_listener<F: Fn(VTKeyEvent) + 'static>(&mut self, callback: F) -> Result<usize, ()> {
		// Generate a key based on raw address (extreamly unsafe)
		let key: usize = unsafe { core::mem::transmute(&callback) };

		let val = self.soft_key_event_callbacks.insert(key, Box::new(callback));
		if val.is_none() {
			// log::debug!("vt_soft_key_event_listener registered! key:{key}");
			Ok(key)
		} else {
			log::error!("VT_SOFT_KEY_EVENT_CALLBACK_LIST_SIZE to small!");
			Err(())
		}
	}
	
	pub fn add_vt_button_event_listener<F: Fn(VTKeyEvent) + 'static>(&mut self, callback: F) -> Result<usize, ()> {
		// Generate a key based on raw address (extreamly unsafe)
		let key: usize = unsafe { core::mem::transmute(&callback) };

		let val = self.button_event_callbacks.insert(key, Box::new(callback));
		if val.is_none() {
			// log::debug!("vt_button_event_listener registered! key:{key}");
			Ok(key)
		} else {
			log::error!("VT_BUTTON_EVENT_CALLBACK_LIST_SIZE to small!");
			Err(())
		}
	}

	pub fn add_vt_pointing_event_listener<F: Fn(VTPointingEvent) + 'static>(&mut self, callback: F) -> Result<usize, ()> {
		// Generate a key based on raw address (extreamly unsafe)
		let key: usize = unsafe { core::mem::transmute(&callback) };

		let val = self.pointing_event_callbacks.insert(key, Box::new(callback));
		if val.is_none() {
			// log::debug!("vt_pointing_event_listener registered! key:{key}");
			Ok(key)
		} else {
			log::error!("VT_POINTING_EVENT_CALLBACK_LIST_SIZE to small!");
			Err(())
		}
	}

	pub fn add_vt_select_input_object_event_listener<F: Fn(VTSelectInputObjectEvent) + 'static>(&mut self, callback: F) -> Result<usize, ()> {
		// Generate a key based on raw address (extreamly unsafe)
		let key: usize = unsafe { core::mem::transmute(&callback) };

		let val = self.select_input_object_event_callbacks.insert(key, Box::new(callback));
		if val.is_none() {
			// log::debug!("vt_select_input_object_event_listener registered! key:{key}");
			Ok(key)
		} else {
			log::error!("VT_SELECT_INPUT_OBJECT_EVENT_CALLBACK_LIST_SIZE to small!");
			Err(())
		}
	}

	pub fn add_vt_esc_message_event_listener<F: Fn(VTESCMessageEvent) + 'static>(&mut self, callback: F) -> Result<usize, ()> {
		// Generate a key based on raw address (extreamly unsafe)
		let key: usize = unsafe { core::mem::transmute(&callback) };

		let val = self.esc_message_event_callbacks.insert(key, Box::new(callback));
		if val.is_none() {
			// log::debug!("vt_esc_message_event_listener registered! key:{key}");
			Ok(key)
		} else {
			log::error!("VT_ESC_MESSAGE_EVENT_CALLBACK_LIST_SIZE to small!");
			Err(())
		}
	}

	pub fn add_vt_change_numeric_value_event_listener<F: Fn(VTChangeNumericValueEvent) + 'static>(&mut self, callback: F) -> Result<usize, ()> {
		// Generate a key based on raw address (extreamly unsafe)
		let key: usize = unsafe { core::mem::transmute(&callback) };

		let val = self.change_numeric_value_event_callbacks.insert(key, Box::new(callback));
		if val.is_none() {
			// log::debug!("vt_change_numeric_value_event_listener registered! key:{key}");
			Ok(key)
		} else {
			log::error!("VT_CHANGE_NUMERIC_VALUE_EVENT_CALLBACK_LIST_SIZE to small!");
			Err(())
		}
	}

	pub fn add_vt_change_active_mask_event_listener<F: Fn(VTChangeActiveMaskEvent) + 'static>(&mut self, callback: F) -> Result<usize, ()> {
		// Generate a key based on raw address (extreamly unsafe)
		let key: usize = unsafe { core::mem::transmute(&callback) };

		let val = self.change_active_mask_event_callbacks.insert(key, Box::new(callback));
		if val.is_none() {
			// log::debug!("vt_change_active_mask_event_listener registered! key:{key}");
			Ok(key)
		} else {
			log::error!("VT_CHANGE_ACTIVE_MASK_EVENT_CALLBACK_LIST_SIZE to small!");
			Err(())
		}
	}

	pub fn add_vt_change_soft_key_mask_event_listener<F: Fn(VTChangeSoftKeyMaskEvent) + 'static>(&mut self, callback: F) -> Result<usize, ()> {
		// Generate a key based on raw address (extreamly unsafe)
		let key: usize = unsafe { core::mem::transmute(&callback) };

		let val = self.change_soft_key_mask_event_callbacks.insert(key, Box::new(callback));
		if val.is_none() {
			// log::debug!("vt_change_soft_key_mask_event_listener registered! key:{key}");
			Ok(key)
		} else {
			log::error!("VT_CHANGE_SOFT_KEY_MASK_CALLBACK_LIST_SIZE to small!");
			Err(())
		}
	}

	pub fn add_vt_change_string_value_event_listener<F: Fn(VTChangeStringValueEvent) + 'static>(&mut self, callback: F) -> Result<usize, ()> {
		// Generate a key based on raw address (extreamly unsafe)
		let key: usize = unsafe { core::mem::transmute(&callback) };

		let val = self.change_string_value_event_callbacks.insert(key, Box::new(callback));
		if val.is_none() {
			// log::debug!("vt_change_string_value_event_listener registered! key:{key}");
			Ok(key)
		} else {
			log::error!("VT_CHANGE_STRING_VALUE_CALLBACK_LIST_SIZE to small!");
			Err(())
		}
	}

	pub fn add_vt_user_layout_hide_show_event_listener<F: Fn(VTUserLayoutHideShowEvent) + 'static>(&mut self, callback: F) -> Result<usize, ()> {
		// Generate a key based on raw address (extreamly unsafe)
		let key: usize = unsafe { core::mem::transmute(&callback) };

		let val = self.user_layout_hide_show_event_callbacks.insert(key, Box::new(callback));
		if val.is_none() {
			// log::debug!("vt_user_layout_hide_show_event_listener registered! key:{key}");
			Ok(key)
		} else {
			log::error!("VT_USER_LAYOUT_HIDE_SHOW_CALLBACK_LIST_SIZE to small!");
			Err(())
		}
	}

	pub fn add_vt_audio_signal_termination_event_listener<F: Fn(VTAudioSignalTerminationEvent) + 'static>(&mut self, callback: F) -> Result<usize, ()> {
		// Generate a key based on raw address (extreamly unsafe)
		let key: usize = unsafe { core::mem::transmute(&callback) };

		let val = self.audio_signal_termination_event_callbacks.insert(key, Box::new(callback));
		if val.is_none() {
			// log::debug!("vt_audio_signal_termination_event_listener registered! key:{key}");
			Ok(key)
		} else {
			log::error!("VT_AUDIO_SIGNAL_TERMINATION_CALLBACK_LIST_SIZE to small!");
			Err(())
		}
	}

	pub fn add_auxiliary_function_event_listener<F: Fn(AuxiliaryFunctionEvent) + 'static>(&mut self, callback: F) -> Result<usize, ()> {
		// Generate a key based on raw address (extreamly unsafe)
		let key: usize = unsafe { core::mem::transmute(&callback) };

		let val = self.auxiliary_function_event_callbacks.insert(key, Box::new(callback));
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
		let _ = self.audio_signal_termination_event_callbacks.remove(&handle);
	}
	
	pub fn remove_auxiliary_function_event_listener(&mut self, handle: usize) {
		let _ = self.auxiliary_function_event_callbacks.remove(&handle);
	}
	








	pub fn send_change_numeric_value(&mut self, network_manager: &CanNetworkManager, object_id: u16, value: u32) {
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

		let message = CanMessage::new(
			CanPriority::PriorityLowest7,
			ParameterGroupNumber::ECUtoVirtualTerminal,
			self.internal_control_function.address(),
			self.partnered_control_function.address(),
			&data,
		);
		network_manager.send_can_message(message);
	}







    pub fn update(&mut self, network_manager: &CanNetworkManager) {
        // Firt update the connected control functions.
        self.internal_control_function.update(network_manager);
        // self.partnered_control_function.update(network_manager);

		// Process received messages and update internal state.
        network_manager.handle_message(|message| self.process_can_message(message));


		// Limit the size of the event queue in case it is not used.
		// In Rust, using the event queue is prefered over using callbacks.
		while self.event_queue.len() > MAX_EVENT_QUEUE_SIZE {
			self.event_queue.pop_front();
		}


		// Do stuff based on the current internal state.

        // StateMachineState previousStateMachineState = state; // Save state to see if it changes this update

		// if (nullptr != partnerControlFunction)
		// {
			// match self.state
			// {
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



	pub fn process_can_message(&mut self, message: &CanMessage) -> bool {
		let mut handled = false;
		match message.pgn() {
			ParameterGroupNumber::Acknowledge => {
				// if AcknowledgementType::Negative as u8 == message.data()[0] {
				// 	if ParameterGroupNumber::ECUtoVirtualTerminal == message.data()[5..8].into() {
				// 		log::error!("[VT]: The VT Server is NACK-ing our VT messages. Disconnecting.");
				// 		self.set_state(StateMachineState::Disconnected);
				// 	}
				// }
			},
			ParameterGroupNumber::VirtualTerminalToECU => {
				if let Ok(vt_function) = message.get_u8_at(0).try_into() {
					match vt_function {
						VTFunction::SoftKeyActivationMessage |
						VTFunction::ButtonActivationMessage => {
							let key_event: KeyActivationCode = message.get_u8_at(1).try_into().unwrap_or_else(|_| KeyActivationCode::ButtonPressAborted);
							let object_id: u16 = message.get_u16_at(2);
							let parent_object_id: u16 = message.get_u16_at(4);
							let key_number: u8 = message.get_u8_at(6);
							// if self.partnered_control_function.get_vt_version_supported(VTVersion::Version6) {
							// 	// TODO: process TAN
							// }
							let event = VTKeyEvent{
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
				// 		case static_cast<std::uint8_t>(Function::PointingEventMessage):
				// 		{
				// 			std::uint16_t xPosition = message.get_uint16_at(1);
				// 			std::uint16_t yPosition = message.get_uint16_at(3);
				// 			std::uint8_t touchState = static_cast<std::uint8_t>(KeyActivationCode::ButtonPressedOrLatched);
				// 			std::uint16_t parentMaskObjectID = NULL_OBJECT_ID;
				// 			if (parentVT->get_vt_version_supported(VTVersion::Version6))
				// 			{
				// 				// VT version is at least 6
				// 				touchState = message.get_uint8_at(5) & 0x0F;
				// 				parentMaskObjectID = message.get_uint16_at(6);
				// 				//! @todo process TAN
				// 			}
				// 			else if (parentVT->get_vt_version_supported(VTVersion::Version4))
				// 			{
				// 				// VT version is either 4 or 5
				// 				touchState = message.get_uint8_at(5);
				// 			}
				// 			if (touchState <= static_cast<std::uint8_t>(KeyActivationCode::ButtonPressAborted))
				// 			{
				// 				parentVT->pointingEventDispatcher.invoke({ parentVT, xPosition, yPosition, parentMaskObjectID, static_cast<KeyActivationCode>(touchState) });
				// 			}
				// 		}
				// 		break;
				// 		case static_cast<std::uint8_t>(Function::VTSelectInputObjectMessage):
				// 		{
				// 			std::uint16_t objectID = message.get_uint16_at(1);
				// 			bool objectSelected = (0x01 == message.get_uint8_at(3));
				// 			bool objectOpenForInput = true;
				// 			if (parentVT->get_vt_version_supported(VTVersion::Version4))
				// 			{
				// 				objectOpenForInput = message.get_bool_at(4, 0);
				// 			}
				// 			if (parentVT->get_vt_version_supported(VTVersion::Version6))
				// 			{
				// 				//! @todo process TAN
				// 			}
				// 			parentVT->selectInputObjectEventDispatcher.invoke({ parentVT, objectID, objectSelected, objectOpenForInput });
				// 		}
				// 		break;
				// 		case static_cast<std::uint8_t>(Function::VTESCMessage):
				// 		{
				// 			std::uint16_t objectID = message.get_uint16_at(1);
				// 			std::uint8_t errorCode = message.get_uint8_at(3) & 0x1F;
				// 			if ((errorCode == static_cast<std::uint8_t>(ESCMessageErrorCode::OtherError)) ||
				// 			    (errorCode <= static_cast<std::uint8_t>(ESCMessageErrorCode::NoInputFieldOpen)))
				// 			{
				// 				if (parentVT->get_vt_version_supported(VTVersion::Version6))
				// 				{
				// 					//! @todo process TAN
				// 				}
				// 				parentVT->escMessageEventDispatcher.invoke({ parentVT, objectID, static_cast<ESCMessageErrorCode>(errorCode) });
				// 			}
				// 		}
				// 		break;
				// 		case static_cast<std::uint8_t>(Function::VTChangeNumericValueMessage):
				// 		{
				// 			std::uint16_t objectID = message.get_uint16_at(1);
				// 			std::uint32_t value = message.get_uint32_at(4);
				// 			if (parentVT->get_vt_version_supported(VTVersion::Version6))
				// 			{
				// 				//! @todo process TAN
				// 			}
				// 			parentVT->changeNumericValueEventDispatcher.invoke({ parentVT, value, objectID });
				// 		}
				// 		break;
				// 		case static_cast<std::uint8_t>(Function::VTChangeActiveMaskMessage):
				// 		{
				// 			std::uint16_t maskObjectID = message.get_uint16_at(1);
				// 			bool missingObjects = message.get_bool_at(3, 2);
				// 			bool maskOrChildHasErrors = message.get_bool_at(3, 3);
				// 			bool anyOtherError = message.get_bool_at(3, 4);
				// 			bool poolDeleted = message.get_bool_at(3, 5);
				// 			std::uint16_t errorObjectID = message.get_uint16_at(4);
				// 			std::uint16_t parentObjectID = message.get_uint16_at(6);
				// 			parentVT->changeActiveMaskEventDispatcher.invoke({ parentVT,
				// 			                                                   maskObjectID,
				// 			                                                   errorObjectID,
				// 			                                                   parentObjectID,
				// 			                                                   missingObjects,
				// 			                                                   maskOrChildHasErrors,
				// 			                                                   anyOtherError,
				// 			                                                   poolDeleted });
				// 		}
				// 		break;
				// 		case static_cast<std::uint8_t>(Function::VTChangeSoftKeyMaskMessage):
				// 		{
				// 			std::uint16_t dataOrAlarmMaskID = message.get_uint16_at(1);
				// 			std::uint16_t softKeyMaskID = message.get_uint16_at(3);
				// 			bool missingObjects = message.get_bool_at(5, 2);
				// 			bool maskOrChildHasErrors = message.get_bool_at(5, 3);
				// 			bool anyOtherError = message.get_bool_at(5, 4);
				// 			bool poolDeleted = message.get_bool_at(5, 5);
				// 			parentVT->changeSoftKeyMaskEventDispatcher.invoke({ parentVT,
				// 			                                                    dataOrAlarmMaskID,
				// 			                                                    softKeyMaskID,
				// 			                                                    missingObjects,
				// 			                                                    maskOrChildHasErrors,
				// 			                                                    anyOtherError,
				// 			                                                    poolDeleted });
				// 		}
				// 		break;
				// 		case static_cast<std::uint8_t>(Function::VTChangeStringValueMessage):
				// 		{
				// 			std::uint16_t objectID = message.get_uint16_at(1);
				// 			std::uint8_t stringLength = message.get_uint8_at(3);
				// 			std::string value = std::string(message.get_data().begin() + 4, message.get_data().begin() + 4 + stringLength);
				// 			parentVT->changeStringValueEventDispatcher.invoke({ value, parentVT, objectID });
				// 		}
				// 		break;
				// 		case static_cast<std::uint8_t>(Function::VTOnUserLayoutHideShowMessage):
				// 		{
				// 			std::uint16_t objectID = message.get_uint16_at(1);
				// 			bool hidden = !message.get_bool_at(3, 0);
				// 			parentVT->userLayoutHideShowEventDispatcher.invoke({ parentVT, objectID, hidden });
				// 			// There could be two layout messages in one packet
				// 			objectID = message.get_uint16_at(4);
				// 			if (objectID != NULL_OBJECT_ID)
				// 			{
				// 				hidden = !message.get_bool_at(6, 0);
				// 				parentVT->userLayoutHideShowEventDispatcher.invoke({ parentVT, objectID, hidden });
				// 			}
				// 			if (parentVT->get_vt_version_supported(VTVersion::Version6))
				// 			{
				// 				//! @todo process TAN
				// 			}
				// 		}
				// 		break;
				// 		case static_cast<std::uint8_t>(Function::VTControlAudioSignalTerminationMessage):
				// 		{
				// 			bool terminated = message.get_bool_at(1, 0);
				// 			parentVT->audioSignalTerminationEventDispatcher.invoke({ parentVT, terminated });
				// 			if (parentVT->get_vt_version_supported(VTVersion::Version6))
				// 			{
				// 				//! @todo process TAN
				// 			}
				// 		}
				// 		break;
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
						}
				// 		break;
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
							self.lastVTStatusTimestamp_ms = TimeDriver::time_elapsed();
							self.activeWorkingSetMasterAddress = message.get_u8_at(1).into();
							self.activeWorkingSetDataMaskObjectID = message.get_u16_at(2).into();
							self.activeWorkingSetSoftKeyMaskObjectID = message.get_u16_at(4).into();
							self.busyCodesBitfield = message.get_u8_at(6);
							self.currentCommandFunctionCode = message.get_u8_at(7);
						}
						VTFunction::GetMemoryMessage => {
				// 			if (StateMachineState::WaitForGetMemoryResponse == parentVT->state)
				// 			{
				// 				parentVT->connectedVTVersion = message.get_uint8_at(1);
				// 				if (0 == message.get_uint8_at(2))
				// 				{
				// 					// There IS enough memory
				// 					parentVT->set_state(StateMachineState::SendGetNumberSoftkeys);
				// 				}
				// 				else
				// 				{
				// 					parentVT->set_state(StateMachineState::Failed);
				// 					CANStackLogger::CAN_stack_log(CANStackLogger::LoggingLevel::Error, "[VT]: Connection Failed Not Enough Memory");
				// 				}
				// 			}
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
						_ => {},
					}
				}
			},
			ParameterGroupNumber::ECUtoVirtualTerminal => {
				if let Ok(vt_function) = message.get_u8_at(0).try_into() {
					match vt_function {
						VTFunction::AuxiliaryInputTypeTwoMaintenanceMessage => {
							let model_identification_code = message.get_u16_at(1);
							let ready = message.get_u8_at(3) != 0;
				
							if ready {

								// TODO: Implement AUX2 devices
					// 			auto result = std::find_if(parentVT->assignedAuxiliaryInputDevices.begin()>,assignedAuxiliaryInputDevices.end(), [&modelIdentificationCode](const AssignedAuxiliaryInputDevice &aux) {
					// 				return aux.modelIdentificationCode == modelIdentificationCode;
					// 			});
					// 			if (result == std::end(parentVT->assignedAuxiliaryInputDevices))
					// 			{
					// 				AssignedAuxiliaryInputDevice inputDevice{ message.get_source_control_function()->get_NAME().get_full_name(), modelIdentificationCode, {} };
					// 				parentVT->assignedAuxiliaryInputDevices.push_back(inputDevice);
									// log::info!("[AUX-N]: New auxiliary input device with name: " + isobus::to_string(inputDevice.name) + " and model identification code: " + isobus::to_string(modelIdentificationCode));
					// 			}
							}
						},
						_ => {},
					}
				}
			},
			_ => {},
		}
		handled
	}
	

}

impl Drop for VirtualTerminalClient {
    fn drop(&mut self) {
        self.terminate();
    }
}
