// Copyright  rafawo (rafawo1@hotmail.com). All rights reserved.
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.
// All files in the project carrying such notice may not be copied, modified, or distributed
// except according to those terms.
// THE SOURCE CODE IS AVAILABLE UNDER THE ABOVE CHOSEN LICENSE "AS IS", WITH NO WARRANTIES.

use crate::schema;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq, Hash)]
pub enum ContainerCredentialGuardTransport {
    LRPC,
    HvSocket,
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq, Hash)]
pub struct ContainerCredentialGuardState {
    #[serde(
        rename = "Cookie",
        serialize_with = "schema::buffer_to_hex",
        deserialize_with = "schema::hex_to_buffer"
    )]
    pub cookie: Vec<u8>,

    #[serde(rename = "RpcEndpoint")]
    pub rpc_endpoint: String,

    #[serde(rename = "Transport")]
    pub transport: ContainerCredentialGuardTransport,

    #[serde(rename = "CredentialSpec")]
    pub credential_spec: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
pub struct ContainerCredentialGuardHvSocketServiceConfig {
    #[serde(rename = "ServiceId")]
    pub service_id: schema::GuidSerde,

    #[serde(rename = "ServiceConfig", skip_serializing_if = "Option::is_none")]
    pub service_config: Option<schema::hvsocket::HvSocketServiceConfig>,
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
pub struct ContainerCredentialGuardInstance {
    #[serde(rename = "Id")]
    pub id: String,

    #[serde(rename = "CredentialGuard")]
    pub credential_guard: ContainerCredentialGuardState,

    #[serde(rename = "HvSocketConfig", skip_serializing_if = "Option::is_none")]
    pub hvsocket_config: Option<ContainerCredentialGuardHvSocketServiceConfig>,
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
pub struct ContainerCredentialGuardSystemInfo {
    #[serde(rename = "Instances")]
    pub instances: Vec<ContainerCredentialGuardInstance>,
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq, Hash)]
pub struct ContainerCredentialGuardAddInstanceRequest {
    #[serde(rename = "Id")]
    pub id: String,

    #[serde(rename = "CredentialSpec")]
    pub credential_spec: String,

    #[serde(rename = "Transport")]
    pub transport: ContainerCredentialGuardTransport,
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq, Hash)]
pub struct ContainerCredentialGuardRemoveInstanceRequest {
    #[serde(rename = "Id")]
    pub id: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq, Hash)]
pub enum ContainerCredentialGuardModifyOperation {
    AddInstance,
    RemoveInstance,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct ContainerCredentialGuardOperationRequest {
    #[serde(rename = "Operation")]
    pub operation: ContainerCredentialGuardModifyOperation,

    #[serde(
        rename = "OperationDetails",
        skip_serializing_if = "serde_json::Value::is_null"
    )]
    pub operation_details: serde_json::Value,
}
