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
