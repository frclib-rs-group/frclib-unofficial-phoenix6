#[derive(Debug, Clone, Copy, PartialEq, Eq, thiserror::Error, num_enum::FromPrimitive)]
#[repr(i32)]
pub enum StatusCode {
    #[error("Diagnostic Server is busy with another command.")]
    TaskIsBusy = -100,
    #[error("InvalidDeviceSpec")]
    InvalidDeviceSpec = -101,
    #[error("Device is not present. Verify the device is connected and powered, and that the CAN bus is terminated.")]
    EcuIsNotPresent = -102,
    #[error("Could not put the device into bootloader mode.")]
    CouldNotEnterBl = -103,
    #[error("Could not confirm the device has entered the bootloader.")]
    CouldNotConfirmBl = -104,
    #[error("Could not erase flash.")]
    CouldNotErase = -105,
    #[error("Could not field upgrade the device.")]
    CouldNotSendFlash = -106,
    #[error("Bootloader could not verify integrity of the flashed application.")]
    CouldNotValidate = -107,
    #[error("Could not run the device firmware application.")]
    CouldNotRunApp = -108,
    #[error("Unable to set ID to this device.")]
    CouldNotReqSetId = -109,
    #[error("Could not verify that the changed ID took effect.")]
    CouldNotConfirmId = -110,
    #[error("Device field upgrade was successful.")]
    FlashWasGood = -111,
    #[error("Device firmware application is too old.")]
    AppTooOld = -112,
    #[error("Unable to set name to this device.")]
    CouldNotReqSetDesc = -113,
    #[error("CompileSzIsWrong")]
    CompileSzIsWrong = -114,
    #[error("Cannot set the ID of a gadgeteer device.")]
    GadgeteerDeviceNoSetId = -115,
    #[error("This diagnostic action is not supported.")]
    InvalidTask = -116,
    #[error("Not Implemented, check latest installer.")]
    NotImplemented = -117,
    #[error("NoDevicesOnBus")]
    NoDevicesOnBus = -118,
    #[error("MoreThanOneFile")]
    MoreThanOneFile = -119,
    #[error("Specified device was not found. Verify the device is connected and powered, and that the CAN bus is terminated.")]
    NodeIsInvalid = -120,
    #[error("InvalidDeviceDescriptor")]
    InvalidDeviceDescriptor = -121,
    #[error("CouldNotSendCanFrame")]
    CouldNotSendCanFrame = -123,
    #[error("NormalModeMsgNotPresent")]
    NormalModeMsgNotPresent = -124,
    #[error("This feature is not supported.")]
    FeatureNotSupported = -125,
    #[error("The diagnostic server is not field upgrading any devices.")]
    NotUpdating = -126,
    #[error("CorruptedPOST")]
    CorruptedPOST = -127,
    #[error("This device did not report any available configs. Verify firmware and diagnostics are up-to-date.")]
    NoConfigs = -128,
    #[error("ConfigFailed")]
    ConfigFailed = -129,
    #[error("Unable to factory default this device.")]
    CouldNotReqFactoryDefault = -130,
    #[error("CustomNameNotSupported")]
    CustomNameNotSupported = -131,
    #[error("The configs read from the device do not match the configs that were written.")]
    ConfigReadWriteMismatch = -132,
    #[error("Could not apply the device configs.")]
    CouldNotReqSetConfigs = -133,
    #[error("InsufficientSz")]
    InsufficientSz = -134,
    #[error("This feature is not supported for this device model.")]
    InvalidModel = -135,
    #[error("CouldNotReqDevInfo")]
    CouldNotReqDevInfo = -140,
    #[error("This device does not support new controls.")]
    NoControls = -141,
    #[error("DeviceIsNull")]
    DeviceIsNull = -142,
    #[error("DeviceDidNotRespondToDiagReq")]
    DeviceDidNotRespondToDiagReq = -143,
    #[error("This feature requires Tuner X.")]
    OnlySupportedInTunerX = -144,
    #[error("Command-line issue with caniv.")]
    CanivCliError = -145,
    #[error("InvalidCrfBadHeader")]
    InvalidCrfBadHeader = -200,
    #[error("InvalidCrfFileSzInvald")]
    InvalidCrfFileSzInvald = -201,
    #[error("Specified CRF is for the wrong product.")]
    InvalidCrfWrongProduct = -202,
    #[error("InvalidCrfNoSects")]
    InvalidCrfNoSects = -203,
    #[error("InvalidCrfBadSectHeader")]
    InvalidCrfBadSectHeader = -204,
    #[error("InvalidCrfBadSectSize")]
    InvalidCrfBadSectSize = -205,
    #[error("Specified CRF file could not be found.")]
    NoCrfFile = -206,
    #[error("CouldNotFindDynamicId")]
    CouldNotFindDynamicId = -300,
    #[error("DidNotGetDhcp")]
    DidNotGetDhcp = -301,
    #[error("DidNotGetFullDhcp")]
    DidNotGetFullDhcp = -302,
    #[error("InvalidLicenseResp")]
    InvalidLicenseResp = -350,
    #[error("InvalidCanivCache")]
    InvalidCanivCache = -351,
    #[error("CannotOpenSerialPort")]
    CannotOpenSerialPort = -500,
    #[error("CannotWriteSerialPort")]
    CannotWriteSerialPort = -501,
    #[error("CannotReadSerialPort")]
    CannotReadSerialPort = -502,
    #[error("CannotSerialToDevice")]
    CannotSerialToDevice = -503,
    #[error("NoSerialControlFrameResp")]
    NoSerialControlFrameResp = -504,
    #[error("CannotOpenUdpPort")]
    CannotOpenUdpPort = -600,
    #[error("CannotWriteUdpPort")]
    CannotWriteUdpPort = -601,
    #[error("CannotReadUdpPort")]
    CannotReadUdpPort = -602,
    #[error("CannotUdpToDevice")]
    CannotUdpToDevice = -603,
    #[error("NoUdpControlFrameResp")]
    NoUdpControlFrameResp = -604,
    #[error("TimeoutIso15Response")]
    TimeoutIso15Response = -605,
    #[error("InvalidJson")]
    InvalidJson = -700,
    #[error("The user application is shutting down.")]
    AppIsTerminating = -800,
    #[error("CAN Message is stale.")]
    CanMessageStale = 1000,
    #[error("Buffer is full, cannot insert more data.")]
    BufferFull = 1006,
    #[error("PulseWidthSensorNotPresent")]
    PulseWidthSensorNotPresent = 1010,
    #[error("General Warning Occurred.")]
    GeneralWarning = 1100,
    #[error("Firm Vers could not be retrieved. Use Phoenix Tuner to check ID and firmware(CRF) version.")]
    FirmVersionCouldNotBeRetrieved = 1103,
    #[error("This feature will be supported in a future update.")]
    FeaturesNotAvailableYet = 1104,
    #[error("The control mode is not valid for this function.")]
    ControlModeNotValid = 1105,
    #[error("This control mode is not supported yet.  A future release will supported this soon.")]
    ControlModeNotSupportedYet = 1106,
    #[error("Motor Controller must have &gt;= 3.2 firmware for motion profile control mode.")]
    MotProfFirmThreshold = 1109,
    #[error("Motor Controller must have &gt;= 3.4 firmware for advanced PID0/PID1 features.")]
    MotProfFirmThreshold2 = 1110,
    #[error("SimDeviceNotFound")]
    SimDeviceNotFound = 1200,
    #[error("SimPhysicsTypeNotSupported")]
    SimPhysicsTypeNotSupported = 1201,
    #[error("SimDeviceAlreadyExists")]
    SimDeviceAlreadyExists = 1202,
    #[error("Could not transmit CAN Frame.")]
    TxFailed = -1001,
    #[error("Incorrect argument passed into function/VI.")]
    InvalidParamValue = -1002,
    #[error("CAN frame not received/too-stale.")]
    RxTimeout = -1003,
    #[error("CAN Transmit timed out.")]
    TxTimeout = -1004,
    #[error("ArbID is incorrect.")]
    UnexpectedArbId = -1005,
    #[error("CanOverflowed")]
    CanOverflowed = -1006,
    #[error("Sensor Not Present.")]
    SensorNotPresent = -1007,
    #[error("Firmware Too Old.  Use Phoenix Tuner to field upgrade your CTRE CAN device firmware(CRF).  Then restart your robot application to clear this error.")]
    FirmwareTooOld = -1008,
    #[error(
        "Control Frame Period could not be changed.  Most likely it is not being transmitted."
    )]
    CouldNotChangePeriod = -1009,
    #[error("BufferFailure")]
    BufferFailure = -1010,
    #[error("Firmware is legacy non-FRC version.  Use Phoenix Tuner to field upgrade your CTRE CAN device firmware(CRF).  Firmware greater than 20.0 required.")]
    FirmwareNonFRC = -1011,
    #[error("General Error Occurred.")]
    #[num_enum(default)]
    GeneralError = -1100,
    #[error("No new response to update signal.")]
    SigNotUpdated = -1200,
    #[error("NotAllPIDValuesUpdated")]
    NotAllPIDValuesUpdated = -1201,
    #[error("GEN_PORT_ERROR")]
    GenPortError = -1300,
    #[error("PORT_MODULE_TYPE_MISMATCH")]
    PostModuleTypeMismatch = -1301,
    #[error("GEN_MODULE_ERROR")]
    GenModuleError = -1400,
    #[error("MODULE_NOT_INIT_SET_ERROR")]
    ModuleNotInitSetError = -1401,
    #[error("MODULE_NOT_INIT_GET_ERROR")]
    ModuleNotInitGetError = -1402,
    #[error("Wheel Radius is too small, cannot get distance traveled.")]
    WheelRadiusTooSmall = -1500,
    #[error("Ticks per revolution is 0, cannot get heading.")]
    TicksPerRevZero = -1501,
    #[error("Distance between wheels is too small, cannot get heading.")]
    DistanceBetweenWheelsTooSmall = -1502,
    #[error("GainsAreNotSet")]
    GainsAreNotSet = -1503,
    #[error("Use RemoteLimitSwitchSource instead of LimitSwitchSource.")]
    WrongRemoteLimitSwitchSource = -1504,
    #[error("Motor Controller Voltage Compensation should not be used with setVoltage().  This causes compensation to happen twice.  Disable Voltage Compensation by calling enableVoltageCompensation(false) in order to use setVoltage().")]
    DoubleVoltageCompensatingWPI = -1505,
    #[error("CANdleAnimSlotOutOfBounds")]
    CANdleAnimSlotOutOfBounds = -1506,
    #[error("IncompatibleMode")]
    IncompatibleMode = -1600,
    #[error("Handle passed into function is incorrect.")]
    InvalidHandle = -1601,
    #[error("Features requires newer firmware version.")]
    FeatureRequiresHigherFirm = -1700,
    #[error("Config factory default features require firmware &gt;=3.10.")]
    ConfigFactoryDefaultRequiresHigherFirm = -1702,
    #[error("Config Motion S Curve Strength features require firmware &gt;=4.16.")]
    ConfigMotionSCurveRequiresHigherFirm = -1703,
    #[error("Talon FX(Falcon 500) Firmware Too Old.  Use Phoenix Tuner to field upgrade your CTRE CAN device firmware(CRF) to &gt;=20.3. Then restart your robot application to clear this error.")]
    TalonFXFirmwarePreVBatDetect = -1704,
    #[error("CANdleAnimationsRequireHigherFirm")]
    CANdleAnimationsRequireHigherFirm = -1705,
    #[error("LibraryCouldNotBeLoaded")]
    LibraryCouldNotBeLoaded = -1800,
    #[error("MissingRoutineInLibrary")]
    MissingRoutineInLibrary = -1801,
    #[error("ResourceNotAvailable")]
    ResourceNotAvailable = -1802,
    #[error("Could not find music file specified, try specifying an absolute path.")]
    MusicFileNotFound = -1900,
    #[error("Music file size is incorrect, could not parse correctly. Ensure you're using Tuner to generate file.")]
    MusicFileWrongSize = -1901,
    #[error("Music file version is too new, update Phoenix to utilize this file.")]
    MusicFileTooNew = -1902,
    #[error("Music file is invalid. Ensure you're using Tuner to generate file.")]
    MusicFileInvalid = -1903,
    #[error("An invalid orchestra action occurred. Ensure a music file is loaded.")]
    InvalidOrchestraAction = -1904,
    #[error("This music file version is too old. Regenerate file using Tuner.")]
    MusicFileTooOld = -1905,
    #[error("Music interrupted due to one of the instruments being commanded a different control mode. Press Play to resume music.")]
    MusicInterrupted = -1906,
    #[error("This device doesn't support MusicTone control mode.")]
    MusicNotSupported = -1907,
    #[error("kInvalidInterface")]
    InvalidInterface = -2000,
    #[error("kInvalidGuid")]
    InvalidGuid = -2001,
    #[error("kInvalidClass")]
    InvalidClass = -2002,
    #[error("kInvalidProtocol")]
    InvalidProtocol = -2003,
    #[error("kInvalidPath")]
    InvalidPath = -2004,
    #[error("kGeneralWinUsbError")]
    GeneralWinUsbError = -2005,
    #[error("kFailedSetup")]
    FailedSetup = -2006,
    #[error("kListenFailed")]
    ListenFailed = -2007,
    #[error("kSendFailed")]
    SendFailed = -2008,
    #[error("kReceiveFailed")]
    ReceiveFailed = -2009,
    #[error("kInvalidRespFormat")]
    InvalidRespFormat = -2010,
    #[error("kWinUsbInitFailed")]
    WinUsbInitFailed = -2011,
    #[error("kWinUsbQueryFailed")]
    WinUsbQueryFailed = -2012,
    #[error("kWinUsbGeneralError")]
    WinUsbGeneralError = -2013,
    #[error("kAccessDenied")]
    AccessDenied = -2014,
    #[error("kFirmwareInvalidResponse")]
    FirmwareInvalidResponse = -2015,
    #[error("This StatusCode has not been initialized. Make sure the StatusCode is getting assigned to the return of a method.")]
    StatusCodeNotInitialized = -10000,
    #[error("WarningNotInitialized")]
    WarningNotInitialized = 10000,
    #[error("The timestamp reported by CANivore is at least 10ms older than the timestamp reported by the system, indicating it's fallen out of sync. This does not impact the data of this message, only the timing.")]
    HwTimestampOutOfSync = 10001,
    #[error("InvalidNetwork")]
    InvalidNetwork = -10001,
    #[error("The CAN bus does not support multi-signal synchronization.")]
    MultiSignalNotSupported = -10002,
    #[error("Could not cast from base value to this particular signal's type")]
    CouldNotCast = -10003,
    #[error("Could not find this value when searching for it")]
    NotFound = -10004,
    #[error("This is not supported")]
    NotSupported = -10005,
    #[error("Could not determine context from this device hash")]
    MissingContext = -10006,
    #[error("Model name in license file does not match model name of selected device.")]
    ModelMismatch = -10007,
    #[error("Serial Number in license file does not match model name of selected device.")]
    SerialMismatch = -10008,
    #[error("Could not find specified file.")]
    NoFile = -10009,
    #[error("License did not successfully download to Device.")]
    LicenseDownloadFailed = -10010,
    #[error("Self Test report does not have any values, is the firmware up to date?")]
    SelfTestIsEmpty = -10011,
    #[error("Failed to lookup signal properties.  This can happen if the fimware is too new and supports signals that older APIs do not support.")]
    SignalLookupFailed = -10012,
    #[error("The current mode of the device is invalid for getting this signal.")]
    InvalidModeToGetSignal = -10013,
    #[error("Device is not licensed. Cannot get any data from it.")]
    UnlicensedDevice = -10014,
    #[error("Size is invalid.")]
    InvalidSize = -10015,
    #[error("InvalidLicenseResponse")]
    InvalidLicenseResponse = -10016,
    #[error("InvalidContext")]
    InvalidContext = -10017,
    #[error("InternalError")]
    InternalError = -10018,
    #[error("kDeviceResponseIncorrect")]
    DeviceResponseIncorrect = -10019,
    #[error("kErrorPollingForDevices")]
    ErrorPollingForDevices = -10020,
    #[error("Device firmware could not be retrieved. Check that the device is running v6 firmware, the device ID is correct, the specified CAN bus is correct, and the device is powered.")]
    CouldNotRetrieveV6Firmware = -10021,
    #[error("Device firmware could not be decoded. Check that the device is running v6 firmware, the device ID is correct, the specified CAN bus is correct, and the device is powered.")]
    CouldNotDecodeDeviceFirmware = -10022,
    #[error("The values specified for master are in valid.  Make sure the Device ID of master are correct.")]
    InvalidIDToFollow = -10023,
    #[error("Using a Pro only feature on an unlicensed device. The device may not behave as expected if it continues to operate while unlicensed.")]
    UsingProFeatureOnUnlicensedDevice = -10024,
    #[error("Firmware Too New.  Use Phoenix Tuner to field upgrade your CTRE CAN device firmware(CRF) to a compatible version.  Then restart your robot application to clear this error.")]
    FirmwareTooNew = -10025,
    #[error("The data frame could not be serialized for transmit.")]
    CouldNotSerialize = -10026,
    #[error("The mechanism is disabled due to a fault in one of the devices.")]
    MechanismFaulted = -10027,
    #[error("Firmware version is not compatible with this version of Phoenix. Make sure your firmware and API major versions match.")]
    FirmwareVersNotCompatible = -10028,
    #[error("Could not find specified directory.")]
    DirectoryMissing = -10029,
    #[error("This API version is too old for the firmware on the device. Either upgrade the API to a newer version or downgrade the device firmware to an older version for correct behavior.")]
    ApiTooOld = -10030,
    #[error(
        "The signal logger is not running. Start the signal logger before writing any signals."
    )]
    LoggerNotRunning = -10031,

    //Custom to rust
    #[error("Could not deserialize string config")]
    CouldNotDeserializeString = -99001,
}
pub trait StatusCodeType {
    fn to_result(self) -> Result<(), StatusCode>;
}
impl StatusCodeType for ::std::os::raw::c_int {
    fn to_result(self) -> Result<(), StatusCode> {
        if self == 0 {
            return Ok(());
        }
        Err(StatusCode::from(self))
    }
}
