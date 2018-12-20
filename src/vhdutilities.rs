//! Wrappers around basic VHD functions used to setup container storage.

use virtdisk_rs::virtdisk::*;
use virtdisk_rs::virtdiskdefs::*;

const GUID_NULL: virtdisk_rs::windefs::Guid = VIRTUAL_STORAGE_TYPE_VENDOR_UNKNOWN;

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
                read_only: read_only as virtdisk_rs::windefs::Bool,
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
