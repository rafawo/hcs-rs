// Copyright  rafawo (rafawo1@hotmail.com). All rights reserved.
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.
// All files in the project carrying such notice may not be copied, modified, or distributed
// except according to those terms.
// THE SOURCE CODE IS AVAILABLE UNDER THE ABOVE CHOSEN LICENSE "AS IS", WITH NO WARRANTIES.

use crate::schema;
use crate::schema::utils::is_default;
use serde::{Deserialize, Serialize};

impl std::default::Default for CcgTransport {
    fn default() -> Self {
        CcgTransport::LRPC
    }
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub enum CcgTransport {
    LRPC,
    HvSocket,
}

#[derive(Default, Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct CcgState {
    #[serde(
        rename = "Cookie",
        serialize_with = "schema::utils::buffer_to_hex",
        deserialize_with = "schema::utils::hex_to_buffer"
    )]
    pub cookie: Vec<u8>,

    #[serde(rename = "RpcEndpoint")]
    pub rpc_endpoint: String,

    #[serde(rename = "Transport")]
    pub transport: CcgTransport,

    #[serde(rename = "CredentialSpec")]
    pub credential_spec: String,
}

#[derive(Default, Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct CcgHvSocketServiceConfig {
    #[serde(rename = "ServiceId")]
    pub service_id: schema::utils::GuidSerde,

    #[serde(default, rename = "ServiceConfig", skip_serializing_if = "is_default")]
    pub service_config: Option<schema::hvsocket::HvSocketServiceConfig>,
}

#[derive(Default, Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct CcgInstance {
    #[serde(rename = "Id")]
    pub id: String,

    #[serde(rename = "CredentialGuard")]
    pub credential_guard: CcgState,

    #[serde(default, rename = "HvSocketConfig", skip_serializing_if = "is_default")]
    pub hvsocket_config: Option<CcgHvSocketServiceConfig>,
}

#[derive(Default, Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct CcgSystemInfo {
    #[serde(rename = "Instances")]
    pub instances: Vec<CcgInstance>,
}

#[derive(Default, Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct CcgAddInstanceRequest {
    #[serde(rename = "Id")]
    pub id: String,

    #[serde(rename = "CredentialSpec")]
    pub credential_spec: String,

    #[serde(rename = "Transport")]
    pub transport: CcgTransport,
}

#[derive(Default, Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct CcgRemoveInstanceRequest {
    #[serde(rename = "Id")]
    pub id: String,
}

impl std::default::Default for CcgModifyOperationType {
    fn default() -> Self {
        CcgModifyOperationType::AddInstance
    }
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub enum CcgModifyOperationType {
    AddInstance,
    RemoveInstance,
}

impl std::default::Default for CcgModifyOperation {
    fn default() -> Self {
        CcgModifyOperation::AddInstance(CcgAddInstanceRequest::default())
    }
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(untagged)]
pub enum CcgModifyOperation {
    AddInstance(CcgAddInstanceRequest),
    RemoveInstance(CcgRemoveInstanceRequest),
}

#[derive(Default, Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct CcgOperationRequest {
    #[serde(rename = "Operation")]
    pub operation: CcgModifyOperationType,

    #[serde(rename = "OperationDetails")]
    pub operation_details: CcgModifyOperation,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ccg_state() {
        assert_eq!(
            &serde_json::to_string(&CcgState {
                cookie: vec![
                    0x01, 0x00, 0x00, 0x00, 0x5B, 0x00, 0x0A, 0x00, 0x8A, 0xEC, 0xE8, 0x0E, 0x37,
                    0x30, 0xCA, 0x44, 0x84, 0x05, 0xFC, 0x42, 0x56, 0x7C, 0xED, 0xC4
                ],
                rpc_endpoint: String::from("rpc_endpoint"),
                transport: CcgTransport::LRPC,
                credential_spec: String::from("some_spec"),
            })
            .unwrap(),
            r#"{"Cookie":"010000005b000a008aece80e3730ca448405fc42567cedc4","RpcEndpoint":"rpc_endpoint","Transport":"LRPC","CredentialSpec":"some_spec"}"#
        )
    }

    #[test]
    fn ccg_request() {
        assert_eq!(
            &serde_json::to_string(&CcgOperationRequest {
                operation: CcgModifyOperationType::RemoveInstance,
                operation_details: CcgModifyOperation::RemoveInstance(CcgRemoveInstanceRequest {
                    id: String::from("some ID"),
                }),
            })
            .unwrap(),
            r#"{"Operation":"RemoveInstance","OperationDetails":{"Id":"some ID"}}"#
        );
    }
}
