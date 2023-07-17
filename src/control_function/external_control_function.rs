use crate::{Name, Address};

pub struct ExternalControlFunction {
    address: Address,
    name: Name,
}

impl ExternalControlFunction {
    pub fn address(&self) -> Address {
        self.address
    }

    pub fn name(&self) -> Name {
        self.name
    }

}
