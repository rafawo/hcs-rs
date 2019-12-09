// Copyright  rafawo (rafawo1@hotmail.com). All rights reserved.
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.
// All files in the project carrying such notice may not be copied, modified, or distributed
// except according to those terms.
// THE SOURCE CODE IS AVAILABLE UNDER THE ABOVE CHOSEN LICENSE "AS IS", WITH NO WARRANTIES.

use serde::{Deserialize, Serialize};

impl std::default::Default for AttachmentType {
    fn default() -> Self {
        AttachmentType::VirtualDisk
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub enum AttachmentType {
    VirtualDisk,
    Iso,
    PassThru,
}

impl std::default::Default for CachingMode {
    fn default() -> Self {
        CachingMode::Uncached
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub enum CachingMode {
    /// Use uncached IO to read and write VHD files (default).
    Uncached,
    /// Use cached IO for all files.
    Cached,
    /// Use cached IO for all read-only files in the VHD chain, and uncached IO for writable files.
    ReadOnlyCached,
}

#[derive(Default, Deserialize, Serialize, Debug, Clone)]
pub struct Attachment {
    #[serde(rename = "Type")]
    pub attachment_type: AttachmentType,

    #[serde(default, rename = "Path")]
    pub path: String,

    #[serde(default, rename = "CachingMode")]
    pub caching_mode: CachingMode,

    #[serde(rename = "ReadOnly")]
    pub read_only: bool,

    #[serde(default, rename = "SupportCompressedVolumes")]
    pub support_compressed_volumes: bool,
}

/// Object describing a SCSI controller.
#[derive(Default, Deserialize, Serialize, Debug, Clone)]
pub struct Scsi {
    /// Map of attachments, where the key is the integer LUN number on the controller.
    #[serde(rename = "Attachments")]
    pub attachments: std::collections::HashMap<u32, Attachment>,
}

impl std::default::Default for VSmbShareFlags {
    fn default() -> Self {
        VSmbShareFlags::None
    }
}

#[repr(u32)]
#[derive(Deserialize, Serialize, Debug, Clone)]
pub enum VSmbShareFlags {
    None = 0x00000000,
    /// read-only shares
    ReadOnly = 0x00000001,
    /// convert exclusive access to shared read access
    ShareRead = 0x00000002,
    /// all opens will use cached I/O
    CacheIO = 0x00000004,
    /// disable oplock support
    NoOplocks = 0x00000008,
    /// Acquire the backup privilege when attempting to open
    TakeBackupPrivilege = 0x00000010,
    /// Use the identity of the share root when opening
    UseShareRootIdentity = 0x00000020,
    /// disable Direct Mapping
    NoDirectmap = 0x00000040,
    /// disable Byterange locks
    NoLocks = 0x00000080,
    /// disable Directory CHange Notifications
    NoDirnotify = 0x00000100,
    /// test mode
    Test = 0x00000200,
    /// share is use for VM shared memory
    VmSharedMemory = 0x00000400,
    /// allow access only to the files specified in AllowedFiles
    RestrictFileAccess = 0x00000800,
    /// disable all oplocks except Level II
    ForceLevelIIOplocks = 0x00001000,
    /// Allow the host to reparse this base layer
    ReparseBaseLayer = 0x00002000,
    /// Enable pseudo-oplocks
    PseudoOplocks = 0x00004000,
    /// All opens will use non-cached IO
    NonCacheIO = 0x00008000,
    /// Enable pseudo directory change notifications
    PseudoDirnotify = 0x00010000,
    /// Content indexing disabled by the host for all files in the share
    DisableIndexing = 0x00020000,
    /// Alternate data streams hidden to the guest (open fails, streams are not enumerated, etc.)
    HideAlternateDataStreams = 0x00040000,
    /// Only FSCTLs listed in AllowedFsctls are allowed against any files in the share
    EnableFsctlFiltering = 0x00080000,
    /// allow access only to the files specified in AllowedFiles, plus allow creation of new files.
    AllowNewCreates = 0x00100000,
}

#[derive(Default, Deserialize, Serialize, Debug, Clone)]
pub struct VirtualSmbShareOptions {
    #[serde(default, rename = "ReadOnly")]
    pub read_only: bool,

    /// convert exclusive access to shared read access
    #[serde(default, rename = "ShareRead")]
    pub share_read: bool,

    /// all opens will use cached I/O
    #[serde(default, rename = "CacheIo")]
    pub cache_io: bool,

    // disable oplock support
    #[serde(default, rename = "NoOplocks")]
    pub no_oplocks: bool,

    /// Acquire the backup privilege when attempting to open
    #[serde(default, rename = "TakeBackupPrivilege")]
    pub take_backup_privilege: bool,

    /// Use the identity of the share root when opening
    #[serde(default, rename = "UseShareRootIdentity")]
    pub use_share_root_identity: bool,

    /// disable Direct Mapping
    #[serde(default, rename = "NoDirectmap")]
    pub no_directmap: bool,

    /// disable Byterange locks
    #[serde(default, rename = "NoLocks")]
    pub no_locks: bool,

    /// disable Directory CHange Notifications
    #[serde(default, rename = "NoDirnotify")]
    pub no_dirnotify: bool,

    /// share is use for VM shared memory
    #[serde(default, rename = "VmSharedMemory")]
    pub vm_shared_memory: bool,

    /// allow access only to the files specified in AllowedFiles
    #[serde(default, rename = "RestrictFileAccess")]
    pub restrict_file_access: bool,

    /// disable all oplocks except Level II
    #[serde(default, rename = "ForceLevelIIOplocks")]
    pub force_level_ii_oplocks: bool,

    /// Allow the host to reparse this base layer
    #[serde(default, rename = "ReparseBaseLayer")]
    pub reparse_base_layer: bool,

    /// Enable pseudo-oplocks
    #[serde(default, rename = "PseudoOplocks")]
    pub pseudo_oplocks: bool,

    /// All opens will use non-cached IO
    #[serde(default, rename = "NonCacheIo")]
    pub non_cache_io: bool,

    /// Enable pseudo directory change notifications
    #[serde(default, rename = "PseudoDirnotify")]
    pub pseudo_dirnotify: bool,

    /// Block directory enumeration, renames, and deletes.
    #[serde(default, rename = "SingleFileMapping")]
    pub single_file_mapping: bool,

    // Support Cloud Files functionality
    #[serde(default, rename = "SupportCloudFiles")]
    pub support_cloud_files: bool,
}

impl std::default::Default for Plan9ShareFlags {
    fn default() -> Self {
        Plan9ShareFlags::None
    }
}

#[repr(u32)]
#[derive(Deserialize, Serialize, Debug, Clone)]
pub enum Plan9ShareFlags {
    None = 0x00000000,
    ReadOnly = 0x00000001,
    LinuxMetadata = 0x00000004,
    CaseSensitive = 0x00000008,
    UseShareRootIdentity = 0x00000010,
    RestrictFileAccess = 0x00000080,
}

#[derive(Default, Deserialize, Serialize, Debug, Clone)]
pub struct VirtualSmbShare {
    #[serde(rename = "Name")]
    pub name: String,

    #[serde(rename = "Path")]
    pub path: String,

    /// Combination of `VSmbShareFlags` as `u32`
    #[serde(default, rename = "Flags")]
    pub flags: u32,

    #[serde(default, rename = "AllowedFiles")]
    pub allowed_files: Vec<String>,

    #[serde(default, rename = "Options")]
    pub options: VirtualSmbShareOptions,
}

#[derive(Default, Deserialize, Serialize, Debug, Clone)]
pub struct VirtualSmb {
    #[serde(default, rename = "Shares")]
    pub shares: Vec<VirtualSmbShare>,

    #[serde(default, rename = "DirectFileMappingInMB")]
    pub direct_file_mapping_in_mb: i64,
}

#[derive(Default, Deserialize, Serialize, Debug, Clone)]
pub struct Plan9Share {
    #[serde(rename = "Name")]
    pub name: String,

    /// The name by which the guest operating system can access this share,
    /// via the name parameter in Plan9 protocol.
    #[serde(default, rename = "AccessName")]
    pub access_name: String,

    #[serde(rename = "Path")]
    pub path: String,

    #[serde(rename = "Port")]
    pub port: u32,

    /// Combination of `Plan9ShareFlags` as `u32`.
    #[serde(default, rename = "Flags")]
    pub flags: u32,

    #[serde(default, rename = "AllowedFiles")]
    pub allowed_files: Vec<String>,
}

#[derive(Default, Deserialize, Serialize, Debug, Clone)]
pub struct Plan9 {
    #[serde(default, rename = "Shares")]
    pub shares: Vec<Plan9Share>,
}

#[derive(Default, Deserialize, Serialize, Debug, Clone)]
pub struct MappedVirtualDisk {
    #[serde(default, rename = "ContainerPath")]
    pub container_path: String,

    #[serde(default, rename = "Lun")]
    pub lun: u8,
}

impl std::default::Default for VirtualPMemImageFormat {
    fn default() -> Self {
        VirtualPMemImageFormat::Vhdx
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub enum VirtualPMemImageFormat {
    Vhdx,
    Vhd1,
}

#[derive(Default, Deserialize, Serialize, Debug, Clone)]
pub struct VirtualPMemMapping {
    #[serde(rename = "HostPath")]
    pub host_path: String,

    #[serde(default, rename = "ImageFormat")]
    pub image_format: VirtualPMemImageFormat,
}

#[derive(Default, Deserialize, Serialize, Debug, Clone)]
pub struct VirtualPMemDevice {
    #[serde(rename = "HostPath")]
    pub host_path: String,

    #[serde(default, rename = "ReadOnly")]
    pub rean_only: bool,

    #[serde(default, rename = "ImageFormat")]
    pub image_format: VirtualPMemImageFormat,

    #[serde(default, rename = "SizeBytes")]
    pub size_bytes: u64,

    #[serde(default, rename = "Mappings")]
    pub mappings: std::collections::HashMap<u64, VirtualPMemMapping>,
}

impl std::default::Default for VirtualPMemBackingType {
    fn default() -> Self {
        VirtualPMemBackingType::Virtual
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub enum VirtualPMemBackingType {
    Virtual,
    Physical,
}

#[derive(Default, Deserialize, Serialize, Debug, Clone)]
pub struct VirtualPMemController {
    #[serde(rename = "Devices")]
    pub devices: std::collections::HashMap<u8, VirtualPMemDevice>,

    /// This field indicates how many empty devices to add to the controller.
    /// If non-zero, additional VirtualPMemDevice objects with no HostPath and
    /// no Mappings will be added to the Devices map to get up to the MaximumCount.
    /// These devices will be configured with either the MaximumSizeBytes field if non-zero,
    /// or with the default maximum, 512 Mb.
    #[serde(default, rename = "MaximumCount")]
    pub maximum_count: u8,

    #[serde(default, rename = "MaximumSizeBytes")]
    pub maximum_size_bytes: u64,

    #[serde(default, rename = "Backing")]
    pub backing: VirtualPMemBackingType,
}
