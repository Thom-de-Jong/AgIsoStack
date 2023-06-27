

/// PGNs commonly used by the CAN stack.
#[repr(u16)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub enum ParameterGroupNumber {
    Any = 0x0000,
    AgriculturalGuidanceMachineInfo = 0xAC00,
    AgriculturalGuidanceSystemCommand = 0xAD00,
    DiagnosticMessage22 = 0xC300,
    ExtendedTransportProtocolDataTransfer = 0xC700,
    ExtendedTransportProtocolConnectionManagement = 0xC800,
    ProcessData = 0xCB00,
    RequestForRepetitionRate = 0xCC00,
    DiagnosticMessage13 = 0xDF00,
    VirtualTerminalToECU = 0xE600,
    ECUtoVirtualTerminal = 0xE700,
    Acknowledge = 0xE800,
    ParameterGroupNumberRequest = 0xEA00,
    TransportProtocolData = 0xEB00,
    TransportProtocolCommand = 0xEC00,
    AddressClaim = 0xEE00,
    ProprietaryA = 0xEF00,
    MachineSelectedSpeed = 0xF022,
    ProductIdentification = 0xFC8D,
    ControlFunctionFunctionalities = 0xFC8E,
    DiagnosticProtocolIdentification = 0xFD32,
    MachineSelectedSpeedCommand = 0xFD43,
    WorkingSetMaster = 0xFE0D,
    LanguageCommand = 0xFE0F,
    MaintainPower = 0xFE47,
    WheelBasedSpeedAndDistance = 0xFE48,
    GroundBasedSpeedAndDistance = 0xFE49,
    ECUIdentificationInformation = 0xFDC5,
    DiagnosticMessage1 = 0xFECA,
    DiagnosticMessage2 = 0xFECB,
    DiagnosticMessage3 = 0xFECC,
    DiagnosticMessage11 = 0xFED3,
    CommandedAddress = 0xFED8,
    SoftwareIdentification = 0xFEDA,
    AllImplementsStopOperationsSwitchState = 0xFD02
}
