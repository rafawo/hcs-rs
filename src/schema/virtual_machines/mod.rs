// Copyright  rafawo (rafawo1@hotmail.com). All rights reserved.
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.
// All files in the project carrying such notice may not be copied, modified, or distributed
// except according to those terms.
// THE SOURCE CODE IS AVAILABLE UNDER THE ABOVE CHOSEN LICENSE "AS IS", WITH NO WARRANTIES.

pub mod resources;

use crate::schema;
use serde::{Deserialize, Serialize};

#[derive(Default, Deserialize, Serialize, Debug, Clone)]
pub struct Devices {
    #[serde(default, rename = "ComPorts")]
    pub com_ports: std::collections::HashMap<u32, schema::virtual_machines::resources::ComPort>,

    #[serde(
        default,
        rename = "VirtioSerial",
        skip_serializing_if = "Option::is_none"
    )]
    pub virtio_serial: Option<schema::virtual_machines::resources::VirtioSerial>,

    #[serde(default, rename = "Scsi")]
    pub scsi: std::collections::HashMap<String, schema::virtual_machines::resources::storage::Scsi>,

    #[serde(
        default,
        rename = "VirtualPMem",
        skip_serializing_if = "Option::is_none"
    )]
    pub virtual_pmem: Option<schema::virtual_machines::resources::storage::VirtualPMemController>,

    #[serde(default, rename = "NetworkAdapters")]
    pub network_adapters: std::collections::HashMap<
        String,
        schema::virtual_machines::resources::network::NetworkAdapter,
    >,

    #[serde(
        default,
        rename = "VideoMonitor",
        skip_serializing_if = "Option::is_none"
    )]
    pub video_monitor: Option<schema::virtual_machines::resources::VideoMonitor>,

    #[serde(default, rename = "Keyboard", skip_serializing_if = "Option::is_none")]
    pub keyboard: Option<schema::virtual_machines::resources::Keyboard>,

    #[serde(default, rename = "Mouse", skip_serializing_if = "Option::is_none")]
    pub mouse: Option<schema::virtual_machines::resources::Mouse>,

    #[serde(default, rename = "HvSocket", skip_serializing_if = "Option::is_none")]
    pub hvsocket: Option<schema::virtual_machines::resources::HvSocket>,

    #[serde(
        default,
        rename = "EnhancedModeVideo",
        skip_serializing_if = "Option::is_none"
    )]
    pub enhanced_mode_video: Option<schema::virtual_machines::resources::EnhancedModeVideo>,

    #[serde(
        default,
        rename = "GuestCrashReporting",
        skip_serializing_if = "Option::is_none"
    )]
    pub guest_crash_reporting: Option<schema::virtual_machines::resources::GuestCrashReporting>,

    #[serde(
        default,
        rename = "VirtualSmb",
        skip_serializing_if = "Option::is_none"
    )]
    pub virtual_smb: Option<schema::virtual_machines::resources::storage::VirtualSmb>,

    #[serde(default, rename = "Plan9", skip_serializing_if = "Option::is_none")]
    pub plan9: Option<schema::virtual_machines::resources::storage::Plan9>,

    #[serde(default, rename = "Licensing", skip_serializing_if = "Option::is_none")]
    pub licensing: Option<schema::virtual_machines::resources::Licensing>,

    #[serde(default, rename = "Battery", skip_serializing_if = "Option::is_none")]
    pub battery: Option<schema::virtual_machines::resources::Battery>,

    #[serde(default, rename = "FlexibleIov")]
    pub flexible_iov: std::collections::HashMap<
        String,
        schema::virtual_machines::resources::vpci::FlexibleIoDevice,
    >,

    #[serde(
        default,
        rename = "SharedMemory",
        skip_serializing_if = "Option::is_none"
    )]
    pub shared_memory: Option<schema::virtual_machines::resources::SharedMemoryConfiguration>,

    #[serde(
        default,
        rename = "KernelIntegration",
        skip_serializing_if = "Option::is_none"
    )]
    pub kernel_integration: Option<schema::virtual_machines::resources::KernelIntegration>,

    #[serde(default, rename = "VirtualPci")]
    pub virtual_pci: std::collections::HashMap<
        String,
        schema::virtual_machines::resources::vpci::VirtualPciDevice,
    >,
}

#[derive(Default, Deserialize, Serialize, Debug, Clone)]
pub struct SiloSettings {
    /// If running this virtual machine inside a silo, the base OS path to use for the silo.
    #[serde(default, rename = "SiloBaseOsPath")]
    pub silo_base_os_path: String,

    /// Request a notification when the job object for the silo is available.
    #[serde(default, rename = "NotifySiloJobCreated")]
    pub notify_silo_job_created: bool,

    /// The filesystem layers to use for the silo.
    #[serde(default, rename = "FileSystemLayers")]
    pub file_system_layers: Vec<schema::common::resources::Layer>,

    /// The bindings to use for the silo.
    #[serde(default, rename = "Bindings")]
    pub bindings: Vec<schema::common::resources::BatchedBinding>,
}

impl std::default::Default for AppContainerLaunchType {
    fn default() -> Self {
        AppContainerLaunchType::Default
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub enum AppContainerLaunchType {
    /// Use None or global setting.
    Default,

    /// Launch VMWP normally.
    None,

    /// Launch VMWP as an App Container.
    AppContainer,
}

#[derive(Default, Deserialize, Serialize, Debug, Clone)]
pub struct LaunchOptions {
    #[serde(default, rename = "Type")]
    pub launch_type: AppContainerLaunchType,
}

impl std::default::Default for ProcessDumpType {
    fn default() -> Self {
        ProcessDumpType::None
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub enum ProcessDumpType {
    None,
    Heap,
    Mini,
    Custom,
}

/// Configuration for a process dump
#[derive(Default, Deserialize, Serialize, Debug, Clone)]
pub struct ProcessDump {
    #[serde(default, rename = "Type")]
    pub dump_type: ProcessDumpType,

    /// Custom MINIDUMP_TYPE flags used if Type is ProcessDumpType::Custom
    #[serde(default, rename = "CustomDumpFlags")]
    pub custom_dump_flags: u32,

    /// Path to create the dump file. The file must not exists.
    #[serde(rename = "DumpFileName")]
    pub dump_filename: String,
}

#[derive(Default, Deserialize, Serialize, Debug, Clone)]
pub struct DebugOptions {
    /// Capture a save state to the given file if the guest crashes.
    #[serde(default, rename = "BugcheckSavedStateFileName")]
    pub bugcheck_saved_state_file_name: String,

    // Capture a save state to the given file if the guest crashes and no crash dump can be written.
    #[serde(default, rename = "BugcheckNoCrashdumpSavedStateFileName")]
    pub bugcheck_no_crashdump_saved_state_file_name: String,

    /// Capture a save state to the given file if the guest triple faults.
    #[serde(default, rename = "TripleFaultSavedStateFileName")]
    pub triple_fault_saved_state_file_name: String,

    /// Name of a dump file to use for firmware crashes.
    #[serde(default, rename = "FirmwareDumpFileName")]
    pub firmware_dump_file_name: String,
}

#[derive(Default, Deserialize, Serialize, Debug, Clone)]
pub struct GuestState {
    /// The path to an existing file uses for persistent guest state storage.
    /// An empty string indicates the system should initialize new transient, in-memory guest state.
    #[serde(default, rename = "GuestStateFilePath")]
    pub guest_state_file_path: String,

    /// The path to an existing file for persistent runtime state storage.
    /// An empty string indicates the system should initialize new transient, in-memory runtime state.
    #[serde(default, rename = "RuntimeStateFilePath")]
    pub runtime_state_file_path: String,

    /// If true, the guest state and runtime state files will be used as templates
    /// to populate transient, in-memory state instead of using the files as persistent backing store.
    #[serde(default, rename = "ForceTransientState")]
    pub force_transient_state: bool,
}

#[derive(Default, Deserialize, Serialize, Debug, Clone)]
pub struct RestoreState {
    /// The path to the runtime state file to start the system from.
    #[serde(default, rename = "RuntimeStateFilePath")]
    pub runtime_state_file_path: String,

    /// The path to the save state file to restore the system from.
    #[serde(default, rename = "SaveStateFilePath")]
    pub save_state_file_path: String,

    /// The ID of the template system to clone this new system off of. An empty
    /// string indicates the system should not be cloned from a template.
    #[serde(default, rename = "TemplateSystemId")]
    pub template_system_id: String,
}

#[derive(Default, Deserialize, Serialize, Debug, Clone)]
pub struct GuestConnection {
    /// Use Vsock rather than Hyper-V sockets to communicate with the guest service.
    #[serde(default, rename = "UseVsock")]
    pub use_vsock: bool,

    /// Don't disconnect the guest connection when pausing the virtual machine.
    #[serde(default, rename = "UseConnectedSuspend")]
    pub use_connected_suspend: bool,

    /// Set the guest's time zone to that of the host
    #[serde(default, rename = "UseHostTimeZone")]
    pub use_host_time_zone: bool,
}
