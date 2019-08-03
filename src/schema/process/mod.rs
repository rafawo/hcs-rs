// Copyright  rafawo (rafawo1@hotmail.com). All rights reserved.
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.
// All files in the project carrying such notice may not be copied, modified, or distributed
// except according to those terms.
// THE SOURCE CODE IS AVAILABLE UNDER THE ABOVE CHOSEN LICENSE "AS IS", WITH NO WARRANTIES.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
pub struct ProcessParameters {
    #[serde(rename = "ApplicationName", skip_serializing_if = "String::is_empty")]
    pub application_name: String,

    #[serde(rename = "CommandLine", skip_serializing_if = "String::is_empty")]
    pub command_line: String,

    /// optional alternative to CommandLine, currently only supported by Linux GCS
    #[serde(rename = "CommandArgs", skip_serializing_if = "Vec::is_empty")]
    pub command_args: Vec<String>,

    #[serde(rename = "User", skip_serializing_if = "String::is_empty")]
    pub user: String,

    #[serde(rename = "WorkingDirectory", skip_serializing_if = "String::is_empty")]
    pub working_directory: String,

    #[serde(rename = "Environment", skip_serializing_if = "HashMap::is_empty")]
    pub environment: HashMap<String, String>,

    /// if set, will run as low-privilege process
    #[serde(rename = "RestrictedToken")]
    pub restricted_token: bool,

    /// if set, ignore StdErrPipe
    #[serde(rename = "EmulateConsole")]
    pub emulate_console: bool,

    #[serde(rename = "CreateStdInPipe")]
    pub create_std_in_pipe: bool,

    #[serde(rename = "CreateStdOutPipe")]
    pub create_std_out_pipe: bool,

    #[serde(rename = "CreateStdErrPipe")]
    pub create_std_err_pipe: bool,

    /// height then width
    #[serde(rename = "ConsoleSize")]
    pub console_size: [u16; 2],

    /// if set, create the process in the utility VM instead of the container
    #[serde(rename = "CreateInUtilityVM")]
    pub create_in_utility_vm: bool,

    /// if set, find an existing session for the user and create the process in it
    #[serde(rename = "UseExistingLogin")]
    pub use_existing_login: bool,

    /// if set, use the legacy console instead of conhost
    #[serde(rename = "UseLegacyConsole")]
    pub use_legacy_console: bool,
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq, Hash)]
pub enum ModifyOperation {
    /// Update the console size
    ConsoleSize,
    /// Close one or all of the std handles
    CloseHandle,
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq, Hash)]
pub struct ConsoleSize {
    #[serde(rename = "Height")]
    pub height: u16,

    #[serde(rename = "Width")]
    pub width: u16,
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq, Hash)]
pub enum StdHandle {
    StdIn,
    StdOut,
    StdErr,
    All,
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq, Hash)]
pub struct CloseHandle {
    #[serde(rename = "Handle")]
    pub handle: StdHandle,
}

// Passed to HcsRpc_ModifyProcess
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq, Hash)]
pub struct ProcessModifyRequest {
    #[serde(rename = "Operation")]
    pub operation: ModifyOperation,

    #[serde(rename = "ConsoleSize", skip_serializing_if = "Option::is_none")]
    pub console_size: Option<ConsoleSize>,

    #[serde(rename = "CloseHandle", skip_serializing_if = "Option::is_none")]
    pub close_handle: Option<CloseHandle>,
}
