// //================================================================================================
// /// @file can_address_claim_state_machine.hpp
// ///
// /// @brief Defines a class for managing the address claiming process
// /// @author Adrian Del Grosso
// ///
// /// @copyright 2022 Adrian Del Grosso
// //================================================================================================

// #ifndef CAN_ADDRESS_CLAIM_STATE_MACHINE_HPP
// #define CAN_ADDRESS_CLAIM_STATE_MACHINE_HPP

// #include "isobus/isobus/can_NAME.hpp"
// #include "isobus/isobus/can_constants.hpp"

// namespace isobus
// {
// 	class CANMessage; ///< Forward declare CANMessage

// 	//================================================================================================
// 	/// @class AddressClaimStateMachine
// 	///
// 	/// @brief State machine for managing the J1939/ISO11783 address claim process
// 	///
// 	/// @details This class manages address claiming for internal control functions
// 	/// and keeps track of things like requests for address claim.
// 	//================================================================================================
// 	class AddressClaimStateMachine
// 	{
// 	public:
// 		/// @brief The destructor for the address claim state machine
// 		~AddressClaimStateMachine();

// 		/// @brief Returns the current state of the state machine
// 		/// @returns The current state of the state machine
// 		State get_current_state() const;

// 		/// @brief Attempts to process a commanded address.
// 		/// @details If the state machine has claimed successfully before,
// 		/// this will attempt to move a NAME from the claimed address to the new, specified address.
// 		/// @param[in] commandedAddress The address to attempt to claim
// 		void process_commanded_address(std::uint8_t commandedAddress);

// 		/// @brief Enables or disables the address claimer
// 		/// @param[in] value true if you want the class to claim, false if you want to be a sniffer only
// 		void set_is_enabled(bool value);

// 		/// @brief Returns if the address claimer is enabled
// 		/// @returns true if the class will address claim, false if in sniffing mode
// 		bool get_enabled() const;

// 		/// @brief Returns the address claimed by the state machine or 0xFE if none claimed
// 		/// @returns The address claimed by the state machine or 0xFE if no address has been claimed
// 		std::uint8_t get_claimed_address() const;

// 		/// @brief Updates the state machine, should be called periodically
// 		void update();

// 	private:
// 		/// @brief Sets the current state machine state
// 		void set_current_state(State value);

// 		/// @brief Sends the PGN request for the address claim PGN
// 		bool send_request_to_claim() const;

// 		/// @brief Sends the address claim message
// 		/// @param[in] address The address to claim
// 		bool send_address_claim(std::uint8_t address);

// 	};

// } // namespace isobus

// #endif // CAN_ADDRESS_CLAIM_STATE_MACHINE_HPP

use core::time::Duration;

use crate::{Address, name::Name, hardware_integration::{TimeDriver, TimeDriverTrait}, CanFrame};

/// Defines the state machine states for address claiming
enum State {
	None, //< Address claiming is uninitialized
	WaitForClaim, //< State machine is waiting for the random delay time
	SendRequestForClaim, //< State machine is sending the request for address claim
	WaitForRequestContentionPeriod, //< State machine is waiting for the address claim contention period
	SendPreferredAddressClaim, //< State machine is claiming the preferred address
	ContendForPreferredAddress, //< State machine is contending the preferred address
	SendArbitraryAddressClaim, //< State machine is claiming an address
	SendReclaimAddressOnRequest, //< An ECU requested address claim, inform the bus of our current address
	UnableToClaim, //< State machine could not claim an address
	AddressClaimingComplete, //< Addres claiming is complete and we have an address
}

impl Default for State {
    fn default() -> Self {
        State::None
    }
}

pub struct AddressClaimStateMachine {
	name: Name, //< The ISO NAME to claim as
	current_state: State, //< The address claim state machine state
// 	std::uint32_t m_timestamp_ms = 0; ///< A generic timestamp in milliseconds used to find timeouts
    can_port: u8, //< The CAN channel index to claim on
// 	std::uint8_t m_preferredAddress; ///< The address we'd prefer to claim as (we may not get it)
	random_claim_delay: Duration, //< The random delay as required by the ISO11783 standard
    claimed_address: Address, //< The actual address we ended up claiming
    is_enabled: bool, //<  Enable/disable state for this state machine
}

impl AddressClaimStateMachine {
	/// The constructor of the state machine class
	/// @param[in] preferredAddressValue The address you prefer to claim
	/// @param[in] ControlFunctionNAME The NAME you want to claim
	/// @param[in] portIndex The CAN channel index to claim on
	pub fn new(desired_name: Name, preferred_address: Address, can_port: u8) -> Option<AddressClaimStateMachine> {
        if preferred_address == Address::GLOBAL || preferred_address == Address::NULL { //|| port_index >= CAN_PORT_MAXIMUM {
            return None;
        }

        let mut rng = fastrand::Rng::with_seed(TimeDriver::time_elapsed().as_millis() as u64);
		let random_claim_delay = Duration::from_micros(rng.u64(..=255) * 600); // Defined by ISO11783-5

		// CANNetworkManager::CANNetwork.add_global_parameter_group_number_callback(static_cast<std::uint32_t>(CANLibParameterGroupNumber::ParameterGroupNumberRequest), process_rx_message, this);
		// CANNetworkManager::CANNetwork.add_global_parameter_group_number_callback(static_cast<std::uint32_t>(CANLibParameterGroupNumber::AddressClaim), process_rx_message, this);
    
        Some(AddressClaimStateMachine {
            name: desired_name,
            current_state: State::default(),
            can_port,
            claimed_address: Address::default(),
            random_claim_delay,
            is_enabled: false,
        })
    }

	pub fn can_port(&self) -> u8 {
        self.can_port
    }
    
    pub fn name(&self) -> Name {
        self.name
    }

	pub fn set_is_enabled(&mut self, value: bool) {
		self.is_enabled = value;
	}

	pub fn is_enabled(&self) -> bool {
		self.is_enabled
	}

	pub fn claimed_address(&self) -> Address {
		self.claimed_address
	}

	/// Processes a CAN message
	/// @param[in] message The CAN message being received
	/// @param[in] parentPointer A context variable to find the relevant address claimer
	fn process_rx_message(&self, message: &CanFrame) {
		// if (message.can_port_index() == parent.port_index) && (parent.is_enabled()) {
		// 	switch (message.get_identifier().get_parameter_group_number())
		// 	{
		// 		case static_cast<std::uint32_t>(CANLibParameterGroupNumber::ParameterGroupNumberRequest):
		// 		{
		// 			const auto &messageData = message.get_data();
		// 			std::uint32_t requestedPGN = messageData.at(0);
		// 			requestedPGN |= (static_cast<std::uint32_t>(messageData.at(1)) << 8);
		// 			requestedPGN |= (static_cast<std::uint32_t>(messageData.at(2)) << 16);

		// 			if ((static_cast<std::uint32_t>(CANLibParameterGroupNumber::AddressClaim) == requestedPGN) &&
		// 			    (State::AddressClaimingComplete == parent->get_current_state()))
		// 			{
		// 				parent->set_current_state(State::SendReclaimAddressOnRequest);
		// 			}
		// 		}
		// 		break;

		// 		case static_cast<std::uint32_t>(CANLibParameterGroupNumber::AddressClaim):
		// 		{
		// 			if (parent->m_claimedAddress == message.get_identifier().get_source_address())
		// 			{
		// 				const auto &messageData = message.get_data();
		// 				std::uint64_t NAMEClaimed = messageData.at(0);
		// 				NAMEClaimed |= (static_cast<uint64_t>(messageData.at(1)) << 8);
		// 				NAMEClaimed |= (static_cast<uint64_t>(messageData.at(2)) << 16);
		// 				NAMEClaimed |= (static_cast<uint64_t>(messageData.at(3)) << 24);
		// 				NAMEClaimed |= (static_cast<uint64_t>(messageData.at(4)) << 32);
		// 				NAMEClaimed |= (static_cast<uint64_t>(messageData.at(5)) << 40);
		// 				NAMEClaimed |= (static_cast<uint64_t>(messageData.at(6)) << 48);
		// 				NAMEClaimed |= (static_cast<uint64_t>(messageData.at(7)) << 56);

		// 				// Check to see if another ECU is hijacking our address
		// 				// This is not really a needed check, as we can be pretty sure that our address
		// 				// has been stolen if we're running this logic. But, you never know, someone could be
		// 				// spoofing us I guess, or we could be getting an echo? CAN Bridge from another channel?
		// 				// Seemed safest to just confirm.
		// 				if (NAMEClaimed != parent->m_isoname.get_full_name())
		// 				{
		// 					// Wait for things to shake out a bit, then claim a new address.
		// 					parent->set_current_state(State::WaitForRequestContentionPeriod);
		// 					parent->m_claimedAddress = NULL_CAN_ADDRESS;
		// 					CANStackLogger::warn("[AC]: Internal control function %016llx on channel %u must re-arbitrate its address because it was stolen by another ECU with NAME %016llx.",
		// 					                     parent->m_isoname.get_full_name(),
		// 					                     parent->m_portIndex,
		// 					                     NAMEClaimed);
		// 				}
		// 			}
		// 		}
		// 		break;

		// 		default:
		// 		{
		// 		}
		// 		break;
		// 	}
		// }
    }

}
