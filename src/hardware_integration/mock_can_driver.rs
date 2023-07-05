use super::CanDriverTrait;
use crate::CanFrame;

pub struct MockCanDriver {}

impl MockCanDriver {
    pub fn new() -> Self {
        Self {}
    }
}

impl CanDriverTrait for MockCanDriver {
    fn is_valid(&mut self) -> bool {
        true
    }

    fn open(&mut self) {}

    fn close(&mut self) {}

    fn read(&mut self) -> Option<CanFrame> {
        None
    }

    fn write(&mut self, _frame: &CanFrame) -> Result<(), ()> {
        Ok(())
    }
}
