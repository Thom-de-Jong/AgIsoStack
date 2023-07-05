use crate::CanFrame;

pub trait CanDriverTrait {
    fn is_valid(&mut self) -> bool;
    fn open(&mut self);
    fn close(&mut self);
    fn read(&mut self) -> Option<CanFrame>;
    fn write(&mut self, frame: &CanFrame) -> Result<(), ()>;
}
