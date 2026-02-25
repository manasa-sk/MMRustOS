use core::mem::size_of;

#[repr(C)]
#[derive(Debug)]
pub struct Elf64Header {
    pub e_ident: [u8; 16],
    pub e_type: u16,
    pub e_machine: u16,
    pub e_version: u32,
    pub e_entry: u64,
    pub e_phoff: u64,
    pub e_shoff: u64,
    pub e_flags: u32,
    pub e_ehsize: u16,
    pub e_phentsize: u16,
    pub e_phnum: u16,
    pub e_shentsize: u16,
    pub e_shnum: u16,
    pub e_shstrndx: u16,
}

pub fn parse_elf_header(buffer: &[u8]) -> Result<&Elf64Header, &'static str> {
    if buffer.len() < size_of::<Elf64Header>() {
        return Err("Buffer too small for ELF header");
    }

    let header = unsafe {
        &*(buffer.as_ptr() as *const Elf64Header)
    };

    if &header.e_ident[0..4] != b"\x7FELF" {
        return Err("Invalid ELF magic");
    }

    if header.e_ident[4] != 2 {
        return Err("Not ELF64");
    }

    Ok(header)
}