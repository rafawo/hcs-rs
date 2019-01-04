// Copyright © rafawo (rafawo1@hotmail.com). All rights reserved.
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.
// All files in the project carrying such notice may not be copied, modified, or distributed
// except according to those terms.
// THE SOURCE CODE IS AVAILABLE UNDER THE ABOVE CHOSEN LICENSE "AS IS", WITH NO WARRANTIES.

//! This module contains Rust FFI bindings for the public computecore APIs.

use crate::computedefs::*;
use winutils_rs::windefs::*;

#[link(name = "computecore")]
extern "C" {
    /// Enumerates existing compute systems.
    pub fn HcsEnumerateComputeSystems(query: PCWStr, operation: HcsOperation) -> HResult;

    /// Creates a new operation.
    pub fn HcsCreateOperation(
        context: *const Void,
        callback: HcsOperationCompletion,
    ) -> HcsOperation;

    /// Closes an operation.
    pub fn HcsCloseOperation(operation: HcsOperation);

    /// Returns the context pointer of an operation.
    pub fn HcsGetOperationContext(operation: HcsOperation) -> *mut Void;

    /// Sets the context pointer for an operation.
    pub fn HcsSetOperationContext(operation: HcsOperation, context: *const Void) -> HResult;

    /// Returns the handle to compute system associated with an operation.
    pub fn HcsGetComputeSystemFromOperation(operation: HcsOperation) -> HcsSystem;

    /// Returns the handle to the process associated with an operation
    pub fn HcsGetProcessFromOperation(operation: HcsOperation) -> HcsProcess;

    /// Returns the type of an operation.
    pub fn HcsGetOperationType(operation: HcsOperation) -> HcsOperationType;

    /// Returns the ID that uniquely identifies an operation.
    pub fn HcsGetOperationId(operation: HcsOperation) -> u64;

    /// Returns the result of an operation.
    pub fn HcsGetOperationResult(operation: HcsOperation, resultDocument: *mut PWStr) -> HResult;

    /// Returns the result of an operation, including the process information for HcsCreateProcess
    /// and HcsGetProcessInfo.
    pub fn HcsGetOperationResultAndProcessInfo(
        operation: HcsOperation,
        processInformation: *mut HcsProcessInformation,
        resultDocument: *mut PWStr,
    ) -> HResult;

    /// Waits for the completion of an operation and returns the result.
    pub fn HcsWaitForOperationResult(
        operation: HcsOperation,
        timeoutMs: DWord,
        resultDocument: *mut PWStr,
    ) -> HResult;

    /// Waits for the completion of an operation and returns the result, including the process information
    /// for HcsCreateProcess and HcsGetProcessInfo.
    pub fn HcsWaitForOperationResultAndProcessInfo(
        operation: HcsOperation,
        timeoutMs: DWord,
        processInformation: *mut HcsProcessInformation,
        resultDocument: *mut PWStr,
    ) -> HResult;

    /// Sets a callback that is invoked on completion of an operation.
    pub fn HcsSetOperationCallback(
        operation: HcsOperation,
        context: *const Void,
        callback: HcsOperationCompletion,
    ) -> HResult;

    /// Cancels an operation
    pub fn HcsCancelOperation(operation: HcsOperation) -> HResult;

    /// Creates a new compute system.
    pub fn HcsCreateComputeSystem(
        id: PCWStr,
        configuration: PCWStr,
        operation: HcsOperation,
        securityDescriptor: *const SecurityDescriptor,
        computeSystem: *mut HcsSystem,
    ) -> HResult;

    /// Opens a handle to an existing compute system.
    pub fn HcsOpenComputeSystem(
        id: PCWStr,
        requestedAccess: DWord,
        computeSystem: HcsSystem,
    ) -> HResult;

    /// Closes a handle to a compute system.
    pub fn HcsCloseComputeSystem(computeSystem: HcsSystem);

    /// Starts a compute system.
    pub fn HcsStartComputeSystem(
        computeSystem: HcsSystem,
        operation: HcsOperation,
        options: PCWStr,
    ) -> HResult;

    /// Cleanly shuts down a compute system.
    pub fn HcsShutDownComputeSystem(
        computeSystem: HcsSystem,
        operation: HcsOperation,
        options: PCWStr,
    ) -> HResult;

    /// Forcefully terminates a compute system.
    pub fn HcsTerminateComputeSystem(
        computeSystem: HcsSystem,
        operation: HcsOperation,
        options: PCWStr,
    ) -> HResult;

    /// Pauses the execution of a compute system.
    pub fn HcsPauseComputeSystem(
        computeSystem: HcsSystem,
        operation: HcsOperation,
        options: PCWStr,
    ) -> HResult;

    /// Resumes the execution of a compute system.
    pub fn HcsResumeComputeSystem(
        computeSystem: HcsSystem,
        operation: HcsOperation,
        options: PCWStr,
    ) -> HResult;

    /// Saves the state of a compute system.
    pub fn HcsSaveComputeSystem(
        computeSystem: HcsSystem,
        operation: HcsOperation,
        options: PCWStr,
    ) -> HResult;

    /// Returns properties of a compute system.
    pub fn HcsGetComputeSystemProperties(
        computeSystem: HcsSystem,
        operation: HcsOperation,
        propertyQuery: PCWStr,
    ) -> HResult;

    /// Modifies settings of a compute system.
    pub fn HcsModifyComputeSystem(
        computeSystem: HcsSystem,
        operation: HcsOperation,
        configuration: PCWStr,
        identity: Handle,
    ) -> HResult;

    /// Registers a callback function to receive notifications for the compute system.
    pub fn HcsSetComputeSystemCallback(
        computeSystem: HcsSystem,
        callbackOptions: HcsEventOptions,
        context: *const Void,
        callback: HcsEventCallback,
    ) -> HResult;

    /// Starts a process in a compute system.
    pub fn HcsCreateProcess(
        computeSystem: HcsSystem,
        processParameters: PCWStr,
        operation: HcsOperation,
        securityDescriptor: *const SecurityDescriptor,
        process: *mut HcsProcess,
    ) -> HResult;

    /// Opens an existing process in a compute system
    pub fn HcsOpenProcess(
        computeSystem: HcsSystem,
        processId: DWord,
        requestedAccess: DWord,
        process: HcsProcess,
    ) -> HResult;

    /// Closes the handle to a process in a compute system
    pub fn HcsCloseProcess(process: HcsProcess);

    /// Terminates a process in a compute system
    pub fn HcsTerminateProcess(
        process: HcsProcess,
        operation: HcsOperation,
        options: PCWStr,
    ) -> HResult;

    /// Sends a signal to a process in a compute system
    pub fn HcsSignalProcess(
        process: HcsProcess,
        operation: HcsOperation,
        options: PCWStr,
    ) -> HResult;

    /// Returns the initial startup info of a process in a compute system
    pub fn HcsGetProcessInfo(process: HcsProcess, operation: HcsOperation) -> HResult;

    /// Returns the properties of a process in a compute system
    pub fn HcsGetProcessProperties(
        process: HcsProcess,
        operation: HcsOperation,
        propertyQuery: PCWStr,
    ) -> HResult;

    /// Modifies the parameters of a process in a compute system
    pub fn HcsModifyProcess(
        process: HcsProcess,
        operation: HcsOperation,
        settings: PCWStr,
    ) -> HResult;

    /// Registers a callback function to receive notifications for a process in a compute system
    pub fn HcsSetProcessCallback(
        process: HcsProcess,
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

    pub fn IsHcsEnumerateComputeSystemsPresent() -> Boolean;

    pub fn IsHcsCreateOperationPresent() -> Boolean;

    pub fn IsHcsCloseOperationPresent() -> Boolean;

    pub fn IsHcsGetOperationContextPresent() -> Boolean;

    pub fn IsHcsSetOperationContextPresent() -> Boolean;

    pub fn IsHcsGetComputeSystemFromOperationPresent() -> Boolean;

    pub fn IsHcsGetProcessFromOperationPresent() -> Boolean;

    pub fn IsHcsGetOperationTypePresent() -> Boolean;

    pub fn IsHcsGetOperationIdPresent() -> Boolean;

    pub fn IsHcsGetOperationResultPresent() -> Boolean;

    pub fn IsHcsGetOperationResultAndProcessInfoPresent() -> Boolean;

    pub fn IsHcsWaitForOperationResultPresent() -> Boolean;

    pub fn IsHcsWaitForOperationResultAndProcessInfoPresent() -> Boolean;

    pub fn IsHcsSetOperationCallbackPresent() -> Boolean;

    pub fn IsHcsCancelOperationPresent() -> Boolean;

    pub fn IsHcsCreateComputeSystemPresent() -> Boolean;

    pub fn IsHcsOpenComputeSystemPresent() -> Boolean;

    pub fn IsHcsCloseComputeSystemPresent() -> Boolean;

    pub fn IsHcsStartComputeSystemPresent() -> Boolean;

    pub fn IsHcsShutDownComputeSystemPresent() -> Boolean;

    pub fn IsHcsTerminateComputeSystemPresent() -> Boolean;

    pub fn IsHcsPauseComputeSystemPresent() -> Boolean;

    pub fn IsHcsResumeComputeSystemPresent() -> Boolean;

    pub fn IsHcsSaveComputeSystemPresent() -> Boolean;

    pub fn IsHcsGetComputeSystemPropertiesPresent() -> Boolean;

    pub fn IsHcsModifyComputeSystemPresent() -> Boolean;

    pub fn IsHcsSetComputeSystemCallbackPresent() -> Boolean;

    pub fn IsHcsCreateProcessPresent() -> Boolean;

    pub fn IsHcsOpenProcessPresent() -> Boolean;

    pub fn IsHcsCloseProcessPresent() -> Boolean;

    pub fn IsHcsTerminateProcessPresent() -> Boolean;

    pub fn IsHcsSignalProcessPresent() -> Boolean;

    pub fn IsHcsGetProcessInfoPresent() -> Boolean;

    pub fn IsHcsGetProcessPropertiesPresent() -> Boolean;

    pub fn IsHcsModifyProcessPresent() -> Boolean;

    pub fn IsHcsSetProcessCallbackPresent() -> Boolean;

    pub fn IsHcsGetServicePropertiesPresent() -> Boolean;

    pub fn IsHcsModifyServiceSettingsPresent() -> Boolean;

    pub fn IsHcsSubmitWerReportPresent() -> Boolean;

    pub fn IsHcsCreateEmptyGuestStateFilePresent() -> Boolean;

    pub fn IsHcsCreateEmptyRuntimeStateFilePresent() -> Boolean;

    pub fn IsHcsGrantVmAccessPresent() -> Boolean;

    pub fn IsHcsRevokeVmAccessPresent() -> Boolean;
}
