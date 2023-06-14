
use super::{Name, DeviceClass, IndustryGroup, FunctionCode};

/// A struct that associates a NAME parameter with a value of that parameter.
/// This struct is used to match a partner control function with specific criteria that
/// defines it. Use these to define what device you want to talk to.
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub enum NameFilter {
    ArbitraryAddressCapable(bool),
    IndustryGroup(IndustryGroup),
    DeviceClassInstance(u8),
    DeviceClass(DeviceClass),
    FunctionCode(FunctionCode),
    FunctionInstance(u8),
    EcuInstance(u8),
    ManufacturerCode(u16),
    IdentityNumber(u32),
}

impl NameFilter {
    /// Returns true if a NAME matches this filter's component.
    pub fn check_name_matches_filter(&self, name: Name) -> bool {
        match self {
            NameFilter::ArbitraryAddressCapable(val) => { name.arbitrary_address_capable() == *val },
            NameFilter::IndustryGroup(val) => { name.industry_group() == *val },
            NameFilter::DeviceClassInstance(val) => { name.device_class_instance() == *val },
            NameFilter::DeviceClass(val) => { name.device_class() == *val },
            NameFilter::FunctionCode(val) => { name.function_code() == *val },
            NameFilter::FunctionInstance(val) => { name.function_instance() == *val },
            NameFilter::EcuInstance(val) => { name.ecu_instance() == *val },
            NameFilter::ManufacturerCode(val) => { name.manufacturer_code() == *val },
            NameFilter::IdentityNumber(val) => { name.identity_number() == *val },
        }
    }
}
