
use alloc::vec::Vec;

use crate::{Id, CanFrame, Address};

#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct CanMessage {
	id: Id, //< The CAN ID of the message
	data: Vec<u8>, //< A data buffer for the message
	// std::shared_ptr<ControlFunction> source = nullptr; ///< The source control function of the message
	// std::shared_ptr<ControlFunction> destination = nullptr; ///< The destination control function of the message
}

impl CanMessage {
    pub fn new(id: Id, data: &[u8]) -> CanMessage {
        CanMessage {
            id,
            data: Vec::from(data),
        }
    }
    
    pub fn data(&self) -> &[u8] {
        &self.data
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }

    pub fn as_can_frame(&self) -> Result<CanFrame, ()> {
        match self.len() {
            ..=8 => {
                Ok(CanFrame::new(self.id, &self.data))
            },
            _ => { Err(()) },
        }
    }

    // pub fn pgn(&self) -> PGN {
    //     let pgn = PGN::new(
    //         (self.extended_data_page as u32 & 0b1) << 17
    //             | (self.data_page as u32 & 0b1) << 16
    //             | (self.pdu_format as u32) << 8
    //             | if self.pdu_format < 240 {
    //                 0
    //             } else {
    //                 self.pdu_specific
    //             } as u32,
    //     );
    //     // log::debug!("0x{:06X}", pgn.as_u32());
    //     pgn
    // }

    
    pub fn priority(&self) -> u8 {
        ((self.id.as_raw() >> 26) & 0b111) as u8
    }
    pub fn extended_data_page(&self) -> bool {
        ((self.id.as_raw() >> 25) & 0b1) != 0
    }
    pub fn data_page(&self) -> bool {
        ((self.id.as_raw() >> 24) & 0b1) != 0
    }
    pub fn pdu_format(&self) -> u8 {
        (self.id.as_raw() >> 16) as u8
    }
    pub fn pdu_specific(&self) -> u8 {
        (self.id.as_raw() >> 8) as u8
    }
    pub fn destination_address(&self) -> Address {
        Address(self.pdu_specific())
    }
    pub fn source_address(&self) -> Address {
        Address(self.id.as_raw() as u8)
    }

    pub fn is_address_specific(&self, address: Address) -> bool {
        self.is_pdu1() && self.pdu_specific() == address.0
    }
    pub fn is_address_global(&self) -> bool {
        self.is_pdu2() || self.is_address_specific(Address::GLOBAL)
    }
    pub fn is_address_null(&self) -> bool {
        self.is_pdu1() && self.is_address_specific(Address::NULL)
    }
    pub fn is_pdu1(&self) -> bool {
        self.pdu_format() < 240
    }
    pub fn is_pdu2(&self) -> bool {
        self.pdu_format() >= 240
    }
}

impl From<CanFrame> for CanMessage {
    fn from(value: CanFrame) -> Self {
        CanMessage::new(value.id(), value.data())
    }
}



pub trait CanMessageProcessor {
    fn process_can_message(&mut self, message: CanMessage) -> Option<CanMessage>;
}