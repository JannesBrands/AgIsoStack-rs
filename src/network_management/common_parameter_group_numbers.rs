// Copyright 2023 Raven Industries inc.

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum CommonParameterGroupNumbers {
    TractorImplementManagementServerToTimClient = 0x002300,
    TractorImplementManagementClientToTimServer = 0x002400,
    AuthenticationClientToAuthenticationServer = 0x006F00,
    AuthenticationServerToAuthenticationClient = 0x007000,
    NameManagement = 0x009300,
    GuidanceMachineStatus = 0x00AC00,
    GuidanceSystemCommand = 0x00AD00,
    ExtendedTransportProtocolData = 0x00C700,
    ExtendedTransportProtocolCommand = 0x00C800,
    RequestForRepetitionRate = 0x00CC00,
    BinaryDataTransfer = 0x00D700,
    MemoryAccessResponse = 0x00D800,
    MemoryAccessRequest = 0x00D900,
    StopStartBroadcast = 0x00DF00,
    VirtualTerminalToNode = 0x00E600,
    NodeToVirtualTerminal = 0x00E700,
    Acknowledgement = 0x00E800,
    ParameterGroupNumberRequest = 0x00EA00,
    TransportProtocolData = 0x00EB00,
    TransportProtocolCommand = 0x00EC00,
    AddressClaim = 0x00EE00,
    ProprietaryA = 0x00EF00,
    ElectronicEngineController2 = 0x00F003,
    ElectronicEngineController1 = 0x00F004,
    HeartbeatMessage = 0x00F0E4,
    ProductIdentification = 0x00FC8D,
    ControlFunctionFunctionalities = 0x00FC8E,
    DiagnosticProtocol = 0x00FD32,
    IsobusComplianceCertificationMessage = 0x00FD42,
    EcuIdentificationInformation = 0x00FDC5,
    WorkingSetMaster = 0x00FE0D,
    ResponseForRepetitionRate = 0x00FE0E,
    MaintainPower = 0x00FE47,
    WheelBasedSpeedAndDistance = 0x00FE48,
    GroundBasedSpeedAndDistance = 0x00FE49,
    ActiveDiagnosticTroubleCodes = 0x00FECA,
    PreviouslyActiveDiagnosticTroubleCodes = 0x00FECB,
    DiagnosticDataClearResetOfPreviouslyActiveDtcs = 0x00FECC,
    FreezeFrameParameters = 0x00FECD,
    DiagnosticDataClearResetForActiveDtcs = 0x00FED3,
    CommandedAddress = 0x00FED8,
    SoftwareIdentification = 0x00FEDA,
    TimeDate = 0x00FEE6,
    EngineTemperature1 = 0x00FEEE,
    CruiseControlVehicleSpeed1 = 0x00FEF1,
    IntakeExhaustConditions1 = 0x00FEF6,
    NmeaAttitude = 0x01F119,
    NmeaCogSogRapidUpdate = 0x01F802,
    NmeaPositionDeltaHighPrecisionRapidUpdate = 0x01F803,
    NmeaAltitudeDeltaHighPrecisionRapidUpdate = 0x01F804,
    NmeaGnssPositionData = 0x01F805,
    NmeaTimeDate = 0x01F809,
    NmeaGnssDops = 0x01FA03,
    NmeaGnssSatsInView = 0x01FA04,
    NmeaGnssPseudoRangeNoiseStatistics = 0x01FA06,
    NmeaGnssPseudoRangeErrorStatistics = 0x01FA0B,
    AllowAll = 0xFFFFFF,
}