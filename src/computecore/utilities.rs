// Copyright © rafawo (rafawo1@hotmail.com). All rights reserved.
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.
// All files in the project carrying such notice may not be copied, modified, or distributed
// except according to those terms.
// THE SOURCE CODE IS AVAILABLE UNDER THE ABOVE CHOSEN LICENSE "AS IS", WITH NO WARRANTIES.

//! Rust types that provide convenient functionality built on top of the computecore APIs.

// TODO:rafawo Change the implementations in this file to use strongly typed representations
// of the JSON API surface of HCS instead of using straight raw strings.

use crate::compute::defs::*;
use crate::compute::{HcsSafeHandle, HcsWrappedHandleDropPolicy};
use crate::computecore;
use crate::hypervdevicevirtualization::utilities::HdvHost;
use crate::HcsResult;
use winutils_rs::windefs::*;

pub const INFINITE: DWord = winapi::um::winbase::INFINITE;
pub const GENERIC_ALL: DWord = winapi::um::winnt::GENERIC_ALL;

/// Safe wrapper of an HCS Operation handle.
/// When dropped, the underlying handle is closed from the HCS API.
pub struct HcsOperation {
    handle: HcsOperationHandle,
    handle_policy: HcsWrappedHandleDropPolicy,
}

impl std::ops::Drop for HcsOperation {
    fn drop(&mut self) {
        if self.handle != std::ptr::null_mut()
            && self.handle_policy == HcsWrappedHandleDropPolicy::Close
        {
            computecore::close_operation(self.handle).expect("Failed to close operation handle");
        }
    }
}

impl HcsSafeHandle for HcsOperation {
    type SafeHandleWrapper = HcsOperation;

    fn wrap_handle(handle: Handle) -> HcsOperation {
        HcsOperation {
            handle,
            handle_policy: HcsWrappedHandleDropPolicy::Close,
        }
    }

    fn handle(&self) -> Handle {
        self.handle
    }

    fn set_handle_policy(&mut self, handle_policy: HcsWrappedHandleDropPolicy) {
        self.handle_policy = handle_policy;
    }

    fn handle_policy(&self) -> HcsWrappedHandleDropPolicy {
        self.handle_policy
    }
}

/// Safe wrapper of a Compute System handle.
/// When dropped, the underlying handle is closed from the HCS API.
pub struct HcsSystem {
    handle: HcsSystemHandle,
    handle_policy: HcsWrappedHandleDropPolicy,
}

impl std::ops::Drop for HcsSystem {
    fn drop(&mut self) {
        if self.handle != std::ptr::null_mut()
            && self.handle_policy == HcsWrappedHandleDropPolicy::Close
        {
            computecore::close_compute_system(self.handle)
                .expect("Failed to close compute system handle");
        }
    }
}

impl HcsSafeHandle for HcsSystem {
    type SafeHandleWrapper = HcsSystem;

    fn wrap_handle(handle: Handle) -> HcsSystem {
        HcsSystem {
            handle,
            handle_policy: HcsWrappedHandleDropPolicy::Close,
        }
    }

    fn handle(&self) -> Handle {
        self.handle
    }

    fn set_handle_policy(&mut self, handle_policy: HcsWrappedHandleDropPolicy) {
        self.handle_policy = handle_policy;
    }

    fn handle_policy(&self) -> HcsWrappedHandleDropPolicy {
        self.handle_policy
    }
}

/// Safe wrapper of a Compute System Process handle.
/// When dropped, the underlying handle is closed from the HCS API.
pub struct HcsProcess {
    handle: HcsProcessHandle,
    handle_policy: HcsWrappedHandleDropPolicy,
}

impl std::ops::Drop for HcsProcess {
    fn drop(&mut self) {
        if self.handle != std::ptr::null_mut()
            && self.handle_policy == HcsWrappedHandleDropPolicy::Close
        {
            computecore::close_process(self.handle).expect("Failed to close process handle");
        }
    }
}

impl HcsSafeHandle for HcsProcess {
    type SafeHandleWrapper = HcsProcess;

    fn wrap_handle(handle: Handle) -> HcsProcess {
        HcsProcess {
            handle,
            handle_policy: HcsWrappedHandleDropPolicy::Close,
        }
    }

    fn handle(&self) -> Handle {
        self.handle
    }

    fn set_handle_policy(&mut self, handle_policy: HcsWrappedHandleDropPolicy) {
        self.handle_policy = handle_policy;
    }

    fn handle_policy(&self) -> HcsWrappedHandleDropPolicy {
        self.handle_policy
    }
}

unsafe extern "system" fn operation_callback(operation: HcsOperationHandle, context: PVoid) {
    let mut operation = HcsOperation::wrap_handle(operation);
    operation.set_handle_policy(HcsWrappedHandleDropPolicy::Ignore);
    let context_as_closure: &mut &mut dyn FnMut(&HcsOperation) = &mut *(context as *mut _);
    context_as_closure(&operation);
}

/// Thin wrapper of an HCS Operation that interfaces to all HCS APIs that inherently
/// depend on an HCS Operation handle as input and/or output.
impl HcsOperation {
    /// Creates a new HCS Operation, and returns a safe wrapper to the handle.
    pub fn new() -> HcsResult<HcsOperation> {
        Ok(HcsOperation {
            handle: computecore::create_operation(std::ptr::null_mut(), None)?,
            handle_policy: HcsWrappedHandleDropPolicy::Close,
        })
    }

    /// Creates a new HCS Operation with callback, and returns a safe wrapper to the handle.
    /// Callback is expected to be a reference to the trait object of the closure.
    ///
    /// Example:
    /// ```rust,ignore
    /// let mut closure = move |operation: &HcsOperation| {};
    /// let mut trait_obj = &mut dyn FnMut(&HcsOperation) = &mut closure;
    /// let trait_obj_ref = &mut trait_obj;
    /// HcsOperation::create(trait_obj_ref);
    /// ```
    ///
    /// # Safety
    /// Ensure that the supplied callback's closure trait object mutable reference
    /// doesn't change and outlives the HCS operation's lifetime.
    pub unsafe fn create<T>(callback: &mut &mut T) -> HcsResult<HcsOperation>
    where
        T: 'static,
        T: ?Sized,
        T: FnMut(&HcsOperation),
    {
        Ok(HcsOperation {
            handle: computecore::create_operation(
                callback as *mut _ as PVoid,
                Some(operation_callback),
            )?,
            handle_policy: HcsWrappedHandleDropPolicy::Close,
        })
    }

    /// Returns a safe wrapper of a Compute System handle associated to an operation.
    /// The wrapped handle returned is set to drop policy ignored.
    pub fn get_compute_system(&self) -> HcsResult<HcsSystem> {
        Ok(HcsSystem {
            handle: computecore::get_compute_system_from_operation(self.handle)?,
            handle_policy: HcsWrappedHandleDropPolicy::Ignore,
        })
    }

    /// Returns a safe wrapper of a Compute System Process handle associated to an operation.
    /// The wrapped handle returned is set to drop policy ignored.
    pub fn get_process(&self) -> HcsResult<HcsProcess> {
        Ok(HcsProcess {
            handle: computecore::get_process_from_operation(self.handle)?,
            handle_policy: HcsWrappedHandleDropPolicy::Ignore,
        })
    }

    /// Returns the type of the operation.
    pub fn get_type(&self) -> HcsResult<HcsOperationType> {
        computecore::get_operation_type(self.handle)
    }

    /// Returns the ID of the operation.
    pub fn get_id(&self) -> HcsResult<u64> {
        computecore::get_operation_id(self.handle)
    }

    /// Returns the result document of the operation.
    ///
    /// # Note
    /// This is only valid once the operation has been completed.
    pub fn get_result(&self) -> (String, HcsResult<()>) {
        computecore::get_operation_result(self.handle)
    }

    /// Returns the result and process info of the operation.
    ///
    /// # Note
    /// This is only valid once the operation has been completed.
    pub fn get_result_and_process_info(&self) -> (String, HcsResult<HcsProcessInformation>) {
        computecore::get_operation_result_and_process_info(self.handle)
    }

    /// Waits for an operation to complete and returns the result document synchronously.
    pub fn wait_for_result(&self, timeout_ms: DWord) -> (String, HcsResult<()>) {
        computecore::wait_for_operation_result(self.handle, timeout_ms)
    }

    /// Waits for an operation to complete and returns the result document and process info syncrhonously.
    pub fn wait_for_result_and_process_info(
        &self,
        timeout_ms: DWord,
    ) -> (String, HcsResult<HcsProcessInformation>) {
        computecore::wait_for_operation_result_and_process_info(self.handle, timeout_ms)
    }

    /// Sets the operation completion callback.
    /// Callback is expected to be a reference to the trait object of the closure.
    ///
    /// Example:
    /// ```rust,ignore
    /// let mut closure = move |operation: &HcsOperation| {};
    /// let mut trait_obj = &mut dyn FnMut(&HcsOperation) = &mut closure;
    /// let trait_obj_ref = &mut trait_obj;
    /// operation.set_callback(trait_obj_ref);
    /// ```
    ///
    /// # Safety
    /// Ensure that the supplied callback's closure trait object mutable reference
    /// doesn't change and outlives the HCS operation's lifetime.
    pub unsafe fn set_callback<T>(&self, callback: &mut &mut T) -> HcsResult<()>
    where
        T: 'static,
        T: ?Sized,
        T: FnMut(&HcsOperation),
    {
        computecore::set_operation_callback(
            self.handle,
            callback as *mut _ as PVoid,
            Some(operation_callback),
        )
    }

    /// Cancels an operation.
    pub fn cancel(&self) -> HcsResult<()> {
        computecore::cancel_operation(self.handle)
    }
}

unsafe extern "system" fn hcs_system_and_proces_callback(event: *const HcsEvent, context: PVoid) {
    let context_as_closure: &mut &mut dyn FnMut(&HcsEvent) = &mut *(context as *mut _);
    context_as_closure(&*event);
}

/// Thin wrapper of an HCS Compute System that interfaces to all HCS APIs that inherently
/// depend on an HCS Compute System handle as input and/or output.
impl HcsSystem {
    /// Creates a Compute System and returns a safe wrapper of the handle.
    pub fn create(
        id: &str,
        configuration: &str,
        operation: &HcsOperation,
        security_descriptor: Option<&SecurityDescriptor>,
    ) -> HcsResult<HcsSystem> {
        Ok(HcsSystem {
            handle: computecore::create_compute_system(
                id,
                configuration,
                operation.handle,
                security_descriptor,
            )?,
            handle_policy: HcsWrappedHandleDropPolicy::Close,
        })
    }

    /// Opens a compute system and returns a safe wrapper handle to it.
    pub fn open(id: &str, requested_access: DWord) -> HcsResult<HcsSystem> {
        Ok(HcsSystem {
            handle: computecore::open_compute_system(id, requested_access)?,
            handle_policy: HcsWrappedHandleDropPolicy::Close,
        })
    }

    /// Starts a compute system.
    pub fn start(&self, operation: &HcsOperation, options: Option<&str>) -> HcsResult<()> {
        computecore::start_compute_system(self.handle, operation.handle, options)
    }

    /// Shutdowns a compute system.
    pub fn shutdown(&self, operation: &HcsOperation, options: Option<&str>) -> HcsResult<()> {
        computecore::shutdown_compute_system(self.handle, operation.handle, options)
    }

    /// Terminates a compute system.
    pub fn terminate(&self, operation: &HcsOperation, options: Option<&str>) -> HcsResult<()> {
        computecore::terminate_compute_system(self.handle, operation.handle, options)
    }

    /// Pauses a compute system.
    pub fn pause(&self, operation: &HcsOperation, options: Option<&str>) -> HcsResult<()> {
        computecore::pause_compute_system(self.handle, operation.handle, options)
    }

    /// Resumes a compute system.
    pub fn resume(&self, operation: &HcsOperation, options: Option<&str>) -> HcsResult<()> {
        computecore::resume_compute_system(self.handle, operation.handle, options)
    }

    /// Saves a compute system.
    pub fn save(&self, operation: &HcsOperation, options: Option<&str>) -> HcsResult<()> {
        computecore::save_compute_system(self.handle, operation.handle, options)
    }

    /// Queries for a compute system's properties.
    pub fn get_properties(
        &self,
        operation: &HcsOperation,
        property_query: Option<&str>,
    ) -> HcsResult<()> {
        computecore::get_compute_system_properties(self.handle, operation.handle, property_query)
    }

    /// Modifies a compute system.
    pub fn modify(
        &self,
        operation: &HcsOperation,
        configuration: &str,
        identity: Handle,
    ) -> HcsResult<()> {
        computecore::modify_compute_system(self.handle, operation.handle, configuration, identity)
    }

    /// Sets a callback for this specific compute system, called on key events.
    /// Callback is expected to be a reference to the trait object of the closure.
    ///
    /// Example:
    /// ```rust,ignore
    /// let mut closure = move |operation: &HcsEvent| {};
    /// let mut trait_obj = &mut dyn FnMut(&HcsEvent) = &mut closure;
    /// let trait_obj_ref = &mut trait_obj;
    /// system.set_callback(trait_obj_ref);
    /// ```
    ///
    /// # Safety
    /// Ensure that the supplied callback's closure trait object mutable reference
    /// doesn't change and outlives the HCS system's lifetime.
    pub unsafe fn set_callback<T>(
        &self,
        callback_options: HcsEventOptions,
        callback: &mut &mut T,
    ) -> HcsResult<()>
    where
        T: 'static,
        T: ?Sized,
        T: FnMut(&HcsEvent),
    {
        computecore::set_compute_system_callback(
            self.handle,
            callback_options,
            callback as *mut _ as PVoid,
            Some(hcs_system_and_proces_callback),
        )
    }

    /// Creates and returns a new process in the compute system.
    pub fn create_process(
        &self,
        process_parameters: &str,
        operation: &HcsOperation,
        security_descriptor: Option<&SecurityDescriptor>,
    ) -> HcsResult<HcsProcess> {
        Ok(HcsProcess {
            handle: computecore::create_process(
                self.handle,
                process_parameters,
                operation.handle,
                security_descriptor,
            )?,
            handle_policy: HcsWrappedHandleDropPolicy::Close,
        })
    }

    /// Opens a process in the compute system, that has been created through HCS APIs.
    pub fn open_process(
        &self,
        process_id: DWord,
        requested_access: DWord,
    ) -> HcsResult<HcsProcess> {
        Ok(HcsProcess {
            handle: computecore::open_process(self.handle, process_id, requested_access)?,
            handle_policy: HcsWrappedHandleDropPolicy::Close,
        })
    }

    /// Initializes the device emulator host in the caller's process and associates it
    /// with the specified compute system. This function should be called after the compute system
    /// has been created and before it has been started. The compute system's configuration must
    /// indicate that an external device host for the compute system will be present, by means
    /// of specifying the identity (SID) of the user account under which the device host will execute.
    /// If the device host has not been initialized by the time the compute system starts,
    /// the start operation fails.
    pub fn initialize_device_host(&self) -> HcsResult<HdvHost> {
        HdvHost::new(self.handle)
    }
}

/// Thin wrapper of an HCS Compute System Process that interfaces to all HCS APIs that inherently
/// depend on an HCS Compute System Process handle as input and/or output.
impl HcsProcess {
    /// Terminates a compute system process.
    pub fn terminate(&self, operation: &HcsOperation, options: Option<&str>) -> HcsResult<()> {
        computecore::terminate_process(self.handle, operation.handle, options)
    }

    /// Signals a compute system process.
    pub fn signal(&self, operation: &HcsOperation, options: Option<&str>) -> HcsResult<()> {
        computecore::signal_process(self.handle, operation.handle, options)
    }

    /// Gets basic information of the compute system process.
    pub fn get_info(&self, operation: &HcsOperation) -> HcsResult<()> {
        computecore::get_process_info(self.handle, operation.handle)
    }

    /// Gets properties of the compute system process.
    pub fn get_properties(
        &self,
        operation: &HcsOperation,
        property_query: Option<&str>,
    ) -> HcsResult<()> {
        computecore::get_process_properties(self.handle, operation.handle, property_query)
    }

    /// Modifues the compute system process.
    pub fn modify(&self, operation: &HcsOperation, settings: Option<&str>) -> HcsResult<()> {
        computecore::modify_process(self.handle, operation.handle, settings)
    }

    /// Sets a callback to the compute system process, called on key events.
    /// Callback is expected to be a reference to the trait object of the closure.
    ///
    /// Example:
    /// ```rust,ignore
    /// let mut closure = move |operation: &HcsEvent| {};
    /// let mut trait_obj = &mut dyn FnMut(&HcsEvent) = &mut closure;
    /// let trait_obj_ref = &mut trait_obj;
    /// system_process.set_callback(trait_obj_ref);
    /// ```
    ///
    /// # Safety
    /// Ensure that the supplied callback's closure trait object mutable reference
    /// doesn't change and outlives the HCS process's lifetime.
    pub unsafe fn set_callback<T>(
        &self,
        callback_options: HcsEventOptions,
        callback: &mut &mut T,
    ) -> HcsResult<()>
    where
        T: 'static,
        T: ?Sized,
        T: FnMut(&HcsEvent),
    {
        computecore::set_process_callback(
            self.handle,
            callback_options,
            callback as *mut _ as PVoid,
            Some(hcs_system_and_proces_callback),
        )
    }
}
