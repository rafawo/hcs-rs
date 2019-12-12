// Copyright  rafawo (rafawo1@hotmail.com). All rights reserved.
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.
// All files in the project carrying such notice may not be copied, modified, or distributed
// except according to those terms.
// THE SOURCE CODE IS AVAILABLE UNDER THE ABOVE CHOSEN LICENSE "AS IS", WITH NO WARRANTIES.

use crate::schema::utils::is_default;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Default, Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct ProcessParameters {
    #[serde(
        default,
        rename = "ApplicationName",
        skip_serializing_if = "String::is_empty"
    )]
    pub application_name: String,

    #[serde(
        default,
        rename = "CommandLine",
        skip_serializing_if = "String::is_empty"
    )]
    pub command_line: String,

    /// optional alternative to CommandLine, currently only supported by Linux GCS
    #[serde(default, rename = "CommandArgs", skip_serializing_if = "Vec::is_empty")]
    pub command_args: Vec<String>,

    #[serde(default, rename = "User", skip_serializing_if = "String::is_empty")]
    pub user: String,

    #[serde(
        default,
        rename = "WorkingDirectory",
        skip_serializing_if = "String::is_empty"
    )]
    pub working_directory: String,

    #[serde(
        default,
        rename = "Environment",
        skip_serializing_if = "HashMap::is_empty"
    )]
    pub environment: HashMap<String, String>,

    /// if set, will run as low-privilege process
    #[serde(
        default,
        rename = "RestrictedToken",
        skip_serializing_if = "is_default"
    )]
    pub restricted_token: bool,

    /// if set, ignore StdErrPipe
    #[serde(default, rename = "EmulateConsole", skip_serializing_if = "is_default")]
    pub emulate_console: bool,

    #[serde(
        default,
        rename = "CreateStdInPipe",
        skip_serializing_if = "is_default"
    )]
    pub create_std_in_pipe: bool,

    #[serde(
        default,
        rename = "CreateStdOutPipe",
        skip_serializing_if = "is_default"
    )]
    pub create_std_out_pipe: bool,

    #[serde(
        default,
        rename = "CreateStdErrPipe",
        skip_serializing_if = "is_default"
    )]
    pub create_std_err_pipe: bool,

    /// height then width
    #[serde(default, rename = "ConsoleSize", skip_serializing_if = "is_default")]
    pub console_size: [u16; 2],

    /// if set, find an existing session for the user and create the process in it
    #[serde(
        default,
        rename = "UseExistingLogin",
        skip_serializing_if = "is_default"
    )]
    pub use_existing_login: bool,

    /// if set, use the legacy console instead of conhost
    #[serde(
        default,
        rename = "UseLegacyConsole",
        skip_serializing_if = "is_default"
    )]
    pub use_legacy_console: bool,
}

impl std::default::Default for ModifyOperation {
    fn default() -> Self {
        ModifyOperation::ConsoleSize
    }
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq, Hash)]
pub enum ModifyOperation {
    /// Update the console size
    ConsoleSize,
    /// Close one or all of the std handles
    CloseHandle,
}

#[derive(Default, Deserialize, Serialize, Debug, Clone, PartialEq, Eq, Hash)]
pub struct ConsoleSize {
    #[serde(rename = "Height")]
    pub height: u16,

    #[serde(rename = "Width")]
    pub width: u16,
}

impl std::default::Default for StdHandle {
    fn default() -> Self {
        StdHandle::StdIn
    }
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq, Hash)]
pub enum StdHandle {
    StdIn,
    StdOut,
    StdErr,
    All,
}

#[derive(Default, Deserialize, Serialize, Debug, Clone, PartialEq, Eq, Hash)]
pub struct CloseHandle {
    #[serde(rename = "Handle")]
    pub handle: StdHandle,
}

// Passed to HcsRpc_ModifyProcess
#[derive(Default, Deserialize, Serialize, Debug, Clone, PartialEq, Eq, Hash)]
pub struct ProcessModifyRequest {
    #[serde(rename = "Operation")]
    pub operation: ModifyOperation,

    #[serde(default, rename = "ConsoleSize", skip_serializing_if = "is_default")]
    pub console_size: Option<ConsoleSize>,

    #[serde(default, rename = "CloseHandle", skip_serializing_if = "is_default")]
    pub close_handle: Option<CloseHandle>,
}
