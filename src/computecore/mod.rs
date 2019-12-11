// Copyright © rafawo (rafawo1@hotmail.com). All rights reserved.
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.
// All files in the project carrying such notice may not be copied, modified, or distributed
// except according to those terms.
// THE SOURCE CODE IS AVAILABLE UNDER THE ABOVE CHOSEN LICENSE "AS IS", WITH NO WARRANTIES.

//! Rust abstractions of the computecore APIs.
//! These are Rust idiomatic equivalents of the C bindings.

#[cfg(not(feature = "bindings"))]
pub(crate) mod bindings;
#[cfg(feature = "bindings")]
pub mod bindings;

pub mod utilities;

use crate::compute::defs::*;
use crate::compute::errorcodes::{hresult_to_result_code, ResultCode};
use crate::computecore::bindings::*;
use crate::HcsResult;
use widestring::WideCString;
use winutils_rs::utilities::LocalWString;
use winutils_rs::windefs::*;

/// Enumerates all compute systems visible to the caller.
pub fn enumerate_compute_systems(query: &str, operation: HcsOperationHandle) -> HcsResult<()> {
    unsafe {
        match HcsEnumerateComputeSystems(WideCString::from_str(query).unwrap().as_ptr(), operation)
        {
            0 => Ok(()),
            hresult => Err(hresult_to_result_code(&hresult)),
        }
    }
}

/// Creates an operation, used to track an HCS API call.
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

/// Closes an operation, freeing resources used to track an HCS API call.
pub fn close_operation(operation: HcsOperationHandle) -> HcsResult<()> {
    unsafe {
        HcsCloseOperation(operation);
        Ok(())
    }
}

/// Returns the operation context as a raw pointer.
pub fn get_operation_context(operation: HcsOperationHandle) -> HcsResult<*mut Void> {
    unsafe { Ok(HcsGetOperationContext(operation)) }
}

/// Sets the operation context, supplied as a raw pointer.
pub fn set_operation_context(operation: HcsOperationHandle, context: *mut Void) -> HcsResult<()> {
    unsafe {
        match HcsSetOperationContext(operation, context) {
            0 => Ok(()),
            hresult => Err(hresult_to_result_code(&hresult)),
        }
    }
}

/// Returns the compute system handle associated to a given operation.
pub fn get_compute_system_from_operation(
    operation: HcsOperationHandle,
) -> HcsResult<HcsSystemHandle> {
    unsafe {
        match HcsGetComputeSystemFromOperation(operation) {
            handle if handle != std::ptr::null_mut() => Ok(handle),
            _ => Err(ResultCode::HcsSystemNotFound),
        }
    }
}

/// Returns the process handle associated to a given operation.
pub fn get_process_from_operation(operation: HcsOperationHandle) -> HcsResult<HcsProcessHandle> {
    unsafe {
        match HcsGetProcessFromOperation(operation) {
            handle if handle != std::ptr::null_mut() => Ok(handle),
            _ => Err(ResultCode::Unexpected),
        }
    }
}

/// Returns the operation type.
pub fn get_operation_type(operation: HcsOperationHandle) -> HcsResult<HcsOperationType> {
    unsafe { Ok(HcsGetOperationType(operation)) }
}

/// Returns the operation ID. Assigned and valid only after an operation has been passed in
/// to an HCS API.
pub fn get_operation_id(operation: HcsOperationHandle) -> HcsResult<u64> {
    unsafe { Ok(HcsGetOperationId(operation)) }
}

/// Returns the operation result as a JSON document.
pub fn get_operation_result(operation: HcsOperationHandle) -> (String, HcsResult<()>) {
    let mut result_document = LocalWString::new();

    unsafe {
        match HcsGetOperationResult(operation, result_document.ptr_mut()) {
            0 => (result_document.to_string(), Ok(())),
            hresult => (
                result_document.to_string(),
                Err(hresult_to_result_code(&hresult)),
            ),
        }
    }
}

/// Returns the operation result as a JSON document and the process info.
/// Only valid if operation was used to track an HCS Process API call.
pub fn get_operation_result_and_process_info(
    operation: HcsOperationHandle,
) -> (String, HcsResult<HcsProcessInformation>) {
    let mut result_document = LocalWString::new();

    unsafe {
        let mut process_info = std::mem::zeroed::<HcsProcessInformation>();

        match HcsGetOperationResultAndProcessInfo(
            operation,
            &mut process_info,
            result_document.ptr_mut(),
        ) {
            0 => (result_document.to_string(), Ok(process_info)),
            hresult => (
                result_document.to_string(),
                Err(hresult_to_result_code(&hresult)),
            ),
        }
    }
}

/// Waits synchronously for an operation to complete and returns the result as a JSON document.
pub fn wait_for_operation_result(
    operation: HcsOperationHandle,
    timeout_ms: DWord,
) -> (String, HcsResult<()>) {
    let mut result_document = LocalWString::new();

    unsafe {
        match HcsWaitForOperationResult(operation, timeout_ms, result_document.ptr_mut()) {
            0 => (result_document.to_string(), Ok(())),
            hresult => (
                result_document.to_string(),
                Err(hresult_to_result_code(&hresult)),
            ),
        }
    }
}

/// Waits syncrhonously for an operation to complete and returns the result as a JSON document,
/// and the process info. Only valid if operation was used to track an HCS Process API call.
pub fn wait_for_operation_result_and_process_info(
    operation: HcsOperationHandle,
    timeout_ms: DWord,
) -> (String, HcsResult<HcsProcessInformation>) {
    let mut result_document = LocalWString::new();

    unsafe {
        let mut process_info = std::mem::zeroed::<HcsProcessInformation>();

        match HcsWaitForOperationResultAndProcessInfo(
            operation,
            timeout_ms,
            &mut process_info,
            result_document.ptr_mut(),
        ) {
            0 => (result_document.to_string(), Ok(process_info)),
            hresult => (
                result_document.to_string(),
                Err(hresult_to_result_code(&hresult)),
            ),
        }
    }
}

/// Sets an operation callback for when it completes.
pub fn set_operation_callback(
    operation: HcsOperationHandle,
    context: *mut Void,
    callback: HcsOperationCompletion,
) -> HcsResult<()> {
    unsafe {
        match HcsSetOperationCallback(operation, context, callback) {
            0 => Ok(()),
            hresult => Err(hresult_to_result_code(&hresult)),
        }
    }
}

/// Cancels an operation.
pub fn cancel_operation(operation: HcsOperationHandle) -> HcsResult<()> {
    unsafe {
        match HcsCancelOperation(operation) {
            0 => Ok(()),
            hresult => Err(hresult_to_result_code(&hresult)),
        }
    }
}

/// Creates a compute system with the given JSON document describing its configuration.
/// Optionally set security descriptor, which determines who can open a handle to the system.
pub fn create_compute_system(
    id: &str,
    configuration: &str,
    operation: HcsOperationHandle,
    security_descriptor: Option<&SecurityDescriptor>,
) -> HcsResult<HcsSystemHandle> {
    let mut compute_system_handle: HcsSystemHandle = std::ptr::null_mut();

    let security_descriptor_ptr = match security_descriptor {
        Some(security_descriptor) => security_descriptor as *const SecurityDescriptor,
        None => std::ptr::null_mut(),
    };

    unsafe {
        match HcsCreateComputeSystem(
            WideCString::from_str(id).unwrap().as_ptr(),
            WideCString::from_str(configuration).unwrap().as_ptr(),
            operation,
            security_descriptor_ptr,
            &mut compute_system_handle,
        ) {
            0 => Ok(compute_system_handle),
            hresult => Err(hresult_to_result_code(&hresult)),
        }
    }
}

/// Opens a compute system and returns a handle to it, with the specified access rights.
///
/// # Note
/// Requested access must be a valid `ACCESS_MASK` DWORD value.
/// `GENERIC_ALL` (0x10000000) is the most common access level to request.
/// Refer to https://docs.microsoft.com/en-us/openspecs/windows_protocols/ms-samr/262970b7-cd4a-41f4-8c4d-5a27f0092aaa
pub fn open_compute_system(id: &str, requested_access: DWord) -> HcsResult<HcsSystemHandle> {
    let mut compute_system_handle: HcsSystemHandle = std::ptr::null_mut();

    unsafe {
        match HcsOpenComputeSystem(
            WideCString::from_str(id).unwrap().as_ptr(),
            requested_access,
            &mut compute_system_handle,
        ) {
            0 => Ok(compute_system_handle),
            hresult => Err(hresult_to_result_code(&hresult)),
        }
    }
}

/// Closes a handle to a compute system.
///
/// # Note
/// In some cases, a compute system could have been created with the flag `TerminateOnLastHandleClosed`,
/// which would cause the system to be tore down forcefully.
pub fn close_compute_system(compute_system: HcsSystemHandle) -> HcsResult<()> {
    unsafe {
        HcsCloseComputeSystem(compute_system);
        Ok(())
    }
}

/// Starts a compute system.
/// Optionally supply additional options as a JSON document.
pub fn start_compute_system(
    compute_system: HcsSystemHandle,
    operation: HcsOperationHandle,
    options: Option<&str>,
) -> HcsResult<()> {
    let options_str = match options {
        Some(options) => options,
        None => "",
    };

    unsafe {
        match HcsStartComputeSystem(
            compute_system,
            operation,
            WideCString::from_str(options_str).unwrap().as_ptr(),
        ) {
            0 => Ok(()),
            hresult => Err(hresult_to_result_code(&hresult)),
        }
    }
}

/// Shutdowns a compute system.
/// Optionally supply additional options as a JSON document.
///
/// # Note
/// Cleanly shutting down implies that the compute system's guest operating system
/// will be requested to shut down. This API call fails if such request can't be
/// serviced by the guest operating system.
pub fn shutdown_compute_system(
    compute_system: HcsSystemHandle,
    operation: HcsOperationHandle,
    options: Option<&str>,
) -> HcsResult<()> {
    let options_str = match options {
        Some(options) => options,
        None => "",
    };

    unsafe {
        match HcsShutDownComputeSystem(
            compute_system,
            operation,
            WideCString::from_str(options_str).unwrap().as_ptr(),
        ) {
            0 => Ok(()),
            hresult => Err(hresult_to_result_code(&hresult)),
        }
    }
}

/// Terminates a compute system.
/// Optionally supply additional options as a JSON document.
///
/// # Note
/// Terminating a compute system is a last resort mechanism to force it
/// to be shut down and cleaned up. This won't ask the guest operating system
/// to cleanly shut down, and should be treated as an equivalent to unplugging
/// hardware. This means that it could lead to data loss.
pub fn terminate_compute_system(
    compute_system: HcsSystemHandle,
    operation: HcsOperationHandle,
    options: Option<&str>,
) -> HcsResult<()> {
    let options_str = match options {
        Some(options) => options,
        None => "",
    };

    unsafe {
        match HcsTerminateComputeSystem(
            compute_system,
            operation,
            WideCString::from_str(options_str).unwrap().as_ptr(),
        ) {
            0 => Ok(()),
            hresult => Err(hresult_to_result_code(&hresult)),
        }
    }
}

/// Pauses a compute system.
/// Optionally supply additional options as a JSON document.
///
/// # Note
/// Not all compute systems support pause, and this API is expected to fail.
pub fn pause_compute_system(
    compute_system: HcsSystemHandle,
    operation: HcsOperationHandle,
    options: Option<&str>,
) -> HcsResult<()> {
    let options_str = match options {
        Some(options) => options,
        None => "",
    };

    unsafe {
        match HcsPauseComputeSystem(
            compute_system,
            operation,
            WideCString::from_str(options_str).unwrap().as_ptr(),
        ) {
            0 => Ok(()),
            hresult => Err(hresult_to_result_code(&hresult)),
        }
    }
}

/// Resumes a compute system.
/// Optionally supply additional options as a JSON document.
///
/// # Note
/// Not all compute systems support resume, and this API is expected to fail.
pub fn resume_compute_system(
    compute_system: HcsSystemHandle,
    operation: HcsOperationHandle,
    options: Option<&str>,
) -> HcsResult<()> {
    let options_str = match options {
        Some(options) => options,
        None => "",
    };

    unsafe {
        match HcsResumeComputeSystem(
            compute_system,
            operation,
            WideCString::from_str(options_str).unwrap().as_ptr(),
        ) {
            0 => Ok(()),
            hresult => Err(hresult_to_result_code(&hresult)),
        }
    }
}

/// Saves a compute system.
/// Optionally supply additional options as a JSON document.
///
/// # Note
/// Not all compute systems support save, and this API is expected to fail.
pub fn save_compute_system(
    compute_system: HcsSystemHandle,
    operation: HcsOperationHandle,
    options: Option<&str>,
) -> HcsResult<()> {
    let options_str = match options {
        Some(options) => options,
        None => "",
    };

    unsafe {
        match HcsSaveComputeSystem(
            compute_system,
            operation,
            WideCString::from_str(options_str).unwrap().as_ptr(),
        ) {
            0 => Ok(()),
            hresult => Err(hresult_to_result_code(&hresult)),
        }
    }
}

/// Requests for a compute system's properties.
/// Optionally supply a JSON document to request specific properties that are not included by default.
pub fn get_compute_system_properties(
    compute_system: HcsSystemHandle,
    operation: HcsOperationHandle,
    property_query: Option<&str>,
) -> HcsResult<()> {
    let property_query_str = match property_query {
        Some(property_query) => property_query,
        None => "",
    };

    unsafe {
        match HcsGetComputeSystemProperties(
            compute_system,
            operation,
            WideCString::from_str(property_query_str).unwrap().as_ptr(),
        ) {
            0 => Ok(()),
            hresult => Err(hresult_to_result_code(&hresult)),
        }
    }
}

/// Modifies a compute system, described by the supplied JSON document configuration.
/// Optionally supply a handle to an identity, which is used to impersonate the specific modify request.
///
/// # Note
/// The JSON document used for a modify request can be of various types of resource types,
/// referenced by resource URI as they would correspond on the compute system's configuration JSON document.
/// Specific data to the resource to modify dictates how the JSON document will look like.
/// Not all resource URI are equally supported by all compute systems.
pub fn modify_compute_system(
    compute_system: HcsSystemHandle,
    operation: HcsOperationHandle,
    configuration: &str,
    identity: Handle,
) -> HcsResult<()> {
    unsafe {
        match HcsModifyComputeSystem(
            compute_system,
            operation,
            WideCString::from_str(configuration).unwrap().as_ptr(),
            identity,
        ) {
            0 => Ok(()),
            hresult => Err(hresult_to_result_code(&hresult)),
        }
    }
}

/// Sets a per compute system callback, that is used to receive 'notifications' of different
/// events that can happen on a compute system.
/// Optionally supply a context as a raw pointer, which will be included in the callback function.
pub fn set_compute_system_callback(
    compute_system: HcsSystemHandle,
    callback_options: HcsEventOptions,
    context: *mut Void,
    callback: HcsEventCallback,
) -> HcsResult<()> {
    unsafe {
        match HcsSetComputeSystemCallback(compute_system, callback_options, context, callback) {
            0 => Ok(()),
            hresult => Err(hresult_to_result_code(&hresult)),
        }
    }
}

/// Creates a process on a compute system, described by the supplied JSON document.
/// Optionally set security descriptor, which determines who can open a handle to the process.
pub fn create_process(
    compute_system: HcsSystemHandle,
    process_parameters: &str,
    operation: HcsOperationHandle,
    security_descriptor: Option<&SecurityDescriptor>,
) -> HcsResult<HcsProcessHandle> {
    let mut process_handle: HcsProcessHandle = std::ptr::null_mut();

    let security_descriptor_ptr = match security_descriptor {
        Some(security_descriptor) => security_descriptor as *const SecurityDescriptor,
        None => std::ptr::null_mut(),
    };

    unsafe {
        match HcsCreateProcess(
            compute_system,
            WideCString::from_str(process_parameters).unwrap().as_ptr(),
            operation,
            security_descriptor_ptr,
            &mut process_handle,
        ) {
            0 => Ok(process_handle),
            hresult => Err(hresult_to_result_code(&hresult)),
        }
    }
}

/// Opens a handle to a process, from a given compute system, with the specified access rights.
///
/// # Note
/// Requested access must be a valid `ACCESS_MASK` DWORD value.
/// `GENERIC_ALL` (0x10000000) is the most common access level to request.
/// Refer to https://docs.microsoft.com/en-us/openspecs/windows_protocols/ms-samr/262970b7-cd4a-41f4-8c4d-5a27f0092aaa
pub fn open_process(
    compute_system: HcsSystemHandle,
    process_id: DWord,
    requested_access: DWord,
) -> HcsResult<HcsProcessHandle> {
    let mut process_handle: HcsProcessHandle = std::ptr::null_mut();

    unsafe {
        match HcsOpenProcess(
            compute_system,
            process_id,
            requested_access,
            &mut process_handle,
        ) {
            0 => Ok(process_handle),
            hresult => Err(hresult_to_result_code(&hresult)),
        }
    }
}

/// Closes a handle to a process running in a compute system.
///
/// # Note
/// If after closing a handle to the process it will be opened at a later point,
/// make sure to save the process ID since that's required to correctly find
/// such process. Otherwise, there's no way to open the process again.
pub fn close_process(process: HcsProcessHandle) -> HcsResult<()> {
    unsafe {
        HcsCloseProcess(process);
        Ok(())
    }
}

/// Terminates a process running in a compute system.
/// Optionally supply options described as a JSON document.
pub fn terminate_process(
    process: HcsProcessHandle,
    operation: HcsOperationHandle,
    options: Option<&str>,
) -> HcsResult<()> {
    let options_str = match options {
        Some(options) => options,
        None => "",
    };

    unsafe {
        match HcsTerminateProcess(
            process,
            operation,
            WideCString::from_str(options_str).unwrap().as_ptr(),
        ) {
            0 => Ok(()),
            hresult => Err(hresult_to_result_code(&hresult)),
        }
    }
}

/// Signals a process running in a compute system.
/// Describe the signal to send as additional options in a JSON document.
pub fn signal_process(
    process: HcsProcessHandle,
    operation: HcsOperationHandle,
    options: Option<&str>,
) -> HcsResult<()> {
    let options_str = match options {
        Some(options) => options,
        None => "",
    };

    unsafe {
        match HcsSignalProcess(
            process,
            operation,
            WideCString::from_str(options_str).unwrap().as_ptr(),
        ) {
            0 => Ok(()),
            hresult => Err(hresult_to_result_code(&hresult)),
        }
    }
}

/// Returns basic information about a process running on a compute system.
pub fn get_process_info(process: HcsProcessHandle, operation: HcsOperationHandle) -> HcsResult<()> {
    unsafe {
        match HcsGetProcessInfo(process, operation) {
            0 => Ok(()),
            hresult => Err(hresult_to_result_code(&hresult)),
        }
    }
}

/// Returns properties of a process running on a compute system.
/// Optionally supply a JSON document to request specific properties that are not included by default.
pub fn get_process_properties(
    process: HcsProcessHandle,
    operation: HcsOperationHandle,
    property_query: Option<&str>,
) -> HcsResult<()> {
    let property_query_str = match property_query {
        Some(property_query) => property_query,
        None => "",
    };

    unsafe {
        match HcsGetProcessProperties(
            process,
            operation,
            WideCString::from_str(property_query_str).unwrap().as_ptr(),
        ) {
            0 => Ok(()),
            hresult => Err(hresult_to_result_code(&hresult)),
        }
    }
}

/// Requests to modify a process running on a compute system.
/// Supply the modify request as a JSON document.
pub fn modify_process(
    process: HcsProcessHandle,
    operation: HcsOperationHandle,
    settings: Option<&str>,
) -> HcsResult<()> {
    let settings_str = match settings {
        Some(settings) => settings,
        None => "",
    };

    unsafe {
        match HcsModifyProcess(
            process,
            operation,
            WideCString::from_str(settings_str).unwrap().as_ptr(),
        ) {
            0 => Ok(()),
            hresult => Err(hresult_to_result_code(&hresult)),
        }
    }
}

/// Sets a per process callback, that is used to receive 'notifications' of different
/// events that can happen on a process running in a compute system.
/// Optionally supply a context as a raw pointer, which will be included in the callback function.
pub fn set_process_callback(
    process: HcsProcessHandle,
    callback_options: HcsEventOptions,
    context: *mut Void,
    callback: HcsEventCallback,
) -> HcsResult<()> {
    unsafe {
        match HcsSetProcessCallback(process, callback_options, context, callback) {
            0 => Ok(()),
            hresult => Err(hresult_to_result_code(&hresult)),
        }
    }
}

/// Returns Host Compute Service properties as a JSON document.
/// Determine what to query through a JSON document.
pub fn get_service_properties(property_query: &str) -> HcsResult<String> {
    unsafe {
        let mut result = LocalWString::new();

        match HcsGetServiceProperties(
            WideCString::from_str(property_query).unwrap().as_ptr(),
            result.ptr_mut(),
        ) {
            0 => Ok(result.to_string()),
            hresult => Err(hresult_to_result_code(&hresult)),
        }
    }
}

/// Modifies Host Compute Service - wide settings, described by the supplied JSON document.
pub fn modify_service_settings(settings: &str) -> HcsResult<String> {
    unsafe {
        let mut result = LocalWString::new();

        match HcsModifyServiceSettings(
            WideCString::from_str(settings).unwrap().as_ptr(),
            result.ptr_mut(),
        ) {
            0 => Ok(result.to_string()),
            hresult => Err(hresult_to_result_code(&hresult)),
        }
    }
}

/// Submits a **Windows Error Reporting** report, described by the supplied JSON document.
///
/// # Note
/// Refer to https://docs.microsoft.com/en-us/windows/desktop/wer/windows-error-reporting
/// for detailed information about WER.
pub fn submit_wer_report(settings: &str) -> HcsResult<()> {
    unsafe {
        match HcsSubmitWerReport(WideCString::from_str(settings).unwrap().as_ptr()) {
            0 => Ok(()),
            hresult => Err(hresult_to_result_code(&hresult)),
        }
    }
}

/// Creates an empty VMGS (Virtual Machine Guest State) file, that can be used later
/// when creating a Virtual Machine compute system to store guest specific information.
pub fn create_empty_guest_state_file(guest_state_file_path: &str) -> HcsResult<()> {
    unsafe {
        match HcsCreateEmptyGuestStateFile(
            WideCString::from_str(guest_state_file_path)
                .unwrap()
                .as_ptr(),
        ) {
            0 => Ok(()),
            hresult => Err(hresult_to_result_code(&hresult)),
        }
    }
}

/// Creates an empty VMRS (Virtual Machine Runtime State) file, that can be used later
/// when saving a Virtual Machine compute system to store save state information.
/// This can be then used to restore runtime state of a Virtual Machine when creating
/// a new compute system based on this.
pub fn create_empty_runtime_state_file(runtime_state_file_path: &str) -> HcsResult<()> {
    unsafe {
        match HcsCreateEmptyRuntimeStateFile(
            WideCString::from_str(runtime_state_file_path)
                .unwrap()
                .as_ptr(),
        ) {
            0 => Ok(()),
            hresult => Err(hresult_to_result_code(&hresult)),
        }
    }
}

/// Grants VM access to a file. This allows Virtual Machine compute systems to have
/// access during runtime to a file (like virtual hard disks).
pub fn grant_vm_access(vm_id: &str, file_path: &str) -> HcsResult<()> {
    unsafe {
        match HcsGrantVmAccess(
            WideCString::from_str(vm_id).unwrap().as_ptr(),
            WideCString::from_str(file_path).unwrap().as_ptr(),
        ) {
            0 => Ok(()),
            hresult => Err(hresult_to_result_code(&hresult)),
        }
    }
}

/// Revokes VM access to a file. This prevents Virtual Machine compute systems to have
/// access during runtime to a file (like virtual hard disks).
pub fn revoke_vm_access(vm_id: &str, file_path: &str) -> HcsResult<()> {
    unsafe {
        match HcsRevokeVmAccess(
            WideCString::from_str(vm_id).unwrap().as_ptr(),
            WideCString::from_str(file_path).unwrap().as_ptr(),
        ) {
            0 => Ok(()),
            hresult => Err(hresult_to_result_code(&hresult)),
        }
    }
}
