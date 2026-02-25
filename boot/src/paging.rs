use uefi::prelude::*;
use uefi::table::boot::{AllocateType, MemoryType};

const PAGE_SIZE: usize = 4096;
const PRESENT: u64 = 1 << 0;
const WRITABLE: u64 = 1 << 1;
const HUGE_PAGE: u64 = 1 << 7;
const PAGE_PRESENT: u64 = 1 << 0;
const PAGE_WRITABLE: u64 = 1 << 1;

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

unsafe fn get_or_create_table(
    bs: &uefi::table::boot::BootServices,
    entry: *mut u64,
) -> *mut u64 {
    if *entry & PAGE_PRESENT != 0 {
        return (*entry & 0x000fffff_fffff000) as *mut u64;
    }

    let table = bs.allocate_pages(
        AllocateType::AnyPages,
        MemoryType::LOADER_DATA,
        1,
    ).expect("table alloc failed") as *mut u64;

    core::ptr::write_bytes(table as *mut u8, 0, PAGE_SIZE);

    *entry = table as u64 | PAGE_PRESENT | PAGE_WRITABLE;

    table
}

pub unsafe fn map_page(
    st: &SystemTable<Boot>,
    pml4: *mut u64,
    virt: u64,
    phys: u64,
) {
    let bs = st.boot_services();

    let pml4_index = ((virt >> 39) & 0x1ff) as usize;
    let pdpt_index = ((virt >> 30) & 0x1ff) as usize;
    let pd_index   = ((virt >> 21) & 0x1ff) as usize;
    let pt_index   = ((virt >> 12) & 0x1ff) as usize;

    let pdpt = get_or_create_table(bs, pml4.add(pml4_index));
    let pd   = get_or_create_table(bs, pdpt.add(pdpt_index));
    let pt   = get_or_create_table(bs, pd.add(pd_index));

    *pt.add(pt_index) = (phys & 0x000fffff_fffff000)
        | PAGE_PRESENT
        | PAGE_WRITABLE;
}