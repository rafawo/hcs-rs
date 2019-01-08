// Copyright © rafawo (rafawo1@hotmail.com). All rights reserved.
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.
// All files in the project carrying such notice may not be copied, modified, or distributed
// except according to those terms.
// THE SOURCE CODE IS AVAILABLE UNDER THE ABOVE CHOSEN LICENSE "AS IS", WITH NO WARRANTIES.

//! Rust abstractions of the computecore APIs.
//! These are Rust idiomatic equivalents of the C bindings.

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

pub fn get_operation_context(operation: HcsOperationHandle) -> HcsResult<*mut Void> {
    unsafe { Ok(HcsGetOperationContext(operation)) }
}

pub fn set_operation_context(operation: HcsOperationHandle, context: *mut Void) -> HcsResult<()> {
    unsafe {
        match HcsSetOperationContext(operation, context) {
            0 => Ok(()),
            hresult => Err(hresult_to_result_code(&hresult)),
        }
    }
}

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

pub fn get_process_from_operation(operation: HcsOperationHandle) -> HcsResult<HcsProcessHandle> {
    unsafe {
        match HcsGetProcessFromOperation(operation) {
            handle if handle != std::ptr::null_mut() => Ok(handle),
            _ => Err(ResultCode::Unexpected),
        }
    }
}

pub fn get_operation_type(operation: HcsOperationHandle) -> HcsResult<HcsOperationType> {
    unsafe { Ok(HcsGetOperationType(operation)) }
}

pub fn get_operation_id(operation: HcsOperationHandle) -> HcsResult<u64> {
    unsafe { Ok(HcsGetOperationId(operation)) }
}

pub fn get_operation_result(operation: HcsOperationHandle) -> HcsResult<String> {
    let result_document_ptr: *mut PWStr = std::ptr::null_mut();

    unsafe {
        match HcsGetOperationResult(operation, result_document_ptr) {
            0 => {
                let result_document =
                    WideCString::from_ptr_str(*result_document_ptr).to_string_lossy();
                winapi::um::combaseapi::CoTaskMemFree(result_document_ptr as LPVoid);
                Ok(result_document)
            }
            hresult => Err(hresult_to_result_code(&hresult)),
        }
    }
}

pub fn get_operation_result_and_process_info(
    operation: HcsOperationHandle,
) -> HcsResult<(HcsProcessInformation, String)> {
    let result_document_ptr: *mut PWStr = std::ptr::null_mut();

    unsafe {
        let mut process_info = std::mem::zeroed::<HcsProcessInformation>();

        match HcsGetOperationResultAndProcessInfo(operation, &mut process_info, result_document_ptr)
        {
            0 => {
                let result_document =
                    WideCString::from_ptr_str(*result_document_ptr).to_string_lossy();
                winapi::um::combaseapi::CoTaskMemFree(result_document_ptr as LPVoid);
                Ok((process_info, result_document))
            }
            hresult => Err(hresult_to_result_code(&hresult)),
        }
    }
}

pub fn wait_for_operation_result(
    operation: HcsOperationHandle,
    timeout_ms: DWord,
) -> HcsResult<String> {
    let result_document_ptr: *mut PWStr = std::ptr::null_mut();

    unsafe {
        match HcsWaitForOperationResult(operation, timeout_ms, result_document_ptr) {
            0 => {
                let result_document =
                    WideCString::from_ptr_str(*result_document_ptr).to_string_lossy();
                winapi::um::combaseapi::CoTaskMemFree(result_document_ptr as LPVoid);
                Ok(result_document)
            }
            hresult => Err(hresult_to_result_code(&hresult)),
        }
    }
}

pub fn wait_for_operation_result_and_process_info(
    operation: HcsOperationHandle,
    timeout_ms: DWord,
) -> HcsResult<(HcsProcessInformation, String)> {
    let result_document_ptr: *mut PWStr = std::ptr::null_mut();

    unsafe {
        let mut process_info = std::mem::zeroed::<HcsProcessInformation>();

        match HcsWaitForOperationResultAndProcessInfo(
            operation,
            timeout_ms,
            &mut process_info,
            result_document_ptr,
        ) {
            0 => {
                let result_document =
                    WideCString::from_ptr_str(*result_document_ptr).to_string_lossy();
                winapi::um::combaseapi::CoTaskMemFree(result_document_ptr as LPVoid);
                Ok((process_info, result_document))
            }
            hresult => Err(hresult_to_result_code(&hresult)),
        }
    }
}

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

pub fn cancel_operation(operation: HcsOperationHandle) -> HcsResult<()> {
    unsafe {
        match HcsCancelOperation(operation) {
            0 => Ok(()),
            hresult => Err(hresult_to_result_code(&hresult)),
        }
    }
}

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

pub fn close_compute_system(compute_system: HcsSystemHandle) -> HcsResult<()> {
    unsafe {
        HcsCloseComputeSystem(compute_system);
        Ok(())
    }
}

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

pub fn close_process(process: HcsProcessHandle) -> HcsResult<()> {
    unsafe {
        HcsCloseProcess(process);
        Ok(())
    }
}

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

pub fn get_process_info(process: HcsProcessHandle, operation: HcsOperationHandle) -> HcsResult<()> {
    unsafe {
        match HcsGetProcessInfo(process, operation) {
            0 => Ok(()),
            hresult => Err(hresult_to_result_code(&hresult)),
        }
    }
}

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

pub fn get_service_properties(property_query: &str) -> HcsResult<String> {
    unsafe {
        let result_ptr: *mut PWStr = std::ptr::null_mut();

        match HcsGetServiceProperties(
            WideCString::from_str(property_query).unwrap().as_ptr(),
            result_ptr,
        ) {
            0 => {
                let result = WideCString::from_ptr_str(*result_ptr).to_string_lossy();
                winapi::um::combaseapi::CoTaskMemFree(result_ptr as LPVoid);
                Ok(result)
            }
            hresult => Err(hresult_to_result_code(&hresult)),
        }
    }
}

pub fn modify_service_settings(settings: &str) -> HcsResult<String> {
    unsafe {
        let result_ptr: *mut PWStr = std::ptr::null_mut();

        match HcsModifyServiceSettings(
            WideCString::from_str(settings).unwrap().as_ptr(),
            result_ptr,
        ) {
            0 => {
                let result = WideCString::from_ptr_str(*result_ptr).to_string_lossy();
                winapi::um::combaseapi::CoTaskMemFree(result_ptr as LPVoid);
                Ok(result)
            }
            hresult => Err(hresult_to_result_code(&hresult)),
        }
    }
}

pub fn submit_wer_report(settings: &str) -> HcsResult<()> {
    unsafe {
        match HcsSubmitWerReport(WideCString::from_str(settings).unwrap().as_ptr()) {
            0 => Ok(()),
            hresult => Err(hresult_to_result_code(&hresult)),
        }
    }
}

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
