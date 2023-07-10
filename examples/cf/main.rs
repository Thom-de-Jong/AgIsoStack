use std::{sync::mpsc::*, thread, time::Duration};

use agisostack::{control_function::*, name::*, Address, CanFrame, CanNetworkManager};

fn main() {
    // Setup the logging interface.
    env_logger::builder()
        .filter_level(log::LevelFilter::Debug)
        .format_timestamp_millis()
        .init();

    // Create the channels used to send CanFrames between the Isobus threads.
    let (isobus1_tx, isobus1_rx): (Sender<CanFrame>, Receiver<CanFrame>) = channel();
    let (isobus2_tx, isobus2_rx): (Sender<CanFrame>, Receiver<CanFrame>) = channel();

    // Start the Isobus 1 thread.
    thread::spawn(move || isobus_task(0, isobus1_tx, isobus2_rx));

    // Wait for 1 second and drop all send CanFrames.
    thread::sleep(Duration::from_secs(1));
    while isobus1_rx.try_recv().is_ok() {}

    // Start the Isobus 2 thread.
    thread::spawn(move || isobus_task(1, isobus2_tx, isobus1_rx));

    // For example; Do all of our GUI in the main thread.
    loop {
        // log::info!("Time: {:?}", TimeDriver::time_elapsed());
        thread::sleep(Duration::from_secs(1));
    }
}

fn name(id: u8) -> Name {
    //NOTE: Make sure you change these for your device!
    //NOTE: This is an example device that is using a manufacturer code that is currently unused at time of writing.
    Name::builder()
        .arbitrary_address_capable(true)
        .industry_group(IndustryGroup::AgriculturalAndForestryEquipment)
        .device_class(DeviceClass::Fertilizers)
        .function_code(FunctionCode::MachineControl)
        .identity_number(2)
        .ecu_instance(id)
        .function_instance(0)
        .device_class_instance(0)
        .manufacturer_code(64)
        .build()
}

fn isobus_task(id: u8, tx: Sender<CanFrame>, rx: Receiver<CanFrame>) {
    // Create a new mannager for the CAN network we are connecting to.
    let mut network_manager: CanNetworkManager = CanNetworkManager::new();

    // Bind a callback to the network manager to be called when we send a can frame.
    // This is the "glue" between the network manager and the CAN Driver.
    let callback = |f| {
        log::debug!("{id} Send: {f}");
        let _ = tx.send(f);
    };
    network_manager.send_can_frame_callback(&callback);

    let test_device_name = name(id);
    let test_device_address = Address(0x80);

    let mut test_internal_ecu =
        InternalControlFunction::new(test_device_name, test_device_address).unwrap();

    // Initialize the internal control function.
    test_internal_ecu.initialize();

    loop {
        // Receive a CanFrame without blocking
        if let Ok(frame) = rx.try_recv() {
            log::debug!("{id} Read: {frame}");
            network_manager.process_can_frame(frame);
        }

        // Update the internal control function
        test_internal_ecu.update(&mut network_manager);
    }
}
