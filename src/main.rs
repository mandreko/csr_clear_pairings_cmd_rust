extern crate rusb;

use std::time::Duration;
use rusb::{Context, Device, DeviceHandle, Direction, Recipient, RequestType, Result, UsbContext};
use clap::Parser;

const VENDOR_HID: u16 = 0x0a12;
const PRODUCT_HID: u16 = 0x100b;
// const VENDOR_HCI: u16 = 0x0a12;
// const PRODUCT_HCI: u16 = 0x0001;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Toggles the mode of operation. A power cycle is required to reset the module and re-enable HID mode.
    #[arg(short, long)]
    toggle_mode: bool,

    /// Clear the list of paired devices.
    #[arg(short, long)]
    clear_pairings: bool,
}


fn open_device<T: UsbContext>(
    context: &mut T,
    vid: u16,
    pid: u16,
) -> Option<(Device<T>, DeviceHandle<T>)> {
    let devices = match context.devices() {
        Ok(d) => d,
        Err(_) => return None,
    };

    for device in devices.iter() {
        let device_desc = match device.device_descriptor() {
            Ok(d) => d,
            Err(_) => continue,
        };

        if device_desc.vendor_id() == vid && device_desc.product_id() == pid {
            match device.open() {
                Ok(handle) => return Some((device, handle)),
                Err(_) => continue,
            }
        }
    }

    None
}

fn main()  -> Result<()> {
    println!("\n\r+++ Bluetooth-USB-Bridge / HID Proxy Dongle Command Line Interface +++\n\n");

    let args = Args::parse();

    let mut context = Context::new()?;
    let (_device, handle) =
        open_device(&mut context, VENDOR_HID, PRODUCT_HID).expect("Failed to open USB device");

    if args.toggle_mode {
        let ctrl_buf : [u8; 2] = [1, 5];
        let request_type = rusb::request_type(Direction::Out, RequestType::Class, Recipient::Device);
        let request : u8 = 0;
        let value : u16 = 0;
        let index : u16 = 9;
        let timeout : Duration = Duration::from_secs(1);
        match handle.write_control(request_type, request, value, index, &ctrl_buf, timeout)
        {
            Ok(_) => {}
            Err(err) => println!("Could not read from endpoint: {}", err),
        }

        let ctrl_buf : [u8; 2] = [5, 0];
        let index : u16 = 8;
        match handle.write_control(request_type, request, value, index, &ctrl_buf, timeout)
        {
            Ok(_) => {}
            Err(err) => println!("Could not read from endpoint: {}", err),
        }

        println!("Switching from HID proxy to HCI mode.\n");
    }
    if args.clear_pairings {
        let ctrl_buf : [u8; 2] = [1, 6];
        let request_type = rusb::request_type(Direction::Out, RequestType::Class, Recipient::Device);
        let request : u8 = 0;
        let value : u16 = 0;
        let index : u16 = 9;
        let timeout : Duration = Duration::from_secs(1);
        match handle.write_control(request_type, request, value, index, &ctrl_buf, timeout)
        {
            Ok(_) => {}
            Err(err) => println!("Could not read from endpoint: {}", err),
        }

        let ctrl_buf : [u8; 2] = [6, 0];
        let index : u16 = 8;
        match handle.write_control(request_type, request, value, index, &ctrl_buf, timeout)
        {
            Ok(_) => {}
            Err(err) => println!("Could not read from endpoint: {}", err),
        }

        println!("Pairings cleared.\n");
    }
    Ok(())
}
