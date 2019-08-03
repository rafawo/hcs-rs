// Copyright  rafawo (rafawo1@hotmail.com). All rights reserved.
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.
// All files in the project carrying such notice may not be copied, modified, or distributed
// except according to those terms.
// THE SOURCE CODE IS AVAILABLE UNDER THE ABOVE CHOSEN LICENSE "AS IS", WITH NO WARRANTIES.

use crate::schema;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
pub enum ModifyResourceType {
    Memory,
    MappedDirectory,
    MappedPipe,
    MappedVirtualDisk,
    CombinedLayers,
    NetworkNamespace,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct GuestModifySettingRequest {
    #[serde(rename = "ResourceType")]
    pub resource_type: ModifyResourceType,

    #[serde(rename = "RequestType")]
    pub request_type: schema::requests::ModifyRequestType,

    #[serde(
        rename = "Settings",
        skip_serializing_if = "serde_json::Value::is_null"
    )]
    pub settings: serde_json::Value,
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
pub enum NetworkModifyRequestType {
    PreAdd,
    Add,
    Remove,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub enum NetworkAdapterId {
    AdapterId(String),
    /// Used for RS4 guests
    AdapterInstanceID(schema::utils::GuidSerde),
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct NetworkModifySettingRequest {
    #[serde(rename = "RequestType")]
    pub request_type: NetworkModifyRequestType,

    #[serde(flatten)]
    pub adapter_id: NetworkAdapterId,

    #[serde(
        rename = "Settings",
        skip_serializing_if = "serde_json::Value::is_null"
    )]
    pub settings: serde_json::Value,
}
