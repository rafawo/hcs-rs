// Copyright © rafawo (rafawo1@hotmail.com). All rights reserved.
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.
// All files in the project carrying such notice may not be copied, modified, or distributed
// except according to those terms.
// THE SOURCE CODE IS AVAILABLE UNDER THE ABOVE CHOSEN LICENSE "AS IS", WITH NO WARRANTIES.

//! Common definitions and error codes used by the HCS APIs.

pub mod defs;
pub mod errorcodes;

use winutils_rs::windefs::Handle;

/// Policies supported by HCS Safe Handle wrappers, which determine
/// what is done with the wrapped handle when the wrapping object is dropped.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum HcsWrappedHandleDropPolicy {
    Close,
    Ignore,
}

/// Trait that defines the contract for an HCS Safe Handle wrapper.
pub trait HcsSafeHandle {
    type SafeHandleWrapper;

    /// Wraps an HCS Operation handle and returns the `T` safe wrapper object.
    fn wrap_handle(handle: Handle) -> Self::SafeHandleWrapper;

    /// Returns a copy of the underlying handle.
    ///
    /// # Note
    /// This function is useful when the safe wrapper object must interact
    /// with the HCS API by using the handle directly, but there's no desire
    /// to 'unwrap' the handle.
    fn handle(&self) -> Handle;

    /// Sets the wrapped handle policy.
    ///
    /// # Note
    /// Setting the handle policy to `HcsWrappedHandleDropPolicy::Ignore` will make sure
    /// that when the safe wrapper is dropped, the underlying handle will not be closed
    /// using the HCS APIs.
    ///
    /// Ignoring the underlying handle can lead to potential leaks, since it still
    /// needs to be closed through the HCS APIs at some point.
    fn set_handle_policy(&mut self, handle_policy: HcsWrappedHandleDropPolicy);

    /// Returns the safe wrapped handle close policy.
    fn handle_policy(&self) -> HcsWrappedHandleDropPolicy;
}
