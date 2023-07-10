mod events;
pub use events::*;

mod virtual_terminal_client_state_machine;
use virtual_terminal_client_state_machine::VirtualTerminalClientStateMachine;

mod virtual_terminal_client;
pub use virtual_terminal_client::*;

/// Enumerates the states that can be sent with a hide/show object command
#[repr(u8)]
#[derive(Debug, PartialEq)]
pub enum HideShowObjectCommand {
    HideObject = 0, //< Hides the object
    ShowObject = 1, //< Shows an object
}

/// Enumerates the states that can be sent with an enable/disable object command
#[repr(u8)]
#[derive(Debug, PartialEq)]
pub enum EnableDisableObjectCommand {
    DisableObject = 0, //< Disables a compatible object
    EnableObject = 1,  //< Enables a compatible object
}

/// Enumerates the states that can be sent with a select input object options command
#[repr(u8)]
#[derive(Debug, PartialEq)]
pub enum SelectInputObjectOptions {
    ActivateObjectForDataInput = 0x00, //< Activates an object for data input
    SetFocusToObject = 0xFF, //< Focuses the object (usually this draws a temporary box around it)
}

/// The different VT versions that a client or server might support
#[repr(u8)]
#[derive(Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum VTVersion {
    Version2OrOlder = 2,        //< Client or server supports VT version 2 or lower
    Version3 = 3,               //< Client or server supports all of VT version 3
    Version4 = 4,               //< Client or server supports all of VT version 4
    Version5 = 5,               //< Client or server supports all of VT version 5
    Version6 = 6,               //< Client or server supports all of VT version 6
    ReservedOrUnknown = 0xFF,   //< Reserved value, not to be used
}

impl Default for VTVersion {
    fn default() -> Self {
        Self::ReservedOrUnknown
    }
}

impl From<u8> for VTVersion {
    fn from(value: u8) -> Self {
        match value {
            0..=2 => Self::Version2OrOlder,
            3 => Self::Version3,
            4 => Self::Version4,
            5 => Self::Version5,
            6 => Self::Version6,
            _ => Self::ReservedOrUnknown,
        }
    }
}

/// Enumerates the different line directions that can be used when changing an endpoint of an object
#[repr(u8)]
#[derive(Debug, PartialEq)]
pub enum LineDirection {
    TopLeftToBottomRightOfEnclosingVirtualRectangle = 0, //< Draws the line from top left to bottom right of the enclosing virtual rectangle
    BottomLeftToTopRightOfEnclosingVirtualRectangle = 1, //< Draws the line from bottom left to top right of the enclosing virtual rectangle
}

/// Enumerates the different font sizes
#[repr(u8)]
#[derive(Debug, PartialEq)]
pub enum FontSize {
    Size6x8 = 0,      //< 6x8 Font size
    Size8x8 = 1,      //< 8x8 Font size
    Size8x12 = 2,     //< 8x12 Font size
    Size12x16 = 3,    //< 12x16 Font size
    Size16x16 = 4,    //< 16x16 Font size
    Size16x24 = 5,    //< 16x24 Font size
    Size24x32 = 6,    //< 24x32 Font size
    Size32x32 = 7,    //< 32x32 Font size
    Size32x48 = 8,    //< 32x48 Font size
    Size48x64 = 9,    //< 48x64 Font size
    Size64x64 = 10,   //< 64x64 Font size
    Size64x96 = 11,   //< 64x96 Font size
    Size96x128 = 12,  //< 96x128 Font size
    Size128x128 = 13, //< 128x128 Font size
    Size128x192 = 14, //< 128x192 Font size
}

/// Enumerates the font style options that can be encoded in a font style bitfield
#[repr(u8)]
#[derive(Debug, PartialEq)]
pub enum FontStyleBits {
    Bold = 0,                      //< Bold font style
    CrossedOut = 1,                //< Crossed-out font style (strikethrough)
    Underlined = 2,                //< Underlined font style
    Italic = 3,                    //< Italic font style
    Inverted = 4,                  //< Inverted font style (upside down)
    Flashing = 5,                  //< Flashing font style
    FlashingHidden = 6,            //< Flashing between hidden and shown font style
    ProportionalFontRendering = 7, //< Enables proportional font rendering if supported by the server
}

/// Enumerates the different font types
#[repr(u8)]
#[derive(Debug, PartialEq)]
pub enum FontType {
    ISO8859_1 = 0,          //< ISO Latin 1
    ISO8859_15 = 1,         //< ISO Latin 9
    ISO8859_2 = 2,          //< ISO Latin 2
    Reserved1 = 3,          //< Reserved
    ISO8859_4 = 4,          //< ISO Latin 4
    ISO8859_5 = 5,          //< Cyrillic
    Reserved2 = 6,          //< Reserved
    ISO8859_7 = 7,          //< Greek
    ReservedEnd = 239,      //< Reserved from ISO8859_7 to this value
    ProprietaryBegin = 240, //< The beginning of the proprietary range
    ProprietaryEnd = 255,   //< The end of the proprietary region
}

/// Enumerates the different fill types for an object
#[repr(u8)]
#[derive(Debug, PartialEq)]
pub enum FillType {
    NoFill = 0,                                       //< No fill will be applied
    FillWithLineColour = 1, //< Fill with the colour of the outline of the shape
    FillWithSpecifiedColourInFillColourAttribute = 2, //< Fill with the colour specified by a fill attribute
    FillWithPatternGivenByFillPatternAttribute = 3, //< Fill with a pattern provided by a fill pattern attribute
}

/// The types of object pool masks
#[repr(u8)]
#[derive(Debug, PartialEq)]
pub enum MaskType {
    DataMask = 1,  //< A data mask, used in normal circumstances
    AlarmMask = 2, //< An alarm mask, which has different metadata related to popping up alarms, like priority
}

/// The allowable priorities of an alarm mask
#[repr(u8)]
#[derive(Debug, PartialEq)]
pub enum AlarmMaskPriority {
    High = 0,   //< Overrides lower priority alarm masks
    Medium = 1, //< Overrides low priority alarm masks
    Low = 2,    //< Overrides data masks
}

/// Denotes the lock/unlock state of a mask. Used to freeze/unfreeze rendering of a mask.
#[repr(u8)]
#[derive(Debug, PartialEq)]
pub enum MaskLockState {
    UnlockMask = 0, //< Renders the mask normally
    LockMask = 1, //< Locks the mask so rendering of it is not updated until it is unlocked or a timeout occurs
}

/// The different key activation codes that a button press can generate
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub enum KeyActivationCode {
    ButtonUnlatchedOrReleased = 0, //< Button is released
    ButtonPressedOrLatched = 1,    //< Button is pressed
    ButtonStillHeld = 2,           //< Button is being held down (sent cyclically)
    ButtonPressAborted = 3, //< Press was aborted (user navigated away from the button and did not release it)
}

impl TryFrom<u8> for KeyActivationCode {
    type Error = ();

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::ButtonUnlatchedOrReleased),
            1 => Ok(Self::ButtonPressedOrLatched),
            2 => Ok(Self::ButtonStillHeld),
            3 => Ok(Self::ButtonPressAborted),
            _ => Err(()),
        }
    }
}

/// Enumerates the errors that can be present in an ESC message
#[repr(u8)]
#[derive(Debug, PartialEq)]
pub enum ESCMessageErrorCode {
    NoError = 0,          //< No error occurred
    NoInputFieldOpen = 1, //< No input field is open
    OtherError = 5,       //< Error is not one of the above
}

/// Enumerates the different events that can be associated with a macro
#[repr(u8)]
#[derive(Debug, PartialEq)]
pub enum MacroEventID {
    Reserved = 0,                    //< Reserved
    OnActivate = 1,                  //< Event on activation of an object (such as for data input)
    OnDeactivate = 2,                //< Event on deactivation of an object
    OnShow = 3,                      //< Event on an object being shown
    OnHide = 4,                      //< Event on an object being hidden
    OnEnable = 5,                    //< Event on enable of an object
    OnDisable = 6,                   //< Event on disabling an object
    OnChangeActiveMask = 7,          //< Event on changing the active mask
    OnChangeSoftKeyMask = 8,         //< Event on change of the soft key mask
    OnChangeAttribute = 9,           //< Event on change of an attribute value
    OnChangeBackgroundColour = 10,   //< Event on change of a background colour
    OnChangeFontAttributes = 11,     //< Event on change of a font attribute
    OnChangeLineAttributes = 12,     //< Event on change of a line attribute
    OnChangeFillAttributes = 13,     //< Event on change of a fill attribute
    OnChangeChildLocation = 14,      //< Event on change of a child objects location
    OnChangeSize = 15,               //< Event on change of an object size
    OnChangeValue = 16, //< Event on change of an object value (like via `change numeric value`)
    OnChangePriority = 17, //< Event on change of a mask's priority
    OnChangeEndPoint = 18, //< Event on change of an object endpoint
    OnInputFieldSelection = 19, //< Event when an input field is selected
    OnInputFieldDeselection = 20, //< Event on deselection of an input field
    OnESC = 21,         //< Event on ESC (escape)
    OnEntryOfValue = 22, //< Event on entry of a value
    OnEntryOfNewValue = 23, //< Event on entry of a *new* value
    OnKeyPress = 24,    //< Event on the press of a key
    OnKeyRelease = 25,  //< Event on the release of a key
    OnChangeChildPosition = 26, //< Event on changing a child object's position
    OnPointingEventPress = 27, //< Event on a pointing event press
    OnPointingEventRelease = 28, //< Event on a pointing event release
    ReservedBegin = 29, //< Beginning of the reserved range
    ReservedEnd = 254,  //< End of the reserved range
    UseExtendedMacroReference = 255, //< Use extended macro reference
}

/// Enumerates the various VT server graphics modes
#[repr(u8)]
#[derive(Debug, PartialEq)]
pub enum GraphicMode {
    Monochrome = 0,               //< Monochromatic graphics mode (1 bit)
    SixteenColour = 1,            //< 16 Colour mode (4 bit)
    TwoHundredFiftySixColour = 2, //< 256 Colour mode (8 bit)
}

/// Enumerates the various auxiliary input function types
#[repr(u8)]
#[derive(Debug, PartialEq)]
pub enum AuxiliaryTypeTwoFunctionType {
    BooleanLatching = 0, //< Two-position switch (maintains position) (Single Pole, Double Throw)
    AnalogueLatching = 1, //< Two-way analogue (Maintains position setting)
    BooleanMomentary = 2, //< Two-position switch (returns to off) (Momentary Single Pole, Single Throw)
    AnalogueMomentaryTwoWay = 3, //< Two-way analogue (returns to centre position - 50%)
    AnalogueMomentaryOneWay = 4, //< One-way analogue (returns to 0%)
    DualBooleanLatching = 5, //< Three-position switch (maintains position) (Single Pole, Three Positions, Centre Off)
    DualBooleanMomentary = 6, //< Three-position switch (returns to off/centre position) (Momentary Single Pole, Three Positions, Centre Off)
    DualBooleanLatchingUpOnly = 7, //< Three-position switch (maintains position only in up position) (Single Pole, Three Positions, Centre Off)
    DualBooleanLatchingDownpOnly = 8, //< Three-position switch (maintains position only in down position) (Momentary Single Pole, Three Positions, Centre Off)
    AnalogueMomentaryBooleanLatching = 9, //< two-way analogue (returns to centre position) with latching Boolean at 0% and 100% positions
    AnalogueLatchingBooleanLatching = 10, //< two-way analogue (maintains position setting) with momentary Boolean at 0% and 100% positions
    QuadratureBooleanMomentary = 11, //< Two Quadrature mounted Three-position switches (returns to centre position) (Momentary Single Pole, Three Position Single Throw, Centre Off)
    QuadratureAnalogueLatching = 12, //< Two Quadrature mounted Two-way analogue (maintains position)
    QuadratureAnalogueMomentary = 13, //< Two Quadrature mounted Two-way analogue (returns to centre position - 50%)
    BidirectionalEncoder = 14, //< Count increases when turning in the encoders "increase" direction, and decreases when turning in the opposite direction
    Reserved = 30,             //< 15-30 Reserved
    ReservedRemoveAssignment = 31, //< Used for Remove assignment command
}

/// Enumerates the multiplexor byte values for VT commands
#[repr(u8)]
#[derive(Debug, PartialEq)]
enum VTFunction {
    SoftKeyActivationMessage = 0x00,
    ButtonActivationMessage = 0x01,
    PointingEventMessage = 0x02,
    VTSelectInputObjectMessage = 0x03,
    VTESCMessage = 0x04,
    VTChangeNumericValueMessage = 0x05,
    VTChangeActiveMaskMessage = 0x06,
    VTChangeSoftKeyMaskMessage = 0x07,
    VTChangeStringValueMessage = 0x08,
    VTOnUserLayoutHideShowMessage = 0x09,
    VTControlAudioSignalTerminationMessage = 0x0A,
    ObjectPoolTransferMessage = 0x11,
    EndOfObjectPoolMessage = 0x12,
    AuxiliaryAssignmentTypeOneCommand = 0x20,
    AuxiliaryInputTypeOneStatus = 0x21,
    PreferredAssignmentCommand = 0x22,
    AuxiliaryInputTypeTwoMaintenanceMessage = 0x23,
    AuxiliaryAssignmentTypeTwoCommand = 0x24,
    AuxiliaryInputStatusTypeTwoEnableCommand = 0x25,
    AuxiliaryInputTypeTwoStatusMessage = 0x26,
    AuxiliaryCapabilitiesRequest = 0x27,
    SelectActiveWorkingSet = 0x90,
    ESCCommand = 0x92,
    HideShowObjectCommand = 0xA0,
    EnableDisableObjectCommand = 0xA1,
    SelectInputObjectCommand = 0xA2,
    ControlAudioSignalCommand = 0xA3,
    SetAudioVolumeCommand = 0xA4,
    ChangeChildLocationCommand = 0xA5,
    ChangeSizeCommand = 0xA6,
    ChangeBackgroundColourCommand = 0xA7,
    ChangeNumericValueCommand = 0xA8,
    ChangeEndPointCommand = 0xA9,
    ChangeFontAttributesCommand = 0xAA,
    ChangeLineAttributesCommand = 0xAB,
    ChangeFillAttributesCommand = 0xAC,
    ChangeActiveMaskCommand = 0xAD,
    ChangeSoftKeyMaskCommand = 0xAE,
    ChangeAttributeCommand = 0xAF,
    ChangePriorityCommand = 0xB0,
    ChangeListItemCommand = 0xB1,
    DeleteObjectPoolCommand = 0xB2,
    ChangeStringValueCommand = 0xB3,
    ChangeChildPositionCommand = 0xB4,
    ChangeObjectLabelCommand = 0xB5,
    ChangePolygonPointCommand = 0xB6,
    ChangePolygonScaleCommand = 0xB7,
    GraphicsContextCommand = 0xB8,
    GetAttributeValueMessage = 0xB9,
    SelectColourMapCommand = 0xBA,
    IdentifyVTMessage = 0xBB,
    ExecuteExtendedMacroCommand = 0xBC,
    LockUnlockMaskCommand = 0xBD,
    ExecuteMacroCommand = 0xBE,
    GetMemoryMessage = 0xC0,
    GetSupportedWidecharsMessage = 0xC1,
    GetNumberOfSoftKeysMessage = 0xC2,
    GetTextFontDataMessage = 0xC3,
    GetWindowMaskDataMessage = 0xC4,
    GetSupportedObjectsMessage = 0xC5,
    GetHardwareMessage = 0xC7,
    StoreVersionCommand = 0xD0,
    LoadVersionCommand = 0xD1,
    DeleteVersionCommand = 0xD2,
    ExtendedGetVersionsMessage = 0xD3,
    ExtendedStoreVersionCommand = 0xD4,
    ExtendedLoadVersionCommand = 0xD5,
    ExtendedDeleteVersionCommand = 0xD6,
    GetVersionsMessage = 0xDF,
    GetVersionsResponse = 0xE0,
    UnsupportedVTFunctionMessage = 0xFD,
    VTStatusMessage = 0xFE,
    WorkingSetMaintenanceMessage = 0xFF,
}

impl TryFrom<u8> for VTFunction {
    type Error = ();

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0x00 => Ok(Self::SoftKeyActivationMessage),
            0x01 => Ok(Self::ButtonActivationMessage),
            0x02 => Ok(Self::PointingEventMessage),
            0x03 => Ok(Self::VTSelectInputObjectMessage),
            0x04 => Ok(Self::VTESCMessage),
            0x05 => Ok(Self::VTChangeNumericValueMessage),
            0x06 => Ok(Self::VTChangeActiveMaskMessage),
            0x07 => Ok(Self::VTChangeSoftKeyMaskMessage),
            0x08 => Ok(Self::VTChangeStringValueMessage),
            0x09 => Ok(Self::VTOnUserLayoutHideShowMessage),
            0x0A => Ok(Self::VTControlAudioSignalTerminationMessage),
            0x11 => Ok(Self::ObjectPoolTransferMessage),
            0x12 => Ok(Self::EndOfObjectPoolMessage),
            0x20 => Ok(Self::AuxiliaryAssignmentTypeOneCommand),
            0x21 => Ok(Self::AuxiliaryInputTypeOneStatus),
            0x22 => Ok(Self::PreferredAssignmentCommand),
            0x23 => Ok(Self::AuxiliaryInputTypeTwoMaintenanceMessage),
            0x24 => Ok(Self::AuxiliaryAssignmentTypeTwoCommand),
            0x25 => Ok(Self::AuxiliaryInputStatusTypeTwoEnableCommand),
            0x26 => Ok(Self::AuxiliaryInputTypeTwoStatusMessage),
            0x27 => Ok(Self::AuxiliaryCapabilitiesRequest),
            0x90 => Ok(Self::SelectActiveWorkingSet),
            0x92 => Ok(Self::ESCCommand),
            0xA0 => Ok(Self::HideShowObjectCommand),
            0xA1 => Ok(Self::EnableDisableObjectCommand),
            0xA2 => Ok(Self::SelectInputObjectCommand),
            0xA3 => Ok(Self::ControlAudioSignalCommand),
            0xA4 => Ok(Self::SetAudioVolumeCommand),
            0xA5 => Ok(Self::ChangeChildLocationCommand),
            0xA6 => Ok(Self::ChangeSizeCommand),
            0xA7 => Ok(Self::ChangeBackgroundColourCommand),
            0xA8 => Ok(Self::ChangeNumericValueCommand),
            0xA9 => Ok(Self::ChangeEndPointCommand),
            0xAA => Ok(Self::ChangeFontAttributesCommand),
            0xAB => Ok(Self::ChangeLineAttributesCommand),
            0xAC => Ok(Self::ChangeFillAttributesCommand),
            0xAD => Ok(Self::ChangeActiveMaskCommand),
            0xAE => Ok(Self::ChangeSoftKeyMaskCommand),
            0xAF => Ok(Self::ChangeAttributeCommand),
            0xB0 => Ok(Self::ChangePriorityCommand),
            0xB1 => Ok(Self::ChangeListItemCommand),
            0xB2 => Ok(Self::DeleteObjectPoolCommand),
            0xB3 => Ok(Self::ChangeStringValueCommand),
            0xB4 => Ok(Self::ChangeChildPositionCommand),
            0xB5 => Ok(Self::ChangeObjectLabelCommand),
            0xB6 => Ok(Self::ChangePolygonPointCommand),
            0xB7 => Ok(Self::ChangePolygonScaleCommand),
            0xB8 => Ok(Self::GraphicsContextCommand),
            0xB9 => Ok(Self::GetAttributeValueMessage),
            0xBA => Ok(Self::SelectColourMapCommand),
            0xBB => Ok(Self::IdentifyVTMessage),
            0xBC => Ok(Self::ExecuteExtendedMacroCommand),
            0xBD => Ok(Self::LockUnlockMaskCommand),
            0xBE => Ok(Self::ExecuteMacroCommand),
            0xC0 => Ok(Self::GetMemoryMessage),
            0xC1 => Ok(Self::GetSupportedWidecharsMessage),
            0xC2 => Ok(Self::GetNumberOfSoftKeysMessage),
            0xC3 => Ok(Self::GetTextFontDataMessage),
            0xC4 => Ok(Self::GetWindowMaskDataMessage),
            0xC5 => Ok(Self::GetSupportedObjectsMessage),
            0xC7 => Ok(Self::GetHardwareMessage),
            0xD0 => Ok(Self::StoreVersionCommand),
            0xD1 => Ok(Self::LoadVersionCommand),
            0xD2 => Ok(Self::DeleteVersionCommand),
            0xD3 => Ok(Self::ExtendedGetVersionsMessage),
            0xD4 => Ok(Self::ExtendedStoreVersionCommand),
            0xD5 => Ok(Self::ExtendedLoadVersionCommand),
            0xD6 => Ok(Self::ExtendedDeleteVersionCommand),
            0xDF => Ok(Self::GetVersionsMessage),
            0xE0 => Ok(Self::GetVersionsResponse),
            0xFD => Ok(Self::UnsupportedVTFunctionMessage),
            0xFE => Ok(Self::VTStatusMessage),
            0xFF => Ok(Self::WorkingSetMaintenanceMessage),
            _ => Err(()),
        }
    }
}
