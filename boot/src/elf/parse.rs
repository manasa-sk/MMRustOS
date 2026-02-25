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

#[repr(C)]
#[derive(Debug)]
pub struct Elf64ProgramHeader {
    pub p_type: u32,
    pub p_flags: u32,
    pub p_offset: u64,
    pub p_vaddr: u64,
    pub p_paddr: u64,
    pub p_filesz: u64,
    pub p_memsz: u64,
    pub p_align: u64,
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

pub fn program_headers<'a>(
    buffer: &'a [u8],
    header: &Elf64Header,
) -> Result<&'a [Elf64ProgramHeader], &'static str> {

    let phoff = header.e_phoff as usize;
    let phnum = header.e_phnum as usize;
    let entsize = header.e_phentsize as usize;

    if entsize != core::mem::size_of::<Elf64ProgramHeader>() {
        return Err("Unexpected program header size");
    }

    let total_size = phoff + phnum * entsize;

    if buffer.len() < total_size {
        return Err("Program headers out of bounds");
    }

    let ptr = unsafe {
        buffer.as_ptr().add(phoff) as *const Elf64ProgramHeader
    };

    let slice = unsafe {
        core::slice::from_raw_parts(ptr, phnum)
    };

    Ok(slice)
}