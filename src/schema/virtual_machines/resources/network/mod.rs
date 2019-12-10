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
pub struct NetworkAdapter {
    #[serde(rename = "EndpointId")]
    pub endpoint_id: schema::utils::GuidSerde,

    #[serde(default, rename = "MacAddress")]
    pub mac_address: Option<String>,

    #[serde(rename = "InstanceId")]
    pub instance_id: schema::utils::GuidSerde,
}
