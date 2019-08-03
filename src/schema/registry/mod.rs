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
pub enum RegistryHive {
    System,
    Software,
    Security,
    Sam,
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
pub enum RegistryValueType {
    None,
    String,
    ExpandedString,
    MultiString,
    Binary,
    DWord,
    QWord,
    CustomType,
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
pub enum RegistryValueData {
    StringValue(String),
    #[serde(
        serialize_with = "schema::utils::as_base64",
        deserialize_with = "schema::utils::from_base64"
    )]
    BinaryValue(Vec<u8>),
    DWordValue(u32),
    QWordValue(u64),
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
pub struct RegistryKey {
    #[serde(rename = "Hive")]
    pub hive: RegistryHive,

    #[serde(rename = "Name")]
    pub name: String,

    #[serde(rename = "Volatile")]
    pub volatile: bool,
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
pub struct RegistryValue {
    #[serde(rename = "Key")]
    pub key: RegistryKey,

    #[serde(rename = "Name")]
    pub name: String,

    #[serde(rename = "Type")]
    pub value_type: RegistryValueType,

    #[serde(flatten, skip_serializing_if = "Option::is_none")]
    pub value_data: Option<RegistryValueData>,

    /// Only used if RegistryValueType is CustomType
    /// The data is in BinaryValue
    #[serde(rename = "CustomType", skip_serializing_if = "Option::is_none")]
    pub custom_type: Option<u32>,
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
pub struct RegistryChanges {
    #[serde(rename = "AddValues", skip_serializing_if = "Vec::is_empty")]
    pub add_values: Vec<RegistryValue>,

    #[serde(rename = "DeleteValues", skip_serializing_if = "Vec::is_empty")]
    pub delete_values: Vec<RegistryValue>,
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_sample_regkey() -> RegistryKey {
        RegistryKey {
            hive: RegistryHive::Software,
            name: String::from("RegistryName"),
            volatile: false,
        }
    }

    #[test]
    fn registry_value_none() {
        assert_eq!(
            &serde_json::to_string(&RegistryValue {
                key: get_sample_regkey(),
                name: String::from("OtherRegistryName"),
                value_type: RegistryValueType::None,
                value_data: None,
                custom_type: None,
            })
            .unwrap(),
            r#"{"Key":{"Hive":"Software","Name":"RegistryName","Volatile":false},"Name":"OtherRegistryName","Type":"None"}"#
        )
    }

    #[test]
    fn registry_value_string() {
        assert_eq!(
            &serde_json::to_string(&RegistryValue {
                key: get_sample_regkey(),
                name: String::from("OtherRegistryName"),
                value_type: RegistryValueType::String,
                value_data: Some(RegistryValueData::StringValue(String::from("string_val"))),
                custom_type: None,
            })
            .unwrap(),
            r#"{"Key":{"Hive":"Software","Name":"RegistryName","Volatile":false},"Name":"OtherRegistryName","Type":"String","StringValue":"string_val"}"#
        )
    }

    #[test]
    fn registry_value_expanded_string() {
        assert_eq!(
            &serde_json::to_string(&RegistryValue {
                key: get_sample_regkey(),
                name: String::from("OtherRegistryName"),
                value_type: RegistryValueType::ExpandedString,
                value_data: Some(RegistryValueData::StringValue(String::from("string_val"))),
                custom_type: None,
            })
            .unwrap(),
            r#"{"Key":{"Hive":"Software","Name":"RegistryName","Volatile":false},"Name":"OtherRegistryName","Type":"ExpandedString","StringValue":"string_val"}"#
        )
    }

    #[test]
    fn registry_value_multi_string() {
        assert_eq!(
            &serde_json::to_string(&RegistryValue {
                key: get_sample_regkey(),
                name: String::from("OtherRegistryName"),
                value_type: RegistryValueType::ExpandedString,
                value_data: Some(RegistryValueData::StringValue(String::from("string_val1\0string_val2"))),
                custom_type: None,
            })
            .unwrap(),
            r#"{"Key":{"Hive":"Software","Name":"RegistryName","Volatile":false},"Name":"OtherRegistryName","Type":"ExpandedString","StringValue":"string_val1\u0000string_val2"}"#
        )
    }

    #[test]
    fn registry_value_binary() {
        assert_eq!(
            &serde_json::to_string(&RegistryValue {
                key: get_sample_regkey(),
                name: String::from("OtherRegistryName"),
                value_type: RegistryValueType::Binary,
                value_data: Some(RegistryValueData::BinaryValue(vec![1, 2, 3, 4])),
                custom_type: None,
            })
            .unwrap(),
            r#"{"Key":{"Hive":"Software","Name":"RegistryName","Volatile":false},"Name":"OtherRegistryName","Type":"Binary","BinaryValue":"AQIDBA=="}"#
        )
    }

    #[test]
    fn registry_value_dword() {
        assert_eq!(
            &serde_json::to_string(&RegistryValue {
                key: get_sample_regkey(),
                name: String::from("OtherRegistryName"),
                value_type: RegistryValueType::DWord,
                value_data: Some(RegistryValueData::DWordValue(56)),
                custom_type: None,
            })
            .unwrap(),
            r#"{"Key":{"Hive":"Software","Name":"RegistryName","Volatile":false},"Name":"OtherRegistryName","Type":"DWord","DWordValue":56}"#
        )
    }

    #[test]
    fn registry_value_qword() {
        assert_eq!(
            &serde_json::to_string(&RegistryValue {
                key: get_sample_regkey(),
                name: String::from("OtherRegistryName"),
                value_type: RegistryValueType::QWord,
                value_data: Some(RegistryValueData::QWordValue(5665)),
                custom_type: None,
            })
            .unwrap(),
            r#"{"Key":{"Hive":"Software","Name":"RegistryName","Volatile":false},"Name":"OtherRegistryName","Type":"QWord","QWordValue":5665}"#
        )
    }

    #[test]
    fn registry_value_custom() {
        assert_eq!(
            &serde_json::to_string(&RegistryValue {
                key: get_sample_regkey(),
                name: String::from("OtherRegistryName"),
                value_type: RegistryValueType::CustomType,
                value_data: Some(RegistryValueData::BinaryValue(vec![1,2,3,4,5])),
                custom_type: Some(5),
            })
            .unwrap(),
            r#"{"Key":{"Hive":"Software","Name":"RegistryName","Volatile":false},"Name":"OtherRegistryName","Type":"CustomType","BinaryValue":"AQIDBAU=","CustomType":5}"#
        )
    }
}
