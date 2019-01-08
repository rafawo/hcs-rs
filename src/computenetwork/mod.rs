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
use winutils_rs::utilities::CoTaskMemWString;
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
        let networks = CoTaskMemWString::new();
        let error_record = CoTaskMemWString::new();

        match HcnEnumerateNetworks(
            WideCString::from_str(query).unwrap().as_ptr(),
            networks.ptr,
            error_record.ptr,
        ) {
            0 => Ok(networks.to_string()),
            hresult => Err(ErrorResult {
                error_record: error_record.to_string(),
                result_code: hresult_to_result_code(&hresult),
            }),
        }
    }
}

pub fn create_network(id: &Guid, settings: &str) -> HcnResult<HcnNetworkHandle> {
    unsafe {
        let mut network_handle: HcnNetworkHandle = std::ptr::null_mut();
        let error_record = CoTaskMemWString::new();

        match HcnCreateNetwork(
            id,
            WideCString::from_str(settings).unwrap().as_ptr(),
            &mut network_handle,
            error_record.ptr,
        ) {
            0 => Ok(network_handle),
            hresult => Err(ErrorResult {
                error_record: error_record.to_string(),
                result_code: hresult_to_result_code(&hresult),
            }),
        }
    }
}
