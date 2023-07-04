


/// The internal state machine state of the VT client, mostly just public so tests can access it
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
enum State {
	Disconnected, //< VT is not connected, and is not trying to connect yet
	WaitForPartnerVTStatusMessage, //< VT client is initialized, waiting for a VT server to come online
	SendWorkingSetMasterMessage, //< Client is sending the working state master message
	ReadyForObjectPool, //< Client needs an object pool before connection can continue
	SendGetMemory, //< Client is sending the "get memory" message to see if VT has enough memory available
	WaitForGetMemoryResponse, //< Client is waiting for a response to the "get memory" message
	SendGetNumberSoftkeys, //< Client is sending the "get number of soft keys" message
	WaitForGetNumberSoftKeysResponse, //< Client is waiting for a response to the "get number of soft keys" message
	SendGetTextFontData, //< Client is sending the "get text font data" message
	WaitForGetTextFontDataResponse, //< Client is waiting for a response to the "get text font data" message
	SendGetHardware, //< Client is sending the "get hardware" message
	WaitForGetHardwareResponse, //< Client is waiting for a response to the "get hardware" message
	SendGetVersions, //< If a version label was specified, check to see if the VT has that version already
	WaitForGetVersionsResponse, //< Client is waiting for a response to the "get versions" message
	SendStoreVersion, //< Sending the store version command
	WaitForStoreVersionResponse, //< Client is waiting for a response to the store version command
	SendLoadVersion, //< Sending the load version command
	WaitForLoadVersionResponse, //< Client is waiting for the VT to respond to the "Load Version" command
	UploadObjectPool, //< Client is uploading the object pool
	SendEndOfObjectPool, //< Client is sending the end of object pool message
	WaitForEndOfObjectPoolResponse, //< Client is waiting for the end of object pool response message
	Connected, //< Client is connected to the VT server and the application layer is in control
	Failed //< Client could not connect to the VT due to an error
}

impl Default for State {
    fn default() -> Self {
        Self::Disconnected
    }
}


pub struct VirtualTerminalClientStateMachine {
    
}

impl VirtualTerminalClientStateMachine {
    pub fn new() -> Self {
        Self {  }
    }
}