
use super::{IndustryGroup, DeviceClass, FunctionCode};

// /// The encoded components that comprise a NAME.
// #[derive(Clone, Copy, PartialEq, PartialOrd, Eq, Ord)]
// pub enum NameParameters {
//     IdentityNumber, //< Usually the serial number of the ECU, unique for all similar control functions.
//     ManufacturerCode, //< The J1939/ISO11783 manufacturer code of the ECU with this NAME.
//     EcuInstance, //< The ECU instance of the ECU with this NAME. Usually increments in NAME order with similar CFs.
//     FunctionInstance, //< The function instance of the ECU. Similar to Virtual Terminal number.
//     FunctionCode, //< The function of the ECU, as defined by ISO11783.
//     DeviceClass, //< Also known as the vehicle system from J1939, describes general ECU type.
//     DeviceClassInstance, //< The instance number of this device class.
//     IndustryGroup, //< The industry group associated with this ECU, such as "agricultural".
//     ArbitraryAddressCapable, //< Defines if this ECU supports address arbitration.
// }

#[derive(Copy, Clone, Default, PartialEq, PartialOrd, Eq, Ord)]
pub struct Name {
    value: u64,
}

impl Name {
    pub fn builder() -> NameBuilder {
        NameBuilder::default()
    }

    pub fn arbitrary_address_capable(&self) -> bool {
        self.value >> 63 != 0
    }
    pub fn industry_group(&self) -> IndustryGroup {
        ((self.value >> 60 & 0x7) as u8).into()
    }
    pub fn device_class_instance(&self) -> u8 {
        (self.value >> 56 & 0xF) as u8
    }
    pub fn device_class(&self) -> DeviceClass {
        ((self.value >> 49 & 0x7F) as u8, Some(self.industry_group())).into()
    }
    pub fn function_code(&self) -> FunctionCode {
        ((self.value >> 40 & 0xFF) as u8).into()
    }
    pub fn function_instance(&self) -> u8 {
        (self.value >> 35 & 0x1F) as u8
    }
    pub fn ecu_instance(&self) -> u8 {
        (self.value >> 32 & 0x7) as u8
    }
    pub fn manufacturer_code(&self) -> u16 {
        (self.value >> 21 & 0x7FF) as u16
    }
    pub fn identity_number(&self) -> u32 {
        (self.value & 0x1FFFFF) as u32
    }
}

impl core::fmt::Display for Name {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "0x{:08X}", self.value)
    }
}

impl core::fmt::Debug for Name {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("Name")
            .field("arbitrary_address_capable", &format_args!("{}", self.arbitrary_address_capable()))
            .field("industry_group", &format_args!("{}", self.industry_group()))
            .field("device_class_instance", &format_args!("{}", self.device_class_instance()))
            .field("device_class", &format_args!("{}", self.device_class()))
            .field("function_code", &format_args!("{}", self.function_code()))
            .field("function_instance", &format_args!("{}", self.function_instance()))
            .field("ecu_instance", &format_args!("{}", self.ecu_instance()))
            .field("manufacturer_code", &format_args!("{}", self.manufacturer_code()))
            .field("identity_number", &format_args!("{}", self.identity_number()))
            .finish()
    }
}

impl From<u64> for Name {
    fn from(value: u64) -> Self {
        Name { value }
    }
}

impl From<&[u8]> for Name {
    fn from(value: &[u8]) -> Self {
        let mut temp: [u8; 8] = [0; 8];

        temp[..usize::min(value.len(), 8)].copy_from_slice(&value[..usize::min(value.len(), 8)]);

        Name {
            value: u64::from_le_bytes(temp),
        }
    }
}

impl From<Name> for u64 {
    fn from(name: Name) -> Self {
        name.value
    }
}

impl From<Name> for [u8; 8] {
    fn from(name: Name) -> Self {
        name.value.to_le_bytes()
    }
}



#[derive(Default)]
pub struct NameBuilder {
    arbitrary_address_capable: bool,
    industry_group: u8,
    device_class_instance: u8,
    device_class: u8,
    function_code: u8,
    function_instance: u8,
    ecu_instance: u8,
    manufacturer_code: u16,
    identity_number: u32,
}

impl NameBuilder {
    pub fn new() -> NameBuilder {
        NameBuilder::default()
    }

    pub fn build(&self) -> Name {
        Name {
            value: (self.arbitrary_address_capable as u64) << 63
                | (self.industry_group as u64 & 0x7) << 60
                | (self.device_class_instance as u64 & 0xF) << 56
                | (self.device_class as u64 & 0x7F) << 49
                | (self.function_code as u64 & 0xFF) << 40
                | (self.function_instance as u64 & 0x1F) << 35
                | (self.ecu_instance as u64 & 0x7) << 32
                | (self.manufacturer_code as u64 & 0x7FF) << 21
                | self.identity_number as u64 & 0x1FFFFF,
        }
    }

    pub fn arbitrary_address_capable(&mut self, value: bool) -> &mut NameBuilder {
        self.arbitrary_address_capable = value;
        self
    }
    pub fn industry_group(&mut self, value: impl Into<u8>) -> &mut NameBuilder {
        self.industry_group = value.into();
        self
    }
    pub fn device_class_instance(&mut self, value: u8) -> &mut NameBuilder {
        self.device_class_instance = value;
        self
    }
    pub fn device_class(&mut self, value: impl Into<u8>) -> &mut NameBuilder {
        self.device_class = value.into();
        self
    }
    pub fn function_code(&mut self, value: impl Into<u8>) -> &mut NameBuilder {
        self.function_code = value.into();
        self
    }
    pub fn function_instance(&mut self, value: u8) -> &mut NameBuilder {
        self.function_instance = value;
        self
    }
    pub fn ecu_instance(&mut self, value: u8) -> &mut NameBuilder {
        self.ecu_instance = value;
        self
    }
    pub fn manufacturer_code(&mut self, value: u16) -> &mut NameBuilder {
        self.manufacturer_code = value;
        self
    }
    pub fn identity_number(&mut self, value: u32) -> &mut NameBuilder {
        self.identity_number = value;
        self
    }
}

impl From<Name> for NameBuilder {
    fn from(value: Name) -> Self {
        let value: u64 = value.into();
        NameBuilder {
            arbitrary_address_capable: (value >> 63) != 0,
            industry_group: (value >> 60 & 0x7) as u8,
            device_class_instance: (value >> 56 & 0xF) as u8,
            device_class: (value >> 49 & 0x7F) as u8,
            function_code: (value >> 40 & 0xFF) as u8,
            function_instance: (value >> 35 & 0x1F) as u8,
            ecu_instance: (value >> 32 & 0x7) as u8,
            manufacturer_code: (value >> 21 & 0x7FF) as u16,
            identity_number: (value & 0x1FFFFF) as u32,
        }
    }
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn name_builder() {
        
        let name = Name::builder()
            .arbitrary_address_capable(true)
            .industry_group(0)
            .device_class_instance(0xFF)
            .device_class(0)
            .function_code(0xFF)
            .function_instance(0)
            .ecu_instance(0xFF)
            .manufacturer_code(0)
            .identity_number(0xFFFFFFFF)
            .build();

        assert_eq!(
            name,
            Name {
                value: 0b1000111100000000111111110000011100000000000111111111111111111111
            }
        );
    }

    #[test]
    fn name_arbitrary_address_capable() {
        let name = Name::from(0b1000111100000000111111110000011100000000000111111111111111111111);
        assert_eq!(name.arbitrary_address_capable(), true);
    }

    #[test]
    fn name_industry_group() {
        let name = Name::from(0b1000111100000000111111110000011100000000000111111111111111111111);
        assert_eq!(name.industry_group(), IndustryGroup::Global);
    }

    #[test]
    fn name_device_class_instance() {
        let name = Name::from(0b1000111100000000111111110000011100000000000111111111111111111111);
        assert_eq!(name.device_class_instance(), 0b1111);
    }

    #[test]
    fn name_device_class() {
        let name = Name::from(0b1000111100000000111111110000011100000000000111111111111111111111);
        assert_eq!(name.device_class(), DeviceClass::NonSpecificSystem(IndustryGroup::Global));
    }

    #[test]
    fn name_function_code() {
        let name = Name::from(0b1000111100000000111111110000011100000000000111111111111111111111);
        assert_eq!(name.function_code(), FunctionCode::NotAvailable);
    }

    #[test]
    fn name_function_instance() {
        let name = Name::from(0b1000111100000000111111110000011100000000000111111111111111111111);
        assert_eq!(name.function_instance(), 0);
    }

    #[test]
    fn name_ecu_instance() {
        let name = Name::from(0b1000111100000000111111110000011100000000000111111111111111111111);
        assert_eq!(name.ecu_instance(), 0b111);
    }

    #[test]
    fn name_manufacturer_code() {
        let name = Name::from(0b1000111100000000111111110000011100000000000111111111111111111111);
        assert_eq!(name.manufacturer_code(), 0);
    }

    #[test]
    fn name_identity_number() {
        let name = Name::from(0b1000111100000000111111110000011100000000000111111111111111111111);
        assert_eq!(name.identity_number(), 0b111111111111111111111);
    }

    #[test]
    fn name_default() {
        assert_eq!(Name::default(), Name { value: 0 });
    }

    #[test]
    fn name_from_u64() {
        let name = Name::from(0b1000111100000000111111110000011100000000000111111111111111111111);
        assert_eq!(
            name,
            Name {
                value: 0b1000111100000000111111110000011100000000000111111111111111111111
            }
        );
    }

    #[test]
    fn name_from_u8_arr() {
        let array: &[u8] = &[
            0b11111111u8,
            0b11111111u8,
            0b00011111u8,
            0b00000000u8,
            0b00000111u8,
            0b11111111u8,
            0b00000000u8,
            0b10001111u8,
        ];
        let name = Name::from(array);
        assert_eq!(
            name,
            Name {
                value: 0b1000111100000000111111110000011100000000000111111111111111111111
            }
        );
    }

    #[test]
    fn u64_from_name() {
        let name: Name = Name {
            value: 0b1000111100000000111111110000011100000000000111111111111111111111,
        };
        assert_eq!(
            u64::from(name),
            0b1000111100000000111111110000011100000000000111111111111111111111
        );
    }

    #[test]
    fn u8_arr_from_name() {
        let name: Name = Name {
            value: 0b1000111100000000111111110000011100000000000111111111111111111111,
        };
        assert_eq!(
            <[u8; 8]>::from(name),
            [
                0b11111111u8,
                0b11111111u8,
                0b00011111u8,
                0b00000000u8,
                0b00000111u8,
                0b11111111u8,
                0b00000000u8,
                0b10001111u8
            ]
        );
    }
}
