// Copyright  rafawo (rafawo1@hotmail.com). All rights reserved.
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.
// All files in the project carrying such notice may not be copied, modified, or distributed
// except according to those terms.
// THE SOURCE CODE IS AVAILABLE UNDER THE ABOVE CHOSEN LICENSE "AS IS", WITH NO WARRANTIES.

pub mod resources {
    use crate::schema::GuidSerde;
    use serde::{Deserialize, Serialize};

    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq, Hash)]
    pub struct StorageQoS {
        #[serde(rename = "IopsMaximum")]
        pub iops_maximum: u64,
        #[serde(rename = "BandwidthMaximum")]
        pub bandwidth_maximum: u64,
    }

    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq, Hash)]
    pub enum CacheMode {
        Disabled,
        Enabled,
        Private,
        PrivateAllowSharing,
    }

    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq, Hash)]
    pub enum PathType {
        AbsolutePath,
        VirtualSmbShareName,
    }

    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq, Hash)]
    pub struct Layer {
        #[serde(rename = "Id")]
        pub id: GuidSerde,
        #[serde(rename = "Path")]
        pub path: String,
        #[serde(rename = "PathType")]
        pub path_type: PathType,
        #[serde(rename = "Cache", skip_serializing_if = "Option::is_none")]
        pub cache: Option<CacheMode>,
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn storage_qos() {
            assert_eq!(
                &serde_json::to_string(&StorageQoS {
                    iops_maximum: 123,
                    bandwidth_maximum: 456
                })
                .unwrap(),
                r#"{"IopsMaximum":123,"BandwidthMaximum":456}"#
            );
        }

        #[test]
        fn cache_mode() {
            assert_eq!(
                &serde_json::to_string(&CacheMode::Disabled).unwrap(),
                r#""Disabled""#
            );
            assert_eq!(
                &serde_json::to_string(&CacheMode::Enabled).unwrap(),
                r#""Enabled""#
            );
            assert_eq!(
                &serde_json::to_string(&CacheMode::Private).unwrap(),
                r#""Private""#
            );
            assert_eq!(
                &serde_json::to_string(&CacheMode::PrivateAllowSharing).unwrap(),
                r#""PrivateAllowSharing""#
            );
        }

        #[test]
        fn path_type() {
            assert_eq!(
                &serde_json::to_string(&PathType::AbsolutePath).unwrap(),
                r#""AbsolutePath""#
            );
            assert_eq!(
                &serde_json::to_string(&PathType::VirtualSmbShareName).unwrap(),
                r#""VirtualSmbShareName""#
            );
        }

        #[test]
        fn layer() {
            assert_eq!(
                &serde_json::to_string(&Layer {
                    id: GuidSerde::new(),
                    path: String::from("some\\path"),
                    path_type: PathType::AbsolutePath,
                    cache: None,
                }).unwrap(),
                r#"{"Id":"00000000-0000-0000-0000-000000000000","Path":"some\\path","PathType":"AbsolutePath"}"#
            );
            assert_eq!(
                &serde_json::to_string(&Layer {
                    id: GuidSerde::new(),
                    path: String::from("some\\path"),
                    path_type: PathType::VirtualSmbShareName,
                    cache: Some(CacheMode::Enabled),
                }).unwrap(),
                r#"{"Id":"00000000-0000-0000-0000-000000000000","Path":"some\\path","PathType":"VirtualSmbShareName","Cache":"Enabled"}"#
            );
        }
    }
}
