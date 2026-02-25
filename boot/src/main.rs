#![no_std]
#![no_main]

extern crate alloc;

use core::fmt::Write;
use uefi::prelude::*;

mod elf;

#[entry]
fn efi_main(handle: Handle, mut st: SystemTable<Boot>) -> Status {
    if let Err(e) = uefi_services::init(&mut st) {
        return e.status();
    }

    st.stdout().reset(false).ok();
    st.stdout().write_str("Bootloader started\n").ok();

    match elf::load::load_kernel(handle, &st) {
    Ok(kernel) => {
        st.stdout().write_fmt(format_args!(
            "Kernel loaded: {} bytes\n",
            kernel.len()
        )).ok();

        match elf::parse::parse_elf_header(&kernel) {
            Ok(header) => {
                st.stdout().write_fmt(format_args!(
                    "ELF entry: {:#x}\n",
                    header.e_entry
                )).ok();

                st.stdout().write_fmt(format_args!(
                    "Program headers: {}\n",
                    header.e_phnum
                )).ok();
            }
            Err(err) => {
                st.stdout().write_fmt(format_args!(
                    "ELF parse error: {}\n",
                    err
                )).ok();
            }
        }
    }
    Err(e) => {
        st.stdout().write_fmt(format_args!(
            "Failed to load kernel: {:?}\n",
            e
        )).ok();
    }
}

    loop {}
}