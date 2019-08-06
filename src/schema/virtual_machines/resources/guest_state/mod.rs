// Copyright  rafawo (rafawo1@hotmail.com). All rights reserved.
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.
// All files in the project carrying such notice may not be copied, modified, or distributed
// except according to those terms.
// THE SOURCE CODE IS AVAILABLE UNDER THE ABOVE CHOSEN LICENSE "AS IS", WITH NO WARRANTIES.

use crate::schema;
use serde::{Deserialize, Serialize};

#[derive(Default, Deserialize, Serialize, Debug, Clone)]
pub struct NvramVariable {
    #[serde(rename = "Attributes")]
    pub attributes: u32,

    #[serde(default, rename = "Timestamp")]
    pub timestamp: Option<chrono::DateTime<chrono::Utc>>,

    #[serde(rename = "Data")]
    pub data: Vec<u8>,
}

#[derive(Default, Deserialize, Serialize, Debug, Clone)]
pub struct NvramVendor {
    #[serde(rename = "Variables")]
    pub variables: std::collections::HashMap<String, NvramVariable>,
}

#[derive(Default, Deserialize, Serialize, Debug, Clone)]
pub struct Nvram {
    #[serde(rename = "Vendors")]
    pub vendors: std::collections::HashMap<schema::utils::GuidSerde, NvramVendor>,

    #[serde(default, rename = "LastUpdateTime")]
    pub last_update_time: Option<chrono::DateTime<chrono::Utc>>,
}

#[derive(Default, Deserialize, Serialize, Debug, Clone)]
pub struct Device {
    #[serde(rename = "Version")]
    pub version: schema::Version,

    #[serde(rename = "Type")]
    pub device_type: schema::utils::GuidSerde,

    #[serde(rename = "States")]
    pub states: std::collections::HashMap<String, serde_json::Value>,
}

#[derive(Default, Deserialize, Serialize, Debug, Clone)]
pub struct RuntimeState {
    #[serde(rename = "Version")]
    pub version: schema::Version,

    #[serde(rename = "Devices")]
    pub devices: std::collections::HashMap<schema::utils::GuidSerde, Device>,
}
