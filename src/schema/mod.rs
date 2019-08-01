// Copyright © rafawo (rafawo1@hotmail.com). All rights reserved.
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.
// All files in the project carrying such notice may not be copied, modified, or distributed
// except according to those terms.
// THE SOURCE CODE IS AVAILABLE UNDER THE ABOVE CHOSEN LICENSE "AS IS", WITH NO WARRANTIES.

//! Contains all the JSON schema definitions used by the HCS APIs

pub mod common;
pub mod containers;
pub mod device_assignment;
pub mod hvsocket;
pub mod layer_management;
pub mod linux;
pub mod options;
pub mod process;
pub mod registry;
pub mod requests;
pub mod responses;
pub mod virtual_machines;

pub mod guid_serde {
    use winutils_rs::windefs::Guid;

    pub fn serialize<S>(guid: &Guid, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        let guid_string = format!(
            "{:08x}-{:04x}-{:04x}-{:02x}{:02x}-{:02x}{:02x}{:02x}{:02x}{:02x}{:02x}",
            guid.Data1,
            guid.Data2,
            guid.Data3,
            guid.Data4[0],
            guid.Data4[1],
            guid.Data4[2],
            guid.Data4[3],
            guid.Data4[4],
            guid.Data4[5],
            guid.Data4[6],
            guid.Data4[7],
        );
        serializer.serialize_str(&guid_string)
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Guid, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        use serde::de::Deserialize;
        let string_guid = String::deserialize(deserializer)?;
        Ok(winutils_rs::utilities::parse_guid(&string_guid)
            .expect(&format!("Failed to parse guid {}", &string_guid)))
    }

    #[cfg(test)]
    mod tests {
        use winutils_rs::windefs::Guid;
        use winutils_rs::windefs::GUID_NULL;

        #[derive(serde::Serialize, serde::Deserialize)]
        struct GuidWrapper {
            #[serde(with = "crate::schema::guid_serde")]
            guid: Guid,
        }

        const GUID_TEST: Guid = Guid {
            Data1: 0xdb20fa3e,
            Data2: 0xc476,
            Data3: 0x447f,
            Data4: [0x94, 0xa5, 0x51, 0xb8, 0x32, 0x2c, 0x4c, 0x4f],
        };

        macro_rules! guid_null_string {
            () => {
                r#"{"guid":"00000000-0000-0000-0000-000000000000"}"#
            };
        }

        macro_rules! guid_test_string {
            () => {
                r#"{"guid":"db20fa3e-c476-447f-94a5-51b8322c4c4f"}"#
            };
        }

        #[test]
        fn guid_null_to_string() {
            assert_eq!(
                &serde_json::to_string(&GuidWrapper { guid: GUID_NULL }).unwrap(),
                guid_null_string!()
            );
        }

        #[test]
        fn string_to_guid_null() {
            let wrapper: GuidWrapper = serde_json::from_str(guid_null_string!()).unwrap();
            assert!(winutils_rs::utilities::guid_are_equal(
                &wrapper.guid,
                &GUID_NULL
            ));
        }

        #[test]
        fn guid_to_string() {
            assert_eq!(
                &serde_json::to_string(&GuidWrapper { guid: GUID_TEST }).unwrap(),
                guid_test_string!()
            );
        }

        #[test]
        fn string_to_guid() {
            let wrapper: GuidWrapper = serde_json::from_str(guid_test_string!()).unwrap();
            assert!(winutils_rs::utilities::guid_are_equal(
                &wrapper.guid,
                &GUID_TEST
            ));
        }
    }
}
