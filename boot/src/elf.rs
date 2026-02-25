use alloc::vec::Vec;
use uefi::prelude::*;
use uefi::proto::media::file::{File, FileMode, FileAttribute};
use uefi::proto::media::fs::SimpleFileSystem;
use uefi::table::boot::{OpenProtocolAttributes, OpenProtocolParams};
use uefi::CStr16;

pub fn load_kernel(
    handle: Handle,
    st: &SystemTable<Boot>,
) -> Result<Vec<u8>, Status> {
    let bt = st.boot_services();

    let loaded_image = unsafe {
        bt.open_protocol::<uefi::proto::loaded_image::LoadedImage>(
            OpenProtocolParams {
                handle,
                agent: handle,
                controller: None,
            },
            OpenProtocolAttributes::GetProtocol,
        )
    }.map_err(|e: uefi::Error| e.status())?;

    let device = loaded_image.device().ok_or(Status::LOAD_ERROR)?;

    let mut fs = unsafe {
        bt.open_protocol::<SimpleFileSystem>(
            OpenProtocolParams {
                handle: device,
                agent: handle,
                controller: None,
            },
            OpenProtocolAttributes::GetProtocol,
        )
    }.map_err(|e: uefi::Error| e.status())?;

    let mut root = fs.open_volume()
        .map_err(|e: uefi::Error| e.status())?;

    let mut filename_buf = [0u16; 32];

    let filename = CStr16::from_str_with_buf(
        "kernel.elf",
        &mut filename_buf,
    ).map_err(|_| Status::LOAD_ERROR)?;

    let file = root.open(
        filename,
        FileMode::Read,
        FileAttribute::empty(),
    ).map_err(|e: uefi::Error| e.status())?;

    let mut file = file.into_regular_file()
        .ok_or(Status::LOAD_ERROR)?;

    use uefi::proto::media::file::FileInfo;

    let mut info_buffer = [0u8; 512];

    let info = file.get_info::<FileInfo>(&mut info_buffer)
        .map_err(|_| Status::LOAD_ERROR)?;

    let size = info.file_size() as usize;

    let mut buffer = Vec::with_capacity(size);
    unsafe { buffer.set_len(size); }

    file.read(&mut buffer)
        .map_err(|e: uefi::Error| e.status())?;

    Ok(buffer)
}