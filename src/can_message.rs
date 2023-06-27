
use crate::{Id, CanFrame};


#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct CanMessage<const N: usize> {
	id: Id, //< The CAN ID of the message
	data: [u8; N], //< A data buffer for the message
	// std::shared_ptr<ControlFunction> source = nullptr; ///< The source control function of the message
	// std::shared_ptr<ControlFunction> destination = nullptr; ///< The destination control function of the message
}

impl<const N: usize> CanMessage<N> {
    pub fn new(id: Id, data: [u8; N]) -> CanMessage<N> {
        CanMessage {
            id,
            data,
        }
    }

    pub fn data_len(&self) -> usize {
        self.data.len()
    }
}

impl From<CanFrame> for CanMessage<8> {
    fn from(value: CanFrame) -> Self {
        let mut data: [u8; 8] = [0x00; 8];
        data.clone_from_slice(value.data());

        Self {
            id: value.id(),
            data,
        }
    }
}

impl<const N: usize> TryFrom<&CanMessage<N>> for CanFrame {
    type Error = ();

    fn try_from(value: &CanMessage<N>) -> Result<Self, Self::Error> {
        match value.data_len() {
            ..=8 => {
                let mut data: [u8; 8] = [0x00; 8];
                data[..value.data_len()].clone_from_slice(&value.data);
                Ok(CanFrame::new(value.id, &data))
            },
            _ => { Err(()) },
        }
    }
}