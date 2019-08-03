// Copyright  rafawo (rafawo1@hotmail.com). All rights reserved.
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.
// All files in the project carrying such notice may not be copied, modified, or distributed
// except according to those terms.
// THE SOURCE CODE IS AVAILABLE UNDER THE ABOVE CHOSEN LICENSE "AS IS", WITH NO WARRANTIES.

use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq, Hash)]
pub enum SaveType {
    /// The system's memory and device states are saved to the runtime state file.
    ToFile,
    /// The system's device state is saved to the runtime state file. The system
    /// is then placed in a state such that other systems can be cloned from it.
    AsTemplate,
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq, Hash)]
pub struct SaveOptions {
    /// The type of save operation to be performed.
    #[serde(rename = "SaveType", skip_serializing_if = "Option::is_none")]
    pub save_type: Option<SaveType>,

    /// The path to the runtime state file.
    #[serde(rename = "RuntimeStateFilePath")]
    pub runtime_state_filepath: String,

    /// The path to the file that will container the saved state.
    #[serde(rename = "SaveStateFilePath")]
    pub saved_state_filepath: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq, Hash)]
pub enum PauseSuspensionLevel {
    Suspend,
    MemoryLow,
    MemoryMedium,
    MemoryHigh,
}

// Pause reason that is indicated to components running in the Virtual Machine.
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq, Hash)]
pub enum PauseReason {
    None,
    Save,
    Template,
}

// Notification data that is indicated to components running in the Virtual Machine.
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq, Hash)]
pub struct PauseNotification {
    #[serde(rename = "Reason")]
    pub reason: PauseReason,
}

// Options for HcsPauseComputeSystem
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq, Hash)]
pub struct PauseOptions {
    #[serde(rename = "PauseSuspensionLevel")]
    pub suspension_level: PauseSuspensionLevel,

    #[serde(rename = "HostedNotification")]
    pub hosted_notification: PauseNotification,
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq, Hash)]
pub struct ExportLayerOptions {
    #[serde(rename = "IsWritableLayer")]
    pub is_writable_layer: bool,
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq, Hash)]
pub enum OsLayerType {
    Container,
    Vm,
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq, Hash)]
pub struct OsLayerOptions {
    #[serde(rename = "Type")]
    pub oslayer_type: OsLayerType,

    #[serde(rename = "DisableCiCacheOptimization")]
    pub disable_ci_cache_optimization: bool,
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq, Hash)]
pub enum ProcessSignal {
    CtrlC,
    CtrlBreak,
    CtrlClose,
    CtrlLogOff,
    CtrlShutdown,
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq, Hash)]
pub struct SignalProcessOptions {
    #[serde(rename = "ProcessSignal")]
    pub signal: ProcessSignal,
}
