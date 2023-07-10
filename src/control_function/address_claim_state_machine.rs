use core::time::Duration;

use crate::{
    hardware_integration::{TimeDriver, TimeDriverTrait},
    name::Name,
    Address, CanMessage, CanNetworkManager, ParameterGroupNumber,
};

/// Defines the state machine states for address claiming
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
enum State {
    None,                           //< Address claiming is uninitialized
    WaitForClaim,                   //< State machine is waiting for the random delay time
    SendRequestForClaim,            //< State machine is sending the request for address claim
    WaitForRequestContentionPeriod, //< State machine is waiting for the address claim contention period
    SendPreferredAddressClaim,      //< State machine is claiming the preferred address
    ContendForPreferredAddress,     //< State machine is contending the preferred address
    SendArbitraryAddressClaim,      //< State machine is claiming an address
    SendReclaimAddressOnRequest, //< An ECU requested address claim, inform the bus of our current address
    UnableToClaim,               //< State machine could not claim an address
    AddressClaimingComplete,     //< Addres claiming is complete and we have an address
}

impl Default for State {
    fn default() -> Self {
        Self::None
    }
}

pub struct AddressClaimStateMachine {
    name: Name,                   //< The ISO NAME to claim as
    current_state: State,         //< The address claim state machine state
    timestamp: Duration,          //< A timestamp used to find timeouts
    random_claim_delay: Duration, //< The random delay as required by the ISO11783 standard
    preferred_address: Address,   //< The address we'd prefer to claim as (we may not get it)
    claimed_address: Address,     //< A cached version of the actual address we claimed
    is_enabled: bool,             //< Enable/disable state for this state machine
}

impl AddressClaimStateMachine {
    pub fn new(desired_name: Name, preferred_address: Address) -> Option<Self> {
        if preferred_address == Address::GLOBAL || preferred_address == Address::NULL {
            return None;
        }

        let timestamp = TimeDriver::time_elapsed();
        let mut rng = fastrand::Rng::with_seed(timestamp.as_millis() as u64);
        let random_claim_delay = Duration::from_micros(rng.u64(..=255) * 600); // Defined by ISO11783-5

        Some(Self {
            name: desired_name,
            current_state: State::default(),
            timestamp,
            random_claim_delay,
            preferred_address,
            claimed_address: Address::NULL,
            is_enabled: false,
        })
    }

    pub fn name(&self) -> Name {
        self.name
    }

    pub fn enable(&mut self) {
        self.is_enabled = true;
    }
    pub fn disable(&mut self) {
        self.is_enabled = false;
    }
    pub fn is_enabled(&self) -> bool {
        self.is_enabled
    }

    pub fn claimed_address(&self) -> Address {
        self.claimed_address
    }

    /// Processes a CAN message
    pub fn process_can_message(&mut self, message: &CanMessage) -> bool {
        let mut handled = false;
        match message.pgn() {
            ParameterGroupNumber::ParameterGroupNumberRequest => {
                if ParameterGroupNumber::AddressClaim == message.get_pgn_at(0)
                    && State::AddressClaimingComplete == self.current_state
                {
                    self.current_state = State::SendReclaimAddressOnRequest;
                    handled = true;
                }
            }
            ParameterGroupNumber::AddressClaim => {
                if message.is_address_specific(self.claimed_address()) {
                    let name = message.get_name(0);

                    // Check to see if another ECU is hijacking our address
                    // This is not really a needed check, as we can be pretty sure that our address
                    // has been stolen if we're running this logic. But, you never know, someone could be
                    // spoofing us I guess, or we could be getting an echo? CAN Bridge from another channel?
                    // Seemed safest to just confirm.
                    if name != self.name() {
                        // Wait for things to shake out a bit, then claim a new address.
                        self.current_state = State::WaitForRequestContentionPeriod;
                        log::warn!("[AC]: Internal control function {} must re-arbitrate its address because it was stolen by another ECU with NAME {name}.", self.name());
                        handled = true;
                    }
                }
            }
            _ => {}
        }
        handled
    }

    /// Update based on the current state
    pub fn update(&mut self, network_manager: &mut CanNetworkManager) {
        if !self.is_enabled {
            self.current_state = State::None;
        }

        match self.current_state {
            State::None => {
                self.timestamp = TimeDriver::time_elapsed();
                self.current_state = State::WaitForClaim;
            }
            State::WaitForClaim => {
                if TimeDriver::time_elapsed() - self.timestamp >= self.random_claim_delay {
                    self.current_state = State::SendRequestForClaim;
                }
            }
            State::SendRequestForClaim => {
                network_manager.send_request_to_claim();
                self.timestamp = TimeDriver::time_elapsed();
                self.current_state = State::WaitForRequestContentionPeriod;
            }
            State::WaitForRequestContentionPeriod => {
                // Wait for other Control Functions to respond.
                if TimeDriver::time_elapsed() - self.timestamp
                    >= Duration::from_millis(250) + self.random_claim_delay
                {
                    // After the wait, check if our address has been claimed.
                    let other_name = network_manager.get_name_by_address(self.preferred_address);

                    match other_name {
                        Some(name) => {
                            // Check if we are arbitrary address capable.
                            if self.name().arbitrary_address_capable() {
                                // We will move to another address if whoever is in our spot has a lower NAME.
                                if name < self.name() {
                                    self.current_state = State::SendArbitraryAddressClaim;
                                } else {
                                    self.current_state = State::SendPreferredAddressClaim;
                                }
                            } else {
                                if name > self.name() {
                                    // Our address is not free, we cannot be at an arbitrary address, and address is contendable.
                                    self.current_state = State::ContendForPreferredAddress;
                                } else {
                                    // Can't claim because we cannot tolerate an arbitrary address, and the CF at that spot wins contention.
                                    self.current_state = State::UnableToClaim;
                                }
                            }
                        }
                        None => {
                            // Our address is free. This is the best outcome. Claim it.
                            self.current_state = State::SendPreferredAddressClaim
                        }
                    }
                }
            }
            State::SendPreferredAddressClaim => {
                self.send_address_claim(network_manager, self.name(), self.preferred_address);
                log::debug!(
                    "[AC]: Internal control function {} has claimed address {}",
                    self.name(),
                    self.claimed_address()
                );
                self.current_state = State::AddressClaimingComplete;
            }
            State::SendArbitraryAddressClaim => {
                // Request a free address from the network manager.
                match network_manager.next_free_address(self.preferred_address) {
                    Some(address) => {
                        self.send_address_claim(network_manager, self.name(), address);
                        log::debug!("[AC]: Internal control function {} could not use the preferred address, but has claimed address {}", self.name(), address);
                        self.current_state = State::AddressClaimingComplete;
                    }
                    None => {
                        log::debug!(
                            "[AC]: Internal control function {} failed to claim an address",
                            self.name()
                        );
                        self.current_state = State::UnableToClaim;
                    }
                }
            }
            State::SendReclaimAddressOnRequest => {
                self.send_address_claim(network_manager, self.name(), self.preferred_address);
                self.current_state = State::AddressClaimingComplete;
            }
            State::ContendForPreferredAddress => {
                // TODO: Non-arbitratable address contention (there is not a good reason to use this, but we should add support anyways)
            }
            State::UnableToClaim => {}
            State::AddressClaimingComplete => {}
        }

        // Update our claimed address in a cache variable.
        self.claimed_address = network_manager
            .internal_address(self.name())
            .unwrap_or_default()
    }

    pub fn send_address_claim(
        &mut self,
        network_manager: &mut CanNetworkManager,
        name: Name,
        address: Address,
    ) {
        self.claimed_address = address;
        network_manager.send_address_claim(name, address);
    }
}
