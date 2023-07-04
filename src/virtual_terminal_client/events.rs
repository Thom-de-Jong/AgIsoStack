
use super::*;

/// A enum containing all VT Events
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub enum Event {
    VTKeyEvent(VTKeyEvent),
    VTPointingEvent(VTPointingEvent),
    VTSelectInputObjectEvent(VTSelectInputObjectEvent),
    VTESCMessageEvent(VTESCMessageEvent),
    VTChangeNumericValueEvent(VTChangeNumericValueEvent),
    VTChangeActiveMaskEvent(VTChangeActiveMaskEvent),
    VTChangeSoftKeyMaskEvent(VTChangeSoftKeyMaskEvent),
    VTChangeStringValueEvent(VTChangeStringValueEvent),
    VTUserLayoutHideShowEvent(VTUserLayoutHideShowEvent),
    VTAudioSignalTerminationEvent(VTAudioSignalTerminationEvent),
    AuxiliaryFunctionEvent(AuxiliaryFunctionEvent),
}

/// A struct for storing information of a VT key input event
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct VTKeyEvent {
	// VirtualTerminalClient *parentPointer; ///< A pointer to the parent VT client
	pub object_id: u16, //< The object ID
	pub parent_object_id: u16, //< The parent object ID
	pub key_number: u8, //< The key number
	pub key_event: KeyActivationCode, //< The key event
}

/// @brief A struct for storing information of a VT pointing event
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct VTPointingEvent {
	// VirtualTerminalClient *parentPointer; ///< A pointer to the parent VT client
	pub x_pos: u16, //< The x position
	pub y_pos: u16, //< The y position
	pub parent_object_id: u16, //< The parent object ID
	pub key_event: KeyActivationCode, //< The key event
}

/// @brief A struct for storing information of a VT input object selection event
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct VTSelectInputObjectEvent {
	// VirtualTerminalClient *parentPointer; ///< A pointer to the parent VT client
	// std::uint16_t objectID; ///< The object ID
	// bool objectSelected; ///< Whether the object is selected
	// bool objectOpenForInput; ///< Whether the object is open for input
}

/// @brief A struct for storing information of a VT ESC message event
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct VTESCMessageEvent {
	// VirtualTerminalClient *parentPointer; ///< A pointer to the parent VT client
	// std::uint16_t objectID; ///< The object ID
	// ESCMessageErrorCode errorCode; ///< The error code
}

/// @brief A struct for storing information of a VT change numeric value event
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct VTChangeNumericValueEvent {
	// VirtualTerminalClient *parentPointer; ///< A pointer to the parent VT client
	// std::uint32_t value; ///< The value
	// std::uint16_t objectID; ///< The object ID
}

/// @brief A struct for storing information of a VT change active mask event
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct VTChangeActiveMaskEvent {
	// VirtualTerminalClient *parentPointer; ///< A pointer to the parent VT client
	// std::uint16_t maskObjectID; ///< The mask object ID
	// std::uint16_t errorObjectID; ///< The error object ID
	// std::uint16_t parentObjectID; ///< The parent object ID
	// bool missingObjects; ///< Whether there are missing objects
	// bool maskOrChildHasErrors; ///< Whether the mask or child has errors
	// bool anyOtherError; ///< Whether there are any other errors
	// bool poolDeleted; ///< Whether the pool has been deleted
}

/// @brief A struct for storing information of a VT change soft key mask event
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct VTChangeSoftKeyMaskEvent {
	// VirtualTerminalClient *parentPointer; ///< A pointer to the parent VT client
	// std::uint16_t dataOrAlarmMaskObjectID; ///< The data or alarm mask object ID
	// std::uint16_t softKeyMaskObjectID; ///< The soft key mask object ID
	// bool missingObjects; ///< Whether there are missing objects
	// bool maskOrChildHasErrors; ///< Whether the mask or child has errors
	// bool anyOtherError; ///< Whether there are any other errors
	// bool poolDeleted; ///< Whether the pool has been deleted
}

/// @brief A struct for storing information of a VT change string value event
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct VTChangeStringValueEvent {
// 	std::string value; ///< The value
// 	VirtualTerminalClient *parentPointer; ///< A pointer to the parent VT client
// 	std::uint16_t objectID; ///< The object ID
}

/// @brief A struct for storing information of a VT on user-layout hide/show event
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct VTUserLayoutHideShowEvent {
	// VirtualTerminalClient *parentPointer; ///< A pointer to the parent VT client
	// std::uint16_t objectID; ///< The object ID
	// bool isHidden; ///< Whether the object is hidden
}

/// @brief A struct for storing information of a VT control audio signal termination event
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct VTAudioSignalTerminationEvent {
	// VirtualTerminalClient *parentPointer; ///< A pointer to the parent VT client
	// bool isTerminated; ///< Whether the audio signal is terminated
}

/// @brief A struct for storing information of an auxilary function event
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct AuxiliaryFunctionEvent {
	// AssignedAuxiliaryFunction function; ///< The function
	// VirtualTerminalClient *parentPointer; ///< A pointer to the parent VT client
	// std::uint16_t value1; ///< The first value
	// std::uint16_t value2; ///< The second value
}
