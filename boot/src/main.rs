#![no_std]
#![no_main]

extern crate alloc;

use uefi::prelude::*;
use core::fmt::Write;

#[entry]
fn efi_main(handle: Handle, mut st: SystemTable<Boot>) -> Status {
    // Init UEFI
    if let Err(e) = uefi_services::init(&mut st) {
        return e.status();
    }

    let stdout = st.stdout();

    let _ = stdout.reset(false);
    let _ = stdout.write_str("Bootloader started (x86_64 UEFI)\n");

    match elf::load_kernel(handle, &st) {
        Ok(kernel) => {
            let _ = stdout.write_fmt(format_args!(
                "Kernel loaded: {} bytes\n",
                kernel.len()
            ));
        }
        Err(e) => {
            let _ = stdout.write_fmt(format_args!(
                "Failed to load kernel: {:?}\n",
                e
            ));
        }
    }

    loop {}
}