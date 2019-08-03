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
pub struct ServiceProperties {
    #[serde(default, rename = "Properties", skip_serializing_if = "Vec::is_empty")]
    pub properties: Vec<serde_json::Value>,
}

#[derive(Default, Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
pub struct BasicInformation {
    #[serde(
        default,
        rename = "SupportedSchemaVersions",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub supported_schema_versions: Vec<schema::Version>,
}

#[derive(Default, Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
pub struct QoSCapabilities {
    #[serde(default, rename = "ProcessorQoSSupported")]
    pub processor_qo_s_supported: bool,
}

impl std::default::Default for EventDataType {
    fn default() -> Self {
        EventDataType::Empty
    }
}

// Data types for event data elements, based on EVT_VARIANT_TYPE
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
pub enum EventDataType {
    Empty,
    String,
    AnsiString,
    SByte,
    Byte,
    Int16,
    UInt16,
    Int32,
    UInt32,
    Int64,
    UInt64,
    Single,
    Double,
    Boolean,
    Binary,
    Guid,
}

// Event data element
#[derive(Default, Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
pub struct EventData {
    #[serde(rename = "Type")]
    pub data_type: EventDataType,

    #[serde(rename = "Value")]
    pub value: String,
}

// Error descriptor that provides the info of an error object
#[derive(Default, Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
pub struct ErrorEvent {
    #[serde(rename = "Message")]
    pub message: String,

    #[serde(default, rename = "StackTrace")]
    pub stack_trace: String,

    #[serde(rename = "Provider")]
    pub provider: schema::utils::GuidSerde,

    #[serde(rename = "EventId")]
    pub event_id: u16,

    #[serde(default, rename = "Flags")]
    pub flags: u32,

    #[serde(default, rename = "Source")]
    pub source: String,

    #[serde(default, rename = "Data", skip_serializing_if = "Vec::is_empty")]
    pub data: Vec<EventData>,
}

// Extended error information returned by the HCS
#[derive(Default, Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
pub struct ResultError {
    #[serde(rename = "Error")]
    pub error: i32,

    #[serde(rename = "ErrorMessage")]
    pub error_message: String,

    #[serde(default, rename = "ErrorEvents", skip_serializing_if = "Vec::is_empty")]
    pub error_events: Vec<ErrorEvent>,
}
