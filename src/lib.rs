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
pub mod errorcodes;
pub mod hypervdevicevirtualizationdefs;

#[allow(dead_code)]
pub(crate) mod ispresent;

#[allow(dead_code)]
pub(crate) mod computecore_bindings;

#[allow(dead_code)]
pub(crate) mod computenetwork_bindings;

pub(crate) mod computestorage_bindings;

#[allow(dead_code)]
pub(crate) mod hypervdevicevirtualization_bindings;

pub type HcsResult<T> = Result<T, errorcodes::ResultCode>;
