// Copyright  rafawo (rafawo1@hotmail.com). All rights reserved.
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.
// All files in the project carrying such notice may not be copied, modified, or distributed
// except according to those terms.
// THE SOURCE CODE IS AVAILABLE UNDER THE ABOVE CHOSEN LICENSE "AS IS", WITH NO WARRANTIES.

use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
pub enum PropertyType {
    Basic,
    Memory,
    CpuGroup,
    ProcessorTopology,
    CacheAllocation,
    CacheMonitoring,
    ContainerCredentialGuard,
    QoSCapabilities,
    MemoryBwAllocation,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct FilteredPropertyQuery {
    #[serde(rename = "PropertyType")]
    pub property_type: PropertyType,

    #[serde(rename = "Filter", skip_serializing_if = "serde_json::Value::is_null")]
    pub filter: serde_json::Value,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct PropertyQuery {
    #[serde(rename = "PropertyTypes")]
    pub property_types: Vec<PropertyType>,

    #[serde(rename = "FilteredQueries", skip_serializing_if = "Vec::is_empty")]
    pub filtered_queries: Vec<FilteredPropertyQuery>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct ModificationRequest {
    #[serde(rename = "PropertyType")]
    pub property_type: PropertyType,

    #[serde(
        rename = "Settings",
        skip_serializing_if = "serde_json::Value::is_null"
    )]
    pub settings: serde_json::Value,
}
