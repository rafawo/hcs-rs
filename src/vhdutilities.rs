//! Wrappers around basic VHD functions used to setup container storage.

use virtdisk_rs::virtdisk::*;
use virtdisk_rs::virtdiskdefs::*;
use virtdisk_rs::windefs::*;

const GUID_NULL: Guid = Guid {
    Data1: 0x00000000,
    Data2: 0x0000,
    Data3: 0x0000,
    Data4: [0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00],
};

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
    use winapi::um::{fileapi, winnt, winbase};

    unsafe {
        let handle = fileapi::CreateFileW(
            widestring::WideCString::from_str(disk_path).unwrap().as_ptr(),
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

/// Opens a VHD for use as a container sandbox and returns a safe wrapper over the handle.
pub fn open_vhd(filename: &str, read_only: bool) -> Result<VirtualDisk, ResultCode> {
    let default_storage_type = VirtualStorageType {
        device_id: 0,
        vendor_id: VIRTUAL_STORAGE_TYPE_VENDOR_UNKNOWN,
    };

    let parameters = open_virtual_disk::Parameters {
        version: open_virtual_disk::Version::Version2,
        version_details: open_virtual_disk::VersionDetails {
            version2: open_virtual_disk::Version2 {
                get_info_only: 0,
                read_only: read_only as Bool,
                resiliency_guid: GUID_NULL,
            },
        },
    };

    VirtualDisk::open(
        default_storage_type,
        filename,
        VirtualDiskAccessMask::None,
        open_virtual_disk::Flag::ParentCachedIo as u32
            | open_virtual_disk::Flag::IgnoreRelativeParentLocator as u32,
        Some(&parameters),
    )
}
