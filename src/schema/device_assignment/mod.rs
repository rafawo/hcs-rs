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
pub enum DeviceType {
    ClassGuid,
    DeviceInstance,
    /// Make all GPUs on the host visible to the container.
    GpuMirror,
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq, Hash)]
pub struct Device {
    /// The type of device to assign to the container.
    #[serde(rename = "Type")]
    pub device_type: DeviceType,

    /// The interface class guid of the device interfaces to assign to the container.
    /// Only used when Type is ClassGuid.
    #[serde(rename = "InterfaceClassGuid")]
    pub interface_class_guid: schema::GuidSerde,

    /// The location path of the device to assign to the container.
    /// Only used when Type is DeviceInstance.
    #[serde(rename = "LocationPath")]
    pub location_path: String,
}
