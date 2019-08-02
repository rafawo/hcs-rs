// Copyright  rafawo (rafawo1@hotmail.com). All rights reserved.
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.
// All files in the project carrying such notice may not be copied, modified, or distributed
// except according to those terms.
// THE SOURCE CODE IS AVAILABLE UNDER THE ABOVE CHOSEN LICENSE "AS IS", WITH NO WARRANTIES.

use hex::{FromHex, ToHex};
use serde::Deserialize;
use winutils_rs::windefs::Guid;

/// Serializes `buffer` to a lowercase hex string.
pub fn buffer_to_hex<T: AsRef<[u8]>, S>(buffer: &T, serializer: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    let mut s = String::new();
    buffer.write_hex(&mut s).unwrap();
    serializer.serialize_str(&s)
}

/// Deserializes a lowercase hex string to a `Vec<u8>`.
pub fn hex_to_buffer<'de, D>(deserializer: D) -> Result<Vec<u8>, D::Error>
where
    D: serde::de::Deserializer<'de>,
{
    use serde::de::Error;
    String::deserialize(deserializer)
        .and_then(|string| Vec::from_hex(&string).map_err(|err| Error::custom(err.to_string())))
}

/// GUID structure that plays nicely with serde constructs and helpers
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct GuidSerde {
    pub data1: u32,
    pub data2: u16,
    pub data3: u16,
    pub data4: [u8; 8],
}

impl GuidSerde {
    /// Creates a new GuidSerde equivalent to GUID_NULL
    pub fn new() -> GuidSerde {
        GuidSerde {
            data1: 0,
            data2: 0,
            data3: 0,
            data4: [0; 8],
        }
    }

    /// Creates a new GuidSerde that is a straight copy of a given windows GUID
    pub fn from_win_guid(guid: &Guid) -> GuidSerde {
        GuidSerde {
            data1: guid.Data1,
            data2: guid.Data2,
            data3: guid.Data3,
            data4: guid.Data4,
        }
    }

    /// Parses a string to a GUID and stores it on a new GuidSerde
    pub fn from_str(guid_string: &str) -> winutils_rs::errorcodes::WinResult<GuidSerde> {
        Ok(GuidSerde::from_win_guid(
            &winutils_rs::utilities::parse_guid(guid_string)?,
        ))
    }

    /// Returns a windows GUID equivalent to this GuidSerde
    pub fn to_win_guid(&self) -> Guid {
        Guid {
            Data1: self.data1,
            Data2: self.data2,
            Data3: self.data3,
            Data4: self.data4,
        }
    }

    /// Copies a given windows GUID to this GuidSerde
    pub fn copy_from_win_guid(&mut self, guid: &Guid) {
        self.data1 = guid.Data1;
        self.data2 = guid.Data2;
        self.data3 = guid.Data3;
        self.data4 = guid.Data4;
    }
}

impl serde::Serialize for GuidSerde {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        let guid_string = format!(
            "{:08x}-{:04x}-{:04x}-{:02x}{:02x}-{:02x}{:02x}{:02x}{:02x}{:02x}{:02x}",
            self.data1,
            self.data2,
            self.data3,
            self.data4[0],
            self.data4[1],
            self.data4[2],
            self.data4[3],
            self.data4[4],
            self.data4[5],
            self.data4[6],
            self.data4[7],
        );
        serializer.serialize_str(&guid_string)
    }
}

impl<'de> serde::Deserialize<'de> for GuidSerde {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        let string_guid = String::deserialize(deserializer)?;
        Ok(GuidSerde::from_str(&string_guid)
            .expect(&format!("Failed to parse guid {}", &string_guid)))
    }
}

pub const GUID_SERDE_TEST: GuidSerde = GuidSerde {
    data1: 0xdb20fa3e,
    data2: 0xc476,
    data3: 0x447f,
    data4: [0x94, 0xa5, 0x51, 0xb8, 0x32, 0x2c, 0x4c, 0x4f],
};

#[cfg(test)]
mod tests {
    use super::{GuidSerde, GUID_SERDE_TEST};

    macro_rules! guid_null_string {
        () => {
            r#""00000000-0000-0000-0000-000000000000""#
        };
    }

    macro_rules! guid_test_string {
        () => {
            r#""db20fa3e-c476-447f-94a5-51b8322c4c4f""#
        };
    }

    #[test]
    fn guid_null_to_string() {
        assert_eq!(
            &serde_json::to_string(&GuidSerde::new()).unwrap(),
            guid_null_string!()
        );
    }

    #[test]
    fn string_to_guid_null() {
        let guid: GuidSerde = serde_json::from_str(guid_null_string!()).unwrap();
        assert_eq!(guid, GuidSerde::new());
    }

    #[test]
    fn guid_to_string() {
        assert_eq!(
            &serde_json::to_string(&GUID_SERDE_TEST).unwrap(),
            guid_test_string!()
        );
    }

    #[test]
    fn string_to_guid() {
        let guid: GuidSerde = serde_json::from_str(guid_test_string!()).unwrap();
        assert_eq!(guid, GUID_SERDE_TEST);
    }
}
