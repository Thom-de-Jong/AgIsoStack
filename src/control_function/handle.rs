use core::cell::RefCell;

use alloc::rc::{Rc, Weak};

use super::{ControlFunction, InternalControlFunction};


#[derive(Clone, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct ControlFunctionHandle(Rc<RefCell<ControlFunction>>);

impl ControlFunctionHandle {
    pub fn new(cf: ControlFunction) -> ControlFunctionHandle {
        ControlFunctionHandle(Rc::new(RefCell::new(cf)))
    }
}

#[derive(Clone, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct WeakControlFunctionHandle(Weak<RefCell<ControlFunction>>);


// pub type ControlFunctionHandle = Rc<RefCell<ControlFunction>>;
// pub type WeakControlFunctionHandle = Weak<RefCell<ControlFunction>>;

#[derive(Clone)]
pub struct InternalControlFunctionHandle(Rc<RefCell<InternalControlFunction>>);

// #[derive(Clone)]
// pub struct ControlFunctionHandle(Rc<RefCell<ControlFunction>>);

// impl From<Name> for ControlFunctionHandle {
//     fn from(value: Name) -> Self {
//         ControlFunctionHandle(value)
//     }
// }