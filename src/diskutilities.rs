//! Wrappers around basic disk functions used to setup container storage.

use crate::vhdutilities::GUID_NULL;
use virtdisk_rs::virtdisk::ResultCode;
use virtdisk_rs::windefs::*;

pub struct PartitionInfo {
    volume_path: String,
    disk_id: Guid,
    partition_id: Guid,
}

/// Safe abstraction to a disk handle.
pub struct Disk {
    handle: Handle,
}

impl std::ops::Drop for Disk {
    fn drop(&mut self) {
        if self.handle == std::ptr::null_mut() {
            return;
        }

        #[allow(unused_assignments)]
        let mut result: Bool = 0;

        unsafe {
            result = winapi::um::handleapi::CloseHandle(self.handle);
        }

        match result {
            result if result == 0 => {
                panic!("Closing handle failed with error code {}", unsafe {
                    winapi::um::errhandlingapi::GetLastError()
                });
            }
            _ => {}
        }
    }
}

/// Opens a disk by path.
pub fn open_disk(disk_path: &str, access_mask: DWord) -> Result<Disk, ResultCode> {
    use winapi::um::{fileapi, winbase, winnt};

    unsafe {
        let handle = fileapi::CreateFileW(
            widestring::WideCString::from_str(disk_path)
                .unwrap()
                .as_ptr(),
            access_mask,
            winnt::FILE_SHARE_READ | winnt::FILE_SHARE_WRITE,
            std::ptr::null_mut(),
            fileapi::OPEN_EXISTING,
            winnt::FILE_ATTRIBUTE_NORMAL | winbase::FILE_FLAG_NO_BUFFERING,
            std::ptr::null_mut(),
        );

        match handle {
            handle if handle != std::ptr::null_mut() => Ok(Disk { handle }),
            _handle => Err(ResultCode::FileNotFound),
        }
    }
}

pub fn force_online_hard_disk(disk: &Disk) {}

pub fn get_disk_volume_path(disk: &Disk) -> String {
    String::new()
}

pub fn format_disk(disk: &Disk, file_system: &str) -> PartitionInfo {
    PartitionInfo {
        volume_path: String::new(),
        disk_id: GUID_NULL,
        partition_id: GUID_NULL,
    }
}
