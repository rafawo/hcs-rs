// Copyright  rafawo (rafawo1@hotmail.com). All rights reserved.
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.
// All files in the project carrying such notice may not be copied, modified, or distributed
// except according to those terms.
// THE SOURCE CODE IS AVAILABLE UNDER THE ABOVE CHOSEN LICENSE "AS IS", WITH NO WARRANTIES.

use crate::schema::utils::is_default;
use serde::{Deserialize, Serialize};

impl std::default::Default for SaveType {
    fn default() -> Self {
        SaveType::ToFile
    }
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub enum SaveType {
    /// The system's memory and device states are saved to the runtime state file.
    ToFile,
    /// The system's device state is saved to the runtime state file. The system
    /// is then placed in a state such that other systems can be cloned from it.
    AsTemplate,
}

#[derive(Default, Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct SaveOptions {
    /// The type of save operation to be performed.
    #[serde(default, rename = "SaveType", skip_serializing_if = "is_default")]
    pub save_type: Option<SaveType>,

    /// The path to the file that will contain the saved state.
    #[serde(rename = "SaveStateFilePath")]
    pub saved_state_filepath: String,
}

impl std::default::Default for PauseSuspensionLevel {
    fn default() -> Self {
        PauseSuspensionLevel::Suspend
    }
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub enum PauseSuspensionLevel {
    Suspend,
    MemoryLow,
    MemoryMedium,
    MemoryHigh,
}

impl std::default::Default for PauseReason {
    fn default() -> Self {
        PauseReason::None
    }
}

// Pause reason that is indicated to components running in the Virtual Machine.
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub enum PauseReason {
    None,
    Save,
    Template,
}

// Notification data that is indicated to components running in the Virtual Machine.
#[derive(Default, Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct PauseNotification {
    #[serde(default, rename = "Reason", skip_serializing_if = "is_default")]
    pub reason: PauseReason,
}

// Options for HcsPauseComputeSystem
#[derive(Default, Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct PauseOptions {
    #[serde(rename = "PauseSuspensionLevel")]
    pub suspension_level: PauseSuspensionLevel,

    #[serde(
        default,
        rename = "HostedNotification",
        skip_serializing_if = "is_default"
    )]
    pub hosted_notification: PauseNotification,
}

#[derive(Default, Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct ExportLayerOptions {
    #[serde(
        default,
        rename = "IsWritableLayer",
        skip_serializing_if = "is_default"
    )]
    pub is_writable_layer: bool,
}

impl std::default::Default for OsLayerType {
    fn default() -> Self {
        OsLayerType::Container
    }
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub enum OsLayerType {
    Container,
    Vm,
}

#[derive(Default, Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct OsLayerOptions {
    #[serde(default, rename = "Type", skip_serializing_if = "is_default")]
    pub oslayer_type: OsLayerType,

    #[serde(
        default,
        rename = "DisableCiCacheOptimization",
        skip_serializing_if = "is_default"
    )]
    pub disable_ci_cache_optimization: bool,
}

impl std::default::Default for ProcessSignal {
    fn default() -> Self {
        ProcessSignal::CtrlC
    }
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub enum ProcessSignal {
    CtrlC,
    CtrlBreak,
    CtrlClose,
    CtrlLogOff,
    CtrlShutdown,
}

#[derive(Default, Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct SignalProcessOptions {
    #[serde(default, rename = "ProcessSignal", skip_serializing_if = "is_default")]
    pub signal: ProcessSignal,
}
