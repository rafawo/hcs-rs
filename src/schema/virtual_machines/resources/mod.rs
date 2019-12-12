// Copyright  rafawo (rafawo1@hotmail.com). All rights reserved.
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.
// All files in the project carrying such notice may not be copied, modified, or distributed
// except according to those terms.
// THE SOURCE CODE IS AVAILABLE UNDER THE ABOVE CHOSEN LICENSE "AS IS", WITH NO WARRANTIES.

pub mod compute;
pub mod gpu;
pub mod guest_state;
pub mod network;
pub mod storage;
pub mod vpci;

use crate::schema::utils::is_default;
use crate::schema::utils::*;
use serde::{Deserialize, Serialize};

impl std::default::Default for UefiBootDevice {
    fn default() -> Self {
        UefiBootDevice::ScsiDrive
    }
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub enum UefiBootDevice {
    ScsiDrive,
    VmbFs,
    Network,
    File,
}

#[derive(Default, Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct UefiBootEntry {
    #[serde(rename = "DeviceType")]
    pub device_type: UefiBootDevice,

    #[serde(default, rename = "DevicePath", skip_serializing_if = "is_default")]
    pub device_path: String,

    #[serde(default, rename = "DiskNumber", skip_serializing_if = "is_default")]
    pub disk_number: u16,

    #[serde(default, rename = "OptionalData", skip_serializing_if = "is_default")]
    pub optional_data: String,

    #[serde(default, rename = "VmbFsrootPath", skip_serializing_if = "is_default")]
    pub vmbfs_root_path: String,
}

impl std::default::Default for SerialConsole {
    fn default() -> Self {
        SerialConsole::Default
    }
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub enum SerialConsole {
    Default,
    Disabled,
    ComPort1,
    ComPort2,
}

#[derive(Default, Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct Uefi {
    #[serde(default, rename = "EnableDebugger", skip_serializing_if = "is_default")]
    pub enable_debugger: bool,

    #[serde(
        default,
        rename = "SecureBootTemplateId",
        skip_serializing_if = "is_default"
    )]
    pub secure_boot_template_id: Option<GuidSerde>,

    #[serde(default, rename = "BootThis", skip_serializing_if = "is_default")]
    pub boot_this: Option<UefiBootEntry>,

    #[serde(default, rename = "Console", skip_serializing_if = "is_default")]
    pub console: SerialConsole,

    #[cfg(feature = "19h1")]
    #[serde(
        default,
        rename = "StopOnBootFailure",
        skip_serializing_if = "is_default"
    )]
    pub stop_on_boot_failure: bool,
}

#[cfg(feature = "19h1")]
#[derive(Default, Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct LinuxKernelDirect {
    #[serde(default, rename = "KernelFilePath", skip_serializing_if = "is_default")]
    pub kernel_file_path: String,

    #[serde(default, rename = "InitRdPath", skip_serializing_if = "is_default")]
    pub init_rd_path: String,

    #[serde(default, rename = "KernelCmdLine", skip_serializing_if = "is_default")]
    pub kernel_cmd_line: String,
}

#[derive(Default, Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct Chipset {
    #[serde(default, rename = "Uefi", skip_serializing_if = "is_default")]
    pub uefi: Option<Uefi>,

    #[serde(
        default,
        rename = "IsNumLockDisabled",
        skip_serializing_if = "is_default"
    )]
    pub is_num_lock_disabled: bool,

    #[serde(
        default,
        rename = "BaseBoardSerialNumber",
        skip_serializing_if = "is_default"
    )]
    pub base_board_serial_number: String,

    #[serde(
        default,
        rename = "ChassisSerialNumber",
        skip_serializing_if = "is_default"
    )]
    pub chassis_serial_number: String,

    #[serde(default, rename = "ChassisAsetTag", skip_serializing_if = "is_default")]
    pub chassis_asset_tag: String,

    #[serde(default, rename = "UseUtc", skip_serializing_if = "is_default")]
    pub use_utc: bool,

    #[cfg(feature = "19h1")]
    #[serde(
        default,
        rename = "LinuxKernelDirect",
        skip_serializing_if = "is_default"
    )]
    pub linux_kernel_direct: Option<LinuxKernelDirect>,
}

/// Specifies the named pipe that will be used for the port,
/// with empty string indicating a disconnected port.
#[derive(Default, Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct ComPort {
    #[serde(default, rename = "NamedPipe", skip_serializing_if = "is_default")]
    pub named_pipe: String,

    #[serde(
        default,
        rename = "OptimizeForDebugger",
        skip_serializing_if = "is_default"
    )]
    pub optimize_for_debugger: bool,
}

#[cfg(feature = "19h1")]
#[derive(Default, Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct VirtioSerialPort {
    #[serde(default, rename = "NamedPipe", skip_serializing_if = "is_default")]
    pub named_pipe: String,

    #[serde(default, rename = "Name", skip_serializing_if = "is_default")]
    pub name: String,
}

#[cfg(feature = "19h1")]
#[derive(Default, Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct VirtioSerial {
    #[serde(default, rename = "Ports", skip_serializing_if = "is_default")]
    pub ports: std::collections::HashMap<u32, VirtioSerialPort>,
}

#[derive(Default, Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct RdpConnectionOptions {
    #[serde(default, rename = "AccessSids", skip_serializing_if = "is_default")]
    pub access_sids: Vec<String>,

    #[serde(default, rename = "NamedPipe", skip_serializing_if = "is_default")]
    pub named_pipe: String,
}

#[derive(Default, Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct VideoMonitor {
    #[serde(
        default,
        rename = "HorizontalResolution",
        skip_serializing_if = "is_default"
    )]
    pub horizontal_resolution: u16,

    #[serde(
        default,
        rename = "VerticalResolution",
        skip_serializing_if = "is_default"
    )]
    pub vertical_resolution: u16,

    #[serde(
        default,
        rename = "ConnectionOptions",
        skip_serializing_if = "is_default"
    )]
    pub connection_options: Option<RdpConnectionOptions>,
}

#[derive(Default, Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct Mouse {}

#[derive(Default, Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct Keyboard {}

#[derive(Default, Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct EnhancedModeVideo {
    #[serde(
        default,
        rename = "ConnectionOptions",
        skip_serializing_if = "is_default"
    )]
    pub connection_options: Option<RdpConnectionOptions>,
}

#[derive(Default, Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct WindowsCrashReporting {
    #[serde(rename = "DumpFileName")]
    pub dump_filename: String,

    #[serde(default, rename = "MaxDumpSize", skip_serializing_if = "is_default")]
    pub max_dump_size: i64,
}

#[derive(Default, Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct GuestCrashReporting {
    #[serde(
        default,
        rename = "WindowsCrashSettings",
        skip_serializing_if = "is_default"
    )]
    pub windows_crash_settings: Option<WindowsCrashReporting>,
}

/// The settings used to configure guest activation.
#[derive(Default, Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct Licensing {
    #[serde(rename = "ContainerID")]
    pub container_id: GuidSerde,

    #[serde(rename = "PackageFamilyNames")]
    pub package_family_names: Vec<String>,
}

#[derive(Default, Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct Battery {}

#[derive(Default, Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct SystemTime {
    #[serde(rename = "Year")]
    pub year: u16,
    #[serde(rename = "Month")]
    pub month: u16,
    #[serde(rename = "DayOfWeek")]
    pub day_of_week: u16,
    #[serde(rename = "Day")]
    pub day: u16,
    #[serde(rename = "Hour")]
    pub hour: u16,
    #[serde(rename = "Minute")]
    pub minute: u16,
    #[serde(rename = "Second")]
    pub second: u16,
    #[serde(rename = "Milliseconds")]
    pub milliseconds: u16,
}

#[derive(Default, Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct TimeZoneInformation {
    #[serde(rename = "Bias")]
    bias: i32,

    #[serde(rename = "StandardName")]
    standard_name: String,

    #[serde(rename = "StandardDate")]
    standard_date: SystemTime,

    #[serde(rename = "StandardBias")]
    standard_bias: i32,

    #[serde(rename = "DaylightName")]
    daylight_name: String,

    #[serde(rename = "DaylightDate")]
    daylight_date: SystemTime,

    #[serde(rename = "DaylightBias")]
    daylight_bias: i32,
}

/// This class defines address settings applied to a VM
/// by the GCS every time a VM starts or restores.
#[derive(Default, Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct HvSocketAddress {
    #[serde(rename = "LocalAddress")]
    pub local_address: GuidSerde,

    #[serde(rename = "ParentAddress")]
    pub parent_address: GuidSerde,
}

/// HvSocket configuration for a VM
#[derive(Default, Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct HvSocket {
    #[serde(default, rename = "HvSocketConfig", skip_serializing_if = "is_default")]
    pub hvsocket_config: crate::schema::hvsocket::HvSocketSystemConfig,
}

#[derive(Default, Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct CimMount {
    #[serde(default, rename = "ImagePath", skip_serializing_if = "is_default")]
    pub image_path: String,

    #[serde(default, rename = "FileSystemName", skip_serializing_if = "is_default")]
    pub file_system_name: String,

    #[serde(default, rename = "VolumeGuid", skip_serializing_if = "is_default")]
    pub volume_guid: GuidSerde,
}

#[derive(Default, Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct SharedMemoryRegion {
    #[serde(rename = "SectionName")]
    pub section_name: String,

    #[serde(default, rename = "StartOffset", skip_serializing_if = "is_default")]
    pub start_offset: u64,

    #[serde(rename = "Length")]
    pub length: u64,

    #[serde(
        default,
        rename = "AllowGuestWrite",
        skip_serializing_if = "is_default"
    )]
    pub allow_guest_write: bool,

    #[serde(
        default,
        rename = "HiddenFromGuest",
        skip_serializing_if = "is_default"
    )]
    pub hidden_from_guest: bool,
}

#[derive(Default, Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct SharedMemoryConfiguration {
    #[serde(default, rename = "Regions", skip_serializing_if = "is_default")]
    pub regions: Vec<SharedMemoryRegion>,
}

#[derive(Default, Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct KernelIntegration {}
