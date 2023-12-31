use std::{sync::mpsc::*, thread, time::Duration};

use agisostack::{
    control_function::*, hardware_integration::*, name::*, virtual_terminal_client::*, Address,
    CanFrame, CanNetworkManager, ObjectPool,
};

const ALARM_SOFT_KEY: u16 = 5000; //0x1388
const ACKNOWLEDGE_ALARM_SOFT_KEY: u16 = 5001; //0x1389
const PLUS_BUTTON: u16 = 6000; //0x1770
const MINUS_BUTTON: u16 = 6001; //0x1771
const BUTTON_EXAMPLE_NUMBER_VAR_NUM: u16 = 21000; //0x5208

fn main() {
    // Setup the logging interface.
    env_logger::builder()
        .filter_level(log::LevelFilter::Debug)
        .format_timestamp_millis()
        .init();

    // Start the Isobus thread.
    thread::spawn(|| isobus_task());

    // For example; Do all of our GUI in the main thread.
    loop {
        thread::sleep(Duration::from_secs(1));
    }
}

fn isobus_task() {
    // // Create the channel used to send CanFrames to the CanDriver.
    // // This is needed because we use a callback to send can frames instead of passing the driver.
    // let (tx, rx): (Sender<CanFrame>, Receiver<CanFrame>) = channel();

    // Create a instance of a CanDriver
    let mut can_driver = CanDriver::new();
    can_driver.open();

    // TODO: Hack to clear the p-can hardware buffer
    thread::sleep(Duration::from_millis(500));
    can_driver.close();
    can_driver.open();

    // Create a new mannager for the CAN network we are connecting to.
    let mut network_manager = CanNetworkManager::new(can_driver);

    // // Bind a callback to the network manager to be called when we send a can frame.
    // // This is the "glue" between the network manager and the CAN Driver.
    // let callback = |f| {
    //     let _ = tx.send(f);
    // };
    // network_manager.send_can_frame_callback(&callback);

    // Create a new name builder.
    let mut name_builder = Name::builder();

    //NOTE: Make sure you change these for your device!
    //NOTE: This is an example device that is using a manufacturer code that is currently unused at time of writing.
    name_builder
        .arbitrary_address_capable(true)
        .industry_group(IndustryGroup::AgriculturalAndForestryEquipment)
        .device_class(DeviceClass::Fertilizers)
        .function_code(FunctionCode::MachineControl)
        .identity_number(2)
        .ecu_instance(0)
        .function_instance(0)
        .device_class_instance(0)
        .manufacturer_code(64);

    // Build the name and specify the address we want to claim.
    let test_device_name = name_builder.build();
    let test_device_address = Address(0x80);

    // Read iop file and check if we read it.
    let mut iop_data: Vec<u8> = Vec::new();
    if let Ok(data) = std::fs::read("VT3TestPool.iop") {
        iop_data.extend(data);
        log::info!("Loaded object pool from VT3TestPool.iop")
    } else {
        log::error!("Failed to load object pool from VT3TestPool.iop")
    }
    let test_pool: ObjectPool = ObjectPool::from_iop(iop_data);

    // Create the Name filer used to find a Virtual Terminal on the network.
    let filter_virtual_terminal = NameFilter::FunctionCode(FunctionCode::VirtualTerminal);
    let vt_name_filters = vec![filter_virtual_terminal];

    // Create the Control Functions used by the Virtual Terminal
    // let test_internal_ecu = InternalControlFunction::new(test_device_name, test_device_address);
    // let test_partner_vt = PartneredControlFunction::new(&vt_name_filters);
    let test_internal_ecu_handle =  network_manager.new_internal_control_function(test_device_name, test_device_address);
    let test_partner_vt_handle =  network_manager.new_partnered_control_function(test_partner_vt);

    // Create the channel used to send VTKeyEvents from the callback to this task.
    // event_tx and event_rx have to outlive test_virtual_terminal_client, so we define them first.
    let (event_tx, event_rx): (Sender<VTKeyEvent>, Receiver<VTKeyEvent>) = channel();

    // Create a new Virtual Terminal Client (VTC), the main struct used to comunicate with a Virtual Terminal.
    let mut test_virtual_terminal_client = VirtualTerminalClient::new(test_partner_vt_handle, test_internal_ecu_handle);

    // Set the Object pool to be used by our VTC.
    // A VTC can use multiple Object pools, we store our pool at the first pool index (0).
    test_virtual_terminal_client.set_object_pool(0, test_pool);

    // Bind callbacks to VTC events.
    // These callbacks will provide us with event driven notifications of button presses from the stack.
    // Using a channel we can send events to the isobus_task to be processed.
    let vt_soft_key_event_listener_callback = |e| {
        let _ = event_tx.send(e);
    };
    let vt_button_event_listener_callback = |e| {
        let _ = event_tx.send(e);
    };
    let _ = test_virtual_terminal_client
        .add_vt_soft_key_event_listener(&vt_soft_key_event_listener_callback);
    let _ = test_virtual_terminal_client
        .add_vt_button_event_listener(&vt_button_event_listener_callback);

    // Initialize the VTC.
    test_virtual_terminal_client.initialize(&mut network_manager);

    // In the object pool the output number has an offset of -214748364 so we use this to represent 0.
    let mut example_number_output: u32 = 214748364;


    loop {
        // Update the NetworkManager.
        network_manager.update();

        // Update the VirtualTerminalClient.
        test_virtual_terminal_client.update(&mut network_manager);

        // Receive VTKeyEvents without blocking using callback results
        if let Ok(event) = event_rx.try_recv() {
            match event.key_event {
                KeyActivationCode::ButtonUnlatchedOrReleased => {
                    match event.object_id {
                        PLUS_BUTTON => {
                            example_number_output += 1;
                            test_virtual_terminal_client.send_change_numeric_value(
                                &mut network_manager,
                                BUTTON_EXAMPLE_NUMBER_VAR_NUM,
                                example_number_output,
                            );
                        }
                        MINUS_BUTTON => {
                            example_number_output -= 1;
                            test_virtual_terminal_client.send_change_numeric_value(
                                &mut network_manager,
                                BUTTON_EXAMPLE_NUMBER_VAR_NUM,
                                example_number_output,
                            );
                        }
                        ALARM_SOFT_KEY => {
                            // TestVirtualTerminalClient->send_change_active_mask(example_WorkingSet, example_AlarmMask);
                        }
                        ACKNOWLEDGE_ALARM_SOFT_KEY => {
                            // TestVirtualTerminalClient->send_change_active_mask(example_WorkingSet, mainRunscreen_DataMask);
                        }
                        _ => {}
                    };
                }
                _ => {}
            };
        }
    }

    // TestVirtualTerminalClient->terminate();
    // isobus::CANHardwareInterface::stop();
}
