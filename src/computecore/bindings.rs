// Copyright © rafawo (rafawo1@hotmail.com). All rights reserved.
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.
// All files in the project carrying such notice may not be copied, modified, or distributed
// except according to those terms.
// THE SOURCE CODE IS AVAILABLE UNDER THE ABOVE CHOSEN LICENSE "AS IS", WITH NO WARRANTIES.

//! Rust FFI bindings for the public computecore APIs.

use crate::compute::defs::*;
use winutils_rs::windefs::*;

#[link(name = "computecore")]
extern "system" {
    /// Enumerates existing compute systems.
    pub fn HcsEnumerateComputeSystems(query: PCWStr, operation: HcsOperationHandle) -> HResult;

    /// Creates a new operation.
    pub fn HcsCreateOperation(
        context: *mut Void,
        callback: HcsOperationCompletion,
    ) -> HcsOperationHandle;

    /// Closes an operation.
    pub fn HcsCloseOperation(operation: HcsOperationHandle);

    /// Returns the context pointer of an operation.
    pub fn HcsGetOperationContext(operation: HcsOperationHandle) -> *mut Void;

    /// Sets the context pointer for an operation.
    pub fn HcsSetOperationContext(operation: HcsOperationHandle, context: *mut Void) -> HResult;

    /// Returns the handle to compute system associated with an operation.
    pub fn HcsGetComputeSystemFromOperation(operation: HcsOperationHandle) -> HcsSystemHandle;

    /// Returns the handle to the process associated with an operation
    pub fn HcsGetProcessFromOperation(operation: HcsOperationHandle) -> HcsProcessHandle;

    /// Returns the type of an operation.
    pub fn HcsGetOperationType(operation: HcsOperationHandle) -> HcsOperationType;

    /// Returns the ID that uniquely identifies an operation.
    pub fn HcsGetOperationId(operation: HcsOperationHandle) -> u64;

    /// Returns the result of an operation.
    pub fn HcsGetOperationResult(
        operation: HcsOperationHandle,
        resultDocument: *mut PWStr,
    ) -> HResult;

    /// Returns the result of an operation, including the process information for HcsCreateProcess
    /// and HcsGetProcessInfo.
    pub fn HcsGetOperationResultAndProcessInfo(
        operation: HcsOperationHandle,
        processInformation: *mut HcsProcessInformation,
        resultDocument: *mut PWStr,
    ) -> HResult;

    /// Waits for the completion of an operation and returns the result.
    pub fn HcsWaitForOperationResult(
        operation: HcsOperationHandle,
        timeoutMs: DWord,
        resultDocument: *mut PWStr,
    ) -> HResult;

    /// Waits for the completion of an operation and returns the result, including the process information
    /// for HcsCreateProcess and HcsGetProcessInfo.
    pub fn HcsWaitForOperationResultAndProcessInfo(
        operation: HcsOperationHandle,
        timeoutMs: DWord,
        processInformation: *mut HcsProcessInformation,
        resultDocument: *mut PWStr,
    ) -> HResult;

    /// Sets a callback that is invoked on completion of an operation.
    pub fn HcsSetOperationCallback(
        operation: HcsOperationHandle,
        context: *mut Void,
        callback: HcsOperationCompletion,
    ) -> HResult;

    /// Cancels an operation
    pub fn HcsCancelOperation(operation: HcsOperationHandle) -> HResult;

    /// Creates a new compute system.
    pub fn HcsCreateComputeSystem(
        id: PCWStr,
        configuration: PCWStr,
        operation: HcsOperationHandle,
        securityDescriptor: *const SecurityDescriptor,
        computeSystem: *mut HcsSystemHandle,
    ) -> HResult;

    /// Opens a handle to an existing compute system.
    pub fn HcsOpenComputeSystem(
        id: PCWStr,
        requestedAccess: DWord,
        computeSystem: *mut HcsSystemHandle,
    ) -> HResult;

    /// Closes a handle to a compute system.
    pub fn HcsCloseComputeSystem(computeSystem: HcsSystemHandle);

    /// Starts a compute system.
    pub fn HcsStartComputeSystem(
        computeSystem: HcsSystemHandle,
        operation: HcsOperationHandle,
        options: PCWStr,
    ) -> HResult;

    /// Cleanly shuts down a compute system.
    pub fn HcsShutDownComputeSystem(
        computeSystem: HcsSystemHandle,
        operation: HcsOperationHandle,
        options: PCWStr,
    ) -> HResult;

    /// Forcefully terminates a compute system.
    pub fn HcsTerminateComputeSystem(
        computeSystem: HcsSystemHandle,
        operation: HcsOperationHandle,
        options: PCWStr,
    ) -> HResult;

    /// Pauses the execution of a compute system.
    pub fn HcsPauseComputeSystem(
        computeSystem: HcsSystemHandle,
        operation: HcsOperationHandle,
        options: PCWStr,
    ) -> HResult;

    /// Resumes the execution of a compute system.
    pub fn HcsResumeComputeSystem(
        computeSystem: HcsSystemHandle,
        operation: HcsOperationHandle,
        options: PCWStr,
    ) -> HResult;

    /// Saves the state of a compute system.
    pub fn HcsSaveComputeSystem(
        computeSystem: HcsSystemHandle,
        operation: HcsOperationHandle,
        options: PCWStr,
    ) -> HResult;

    /// Returns properties of a compute system.
    pub fn HcsGetComputeSystemProperties(
        computeSystem: HcsSystemHandle,
        operation: HcsOperationHandle,
        propertyQuery: PCWStr,
    ) -> HResult;

    /// Modifies settings of a compute system.
    pub fn HcsModifyComputeSystem(
        computeSystem: HcsSystemHandle,
        operation: HcsOperationHandle,
        configuration: PCWStr,
        identity: Handle,
    ) -> HResult;

    /// Registers a callback function to receive notifications for the compute system.
    pub fn HcsSetComputeSystemCallback(
        computeSystem: HcsSystemHandle,
        callbackOptions: HcsEventOptions,
        context: *mut Void,
        callback: HcsEventCallback,
    ) -> HResult;

    /// Starts a process in a compute system.
    pub fn HcsCreateProcess(
        computeSystem: HcsSystemHandle,
        processParameters: PCWStr,
        operation: HcsOperationHandle,
        securityDescriptor: *const SecurityDescriptor,
        process: *mut HcsProcessHandle,
    ) -> HResult;

    /// Opens an existing process in a compute system
    pub fn HcsOpenProcess(
        computeSystem: HcsSystemHandle,
        processId: DWord,
        requestedAccess: DWord,
        process: *mut HcsProcessHandle,
    ) -> HResult;

    /// Closes the handle to a process in a compute system
    pub fn HcsCloseProcess(process: HcsProcessHandle);

    /// Terminates a process in a compute system
    pub fn HcsTerminateProcess(
        process: HcsProcessHandle,
        operation: HcsOperationHandle,
        options: PCWStr,
    ) -> HResult;

    /// Sends a signal to a process in a compute system
    pub fn HcsSignalProcess(
        process: HcsProcessHandle,
        operation: HcsOperationHandle,
        options: PCWStr,
    ) -> HResult;

    /// Returns the initial startup info of a process in a compute system
    pub fn HcsGetProcessInfo(process: HcsProcessHandle, operation: HcsOperationHandle) -> HResult;

    /// Returns the properties of a process in a compute system
    pub fn HcsGetProcessProperties(
        process: HcsProcessHandle,
        operation: HcsOperationHandle,
        propertyQuery: PCWStr,
    ) -> HResult;

    /// Modifies the parameters of a process in a compute system
    pub fn HcsModifyProcess(
        process: HcsProcessHandle,
        operation: HcsOperationHandle,
        settings: PCWStr,
    ) -> HResult;

    /// Registers a callback function to receive notifications for a process in a compute system
    pub fn HcsSetProcessCallback(
        process: HcsProcessHandle,
        callbackOptions: HcsEventOptions,
        context: *mut Void,
        callback: HcsEventCallback,
    ) -> HResult;

    /// Returns properties of the Host Compute Service
    pub fn HcsGetServiceProperties(propertyQuery: PCWStr, result: *mut PWStr) -> HResult;

    /// Modifies settings of the Host Compute Service
    pub fn HcsModifyServiceSettings(settings: PCWStr, result: *mut PWStr) -> HResult;

    /// Submits a WER report
    pub fn HcsSubmitWerReport(settings: PCWStr) -> HResult;

    /// Creates an empty guest-state file (.vmgs) for a VM.
    pub fn HcsCreateEmptyGuestStateFile(guestStateFilePath: PCWStr) -> HResult;

    /// Creates an empty runtime-state file (.vmrs) for a VM.
    pub fn HcsCreateEmptyRuntimeStateFile(runtimeStateFilePath: PCWStr) -> HResult;

    /// Adds an entry to a file's ACL that grants access for a VM.
    pub fn HcsGrantVmAccess(vmId: PCWStr, filePath: PCWStr) -> HResult;

    /// Removes an entry to a file's ACL that granted access for a VM.
    pub fn HcsRevokeVmAccess(vmId: PCWStr, filePath: PCWStr) -> HResult;
}
