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

use crate::compute::errorcodes::{hresult_to_result_code, ResultCode};
use crate::computenetwork::bindings::*;
use crate::computenetwork::defs::*;
use widestring::WideCString;
use winutils_rs::windefs::*;

pub struct ErrorResult {
    pub error_record: String,
    pub result_code: ResultCode,
}

/// Alias used by HCN results, which on error, contain an error record as a JSON object
/// and the underlying returned result code.
pub type HcnResult<T> = Result<T, ErrorResult>;

pub fn enumerate_networks(query: &str) -> HcnResult<String> {
    unsafe {
        let networks_ptr: *mut PWStr = std::ptr::null_mut();
        let error_record_ptr: *mut PWStr = std::ptr::null_mut();

        match HcnEnumerateNetworks(
            WideCString::from_str(query).unwrap().as_ptr(),
            networks_ptr,
            error_record_ptr,
        ) {
            0 => {
                let networks = WideCString::from_ptr_str(*networks_ptr).to_string_lossy();
                winapi::um::combaseapi::CoTaskMemFree(networks_ptr as LPVoid);
                Ok(networks)
            }
            hresult => {
                let error_record = WideCString::from_ptr_str(*error_record_ptr).to_string_lossy();
                winapi::um::combaseapi::CoTaskMemFree(error_record_ptr as LPVoid);
                Err(ErrorResult {
                    error_record,
                    result_code: hresult_to_result_code(&hresult),
                })
            }
        }
    }
}
