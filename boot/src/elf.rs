use uefi::prelude::*;
use uefi::proto::media::file::{File, FileMode, FileAttribute};
use uefi::proto::media::fs::SimpleFileSystem;
use uefi::table::boot::OpenProtocolAttributes;
use alloc::vec::Vec;

pub fn load_kernel(
    handle: Handle,
    st: &SystemTable<Boot>,
) -> Result<Vec<u8>, Status> {
    let bt = st.boot_services();

    // Get LoadedImage protocol
    let loaded_image = bt.open_protocol::<uefi::proto::loaded_image::LoadedImage>(
        handle,
        OpenProtocolAttributes::GetProtocol,
    ).map_err(|e| e.status())?;

    // Get filesystem from device
    let fs = bt.open_protocol::<SimpleFileSystem>(
        loaded_image.device(),
        OpenProtocolAttributes::GetProtocol,
    ).map_err(|e| e.status())?;

    let mut root = fs.open_volume().map_err(|e| e.status())?;

    // Open kernel file
    let file = root.open(
        "kernel.elf",
        FileMode::Read,
        FileAttribute::empty(),
    ).map_err(|e| e.status())?;

    let mut file = match file.into_type().map_err(|e| e.status())? {
        File::Regular(f) => f,
        _ => return Err(Status::LOAD_ERROR),
    };

    // Get file size
    let info = file.get_boxed_info::<uefi::proto::media::file::FileInfo>()
        .map_err(|e| e.status())?;

    let file_size = info.file_size() as usize;

    // Allocate buffer
    let mut buffer = Vec::with_capacity(file_size);
    unsafe { buffer.set_len(file_size); }

    file.read(&mut buffer).map_err(|e| e.status())?;

    Ok(buffer)
}