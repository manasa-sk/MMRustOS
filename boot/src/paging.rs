use uefi::prelude::*;
use uefi::table::boot::{AllocateType, MemoryType};

const PAGE_SIZE: usize = 4096;
const PRESENT: u64 = 1 << 0;
const WRITABLE: u64 = 1 << 1;
const HUGE_PAGE: u64 = 1 << 7;

pub struct PageTables {
    pub pml4: *mut u64,
}

pub unsafe fn setup_identity_mapping(st: &SystemTable<Boot>) -> PageTables {
    let bs = st.boot_services();

    let pml4 = bs.allocate_pages(
        AllocateType::AnyPages,
        MemoryType::LOADER_DATA,
        1,
    ).expect("PML4 alloc failed") as *mut u64;

    core::ptr::write_bytes(pml4 as *mut u8, 0, PAGE_SIZE);

    let pdpt = bs.allocate_pages(
        AllocateType::AnyPages,
        MemoryType::LOADER_DATA,
        1,
    ).expect("PDPT alloc failed") as *mut u64;

    core::ptr::write_bytes(pml4 as *mut u8, 0, PAGE_SIZE);

    (*pml4.add(0)) = pdpt as u64 | PRESENT | WRITABLE;

    let pd = bs.allocate_pages(
        AllocateType::AnyPages,
        MemoryType::LOADER_DATA,
        1,
    ).expect("PD alloc failed") as *mut u64;

    core::ptr::write_bytes(pml4 as *mut u8, 0, PAGE_SIZE);

    (*pdpt.add(0)) = pd as u64 | PRESENT | WRITABLE;

    for i in 0..512 {
        let addr = (i as u64) * 0x200000; // 2MB
        (*pd.add(i)) = addr | PRESENT | WRITABLE | HUGE_PAGE;
    }

    PageTables { pml4 }
}