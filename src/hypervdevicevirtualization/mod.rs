// Copyright © rafawo (rafawo1@hotmail.com). All rights reserved.
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.
// All files in the project carrying such notice may not be copied, modified, or distributed
// except according to those terms.
// THE SOURCE CODE IS AVAILABLE UNDER THE ABOVE CHOSEN LICENSE "AS IS", WITH NO WARRANTIES.

#[allow(dead_code)]
pub(crate) mod bindings;
pub mod defs;
pub mod ispresent;

use crate::compute::defs::*;
use crate::compute::errorcodes::hresult_to_result_code;
use crate::hypervdevicevirtualization::bindings::*;
use crate::hypervdevicevirtualization::defs::*;
use crate::HcsResult;
use winutils_rs::windefs::*;

pub fn initialize_device_host(compute_system: HcsSystemHandle) -> HcsResult<HdvHostHandle> {
    let mut device_host_handle: HdvHostHandle = std::ptr::null_mut();

    unsafe {
        match HdvInitializeDeviceHost(compute_system, &mut device_host_handle) {
            0 => Ok(device_host_handle),
            hresult => Err(hresult_to_result_code(&hresult)),
        }
    }
}
