// Copyright  rafawo (rafawo1@hotmail.com). All rights reserved.
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.
// All files in the project carrying such notice may not be copied, modified, or distributed
// except according to those terms.
// THE SOURCE CODE IS AVAILABLE UNDER THE ABOVE CHOSEN LICENSE "AS IS", WITH NO WARRANTIES.

use crate::schema::utils::is_default;
use serde::{Deserialize, Serialize};

impl std::default::Default for GpuAssignmentMode {
    fn default() -> Self {
        GpuAssignmentMode::Disabled
    }
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub enum GpuAssignmentMode {
    /// Do not assign GPU to the guest.
    Disabled,
    /// Assign the single default GPU to guest, which currently is POST GPU.
    Default,
    /// Assign the GPU(s)/partition(s) specified in AssignmentRequest to guest.
    /// If AssignmentRequest is empty, do not assign GPU to the guest.
    List,
    /// Assign all current and future GPUs to guest.
    Mirror,
}

#[derive(Default, Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct GpuConfiguration {
    /// The mode used to assign GPUs to the guest.
    #[serde(default, rename = "AssignmentMode", skip_serializing_if = "is_default")]
    pub assignment_mode: GpuAssignmentMode,

    /// This only applies to List mode, and is ignored in other modes.
    /// In GPU-P, string is GPU device interface, and unit16 is partition id. HCS simply assigns the partition with the input id.
    /// In GPU-PV, string is GPU device interface, and unit16 is 0xffff. HCS needs to find an available partition to assign.
    #[serde(
        default,
        rename = "AssignmentRequest",
        skip_serializing_if = "is_default"
    )]
    pub assignment_request: std::collections::HashMap<String, u16>,

    /// Whether we allow vendor extension.
    #[serde(
        default,
        rename = "AllowVendorExtension",
        skip_serializing_if = "is_default"
    )]
    pub allow_vendor_extension: bool,
}
