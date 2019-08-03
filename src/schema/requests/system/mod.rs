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
pub struct ModifySettingRequest {
    #[serde(rename = "ResourcePath")]
    pub resource_path: String,

    #[serde(rename = "RequestType")]
    pub request_type: schema::requests::ModifyRequestType,

    #[serde(
        rename = "Settings",
        skip_serializing_if = "serde_json::Value::is_null"
    )]
    pub settings: serde_json::Value,

    #[serde(
        rename = "GuestRequest",
        skip_serializing_if = "serde_json::Value::is_null"
    )]
    pub guest_request: serde_json::Value,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub enum PropertyType {
    Memory,
    GuestMemory,
    Statistics,
    ProcessList,
    TerminateOnLastHandleClosed,
    SharedMemoryRegion,
    GuestConnection,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct PropertyQuery {
    #[serde(rename = "PropertyTypes", skip_serializing_if = "Vec::is_empty")]
    pub property_types: Vec<PropertyType>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct SystemQuery {
    #[serde(rename = "Ids", skip_serializing_if = "Vec::is_empty")]
    pub ids: Vec<String>,

    #[serde(rename = "Names", skip_serializing_if = "Vec::is_empty")]
    pub names: Vec<String>,

    #[serde(rename = "Types", skip_serializing_if = "Vec::is_empty")]
    pub types: Vec<schema::responses::system::SystemType>,

    #[serde(rename = "Owners", skip_serializing_if = "Vec::is_empty")]
    pub owners: Vec<String>,
}
