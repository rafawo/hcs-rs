// Copyright  rafawo (rafawo1@hotmail.com). All rights reserved.
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.
// All files in the project carrying such notice may not be copied, modified, or distributed
// except according to those terms.
// THE SOURCE CODE IS AVAILABLE UNDER THE ABOVE CHOSEN LICENSE "AS IS", WITH NO WARRANTIES.

use crate::schema;
use serde::{Deserialize, Serialize};

/// Specifies CPU limits for a container.
/// Count, Maximum and Weight are all mutually exclusive.
#[derive(Default, Deserialize, Serialize, Debug, Clone, PartialEq, Eq, Hash)]
pub struct Processor {
    /// Optional property that represents the fraction of the configured processor
    /// count in a container in relation to the processors available in the host.
    /// The fraction ultimately determines the portion of processor cycles that the
    /// threads in a container can use during each scheduling interval,
    /// as the number of cycles per 10,000 cycles.
    #[serde(default, rename = "Count", skip_serializing_if = "Option::is_none")]
    pub count: Option<u32>,

    // Optional property that limits the share of processor time given to the container
    // relative to other workloads on the processor.
    // The processor weight is a value between 0 and 10000.
    #[serde(default, rename = "Weight", skip_serializing_if = "Option::is_none")]
    pub weight: Option<u64>,

    // Optional property that determines the portion of processor cycles
    // that the threads in a container can use during each scheduling interval,
    // as the number of cycles per 10,000 cycles.
    // Set processor maximum to a percentage times 100.
    #[serde(default, rename = "Maximum", skip_serializing_if = "Option::is_none")]
    maximum: Option<u64>,
}

#[derive(Default, Deserialize, Serialize, Debug, Clone, PartialEq, Eq, Hash)]
pub struct Memory {
    #[serde(rename = "SizeInMB")]
    pub size_in_mb: u64,
}

#[derive(Default, Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
pub struct HvSocket {
    #[serde(default, rename = "Config", skip_serializing_if = "Option::is_none")]
    pub config: Option<schema::hvsocket::HvSocketSystemConfig>,

    #[serde(default, rename = "EnablePowerShellDirect")]
    pub enable_powershell_direct: bool,
}

#[derive(Default, Deserialize, Serialize, Debug, Clone, PartialEq, Eq, Hash)]
pub struct Networking {
    #[serde(default, rename = "AllowUnqualifiedDnsQuery")]
    pub allow_unqualified_dns_query: bool,

    #[serde(
        default,
        rename = "DnsSearchList",
        skip_serializing_if = "String::is_empty"
    )]
    pub dns_search_list: String,

    #[serde(
        default,
        rename = "NetworkSharedContainerName",
        skip_serializing_if = "String::is_empty"
    )]
    pub network_shared_container_name: String,

    /// Guid in windows, string in linux
    #[serde(
        default,
        rename = "Namespace",
        skip_serializing_if = "String::is_empty"
    )]
    pub namespace: String,

    #[serde(default, rename = "NetworkAdapters"m skip_serializing_if = "Vec::is_empty")]
    pub network_adapters: Vec<schema::utils::GuidSerde>,
}

/// This class is used by a modify request to add or remove a combined layers
/// structure in the guest.
/// For windows, the GCS applies a filter in ContainerRootPath
/// using the specified layers as the parent content. Ignores property ScratchPath
/// since the container path is already the scratch path.
/// For linux, the GCS unions the specified layers and ScratchPath together, placing
/// the resulting union filesystem at ContainerRootPath.
#[derive(Default, Deserialize, Serialize, Debug, Clone, PartialEq, Eq, Hash)]
pub struct CombinedLayers {
    #[serde(default, rename = "Layers", skip_serializing_if = "Vec::is_empty")]
    pub layers: Vec<schema::common::resources::Layer>,

    #[serde(
        default,
        rename = "ScratchPath",
        skip_serializing_if = "String::is_empty"
    )]
    pub scratch_path: String,

    #[serde(rename = "ContainerRootPath")]
    pub container_root_path: String,
}

#[derive(Default, Deserialize, Serialize, Debug, Clone, PartialEq, Eq, Hash)]
pub struct Storage {
    /// List of layers that describe the parent hierarchy for a container's
    /// storage. These layers combined together, presented as a disposable
    /// and/or committable working storage, are used by the container to
    /// record all changes done to the parent layers.
    #[serde(rename = "Layers", skip_serializing_if = "Vec::is_empty")]
    pub layers: Vec<schema::common::resources::Layer>,

    /// Path that points to the scratch space of a container, where parent
    /// layers are combined together to present a new disposable and/or committable
    /// layer with the changes done during its runtime.
    #[serde(rename = "Path")]
    pub path: String,

    #[serde(default, rename = "QoS", skip_serializing_if = "Option::is_none")]
    pub qos: Option<schema::common::resources::StorageQoS>,
}

#[derive(Default, Deserialize, Serialize, Debug, Clone, PartialEq, Eq, Hash)]
pub struct MappedDirectory {
    #[serde(rename = "HostPath")]
    pub host_path: String,

    #[serde(rename = "HostPathType")]
    pub host_path_type: schema::common::resources::PathType,

    #[serde(rename = "ContainerPath")]
    pub container_path: String,

    #[serde(default, rename = "ReadOnly")]
    pub read_only: bool,

    #[serde(default, rename = "SupportCloudFiles")]
    pub support_cloud_files: bool,
}

impl std::default::Default for MappedPipePathType {
    fn default() -> Self {
        MappedPipePathType::AbsolutePath
    }
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq, Hash)]
pub enum MappedPipePathType {
    AbsolutePath,
    VirtualSmbPipeName,
}

#[derive(Default, Deserialize, Serialize, Debug, Clone, PartialEq, Eq, Hash)]
pub struct MappedPipe {
    #[serde(rename = "ContainerPipeName")]
    pub container_pipe_name: String,

    #[serde(rename = "HostPath")]
    pub host_path: String,

    #[serde(rename = "HostPathType")]
    pub host_path_type: MappedPipePathType,
}

/// Represents the flush state of the registry hive
/// for a windows container's job object.
#[derive(Default, Deserialize, Serialize, Debug, Clone, PartialEq, Eq, Hash)]
pub struct RegistryFlushState {
    /// Determines whether the flush state of the registry hive
    /// is enabled or not.
    /// When not enabled, flushes are ignored and changes to the
    /// registry are not preserved.
    #[serde(rename = "Enabled")]
    enabled: bool,
}
