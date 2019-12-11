// Copyright © rafawo (rafawo1@hotmail.com). All rights reserved.
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.
// All files in the project carrying such notice may not be copied, modified, or distributed
// except according to those terms.
// THE SOURCE CODE IS AVAILABLE UNDER THE ABOVE CHOSEN LICENSE "AS IS", WITH NO WARRANTIES.

//! Contains all the JSON schema definitions used by the HCS APIs

// TODO:rafawo Add conditional compilation to portions of the schema depending the release.

pub mod common;
pub mod containers;
pub mod device_assignment;
pub mod hvsocket;
pub mod layer_management;
pub mod options;
pub mod process;
pub mod registry;
pub mod requests;
pub mod responses;
pub mod utils;
pub mod virtual_machines;

use crate::schema::utils::is_default;
use serde::{Deserialize, Serialize};

#[derive(Default, Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct Version {
    #[serde(rename = "Major")]
    pub major: u32,
    #[serde(rename = "Minor")]
    pub minor: u32,
}

#[derive(Default, Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct GuestOs {
    #[serde(default, rename = "HostName", skip_serializing_if = "is_default")]
    pub hostname: String,
}

#[derive(Default, Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct Container {
    #[serde(default, rename = "GuestOs", skip_serializing_if = "is_default")]
    pub guest_os: GuestOs,

    #[serde(default, rename = "Storage", skip_serializing_if = "is_default")]
    pub storage: containers::resources::Storage,

    #[serde(
        default,
        rename = "MappedDirectories",
        skip_serializing_if = "is_default"
    )]
    pub mapped_directories: Vec<containers::resources::MappedDirectory>,

    #[serde(default, rename = "MappedPipes", skip_serializing_if = "is_default")]
    pub mapped_pipes: Vec<containers::resources::MappedPipe>,

    #[serde(default, rename = "Memory", skip_serializing_if = "is_default")]
    pub memory: Option<containers::resources::Memory>,

    #[serde(default, rename = "Processor", skip_serializing_if = "is_default")]
    pub processor: Option<containers::resources::Processor>,

    #[serde(default, rename = "Networking", skip_serializing_if = "is_default")]
    pub networking: containers::resources::Networking,

    #[serde(default, rename = "HvSocket", skip_serializing_if = "is_default")]
    pub hvsocket: containers::resources::HvSocket,

    #[serde(
        default,
        rename = "ContainerCredentialGuard",
        skip_serializing_if = "is_default"
    )]
    pub container_credential_guard: Option<containers::credential_guard::CcgState>,

    #[serde(
        default,
        rename = "RegistryChanges",
        skip_serializing_if = "is_default"
    )]
    pub registry_changes: registry::RegistryChanges,

    #[serde(
        default,
        rename = "AssignedDevices",
        skip_serializing_if = "is_default"
    )]
    pub assigned_devices: device_assignment::Device,
}

#[derive(Default, Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct VirtualMachine {
    #[serde(default, rename = "StopOnReset", skip_serializing_if = "is_default")]
    pub stop_on_reset: bool,

    #[serde(rename = "Chipset")]
    pub chipset: virtual_machines::resources::Chipset,

    #[serde(rename = "ComputeTopology")]
    pub compute_topology: virtual_machines::resources::compute::Topology,

    #[serde(default, rename = "Devices", skip_serializing_if = "is_default")]
    pub devices: virtual_machines::Devices,

    #[serde(default, rename = "GuestState", skip_serializing_if = "is_default")]
    pub guest_state: Option<virtual_machines::GuestState>,

    #[serde(default, rename = "RestoreState", skip_serializing_if = "is_default")]
    pub restore_state: Option<virtual_machines::RestoreState>,

    #[serde(
        default,
        rename = "RegistryChanges",
        skip_serializing_if = "is_default"
    )]
    pub registry_changes: registry::RegistryChanges,

    #[serde(default, rename = "StorageQoS", skip_serializing_if = "is_default")]
    pub storage_qos: Option<common::resources::StorageQoS>,

    #[serde(
        default,
        rename = "GuestConnection",
        skip_serializing_if = "is_default"
    )]
    pub guest_connection: Option<virtual_machines::GuestConnection>,
}

#[derive(Default, Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct ComputeSystem {
    #[serde(rename = "Owner")]
    pub owner: String,

    #[serde(rename = "SchemaVersion")]
    pub schema_version: Version,

    #[serde(
        default,
        rename = "HostingSystemId",
        skip_serializing_if = "is_default"
    )]
    pub hosting_system_id: String,

    #[serde(
        default,
        rename = "HostedSystem",
        skip_serializing_if = "serde_json::Value::is_null"
    )]
    pub hosted_system: serde_json::Value,

    #[serde(default, rename = "Container", skip_serializing_if = "is_default")]
    pub container: Option<Container>,

    #[serde(default, rename = "VirtualMachine", skip_serializing_if = "is_default")]
    pub virtual_machine: Option<VirtualMachine>,

    #[serde(
        default,
        rename = "ShouldTerminateOnLastHandleClosed",
        skip_serializing_if = "is_default"
    )]
    pub should_terminate_on_last_handle_closed: bool,
}

#[derive(Default, Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct HostedSystem {
    #[serde(rename = "SchemaVersion")]
    pub schema_version: Version,

    #[serde(rename = "Container")]
    pub container: Container,
}
