// Copyright  rafawo (rafawo1@hotmail.com). All rights reserved.
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.
// All files in the project carrying such notice may not be copied, modified, or distributed
// except according to those terms.
// THE SOURCE CODE IS AVAILABLE UNDER THE ABOVE CHOSEN LICENSE "AS IS", WITH NO WARRANTIES.

use crate::schema::utils::GuidSerde;
use serde::{Deserialize, Serialize};

#[derive(Default, Deserialize, Serialize, Debug, Clone)]
pub struct HvSocketServiceConfig {
    /// SDDL string that HvSocket will check before allowing a host process to bind
    /// to this specific service.
    #[serde(
        default,
        rename = "BindSecurityDescriptor",
        skip_serializing_if = "Option::is_none"
    )]
    pub bind_security_descriptor: Option<String>,

    /// SDDL string that HvSocket will check before allowing a host process to connect
    /// to this specific service.
    #[serde(
        default,
        rename = "ConnectSecurityDescriptor",
        skip_serializing_if = "Option::is_none"
    )]
    pub connect_security_descriptor: Option<String>,

    // If true, HvSocket will process wildcard binds for this service/system combination.
    // Wildcard binds are secured in the registry at
    // **SOFTWARE/Microsoft/Windows NT/CurrentVersion/Virtualization/HvSocket/WildcardDescriptors**
    #[serde(default, rename = "AllowWildcardBinds")]
    pub allow_wildcard_binds: bool,
}

#[derive(Default, Deserialize, Serialize, Debug, Clone)]
pub struct HvSocketSystemConfig {
    /// SDDL string that HvSocket will check before allowing a host process to bind
    /// to an unlisted service for this specific container/VM (not wildcard binds).
    #[serde(
        default,
        rename = "DefaultBindSecurityDescriptor",
        skip_serializing_if = "Option::is_none"
    )]
    pub default_bind_security_descriptor: Option<String>,

    /// SDDL string that HvSocket will check before allowing a host process to connect
    /// to an unlisted service in the VM/container.
    #[serde(
        default,
        rename = "DefaultConnectSecurityDescriptor",
        skip_serializing_if = "Option::is_none"
    )]
    pub default_connect_security_descriptor: Option<String>,

    #[serde(
        default,
        rename = "ServiceTable",
        skip_serializing_if = "std::collections::HashMap::is_empty"
    )]
    pub service_table: std::collections::HashMap<GuidSerde, HvSocketServiceConfig>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::schema::utils::GUID_SERDE_TEST;

    #[test]
    fn hvsocket_service_config() {
        assert_eq!(
            &serde_json::to_string(&HvSocketServiceConfig {
                bind_security_descriptor: None,
                connect_security_descriptor: None,
                allow_wildcard_binds: true,
            })
            .unwrap(),
            r#"{"AllowWildcardBinds":true}"#
        );
    }

    #[test]
    fn hvsocket_service_config_with_strings() {
        assert_eq!(&serde_json::to_string(&HvSocketServiceConfig {
            bind_security_descriptor: Some(String::from("SomeBindSecurityDescriptor")),
            connect_security_descriptor: Some(String::from("SomeConnectSecurityDescriptor")),
            allow_wildcard_binds: true,
        }).unwrap(),
        r#"{"BindSecurityDescriptor":"SomeBindSecurityDescriptor","ConnectSecurityDescriptor":"SomeConnectSecurityDescriptor","AllowWildcardBinds":true}"#
        );
    }

    #[test]
    fn hvsocket_system_config() {
        assert_eq!(
            &serde_json::to_string(&HvSocketSystemConfig {
                default_bind_security_descriptor: None,
                default_connect_security_descriptor: None,
                service_table: std::collections::HashMap::new(),
            })
            .unwrap(),
            r#"{}"#
        );
    }

    #[test]
    fn hvsocket_system_config_with_strings() {
        let mut service_table: std::collections::HashMap<GuidSerde, HvSocketServiceConfig> =
            std::collections::HashMap::new();
        service_table.insert(
            GuidSerde::new(),
            HvSocketServiceConfig {
                bind_security_descriptor: None,
                connect_security_descriptor: None,
                allow_wildcard_binds: true,
            },
        );
        service_table.insert(
            GUID_SERDE_TEST,
            HvSocketServiceConfig {
                bind_security_descriptor: Some(String::from("SomeBindSecurityDescriptor")),
                connect_security_descriptor: Some(String::from("SomeConnectSecurityDescriptor")),
                allow_wildcard_binds: true,
            },
        );

        let hvsocket_system_config_string = serde_json::to_string(&HvSocketSystemConfig {
            default_bind_security_descriptor: Some(String::from("SomeBindSecurityDescriptor")),
            default_connect_security_descriptor: Some(String::from(
                "SomeConnectSecurityDescriptor",
            )),
            service_table,
        })
        .unwrap();

        // Unfortunately, serialization of the map is not deterministic and we should take into account both variations
        assert!(&hvsocket_system_config_string == r#"{"DefaultBindSecurityDescriptor":"SomeBindSecurityDescriptor","DefaultConnectSecurityDescriptor":"SomeConnectSecurityDescriptor","ServiceTable":{"db20fa3e-c476-447f-94a5-51b8322c4c4f":{"BindSecurityDescriptor":"SomeBindSecurityDescriptor","ConnectSecurityDescriptor":"SomeConnectSecurityDescriptor","AllowWildcardBinds":true},"00000000-0000-0000-0000-000000000000":{"AllowWildcardBinds":true}}}"#
        || &hvsocket_system_config_string == r#"{"DefaultBindSecurityDescriptor":"SomeBindSecurityDescriptor","DefaultConnectSecurityDescriptor":"SomeConnectSecurityDescriptor","ServiceTable":{"00000000-0000-0000-0000-000000000000":{"AllowWildcardBinds":true},"db20fa3e-c476-447f-94a5-51b8322c4c4f":{"BindSecurityDescriptor":"SomeBindSecurityDescriptor","ConnectSecurityDescriptor":"SomeConnectSecurityDescriptor","AllowWildcardBinds":true}}}"#);
    }
}
