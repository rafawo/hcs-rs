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

#[derive(Default, Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct LayerData {
    #[serde(default, rename = "SchemaVersion", skip_serializing_if = "is_default")]
    pub schema_version: schema::Version,

    #[serde(default, rename = "Layers", skip_serializing_if = "is_default")]
    pub layers: Vec<schema::common::resources::Layer>,
}
