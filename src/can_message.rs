use alloc::vec::Vec;

use crate::{name::Name, Address, CanFrame, CanPriority, ExtendedId, Id, ParameterGroupNumber};

#[derive(Clone, Eq, PartialEq, Hash, Debug)]
pub struct CanMessage {
    priority: CanPriority,     //< The CAN priority of the message
    pgn: ParameterGroupNumber, //< The paramerer group number of the message
    source: Address,           //< The source address of the message
    destination: Address,      //< The destination address of the message
    data: Vec<u8>,             //< A data buffer for the message
}

impl CanMessage {
    pub fn new(
        priority: CanPriority,
        pgn: ParameterGroupNumber,
        source: Address,
        destination: Address,
        data: &[u8],
    ) -> Self {
        CanMessage {
            priority,
            pgn,
            source,
            destination,
            data: data.into(),
        }
    }
    pub fn new_from_pdu2(
        priority: CanPriority,
        pgn: ParameterGroupNumber,
        source: Address,
        data: &[u8],
    ) -> Self {
        CanMessage {
            priority,
            pgn,
            source,
            destination: Address::GLOBAL,
            data: data.into(),
        }
    }
    pub fn new_from_id(id: Id, data: &[u8]) -> Self {
        let pgn: ParameterGroupNumber = ((id.as_raw() >> 8) & 0x03FF00).into();
        CanMessage {
            priority: ((id.as_raw() >> 26) as u8 & 0b111).into(),
            pgn,
            source: Address(id.as_raw() as u8),
            destination: {
                if pgn.is_pdu1() {
                    Address((id.as_raw() >> 8) as u8)
                } else {
                    Address::GLOBAL
                }
            },
            data: data.into(),
        }
    }

    pub fn id(&self) -> Id {
        Id::Extended(
            ExtendedId::new(
                (self.priority as u32) << 26
                    | self.pgn.as_u32() << 8
                    | if self.pgn().is_pdu1() {
                        u8::from(self.destination_address()) as u32
                    } else {
                        0u32
                    } << 8
                    | u8::from(self.source_address()) as u32,
            )
            .unwrap_or_default(),
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
            0..=8 => Ok(CanFrame::new(self.id(), &self.data)),
            _ => Err(()),
        }
    }

    pub fn priority(&self) -> CanPriority {
        self.priority
    }
    pub fn pgn(&self) -> ParameterGroupNumber {
        self.pgn
    }
    pub fn destination_address(&self) -> Address {
        self.destination
    }
    pub fn source_address(&self) -> Address {
        self.source
    }

    pub fn is_address_specific(&self, address: Address) -> bool {
        self.pgn.is_pdu1() && self.destination == address
    }
    pub fn is_address_global(&self) -> bool {
        self.pgn.is_pdu2() || self.is_address_specific(Address::GLOBAL)
    }
    pub fn is_address_null(&self) -> bool {
        self.pgn.is_pdu1() && self.is_address_specific(Address::NULL)
    }

    pub fn builder() -> CanMessageBuilder {
        CanMessageBuilder::new()
    }

    pub fn get_bool_at(&self, index: usize, bit: usize) -> bool {
        match self
            .data
            .get(index)
            .map(|&value| (value & (0b1 << bit)) != 0)
        {
            Some(val) => val,
            None => {
                log::error!("Index out of range!");
                bool::default()
            }
        }
    }
    pub fn get_u8_at(&self, index: usize) -> u8 {
        match self.data.get(index).copied() {
            Some(val) => val,
            None => {
                log::error!("Index out of range!");
                u8::default()
            }
        }
    }
    pub fn get_u16_at(&self, index: usize) -> u16 {
        match self.data.get(index..=index + 1).map(|value| {
            let mut data: [u8; 2] = [0; 2];
            data.copy_from_slice(value);
            u16::from_le_bytes(data)
        }) {
            Some(val) => val,
            None => {
                log::error!("Index out of range!");
                u16::default()
            }
        }
    }
    pub fn get_u24_at(&self, index: usize) -> u32 {
        match self.data.get(index..=index + 2).map(|value| {
            let mut data: [u8; 3] = [0; 3];
            data.copy_from_slice(value);
            u32::from_le_bytes([data[0], data[1], data[2], 0])
        }) {
            Some(val) => val,
            None => {
                log::error!("Index out of range!");
                u32::default()
            }
        }
    }
    pub fn get_u32_at(&self, index: usize) -> u32 {
        match self.data.get(index..=index + 3).map(|value| {
            let mut data: [u8; 4] = [0; 4];
            data.copy_from_slice(value);
            u32::from_le_bytes(data)
        }) {
            Some(val) => val,
            None => {
                log::error!("Index out of range!");
                u32::default()
            }
        }
    }

    pub fn get_pgn_at(&self, index: usize) -> ParameterGroupNumber {
        match self
            .data
            .get(index..index + 3)
            .map(|value| ParameterGroupNumber::from([value[0], value[1], value[2], 0].as_slice()))
        {
            Some(val) => val,
            None => {
                log::error!("Index out of range!");
                ParameterGroupNumber::default()
            }
        }
    }
    pub fn get_name(&self, index: usize) -> Name {
        match self.data.get(index..index + 8).map(|value| value.into()) {
            Some(val) => val,
            None => {
                log::error!("Index out of range!");
                Name::default()
            }
        }
    }
}

impl core::fmt::Display for CanMessage {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(
            f,
            "{{ P: {}, PGN: {:?}, S: {}, D: {}, {:02X?} }}",
            self.priority, self.pgn, self.source, self.destination, self.data
        )
    }
}

impl From<CanFrame> for CanMessage {
    fn from(value: CanFrame) -> Self {
        CanMessage::new_from_id(value.id(), &value.data()[..value.dlc()])
    }
}


#[derive(Default)]
pub struct CanMessageBuilder {
    priority: CanPriority,
    pgn: ParameterGroupNumber,
    source: Address,
    destination: Address,
    data: Vec<u8>,
}

impl CanMessageBuilder {
    pub fn new() -> CanMessageBuilder {
        CanMessageBuilder::default()
    }

    pub fn build(&self) -> CanMessage {
        CanMessage {
            priority: self.priority,
            pgn: self.pgn,
            source: self.source,
            destination: self.destination,
            data: self.data.clone(),
        }
    }

    pub fn priority(&mut self, value: CanPriority) -> &mut Self {
        self.priority = value;
        self
    }
    pub fn pgn(&mut self, value: ParameterGroupNumber) -> &mut Self {
        self.pgn = value;
        self
    }

    pub fn source_address(&mut self, value: Address) -> &mut Self {
        self.source = value;
        self
    }

    pub fn destination_address(&mut self, value: Address) -> &mut Self {
        self.destination = value;
        self
    }

    pub fn data(&mut self, value: &[u8]) -> &mut Self {
        self.data.copy_from_slice(value);
        self
    }
}

