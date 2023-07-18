use crate::{Name, Address};

/// Represents an External Control Function (ECF)
/// 
/// The Name of a ECF is constant and can not change.
/// The Address however can be updated using `.set_address()`.
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct ExternalControlFunction {
    name: Name,
    address: Address,
}

impl ExternalControlFunction {
    pub fn new(name: Name, address: Address) -> Self {
        Self {
            name,
            address,
        }
    }

    pub fn name(&self) -> Name {
        self.name
    }

    pub fn address(&self) -> Address {
        self.address
    }

    pub fn set_address(&mut self, address: Address) {
        self.address = address;
    }
}
