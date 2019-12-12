// Copyright  rafawo (rafawo1@hotmail.com). All rights reserved.
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.
// All files in the project carrying such notice may not be copied, modified, or distributed
// except according to those terms.
// THE SOURCE CODE IS AVAILABLE UNDER THE ABOVE CHOSEN LICENSE "AS IS", WITH NO WARRANTIES.

use crate::schema::utils::is_default;
use serde::{Deserialize, Serialize};

impl std::default::Default for AttachmentType {
    fn default() -> Self {
        AttachmentType::VirtualDisk
    }
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
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

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub enum CachingMode {
    /// Use uncached IO to read and write VHD files (default).
    Uncached,
    /// Use cached IO for all files.
    Cached,
    /// Use cached IO for all read-only files in the VHD chain, and uncached IO for writable files.
    ReadOnlyCached,
}

#[derive(Default, Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct Attachment {
    #[serde(rename = "Type")]
    pub attachment_type: AttachmentType,

    #[serde(default, rename = "Path", skip_serializing_if = "is_default")]
    pub path: String,

    #[serde(default, rename = "CachingMode", skip_serializing_if = "is_default")]
    pub caching_mode: CachingMode,

    #[serde(rename = "ReadOnly")]
    pub read_only: bool,
}

/// Object describing a SCSI controller.
#[derive(Default, Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct Scsi {
    /// Map of attachments, where the key is the integer LUN number on the controller.
    #[serde(rename = "Attachments")]
    pub attachments: std::collections::HashMap<u32, Attachment>,
}

#[derive(Default, Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct VirtualSmbShareOptions {
    #[serde(default, rename = "ReadOnly", skip_serializing_if = "is_default")]
    pub read_only: bool,

    /// convert exclusive access to shared read access
    #[serde(default, rename = "ShareRead", skip_serializing_if = "is_default")]
    pub share_read: bool,

    /// all opens will use cached I/O
    #[serde(default, rename = "CacheIo", skip_serializing_if = "is_default")]
    pub cache_io: bool,

    // disable oplock support
    #[serde(default, rename = "NoOplocks", skip_serializing_if = "is_default")]
    pub no_oplocks: bool,

    /// Acquire the backup privilege when attempting to open
    #[serde(
        default,
        rename = "TakeBackupPrivilege",
        skip_serializing_if = "is_default"
    )]
    pub take_backup_privilege: bool,

    /// Use the identity of the share root when opening
    #[serde(
        default,
        rename = "UseShareRootIdentity",
        skip_serializing_if = "is_default"
    )]
    pub use_share_root_identity: bool,

    /// disable Direct Mapping
    #[serde(default, rename = "NoDirectmap", skip_serializing_if = "is_default")]
    pub no_directmap: bool,

    /// disable Byterange locks
    #[serde(default, rename = "NoLocks", skip_serializing_if = "is_default")]
    pub no_locks: bool,

    /// disable Directory CHange Notifications
    #[serde(default, rename = "NoDirnotify", skip_serializing_if = "is_default")]
    pub no_dirnotify: bool,

    /// share is use for VM shared memory
    #[serde(default, rename = "VmSharedMemory", skip_serializing_if = "is_default")]
    pub vm_shared_memory: bool,

    /// allow access only to the files specified in AllowedFiles
    #[serde(
        default,
        rename = "RestrictFileAccess",
        skip_serializing_if = "is_default"
    )]
    pub restrict_file_access: bool,

    /// disable all oplocks except Level II
    #[serde(
        default,
        rename = "ForceLevelIIOplocks",
        skip_serializing_if = "is_default"
    )]
    pub force_level_ii_oplocks: bool,

    /// Allow the host to reparse this base layer
    #[serde(
        default,
        rename = "ReparseBaseLayer",
        skip_serializing_if = "is_default"
    )]
    pub reparse_base_layer: bool,

    /// Enable pseudo-oplocks
    #[serde(default, rename = "PseudoOplocks", skip_serializing_if = "is_default")]
    pub pseudo_oplocks: bool,

    /// All opens will use non-cached IO
    #[serde(default, rename = "NonCacheIo", skip_serializing_if = "is_default")]
    pub non_cache_io: bool,

    /// Enable pseudo directory change notifications
    #[serde(
        default,
        rename = "PseudoDirnotify",
        skip_serializing_if = "is_default"
    )]
    pub pseudo_dirnotify: bool,

    /// Block directory enumeration, renames, and deletes.
    #[serde(
        default,
        rename = "SingleFileMapping",
        skip_serializing_if = "is_default"
    )]
    pub single_file_mapping: bool,
}

#[derive(Default, Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct VirtualSmbShare {
    #[serde(rename = "Name")]
    pub name: String,

    #[serde(rename = "Path")]
    pub path: String,

    #[serde(default, rename = "AllowedFiles", skip_serializing_if = "is_default")]
    pub allowed_files: Vec<String>,

    #[serde(default, rename = "Options", skip_serializing_if = "is_default")]
    pub options: VirtualSmbShareOptions,
}

#[derive(Default, Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct VirtualSmb {
    #[serde(default, rename = "Shares", skip_serializing_if = "is_default")]
    pub shares: Vec<VirtualSmbShare>,

    #[serde(
        default,
        rename = "DirectFileMappingInMB",
        skip_serializing_if = "is_default"
    )]
    pub direct_file_mapping_in_mb: i64,
}

#[derive(Default, Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct Plan9Share {
    #[serde(rename = "Name")]
    pub name: String,

    /// The name by which the guest operating system can access this share,
    /// via the name parameter in Plan9 protocol.
    #[serde(default, rename = "AccessName", skip_serializing_if = "is_default")]
    pub access_name: String,

    #[serde(rename = "Path")]
    pub path: String,

    #[serde(rename = "Port")]
    pub port: u32,

    #[cfg(feature = "19h1")]
    #[serde(default, rename = "AllowedFiles", skip_serializing_if = "is_default")]
    pub allowed_files: Vec<String>,
}

#[derive(Default, Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct Plan9 {
    #[serde(default, rename = "Shares", skip_serializing_if = "is_default")]
    pub shares: Vec<Plan9Share>,
}

#[derive(Default, Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct MappedVirtualDisk {
    #[serde(default, rename = "ContainerPath", skip_serializing_if = "is_default")]
    pub container_path: String,

    #[serde(default, rename = "Lun", skip_serializing_if = "is_default")]
    pub lun: u8,
}

impl std::default::Default for VirtualPMemImageFormat {
    fn default() -> Self {
        VirtualPMemImageFormat::Vhdx
    }
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub enum VirtualPMemImageFormat {
    Vhdx,
    Vhd1,
}

#[cfg(feature = "19h1")]
#[derive(Default, Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct VirtualPMemMapping {
    #[serde(rename = "HostPath")]
    pub host_path: String,

    #[serde(default, rename = "ImageFormat", skip_serializing_if = "is_default")]
    pub image_format: VirtualPMemImageFormat,
}

#[derive(Default, Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct VirtualPMemDevice {
    #[serde(rename = "HostPath")]
    pub host_path: String,

    #[serde(default, rename = "ReadOnly", skip_serializing_if = "is_default")]
    pub rean_only: bool,

    #[serde(default, rename = "ImageFormat", skip_serializing_if = "is_default")]
    pub image_format: VirtualPMemImageFormat,

    #[cfg(feature = "19h1")]
    #[serde(default, rename = "SizeBytes", skip_serializing_if = "is_default")]
    pub size_bytes: u64,

    #[cfg(feature = "19h1")]
    #[serde(default, rename = "Mappings", skip_serializing_if = "is_default")]
    pub mappings: std::collections::HashMap<u64, VirtualPMemMapping>,
}

impl std::default::Default for VirtualPMemBackingType {
    fn default() -> Self {
        VirtualPMemBackingType::Virtual
    }
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub enum VirtualPMemBackingType {
    Virtual,
    Physical,
}

#[derive(Default, Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct VirtualPMemController {
    #[serde(rename = "Devices")]
    pub devices: std::collections::HashMap<u8, VirtualPMemDevice>,

    /// This field indicates how many empty devices to add to the controller.
    /// If non-zero, additional VirtualPMemDevice objects with no HostPath and
    /// no Mappings will be added to the Devices map to get up to the MaximumCount.
    /// These devices will be configured with either the MaximumSizeBytes field if non-zero,
    /// or with the default maximum, 512 Mb.
    #[serde(default, rename = "MaximumCount", skip_serializing_if = "is_default")]
    pub maximum_count: u8,

    #[serde(
        default,
        rename = "MaximumSizeBytes",
        skip_serializing_if = "is_default"
    )]
    pub maximum_size_bytes: u64,

    #[serde(default, rename = "Backing", skip_serializing_if = "is_default")]
    pub backing: VirtualPMemBackingType,
}
