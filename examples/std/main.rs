// #include "isobus/hardware_integration/available_can_drivers.hpp"
// #include "isobus/hardware_integration/can_hardware_interface.hpp"
// #include "isobus/isobus/can_general_parameter_group_numbers.hpp"
// #include "isobus/isobus/can_network_manager.hpp"
// #include "isobus/isobus/can_partnered_control_function.hpp"
// #include "isobus/isobus/can_stack_logger.hpp"
// #include "isobus/isobus/isobus_virtual_terminal_client.hpp"
// #include "isobus/utility/iop_file_interface.hpp"

// #include "console_logger.cpp"
// #include "objectPoolObjects.h"

// #include <atomic>
// #include <csignal>
// #include <functional>
// #include <iostream>
// #include <memory>

// //! It is discouraged to use global variables, but it is done here for simplicity.
// static std::shared_ptr<isobus::VirtualTerminalClient> TestVirtualTerminalClient = nullptr;
// static std::atomic_bool running = { true };

// void signal_handler(int)
// {
// 	running = false;
// }

// // This callback will provide us with event driven notifications of button presses from the stack
// void handleVTKeyEvents(const isobus::VirtualTerminalClient::VTKeyEvent &event)
// {
// 	static std::uint32_t exampleNumberOutput = 214748364; // In the object pool the output number has an offset of -214748364 so we use this to represent 0.

// 	switch (event.keyEvent)
// 	{
// 		case isobus::VirtualTerminalClient::KeyActivationCode::ButtonUnlatchedOrReleased:
// 		{
// 			switch (event.objectID)
// 			{
// 				case Plus_Button:
// 				{
// 					TestVirtualTerminalClient->send_change_numeric_value(ButtonExampleNumber_VarNum, ++exampleNumberOutput);
// 				}
// 				break;

// 				case Minus_Button:
// 				{
// 					TestVirtualTerminalClient->send_change_numeric_value(ButtonExampleNumber_VarNum, --exampleNumberOutput);
// 				}
// 				break;

// 				case alarm_SoftKey:
// 				{
// 					TestVirtualTerminalClient->send_change_active_mask(example_WorkingSet, example_AlarmMask);
// 				}
// 				break;

// 				case acknowledgeAlarm_SoftKey:
// 				{
// 					TestVirtualTerminalClient->send_change_active_mask(example_WorkingSet, mainRunscreen_DataMask);
// 				}
// 				break;

// 				default:
// 					break;
// 			}
// 		}
// 		break;

// 		default:
// 			break;
// 	}
// }

// int main()
// {
// 	std::signal(SIGINT, signal_handler);

// 	// Automatically load the desired CAN driver based on the available drivers
// 	std::shared_ptr<isobus::CANHardwarePlugin> canDriver = nullptr;
// #if defined(ISOBUS_SOCKETCAN_AVAILABLE)
// 	canDriver = std::make_shared<isobus::SocketCANInterface>("can0");
// #elif defined(ISOBUS_WINDOWSPCANBASIC_AVAILABLE)
// 	canDriver = std::make_shared<isobus::PCANBasicWindowsPlugin>(PCAN_USBBUS1);
// #elif defined(ISOBUS_WINDOWSINNOMAKERUSB2CAN_AVAILABLE)
// 	canDriver = std::make_shared<isobus::InnoMakerUSB2CANWindowsPlugin>(0); // CAN0
// #elif defined(ISOBUS_MACCANPCAN_AVAILABLE)
// 	canDriver = std::make_shared<isobus::MacCANPCANPlugin>(PCAN_USBBUS1);
// #endif
// 	if (nullptr == canDriver)
// 	{
// 		std::cout << "Unable to find a CAN driver. Please make sure you have one of the above drivers installed with the library." << std::endl;
// 		std::cout << "If you want to use a different driver, please add it to the list above." << std::endl;
// 		return -1;
// 	}

// 	isobus::CANStackLogger::set_can_stack_logger_sink(&logger);
// 	isobus::CANStackLogger::set_log_level(isobus::CANStackLogger::LoggingLevel::Info); // Change this to Debug to see more information
// 	isobus::CANHardwareInterface::set_number_of_can_channels(1);
// 	isobus::CANHardwareInterface::assign_can_channel_frame_handler(0, canDriver);

// 	if ((!isobus::CANHardwareInterface::start()) || (!canDriver->get_is_valid()))
// 	{
// 		std::cout << "Failed to start hardware interface. The CAN driver might be invalid." << std::endl;
// 		return -2;
// 	}

// 	std::this_thread::sleep_for(std::chrono::milliseconds(250));

// 	isobus::NAME TestDeviceNAME(0);

// 	//! Make sure you change these for your device!!!!
// 	//! This is an example device that is using a manufacturer code that is currently unused at time of writing
// 	TestDeviceNAME.set_arbitrary_address_capable(true);
// 	TestDeviceNAME.set_industry_group(1);
// 	TestDeviceNAME.set_device_class(0);
// 	TestDeviceNAME.set_function_code(static_cast<std::uint8_t>(isobus::NAME::Function::SteeringControl));
// 	TestDeviceNAME.set_identity_number(2);
// 	TestDeviceNAME.set_ecu_instance(0);
// 	TestDeviceNAME.set_function_instance(0);
// 	TestDeviceNAME.set_device_class_instance(0);
// 	TestDeviceNAME.set_manufacturer_code(64);

// 	std::vector<std::uint8_t> testPool = isobus::IOPFileInterface::read_iop_file("VT3TestPool.iop");

// 	if (testPool.empty())
// 	{
// 		std::cout << "Failed to load object pool from VT3TestPool.iop" << std::endl;
// 		return -3;
// 	}
// 	std::cout << "Loaded object pool from VT3TestPool.iop" << std::endl;

// 	// Generate a unique version string for this object pool (this is optional, and is entirely application specific behavior)
// 	std::string objectPoolHash = isobus::IOPFileInterface::hash_object_pool_to_version(testPool);

// 	const isobus::NAMEFilter filterVirtualTerminal(isobus::NAME::NAMEParameters::FunctionCode, static_cast<std::uint8_t>(isobus::NAME::Function::VirtualTerminal));
// 	const std::vector<isobus::NAMEFilter> vtNameFilters = { filterVirtualTerminal };
// 	auto TestInternalECU = std::make_shared<isobus::InternalControlFunction>(TestDeviceNAME, 0x1C, 0);
// 	auto TestPartnerVT = std::make_shared<isobus::PartneredControlFunction>(0, vtNameFilters);

// 	TestVirtualTerminalClient = std::make_shared<isobus::VirtualTerminalClient>(TestPartnerVT, TestInternalECU);
// 	TestVirtualTerminalClient->set_object_pool(0, isobus::VirtualTerminalClient::VTVersion::Version3, testPool.data(), testPool.size(), objectPoolHash);
// 	auto softKeyListener = TestVirtualTerminalClient->add_vt_soft_key_event_listener(handleVTKeyEvents);
// 	auto buttonListener = TestVirtualTerminalClient->add_vt_button_event_listener(handleVTKeyEvents);
// 	TestVirtualTerminalClient->initialize(true);

// 	while (running)
// 	{
// 		// CAN stack runs in other threads. Do nothing forever.
// 		std::this_thread::sleep_for(std::chrono::milliseconds(1000));
// 	}

// 	TestVirtualTerminalClient->terminate();
// 	isobus::CANHardwareInterface::stop();
// 	return 0;
// }

use std::{thread, time::Duration, sync::mpsc::*, fs::File};

use agisostack::{Address, hardware_integration::*, name::*, control_function::*, CanFrame, VirtualTerminalClient};
use log::warn;

fn main() {
    // Setup the logging interface.
    env_logger::builder()
        .filter_level(log::LevelFilter::Debug)
        .format_timestamp_nanos()
        .init();

    
    let (tx, rx): (Sender<CanFrame>, Receiver<CanFrame>) = mpsc::channel();

    // Start the canbus thread.
    thread::spawn(move |rx| canbus_task(rx));

    // Start the isobus thread.
    thread::spawn(move |tx| isobus_task(tx));

    // For example; Do all of our GUI in the main thread.
    loop {
        log::info!("Time: {:?}", agisostack::hardware_integration::TimeDriver::time_elapsed());
        thread::sleep(Duration::from_secs(1));
    }
}

fn canbus_task(tx: Sender<CanFrame>) {
    let mut can_driver = CanDriver::new();
    can_driver.open();

    while can_driver.is_valid() {
        if let Some(frame) = can_driver.read() {
            let _ = tx.send(frame);
        }

        thread::yield_now();
    }

    error!("Canbus task exited! Driver no longer valid.");
}

fn isobus_task(rx: Receiver<CanFrame>) {
    

//  TODO: Setup global frame handler
// 	isobus::CANHardwareInterface::set_number_of_can_channels(1);
// 	isobus::CANHardwareInterface::assign_can_channel_frame_handler(0, canDriver);

// 	if ((!isobus::CANHardwareInterface::start()) || (!canDriver->get_is_valid()))
// 	{
// 		std::cout << "Failed to start hardware interface. The CAN driver might be invalid." << std::endl;
// 		return -2;
// 	}

    thread::sleep(Duration::from_millis(250));

    // Create a new name builder.
	let mut name_builder = Name::builder();

	//NOTE: Make sure you change these for your device!
	//NOTE: This is an example device that is using a manufacturer code that is currently unused at time of writing.
	name_builder.arbitrary_address_capable(true)
	    .industry_group(IndustryGroup::AgriculturalAndForestryEquipment)
	    .device_class(DeviceClass::Fertilizers)
	    .function_code(FunctionCode::MachineControl)
	    .identity_number(2)
	    .ecu_instance(0)
	    .function_instance(0)
	    .device_class_instance(0)
	    .manufacturer_code(64);

    let test_device_name = name_builder.build();
    let test_device_address = Address(0x1C);

    // Read iop file.
    let test_pool: Vec<u8> = Vec::new().extend(std::fs::read("VT3TestPool.iop"));

    if test_pool.is_empty() {
        log::error!("Failed to load object pool from VT3TestPool.iop")
    } else {
        log::info!("Loaded object pool from VT3TestPool.iop")
    }

    let filter_virtual_terminal = NameFilter::FunctionCode(FunctionCode::VirtualTerminal);
    let vt_name_filters = vec![filter_virtual_terminal];
    
    let test_internal_ecu = ControlFunction::new_internal_control_function(test_device_name, test_device_address, 0);
    let test_partner_vt = ControlFunction::new_partnered_control_function(0, vt_name_filters);

    let test_virtual_terminal_client = VirtualTerminalClient::new(test_partner_vt, test_internal_ecu);
    test_virtual_terminal_client.set_object_pool(0, VirtualTerminalClient::VTVersion::Version3, test_pool);
    let soft_key_listner = test_virtual_terminal_client.add_vt_soft_key_event_listener(handle_vt_key_events);
    let button_listner = test_virtual_terminal_client.add_vt_button_event_listener(handle_vt_key_events);
    test_virtual_terminal_client.initialize(true);

    loop {
        // Receive a CanFrame without blocking
        if let Ok(e) = rx.try_recv() {
            log::info!("{} reveived!", e);
        }

        // Update the VirtualTerminalClient
        test_virtual_terminal_client.update();
    }
}

// This callback will provide us with event driven notifications of button presses from the stack
fn handle_vt_key_events(event: &VirtualTerminalClient::VTKeyEvent) {

}
// void handleVTKeyEvents(const isobus::VirtualTerminalClient::VTKeyEvent &event)
// {
// 	static std::uint32_t exampleNumberOutput = 214748364; // In the object pool the output number has an offset of -214748364 so we use this to represent 0.

// 	switch (event.keyEvent)
// 	{
// 		case isobus::VirtualTerminalClient::KeyActivationCode::ButtonUnlatchedOrReleased:
// 		{
// 			switch (event.objectID)
// 			{
// 				case Plus_Button:
// 				{
// 					TestVirtualTerminalClient->send_change_numeric_value(ButtonExampleNumber_VarNum, ++exampleNumberOutput);
// 				}
// 				break;

// 				case Minus_Button:
// 				{
// 					TestVirtualTerminalClient->send_change_numeric_value(ButtonExampleNumber_VarNum, --exampleNumberOutput);
// 				}
// 				break;

// 				case alarm_SoftKey:
// 				{
// 					TestVirtualTerminalClient->send_change_active_mask(example_WorkingSet, example_AlarmMask);
// 				}
// 				break;

// 				case acknowledgeAlarm_SoftKey:
// 				{
// 					TestVirtualTerminalClient->send_change_active_mask(example_WorkingSet, mainRunscreen_DataMask);
// 				}
// 				break;

// 				default:
// 					break;
// 			}
// 		}
// 		break;

// 		default:
// 			break;
// 	}
// }
