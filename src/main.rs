#![no_std]
#![no_main]
#![feature(asm)]
#![feature(abi_efiapi)]

#[macro_use]
extern crate log;
#[macro_use]
extern crate alloc;

use alloc::string::String;
use core::mem;
use uefi::prelude::*;
use uefi::proto::console::serial::Serial;
use uefi::table::boot::MemoryDescriptor;
use uefi::table::runtime::ResetType;

mod snapshot_ui;
mod snapshot_utils;

#[entry]
fn efi_main(image: Handle, mut st: SystemTable<Boot>) -> Status {

    uefi_services::init(&mut st).expect_success("Failed to initialize utilities");

    // output firmware-vendor (CStr16 to Rust string)
    let mut buf = String::new();
    st.firmware_vendor().as_str_in_buf(&mut buf).unwrap();
    info!("Firmware Vendor: {}", buf.as_str());

    st.stdout()
        .reset(false)
        .expect_success("Failed to reset stdout");

    let bt = st.boot_services();

    // Try retrieving a handle to the file system the image was booted from.
    bt.get_image_file_system(image)
        .expect("Failed to retrieve boot file system")
        .unwrap();

    let rt = unsafe { st.runtime_services() };
    rt.reset(ResetType::Shutdown, Status::SUCCESS, None);
}
