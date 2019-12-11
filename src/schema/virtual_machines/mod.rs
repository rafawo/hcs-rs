// Copyright  rafawo (rafawo1@hotmail.com). All rights reserved.
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.
// All files in the project carrying such notice may not be copied, modified, or distributed
// except according to those terms.
// THE SOURCE CODE IS AVAILABLE UNDER THE ABOVE CHOSEN LICENSE "AS IS", WITH NO WARRANTIES.

pub mod resources;

use crate::schema;
use crate::schema::utils::is_default;
use serde::{Deserialize, Serialize};

#[derive(Default, Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct Devices {
    #[serde(default, rename = "ComPorts", skip_serializing_if = "is_default")]
    pub com_ports: std::collections::HashMap<u32, schema::virtual_machines::resources::ComPort>,

    #[cfg(feature = "19h1")]
    #[serde(default, rename = "VirtioSerial", skip_serializing_if = "is_default")]
    pub virtio_serial: Option<schema::virtual_machines::resources::VirtioSerial>,

    #[serde(default, rename = "Scsi", skip_serializing_if = "is_default")]
    pub scsi: std::collections::HashMap<String, schema::virtual_machines::resources::storage::Scsi>,

    #[serde(default, rename = "VirtualPMem", skip_serializing_if = "is_default")]
    pub virtual_pmem: Option<schema::virtual_machines::resources::storage::VirtualPMemController>,

    #[serde(
        default,
        rename = "NetworkAdapters",
        skip_serializing_if = "is_default"
    )]
    pub network_adapters: std::collections::HashMap<
        String,
        schema::virtual_machines::resources::network::NetworkAdapter,
    >,

    #[serde(default, rename = "VideoMonitor", skip_serializing_if = "is_default")]
    pub video_monitor: Option<schema::virtual_machines::resources::VideoMonitor>,

    #[serde(default, rename = "Keyboard", skip_serializing_if = "is_default")]
    pub keyboard: Option<schema::virtual_machines::resources::Keyboard>,

    #[serde(default, rename = "Mouse", skip_serializing_if = "is_default")]
    pub mouse: Option<schema::virtual_machines::resources::Mouse>,

    #[serde(default, rename = "HvSocket", skip_serializing_if = "is_default")]
    pub hvsocket: Option<schema::virtual_machines::resources::HvSocket>,

    #[serde(
        default,
        rename = "EnhancedModeVideo",
        skip_serializing_if = "is_default"
    )]
    pub enhanced_mode_video: Option<schema::virtual_machines::resources::EnhancedModeVideo>,

    #[serde(
        default,
        rename = "GuestCrashReporting",
        skip_serializing_if = "is_default"
    )]
    pub guest_crash_reporting: Option<schema::virtual_machines::resources::GuestCrashReporting>,

    #[serde(default, rename = "VirtualSmb", skip_serializing_if = "is_default")]
    pub virtual_smb: Option<schema::virtual_machines::resources::storage::VirtualSmb>,

    #[serde(default, rename = "Plan9", skip_serializing_if = "is_default")]
    pub plan9: Option<schema::virtual_machines::resources::storage::Plan9>,

    #[serde(default, rename = "Battery", skip_serializing_if = "is_default")]
    pub battery: Option<schema::virtual_machines::resources::Battery>,

    #[serde(default, rename = "FlexibleIov", skip_serializing_if = "is_default")]
    pub flexible_iov: std::collections::HashMap<
        String,
        schema::virtual_machines::resources::vpci::FlexibleIoDevice,
    >,

    #[serde(default, rename = "SharedMemory", skip_serializing_if = "is_default")]
    pub shared_memory: Option<schema::virtual_machines::resources::SharedMemoryConfiguration>,

    #[serde(
        default,
        rename = "KernelIntegration",
        skip_serializing_if = "is_default"
    )]
    pub kernel_integration: Option<schema::virtual_machines::resources::KernelIntegration>,
}

impl std::default::Default for AppContainerLaunchType {
    fn default() -> Self {
        AppContainerLaunchType::Default
    }
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub enum AppContainerLaunchType {
    /// Use None or global setting.
    Default,

    /// Launch VMWP normally.
    None,

    /// Launch VMWP as an App Container.
    AppContainer,
}

#[derive(Default, Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct LaunchOptions {
    #[serde(default, rename = "Type", skip_serializing_if = "is_default")]
    pub launch_type: AppContainerLaunchType,
}

impl std::default::Default for ProcessDumpType {
    fn default() -> Self {
        ProcessDumpType::None
    }
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub enum ProcessDumpType {
    None,
    Heap,
    Mini,
    Custom,
}

/// Configuration for a process dump
#[derive(Default, Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct ProcessDump {
    #[serde(default, rename = "Type", skip_serializing_if = "is_default")]
    pub dump_type: ProcessDumpType,

    /// Custom MINIDUMP_TYPE flags used if Type is ProcessDumpType::Custom
    #[serde(
        default,
        rename = "CustomDumpFlags",
        skip_serializing_if = "is_default"
    )]
    pub custom_dump_flags: u32,

    /// Path to create the dump file. The file must not exists.
    #[serde(rename = "DumpFileName")]
    pub dump_filename: String,
}

#[derive(Default, Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct DebugOptions {
    /// Capture a save state to the given file if the guest crashes.
    #[serde(
        default,
        rename = "BugcheckSavedStateFileName",
        skip_serializing_if = "is_default"
    )]
    pub bugcheck_saved_state_file_name: String,
}

#[derive(Default, Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct GuestState {
    /// The path to an existing file uses for persistent guest state storage.
    /// An empty string indicates the system should initialize new transient, in-memory guest state.
    #[serde(
        default,
        rename = "GuestStateFilePath",
        skip_serializing_if = "is_default"
    )]
    pub guest_state_file_path: String,

    /// The path to an existing file for persistent runtime state storage.
    /// An empty string indicates the system should initialize new transient, in-memory runtime state.
    #[serde(
        default,
        rename = "RuntimeStateFilePath",
        skip_serializing_if = "is_default"
    )]
    pub runtime_state_file_path: String,

    /// If true, the guest state and runtime state files will be used as templates
    /// to populate transient, in-memory state instead of using the files as persistent backing store.
    #[serde(
        default,
        rename = "ForceTransientState",
        skip_serializing_if = "is_default"
    )]
    pub force_transient_state: bool,
}

#[derive(Default, Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct RestoreState {
    /// The path to the save state file to restore the system from.
    #[serde(
        default,
        rename = "SaveStateFilePath",
        skip_serializing_if = "is_default"
    )]
    pub save_state_file_path: String,

    /// The ID of the template system to clone this new system off of. An empty
    /// string indicates the system should not be cloned from a template.
    #[serde(
        default,
        rename = "TemplateSystemId",
        skip_serializing_if = "is_default"
    )]
    pub template_system_id: String,
}

#[derive(Default, Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct GuestConnection {
    /// Use Vsock rather than Hyper-V sockets to communicate with the guest service.
    #[serde(default, rename = "UseVsock", skip_serializing_if = "is_default")]
    pub use_vsock: bool,

    /// Don't disconnect the guest connection when pausing the virtual machine.
    #[serde(
        default,
        rename = "UseConnectedSuspend",
        skip_serializing_if = "is_default"
    )]
    pub use_connected_suspend: bool,
}
