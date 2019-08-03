// Copyright  rafawo (rafawo1@hotmail.com). All rights reserved.
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.
// All files in the project carrying such notice may not be copied, modified, or distributed
// except according to those terms.
// THE SOURCE CODE IS AVAILABLE UNDER THE ABOVE CHOSEN LICENSE "AS IS", WITH NO WARRANTIES.

use crate::schema;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone)]
pub enum State {
    Created,
    Running,
    Paused,
    Stopped,
    SavedAsTemplate,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub enum OsType {
    Windows,
    Linux,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub enum SystemType {
    Container,
    VirtualMachine,
    Host,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub enum NotificationType {
    None,
    GracefulExit,
    ForcedExit,
    UnexpectedExit,
    Unknown,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct VirtualNodeInfo {
    #[serde(rename = "VirtualNodeIndex")]
    pub virtual_node_index: u8,

    #[serde(rename = "PhysicalNodeNumber")]
    pub physical_node_number: u8,

    #[serde(rename = "VirtualProcessorCount")]
    pub virtual_processor_count: u32,

    #[serde(rename = "MemoryUsageInPages")]
    pub memory_usage_in_pages: u64,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct VmMemory {
    #[serde(rename = "AvailableMemory")]
    pub available_memory: i32,

    #[serde(rename = "AvailableMemoryBuffer")]
    pub available_memory_buffer: i32,

    #[serde(rename = "ReservedMemory")]
    pub reserved_memory: u64,

    #[serde(rename = "AssignedMemory")]
    pub assigned_memory: u64,

    #[serde(rename = "SlpActive")]
    pub slp_active: bool,

    #[serde(rename = "BalancingEnabled")]
    pub balancing_enabled: bool,

    #[serde(rename = "DmOperationInProgress")]
    pub dm_operation_in_progress: bool,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct MemoryInformationForVm {
    #[serde(rename = "VirtualNodeCount")]
    pub virtual_node_count: u8,

    #[serde(rename = "VirtualMachineMemory")]
    pub virtual_machine_memory: VmMemory,

    #[serde(rename = "VirtualNodes")]
    pub virtual_nodes: Vec<VirtualNodeInfo>,
}

// Memory usage as viewed from the guest OS.
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct GuestMemoryInfo {
    #[serde(rename = "TotalPhysicalBytes")]
    pub total_physical_bytes: u64,

    #[serde(rename = "TotalUsage")]
    pub total_usage: u64,

    #[serde(rename = "CommittedBytes")]
    pub committed_bytes: u64,

    #[serde(rename = "SharedCommittedBytes")]
    pub shared_committed_bytes: u64,

    #[serde(rename = "CommitLimitBytes")]
    pub commit_limit_bytes: u64,

    #[serde(rename = "PeakCommitmentBytes")]
    pub peak_commitment_bytes: u64,
}

// CPU runtime statistics
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct ProcessorStats {
    #[serde(rename = "TotalRuntime100ns")]
    pub total_runtime100ns: u64,

    #[serde(rename = "RuntimeUser100ns")]
    pub runtime_user100ns: u64,

    #[serde(rename = "RuntimeKernel100ns")]
    pub runtime_kernel100ns: u64,
}

// Memory runtime statistics
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct MemoryStats {
    #[serde(rename = "MemoryUsageCommitBytes")]
    pub memory_usage_commit_bytes: u64,

    #[serde(rename = "MemoryUsageCommitPeakBytes")]
    pub memory_usage_commit_peak_bytes: u64,

    #[serde(rename = "MemoryUsagePrivateWorkingSetBytes")]
    pub memory_usage_private_working_set_bytes: u64,
}

// Storage runtime statistics
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct StorageStats {
    #[serde(rename = "ReadCountNormalized")]
    pub read_count_normalized: u64,

    #[serde(rename = "ReadSizeBytes")]
    pub read_size_bytes: u64,

    #[serde(rename = "WriteCountNormalized")]
    pub write_count_normalized: u64,

    #[serde(rename = "WriteSizeBytes")]
    pub write_size_bytes: u64,
}

// Runtime statistics for a container
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Statistics {
    #[serde(rename = "Timestamp")]
    pub timestamp: chrono::DateTime<chrono::Utc>,

    #[serde(rename = "ContainerStartTime")]
    pub container_start_time: chrono::DateTime<chrono::Utc>,

    #[serde(rename = "Uptime100ns")]
    pub uptime100ns: u64,

    #[serde(rename = "Processor")]
    pub processor: ProcessorStats,

    #[serde(rename = "Memory")]
    pub memory: MemoryStats,

    #[serde(rename = "Storage")]
    pub storage: StorageStats,
}

// Information about a process running in a container
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct ProcessDetails {
    #[serde(rename = "ProcessId")]
    pub process_id: u32,

    #[serde(rename = "ImageName")]
    pub image_name: String,

    #[serde(rename = "CreateTimestamp")]
    pub create_timestamp: chrono::DateTime<chrono::Utc>,

    #[serde(rename = "UserTime100ns")]
    pub user_time100ns: u64,

    #[serde(rename = "KernelTime100ns")]
    pub kernel_time100ns: u64,

    #[serde(rename = "MemoryCommitBytes")]
    pub memory_commit_bytes: u64,

    #[serde(rename = "MemoryWorkingSetPrivateBytes")]
    pub memory_working_set_private_bytes: u64,

    #[serde(rename = "MemoryWorkingSetSharedBytes")]
    pub memory_working_set_shared_bytes: u64,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct SharedMemoryRegionInfo {
    #[serde(rename = "SectionName")]
    pub section_name: String,

    #[serde(rename = "GuestPhysicalAddress")]
    pub guest_physical_address: u64,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct GuestConnectionInfo {
    /// Each schema version x.y stands for the range of versions a.b where a==x
    /// and b<=y. This list comes from the SupportedSchemaVersions field in GcsCapabilities.
    #[serde(rename = "SupportedSchemaVersions")]
    pub supported_schema_versions: Vec<schema::Version>,

    #[serde(rename = "ProtocolVersion")]
    pub protocol_version: u32,

    #[serde(
        rename = "GuestDefinedCapabilities",
        skip_serializing_if = "serde_json::Value::is_null"
    )]
    pub guest_defined_capabilities: serde_json::Value,
}

// Silo job information
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct SiloProperties {
    #[serde(rename = "Enabled")]
    pub enabled: bool,

    #[serde(rename = "JobName")]
    pub job_name: String,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct CacheQueryStatsResponse {
    #[serde(rename = "L3OccupancyBytes")]
    pub l3_occupancy_bytes: u64,

    #[serde(rename = "L3TotalBwBytes")]
    pub l3_total_bw_bytes: u64,

    #[serde(rename = "L3LocalBwBytes")]
    pub l3_local_bw_bytes: u64,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Properties {
    #[serde(rename = "Id")]
    pub id: String,

    #[serde(rename = "SystemType")]
    pub system_type: SystemType,

    #[serde(rename = "RuntimeOsType")]
    pub runtime_os_type: OsType,

    #[serde(rename = "Name")]
    pub name: String,

    #[serde(rename = "Owner")]
    pub owner: String,

    #[serde(rename = "RuntimeId")]
    pub runtime_id: schema::utils::GuidSerde,

    #[serde(rename = "IsRuntimeTemplate")]
    pub is_runtime_template: bool,

    #[serde(rename = "RuntimeTemplateId")]
    pub runtime_template_id: String,

    #[serde(rename = "State")]
    pub state: State,

    #[serde(rename = "Stopped")]
    pub stopped: bool,

    #[serde(rename = "ExitType")]
    pub exit_type: NotificationType,

    #[serde(rename = "Memory")]
    pub memory: MemoryInformationForVm,

    #[serde(rename = "Statistics")]
    pub statistics: Statistics,

    #[serde(rename = "ProcessList")]
    pub process_list: Vec<ProcessDetails>,

    #[serde(rename = "TerminateOnLastHandleClosed")]
    pub terminate_on_last_handle_closed: bool,

    #[serde(rename = "HostingSystemId")]
    pub hosting_system_id: String,

    #[serde(rename = "SharedMemoryRegionInfo")]
    pub shared_memory_region_info: Vec<SharedMemoryRegionInfo>,

    #[serde(rename = "GuestConnectionInfo")]
    pub guest_connection_info: Option<GuestConnectionInfo>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct SystemExitStatus {
    #[serde(rename = "Status")]
    pub status: i32,
    #[serde(rename = "ExitType")]
    pub exit_type: NotificationType,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct ProcessStatus {
    #[serde(rename = "ProcessId")]
    pub process_id: u32,
    #[serde(rename = "Exited")]
    pub exited: bool,

    #[serde(rename = "ExitCode")]
    pub exit_code: u32,
    #[serde(rename = "LastWaitResult")]
    pub last_wait_result: i32,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub enum WindowsCrashPhase {
    Inactive,
    CrashValues,
    Starting,
    Started,
    Writing,
    Complete,
}

// Windows specific crash information
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct WindowsCrashReport {
    #[serde(rename = "DumpFile")]
    pub dump_file: String,

    #[serde(rename = "OsMajorVersion")]
    pub os_major_version: u32,

    #[serde(rename = "OsMinorVersion")]
    pub os_minor_version: u32,

    #[serde(rename = "OsBuildNumber")]
    pub os_build_number: u32,

    #[serde(rename = "OsServicePackMajorVersion")]
    pub os_service_pack_major_version: u32,

    #[serde(rename = "OsServicePackMinorVersion")]
    pub os_service_pack_minor_version: u32,

    #[serde(rename = "OsSuiteMask")]
    pub os_suite_mask: u32,

    #[serde(rename = "OsProductType")]
    pub os_product_type: u32,

    #[serde(rename = "Status")]
    pub status: i32,

    #[serde(rename = "FinalPhase")]
    pub final_phase: WindowsCrashPhase,
}

// crash information reported through CrashReport notifications
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct CrashReport {
    #[serde(rename = "SystemId")]
    pub system_id: String,

    #[serde(rename = "ActivityId")]
    pub activity_id: schema::utils::GuidSerde,

    #[serde(rename = "WindowsCrashInfo")]
    pub windows_crash_info: Option<WindowsCrashReport>,

    #[serde(rename = "CrashParameters")]
    pub crash_parameters: Vec<u64>,

    #[serde(rename = "CrashLog")]
    pub crash_log: String,
}
