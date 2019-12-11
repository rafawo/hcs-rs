// Copyright  rafawo (rafawo1@hotmail.com). All rights reserved.
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.
// All files in the project carrying such notice may not be copied, modified, or distributed
// except according to those terms.
// THE SOURCE CODE IS AVAILABLE UNDER THE ABOVE CHOSEN LICENSE "AS IS", WITH NO WARRANTIES.

use crate::schema;
use serde::{Deserialize, Serialize};

impl std::default::Default for ModifyResourceType {
    fn default() -> Self {
        ModifyResourceType::Memory
    }
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub enum ModifyResourceType {
    Memory,
    MappedDirectory,
    MappedPipe,
    MappedVirtualDisk,
    CombinedLayers,
    NetworkNamespace,
}

#[derive(Default, Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct GuestModifySettingRequest {
    #[serde(rename = "ResourceType")]
    pub resource_type: ModifyResourceType,

    #[serde(rename = "RequestType")]
    pub request_type: schema::requests::ModifyRequestType,

    #[serde(
        default,
        rename = "Settings",
        skip_serializing_if = "serde_json::Value::is_null"
    )]
    pub settings: serde_json::Value,
}

impl std::default::Default for NetworkModifyRequestType {
    fn default() -> Self {
        NetworkModifyRequestType::PreAdd
    }
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub enum NetworkModifyRequestType {
    PreAdd,
    Add,
    Remove,
}

impl std::default::Default for NetworkAdapterId {
    fn default() -> Self {
        NetworkAdapterId::AdapterId(String::new())
    }
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub enum NetworkAdapterId {
    AdapterId(String),
    /// Used for RS4 guests
    AdapterInstanceID(schema::utils::GuidSerde),
}

#[derive(Default, Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct NetworkModifySettingRequest {
    #[serde(rename = "RequestType")]
    pub request_type: NetworkModifyRequestType,

    #[serde(default, flatten)]
    pub adapter_id: NetworkAdapterId,

    #[serde(
        default,
        rename = "Settings",
        skip_serializing_if = "serde_json::Value::is_null"
    )]
    pub settings: serde_json::Value,
}
