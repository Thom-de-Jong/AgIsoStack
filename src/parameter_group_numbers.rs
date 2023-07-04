

/// PGNs commonly used by the CAN stack.
#[repr(u32)]
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub enum ParameterGroupNumber {
    Any = 0x000000,
    AgriculturalGuidanceMachineInfo = 0x00AC00,
    AgriculturalGuidanceSystemCommand = 0x00AD00,
    DiagnosticMessage22 = 0x00C300,
    ExtendedTransportProtocolDataTransfer = 0x00C700,
    ExtendedTransportProtocolConnectionManagement = 0x00C800,
    ProcessData = 0x00CB00,
    RequestForRepetitionRate = 0x00CC00,
    DiagnosticMessage13 = 0x00DF00,
    VirtualTerminalToECU = 0x00E600,
    ECUtoVirtualTerminal = 0x00E700,
    Acknowledge = 0x00E800,
    ParameterGroupNumberRequest = 0x00EA00,
    TransportProtocolData = 0x00EB00,
    TransportProtocolCommand = 0x00EC00,
    AddressClaim = 0x00EE00,
    ProprietaryA = 0x00EF00,
    MachineSelectedSpeed = 0x00F022,
    ProductIdentification = 0x00FC8D,
    ControlFunctionFunctionalities = 0x00FC8E,
    DiagnosticProtocolIdentification = 0x00FD32,
    MachineSelectedSpeedCommand = 0x00FD43,
    WorkingSetMaster = 0x00FE0D,
    LanguageCommand = 0x00FE0F,
    MaintainPower = 0x00FE47,
    WheelBasedSpeedAndDistance = 0x00FE48,
    GroundBasedSpeedAndDistance = 0x00FE49,
    ECUIdentificationInformation = 0x00FDC5,
    DiagnosticMessage1 = 0x00FECA,
    DiagnosticMessage2 = 0x00FECB,
    DiagnosticMessage3 = 0x00FECC,
    DiagnosticMessage11 = 0x00FED3,
    CommandedAddress = 0x00FED8,
    SoftwareIdentification = 0x00FEDA,
    AllImplementsStopOperationsSwitchState = 0x00FD02
}

impl ParameterGroupNumber {
    pub fn new(value: u32) -> Self {
        value.into()
    }

    pub fn from_le_bytes(bytes: &[u8]) -> Self {
        Self::new(u32::from_le_bytes([bytes[0], bytes[1], bytes[2], 0]))
    }

    pub fn as_u32(&self) -> u32 {
        *self as u32
    }

    pub fn as_bytes(&self) -> [u8; 3] {
        let bytes: [u8; 4] = self.as_u32().to_le_bytes();
        [bytes[0], bytes[1], bytes[2]]
    }

    pub fn extended_data_page(&self) -> bool {
        ((self.as_u32() >> 17) & 0b1) != 0
    }
    pub fn data_page(&self) -> bool {
        ((self.as_u32() >> 16) & 0b1) != 0
    }
    pub fn pdu_format(&self) -> u8 {
        (self.as_u32() >> 8) as u8
    }
    pub fn pdu_specific(&self) -> u8 {
        self.as_u32() as u8
    }
    pub fn is_pdu1(&self) -> bool {
        self.pdu_format() < 240
    }
    pub fn is_pdu2(&self) -> bool {
        self.pdu_format() >= 240
    }
}

impl Default for ParameterGroupNumber {
    fn default() -> Self {
        Self::Any
    }
}

impl From<u16> for ParameterGroupNumber {
    fn from(val: u16) -> Self {
        (val as u32).into()
    }
}

impl From<&[u8]> for ParameterGroupNumber {
    fn from(val: &[u8]) -> Self {
        if val.len() >= 3 {
            u32::from_le_bytes([val[0], val[1], val[2], 0]).into()
        } else {
            ParameterGroupNumber::Any
        }
    }
}

impl From<u32> for ParameterGroupNumber {
    fn from(val: u32) -> Self {
        match val {
            0x00AC00 => Self::AgriculturalGuidanceMachineInfo,
            0x00AD00 => Self::AgriculturalGuidanceSystemCommand,
            0x00C300 => Self::DiagnosticMessage22,
            0x00C700 => Self::ExtendedTransportProtocolDataTransfer,
            0x00C800 => Self::ExtendedTransportProtocolConnectionManagement,
            0x00CB00 => Self::ProcessData,
            0x00CC00 => Self::RequestForRepetitionRate,
            0x00DF00 => Self::DiagnosticMessage13,
            0x00E600 => Self::VirtualTerminalToECU,
            0x00E700 => Self::ECUtoVirtualTerminal,
            0x00E800 => Self::Acknowledge,
            0x00EA00 => Self::ParameterGroupNumberRequest,
            0x00EB00 => Self::TransportProtocolData,
            0x00EC00 => Self::TransportProtocolCommand,
            0x00EE00 => Self::AddressClaim,
            0x00EF00 => Self::ProprietaryA,
            0x00F022 => Self::MachineSelectedSpeed,
            0x00FC8D => Self::ProductIdentification,
            0x00FC8E => Self::ControlFunctionFunctionalities,
            0x00FD32 => Self::DiagnosticProtocolIdentification,
            0x00FD43 => Self::MachineSelectedSpeedCommand,
            0x00FE0D => Self::WorkingSetMaster,
            0x00FE0F => Self::LanguageCommand,
            0x00FE47 => Self::MaintainPower,
            0x00FE48 => Self::WheelBasedSpeedAndDistance,
            0x00FE49 => Self::GroundBasedSpeedAndDistance,
            0x00FDC5 => Self::ECUIdentificationInformation,
            0x00FECA => Self::DiagnosticMessage1,
            0x00FECB => Self::DiagnosticMessage2,
            0x00FECC => Self::DiagnosticMessage3,
            0x00FED3 => Self::DiagnosticMessage11,
            0x00FED8 => Self::CommandedAddress,
            0x00FEDA => Self::SoftwareIdentification,
            0x00FD02 => Self::AllImplementsStopOperationsSwitchState,
            _ => Self::Any
        }
    }
}

impl From<ParameterGroupNumber> for [u8; 3] {
    fn from(val: ParameterGroupNumber) -> Self {
        val.as_bytes()
    }
}