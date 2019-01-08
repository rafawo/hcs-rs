// Copyright © rafawo (rafawo1@hotmail.com). All rights reserved.
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.
// All files in the project carrying such notice may not be copied, modified, or distributed
// except according to those terms.
// THE SOURCE CODE IS AVAILABLE UNDER THE ABOVE CHOSEN LICENSE "AS IS", WITH NO WARRANTIES.

//! Rust abstractions of the computecore APIs.
//! These are only the Rust idiomatic equivalents of the C bindings.

#[allow(dead_code)]
pub(crate) mod bindings;
pub mod ispresent;

use crate::compute::defs::*;
use crate::compute::errorcodes::{hresult_to_result_code, ResultCode};
use crate::computecore::bindings::*;
use crate::HcsResult;
use widestring::WideCString;
use winutils_rs::windefs::*;

pub fn enumerate_compute_systems(query: &str, operation: HcsOperationHandle) -> HcsResult<()> {
    unsafe {
        match HcsEnumerateComputeSystems(WideCString::from_str(query).unwrap().as_ptr(), operation)
        {
            0 => Ok(()),
            hresult => Err(hresult_to_result_code(&hresult)),
        }
    }
}

pub fn create_operation(
    context: *mut Void,
    callback: HcsOperationCompletion,
) -> HcsResult<HcsOperationHandle> {
    unsafe {
        match HcsCreateOperation(context, callback) {
            handle if handle != std::ptr::null_mut() => Ok(handle),
            _ => Err(ResultCode::Unexpected),
        }
    }
}

pub fn close_operation(operation: HcsOperationHandle) -> HcsResult<()> {
    unsafe {
        HcsCloseOperation(operation);
        Ok(())
    }
}
