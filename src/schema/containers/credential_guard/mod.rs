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
pub enum CcgTransport {
    LRPC,
    HvSocket,
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq, Hash)]
pub struct CcgState {
    #[serde(
        rename = "Cookie",
        serialize_with = "schema::buffer_to_hex",
        deserialize_with = "schema::hex_to_buffer"
    )]
    pub cookie: Vec<u8>,

    #[serde(rename = "RpcEndpoint")]
    pub rpc_endpoint: String,

    #[serde(rename = "Transport")]
    pub transport: CcgTransport,

    #[serde(rename = "CredentialSpec")]
    pub credential_spec: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
pub struct CcgHvSocketServiceConfig {
    #[serde(rename = "ServiceId")]
    pub service_id: schema::GuidSerde,

    #[serde(rename = "ServiceConfig", skip_serializing_if = "Option::is_none")]
    pub service_config: Option<schema::hvsocket::HvSocketServiceConfig>,
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
pub struct CcgInstance {
    #[serde(rename = "Id")]
    pub id: String,

    #[serde(rename = "CredentialGuard")]
    pub credential_guard: CcgState,

    #[serde(rename = "HvSocketConfig", skip_serializing_if = "Option::is_none")]
    pub hvsocket_config: Option<CcgHvSocketServiceConfig>,
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
pub struct CcgSystemInfo {
    #[serde(rename = "Instances")]
    pub instances: Vec<CcgInstance>,
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq, Hash)]
pub struct CcgAddInstanceRequest {
    #[serde(rename = "Id")]
    pub id: String,

    #[serde(rename = "CredentialSpec")]
    pub credential_spec: String,

    #[serde(rename = "Transport")]
    pub transport: CcgTransport,
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq, Hash)]
pub struct CcgRemoveInstanceRequest {
    #[serde(rename = "Id")]
    pub id: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq, Hash)]
pub enum CcgModifyOperation {
    AddInstance,
    RemoveInstance,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct CcgOperationRequest {
    #[serde(rename = "Operation")]
    pub operation: CcgModifyOperation,

    #[serde(
        rename = "OperationDetails",
        skip_serializing_if = "serde_json::Value::is_null"
    )]
    pub operation_details: serde_json::Value,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ccg_request() {
        assert_eq!(
            &serde_json::to_string(&CcgOperationRequest {
                operation: CcgModifyOperation::RemoveInstance,
                operation_details: serde_json::json!(CcgRemoveInstanceRequest {
                    id: String::from("some ID"),
                }),
            })
            .unwrap(),
            r#"{"Operation":"RemoveInstance","OperationDetails":{"Id":"some ID"}}"#
        );
    }
}
