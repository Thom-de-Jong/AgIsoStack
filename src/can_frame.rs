
// TODO: Implement embedded-can

use crate::{Id, ExtendedId};

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct CanFrame {
    id: Id,
    dlc: usize,
    data: [u8; 8],
}

impl CanFrame {
    pub fn new(id: impl Into<Id>, data: &[u8]) -> Self {
        let id = id.try_into().unwrap_or(Id::Extended(ExtendedId::MAX));
        let dlc = usize::min(data.len(), 8);
        let mut temp_data: [u8; 8] = [0x00; 8];

        temp_data[..dlc].clone_from_slice(data);

        Self {
            id,
            dlc,
            data: temp_data,
        }
    }

    pub fn is_extended(&self) -> bool {
        match self.id {
            Id::Standard(_) => false,
            Id::Extended(_) => true,
        }
    }

    pub fn id(&self) -> Id {
        self.id
    }

    pub fn dlc(&self) -> usize {
        self.dlc
    }

    pub fn data(&self) -> &[u8] {
        self.data.as_slice()
    }
}

impl core::fmt::Display for CanFrame {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(
            f,
            "0x{:08X}, {}, {:02X?}",
            self.id().as_raw(),
            self.dlc(),
            &self.data
        )
    }
}

impl Default for CanFrame {
    fn default() -> Self {
        CanFrame::new(Id::Extended(ExtendedId::MAX), &[])
    }
}
