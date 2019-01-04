// Copyright © rafawo (rafawo1@hotmail.com). All rights reserved.
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.
// All files in the project carrying such notice may not be copied, modified, or distributed
// except according to those terms.
// THE SOURCE CODE IS AVAILABLE UNDER THE ABOVE CHOSEN LICENSE "AS IS", WITH NO WARRANTIES.

pub mod computedefs;
pub mod computenetworkdefs;
pub mod computestorage;
pub mod hypervdevicevirtualizationdefs;

#[allow(dead_code)]
pub(crate) mod computecore_bindings;

#[allow(dead_code)]
pub(crate) mod computenetwork_bindings;

#[allow(dead_code)]
pub(crate) mod computestorage_bindings;

#[allow(dead_code)]
pub(crate) mod hypervdevicevirtualization_bindings;

/// Common result codes that can be returned by the HCS APIs
#[derive(Debug, PartialEq)]
pub enum ResultCode {
    Success,
    OutOfMemory,
    FileNotFound,
    Fail,
    InvalidArgument,
    Unexpected,
    WindowsHResult(winutils_rs::windefs::HResult),
}

#[allow(overflowing_literals)]
pub(crate) fn hresult_to_result_code(hresult: &winutils_rs::windefs::HResult) -> ResultCode {
    match hresult {
        0 => ResultCode::Success,
        0x8007000E => ResultCode::OutOfMemory,
        0x80070002 => ResultCode::FileNotFound,
        0x80004005 => ResultCode::Fail,
        0x80070057 => ResultCode::InvalidArgument,
        0x8000FFFF => ResultCode::Unexpected,
        other => ResultCode::WindowsHResult(other.clone()),
    }
}
