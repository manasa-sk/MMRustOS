#![no_std]
#![no_main]

extern crate alloc;

use core::fmt::Write;
use uefi::prelude::*;
use uefi::table::boot::MemoryType;

mod elf;
mod paging;

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

        match elf::parse::program_headers(&kernel, header) {
            Ok(phdrs) => {
                for ph in phdrs {
                    if ph.p_type != 1 {
                        continue;
                    }

                    let memsz = ph.p_memsz as usize;
                    let filesz = ph.p_filesz as usize;
                    let offset = ph.p_offset as usize;

                    let pages = (memsz + 4095) / 4096;

                    let phys_addr = st.boot_services()
                        .allocate_pages(
                            uefi::table::boot::AllocateType::AnyPages,
                            MemoryType::LOADER_DATA,
                            pages,
                        )
                        .expect("Failed to allocate pages");

                    let dest = phys_addr as *mut u8;

                    unsafe {
                        core::ptr::copy_nonoverlapping(
                            kernel.as_ptr().add(offset),
                            dest,
                            filesz,
                        );

                        if memsz > filesz {
                            core::ptr::write_bytes(
                                dest.add(filesz),
                                0,
                                memsz - filesz,
                            );
                        }
                    }

                    st.stdout().write_fmt(format_args!(
                        "Loaded PT_LOAD vaddr {:#x} → phys {:#x}\n",
                        ph.p_vaddr,
                        phys_addr
                    )).ok();
                }

                unsafe {
                    let tables = paging::setup_identity_mapping(&st);
                    st.stdout().write_str("Identity mapping ready\n").ok();
                }
            }
            Err(e) => {
                st.stdout().write_fmt(format_args!(
                    "PH parse error: {}\n",
                    e
                )).ok();
            }
        }
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