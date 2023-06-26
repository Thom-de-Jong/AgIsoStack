
use crate::{Address, name::Name};

use super::AddressClaimStateMachine;

static INTERNAL_CONTROL_FUNCTION_LIST: heapless::Vec<InternalControlFunction, 2> = heapless::Vec::new();

/// Lets the network manager know if any ICF changed address since the last update.
static ANY_CHANGED_ADDRESS: bool = false;

pub struct InternalControlFunction {
    object_changed_address_since_last_update: bool,
    state_machine: AddressClaimStateMachine,
}

impl InternalControlFunction {
    pub fn address(&self) -> Address {
        self.state_machine.claimed_address()
    }
    
    pub fn can_port(&self) -> u8 {
        self.state_machine.can_port()
    }
    
    pub fn name(&self) -> Name {
        self.state_machine.name()
    }


    // /// @brief Constructor for an internal control function
	// /// @param[in] desiredName The NAME for this control function to claim as
	// /// @param[in] preferredAddress The preferred NAME for this control function
	// /// @param[in] CANPort The CAN channel index for this control function to use
    // pub fn new(desired_name: Name, prefered_address: u8, can_port: u8) -> InternalControlFunction {
    //     InternalControlFunction {
    //         address: todo!(),
    //         can_port,
    //         name: todo!(),
    //         object_changed_address_since_last_update: false,
    //     }
    //     // let val = &icf;

	// 	// if let Err(_) = INTERNAL_CONTROL_FUNCTION_LIST.push(icf) {
    //     //     // log::error!("Not enough space in the INTERNAL_CONTROL_FUNCTION_LIST. Allocated items; {}", env!("INTERNAL_CONTROL_FUNCTION_LIST_SIZE"))
    //     // }

    //     // ifc
    // }

	/// @brief Destructor for an internal control function
	// ~InternalControlFunction();

	// /// Returns a reference to an internal control function from the list of all internal control functions.
	// pub fn get(index: u32) -> Option<&InternalControlFunction> {
    //     INTERNAL_CONTROL_FUNCTION_LIST.get(index)
    // }

	// /// Returns a mutable reference to an internal control function from the list of all internal control functions.
	// pub fn get_mut(index: u32) -> Option<&mut InternalControlFunction> {
    //     INTERNAL_CONTROL_FUNCTION_LIST.get_mut(index)
    // }

	/// Returns the number of internal control functions that exist.
	// pub fn number_of_internal_control_functions() -> usize {
    //     INTERNAL_CONTROL_FUNCTION_LIST.len()
    // }

	/// Lets network manager know a control function changed address recently.
	/// These tell the network manager when the address table needs to be explicitly
	/// updated for an internal control function claiming a new address.
	/// Other CF types are handled in Rx message processing.
	pub fn any_internal_control_function_changed_address() -> bool { // (CANLibBadge<CANNetworkManager>)
        ANY_CHANGED_ADDRESS//.load(core::sync::atomic::Ordering::Relaxed)
    }

	/// Used to determine if the internal control function changed address since the last network manager update.
	pub fn changed_address_since_last_update(&self) -> bool { // (CANLibBadge<CANNetworkManager>)
        self.object_changed_address_since_last_update
    }

	/// Used by the network manager to tell the ICF that the address claim state machine needs to process a J1939 command to move address.
	pub fn process_commanded_address(&self, commanded_address: u8) { // (CANLibBadge<CANNetworkManager>)
        self.state_machine.process_commanded_address(commanded_address);
    }

	/// Updates all address claim state machines.
    pub fn update_address_claiming() { // (CANLibBadge<CANNetworkManager>)
        ANY_CHANGED_ADDRESS = false;
        for &cf in INTERNAL_CONTROL_FUNCTION_LIST.iter() {
            cf.update();
        }
    }

    pub fn update(&mut self) {
		let previous_address = self.address;
        self.object_changed_address_since_last_update = false;
        self.state_machine.update();
		let address = self.state_machine.get_claimed_address();

		if previous_address != address {
			ANY_CHANGED_ADDRESS = true;
            self.object_changed_address_since_last_update = true;
		}
	}
}

impl Drop for InternalControlFunction {
    fn drop(&mut self) {
        // const std::lock_guard<std::mutex> lock(ControlFunction::controlFunctionProcessingMutex);

		// if (!internalControlFunctionList.empty())
		// {
		// 	auto thisObject = std::find(internalControlFunctionList.begin(), internalControlFunctionList.end(), this);

		// 	if (internalControlFunctionList.end() != thisObject)
		// 	{
		// 		*thisObject = nullptr; // Don't erase, just null it out. Erase could cause a double free.
		// 	}
		// }
    }
}


// friend class CANNetworkManager;
// static std::mutex controlFunctionProcessingMutex; ///< Protects the control function tables
// NAME controlFunctionNAME; ///< The NAME of the control function
// Type controlFunctionType; ///< The Type of the control function
// std::uint8_t address; ///< The address of the control function
// std::uint8_t canPortIndex; ///< The CAN channel index of the control function

//x static std::vector<InternalControlFunction *> internalControlFunctionList; ///< A list of all internal control functions that exist
//x static bool anyChangedAddress; ///< Lets the network manager know if any ICF changed address since the last update
// AddressClaimStateMachine stateMachine; ///< The address claimer for this ICF
// bool objectChangedAddressSinceLastUpdate; ///< Tracks if this object has changed address since the last update
