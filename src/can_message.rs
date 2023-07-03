
use alloc::vec::Vec;

use crate::{Id, CanFrame, Address, ParameterGroupNumber, ExtendedId, CanPriority};

#[derive(Clone, Eq, PartialEq, Hash, Debug)]
pub struct CanMessage {
	priority: CanPriority, //< The CAN priority of the message
    pgn: ParameterGroupNumber,  //< The paramerer group number of the message
    source: Address, //< The source address of the message
    destination: Address, //< The destination address of the message
	data: Vec<u8>, //< A data buffer for the message
}

impl CanMessage {
    pub fn new(priority: CanPriority, pgn: ParameterGroupNumber, source: Address, destination: Address, data: &[u8]) -> Self {
        CanMessage {
            priority,
            pgn,
            source,
            destination,
            data: data.into(),
        }
    }
    pub fn new_from_id(id: Id, data: &[u8]) -> Self {
        let pgn: ParameterGroupNumber = ((id.as_raw() >> 8) & 0xFFFFFF).into();
        CanMessage {
            priority: ((id.as_raw() >> 26) as u8 & 0b111).into(),
            pgn,
            source: { Address(id.as_raw() as u8) },
            destination: { Address(
                if pgn.is_pdu1() {
                    0
                } else {
                    pgn.pdu_specific()
                }
            )},
            data: data.into(),
        }
    }

    pub fn id(&self) -> Id {
        Id::Extended(
            ExtendedId::new(
                (self.priority as u32) << 26 |
                self.pgn.as_u32() << 8 |
                self.source_address().0 as u32
            ).unwrap_or_default()
        )
    }
    
    pub fn data(&self) -> &[u8] {
        &self.data
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }

    pub fn as_can_frame(&self) -> Result<CanFrame, ()> {
        match self.len() {
            0..=8 => {
                Ok(CanFrame::new(self.id(), &self.data))
            },
            _ => { Err(()) },
        }
    }

    // pub fn pgn(&self) -> ParameterGroupNumber {
    //     ParameterGroupNumber::new(
    //         (self.extended_data_page() as u32 & 0b1) << 17
    //             | (self.data_page() as u32 & 0b1) << 16
    //             | (self.pdu_format() as u32) << 8
    //             | if self.is_pdu1() {
    //                 0
    //             } else {
    //                 self.pdu_specific()
    //             } as u32,
    //     )
    // }
    
    pub fn priority(&self) -> CanPriority {
        self.priority
    }
    pub fn pgn(&self) -> ParameterGroupNumber {
        self.pgn
    }
    pub fn destination_address(&self) -> Address {
        Address(self.pgn.pdu_specific())
    }
    pub fn source_address(&self) -> Address {
        self.source
    }

    pub fn is_address_specific(&self, address: Address) -> bool {
        self.pgn.is_pdu1() && self.pgn.pdu_specific() == address.0
    }
    pub fn is_address_global(&self) -> bool {
        self.pgn.is_pdu2() || self.is_address_specific(Address::GLOBAL)
    }
    pub fn is_address_null(&self) -> bool {
        self.pgn.is_pdu1() && self.is_address_specific(Address::NULL)
    }

    // pub fn builder() -> CanMessageBuilder<'a> {

    // }
}

// pub struct CanMessageBuilder<'a> {
//     priority: u8,
//     extended_data_page: bool,
//     data_page: bool,
//     pdu_format: u8,
//     pdu_specific: u8,
//     destination_address: Address,
//     source_address: Address,
//     data: &'a [u8],
// }

// impl CanMessageBuilder<'_> {
//     pub fn build(&self) -> CanMessage {
//         CanMessage {
//             priority: todo!(),
//             pgn: todo!(),
//             source_address: todo!(),
//             destination_address: todo!(),
//             data: todo!(),
//         }
//     }

//     pub fn priority(mut self, value: u8) -> Self {
//         self.priority = value;
//         self
//     }
//     pub fn pgn(mut self, pgn: ParameterGroupNumber) -> Self {
//         self.extended_data_page = pgn;
//         ParameterGroupNumber::new(
//             (self.extended_data_page() as u32 & 0b1) << 17
//                 | (self.data_page() as u32 & 0b1) << 16
//                 | (self.pdu_format() as u32) << 8
//                 | if self.is_pdu1() {
//                     0
//                 } else {
//                     self.pdu_specific()
//                 } as u32,
//         );

//         self
//     }
    
//     pub fn source_address(mut self, value: Address) -> Self {
//         self.source_address = value;
//         self
//     }
//     pub fn destination_address(mut self, value: Address) -> Self {
//         self.destination_address = value;
//         self
//     }
// }

// impl From<CanFrame> for CanMessage<'_> {
//     fn from(value: CanFrame) -> Self {
//         CanMessage::new(value.id(), value.data())
//     }
// }



// pub trait CanMessageProcessor {
//     fn process_can_message(&mut self, message: CanMessage) -> Option<CanMessage>;
// }