//! This file contains Rust abstractions for the public computecore APIs.

use crate::computedefs::*;
use winapi::shared::minwindef::DWORD;
use winapi::shared::ntdef::{BOOLEAN, HANDLE, HRESULT, PCWSTR, PWSTR, VOID};
use winapi::um::winnt::SECURITY_DESCRIPTOR;

extern "C" {

    /// Enumerates existing compute systems.
    pub fn HcsEnumerateComputeSystems(query: PCWSTR, operation: HCS_OPERATION) -> HRESULT;

    /// Creates a new operation.
    pub fn HcsCreateOperation(
        context: *const VOID,
        callback: HCS_OPERATION_COMPLETION,
    ) -> HCS_OPERATION;

    /// Closes an operation.
    pub fn HcsCloseOperation(operation: HCS_OPERATION);

    /// Returns the context pointer of an operation.
    pub fn HcsGetOperationContext(operation: HCS_OPERATION) -> *mut VOID;

    /// Sets the context pointer for an operation.
    pub fn HcsSetOperationContext(operation: HCS_OPERATION, context: *const VOID) -> HRESULT;

    /// Returns the handle to compute system associated with an operation.
    pub fn HcsGetComputeSystemFromOperation(operation: HCS_OPERATION) -> HCS_SYSTEM;

    /// Returns the handle to the process associated with an operation
    pub fn HcsGetProcessFromOperation(operation: HCS_OPERATION) -> HCS_PROCESS;

    /// Returns the type of an operation.
    pub fn HcsGetOperationType(operation: HCS_OPERATION) -> HcsOperationType;

    /// Returns the ID that uniquely identifies an operation.
    pub fn HcsGetOperationId(operation: HCS_OPERATION) -> u64;

    /// Returns the result of an operation.
    pub fn HcsGetOperationResult(operation: HCS_OPERATION, resultDocument: *mut PWSTR) -> HRESULT;

    /// Returns the result of an operation, including the process information for HcsCreateProcess
    /// and HcsGetProcessInfo.
    pub fn HcsGetOperationResultAndProcessInfo(
        operation: HCS_OPERATION,
        processInformation: *mut HcsProcessInformation,
        resultDocument: *mut PWSTR,
    ) -> HRESULT;

    /// Waits for the completion of an operation and returns the result.
    pub fn HcsWaitForOperationResult(
        operation: HCS_OPERATION,
        timeoutMs: DWORD,
        resultDocument: *mut PWSTR,
    ) -> HRESULT;

    /// Waits for the completion of an operation and returns the result, including the process information
    /// for HcsCreateProcess and HcsGetProcessInfo.
    pub fn HcsWaitForOperationResultAndProcessInfo(
        operation: HCS_OPERATION,
        timeoutMs: DWORD,
        processInformation: *mut HcsProcessInformation,
        resultDocument: *mut PWSTR,
    ) -> HRESULT;

    /// Sets a callback that is invoked on completion of an operation.
    pub fn HcsSetOperationCallback(
        operation: HCS_OPERATION,
        context: *const VOID,
        callback: HCS_OPERATION_COMPLETION,
    ) -> HRESULT;

    /// Cancels an operation
    pub fn HcsCancelOperation(operation: HCS_OPERATION) -> HRESULT;

    /// Creates a new compute system.
    pub fn HcsCreateComputeSystem(
        id: PCWSTR,
        configuration: PCWSTR,
        operation: HCS_OPERATION,
        securityDescriptor: *const SECURITY_DESCRIPTOR,
        computeSystem: *mut HCS_SYSTEM,
    ) -> HRESULT;

    /// Opens a handle to an existing compute system.
    pub fn HcsOpenComputeSystem(
        id: PCWSTR,
        requestedAccess: DWORD,
        computeSystem: HCS_SYSTEM,
    ) -> HRESULT;

    /// Closes a handle to a compute system.
    pub fn HcsCloseComputeSystem(computeSystem: HCS_SYSTEM);

    /// Starts a compute system.
    pub fn HcsStartComputeSystem(
        computeSystem: HCS_SYSTEM,
        operation: HCS_OPERATION,
        options: PCWSTR,
    ) -> HRESULT;

    /// Cleanly shuts down a compute system.
    pub fn HcsShutDownComputeSystem(
        computeSystem: HCS_SYSTEM,
        operation: HCS_OPERATION,
        options: PCWSTR,
    ) -> HRESULT;

    /// Forcefully terminates a compute system.
    pub fn HcsTerminateComputeSystem(
        computeSystem: HCS_SYSTEM,
        operation: HCS_OPERATION,
        options: PCWSTR,
    ) -> HRESULT;

    /// Pauses the execution of a compute system.
    pub fn HcsPauseComputeSystem(
        computeSystem: HCS_SYSTEM,
        operation: HCS_OPERATION,
        options: PCWSTR,
    ) -> HRESULT;

    /// Resumes the execution of a compute system.
    pub fn HcsResumeComputeSystem(
        computeSystem: HCS_SYSTEM,
        operation: HCS_OPERATION,
        options: PCWSTR,
    ) -> HRESULT;

    /// Saves the state of a compute system.
    pub fn HcsSaveComputeSystem(
        computeSystem: HCS_SYSTEM,
        operation: HCS_OPERATION,
        options: PCWSTR,
    ) -> HRESULT;

    /// Returns properties of a compute system.
    pub fn HcsGetComputeSystemProperties(
        computeSystem: HCS_SYSTEM,
        operation: HCS_OPERATION,
        propertyQuery: PCWSTR,
    ) -> HRESULT;

    /// Modifies settings of a compute system.
    pub fn HcsModifyComputeSystem(
        computeSystem: HCS_SYSTEM,
        operation: HCS_OPERATION,
        configuration: PCWSTR,
        identity: HANDLE,
    ) -> HRESULT;

    /// Registers a callback function to receive notifications for the compute system.
    pub fn HcsSetComputeSystemCallback(
        computeSystem: HCS_SYSTEM,
        callbackOptions: HcsEventOptions,
        context: *const VOID,
        callback: HCS_EVENT_CALLBACK,
    ) -> HRESULT;

    /// Starts a process in a compute system.
    pub fn HcsCreateProcess(
        computeSystem: HCS_SYSTEM,
        processParameters: PCWSTR,
        operation: HCS_OPERATION,
        securityDescriptor: *const SECURITY_DESCRIPTOR,
        process: *mut HCS_PROCESS,
    ) -> HRESULT;

    /// Opens an existing process in a compute system
    pub fn HcsOpenProcess(
        computeSystem: HCS_SYSTEM,
        processId: DWORD,
        requestedAccess: DWORD,
        process: HCS_PROCESS,
    ) -> HRESULT;

    /// Closes the handle to a process in a compute system
    pub fn HcsCloseProcess(process: HCS_PROCESS);

    /// Terminates a process in a compute system
    pub fn HcsTerminateProcess(
        process: HCS_PROCESS,
        operation: HCS_OPERATION,
        options: PCWSTR,
    ) -> HRESULT;

    /// Sends a signal to a process in a compute system
    pub fn HcsSignalProcess(
        process: HCS_PROCESS,
        operation: HCS_OPERATION,
        options: PCWSTR,
    ) -> HRESULT;

    /// Returns the initial startup info of a process in a compute system
    pub fn HcsGetProcessInfo(process: HCS_PROCESS, operation: HCS_OPERATION) -> HRESULT;

    /// Returns the properties of a process in a compute system
    pub fn HcsGetProcessProperties(
        process: HCS_PROCESS,
        operation: HCS_OPERATION,
        propertyQuery: PCWSTR,
    ) -> HRESULT;

    /// Modifies the parameters of a process in a compute system
    pub fn HcsModifyProcess(
        process: HCS_PROCESS,
        operation: HCS_OPERATION,
        settings: PCWSTR,
    ) -> HRESULT;

    /// Registers a callback function to receive notifications for a process in a compute system
    pub fn HcsSetProcessCallback(
        process: HCS_PROCESS,
        callbackOptions: HcsEventOptions,
        context: *mut VOID,
        callback: HCS_EVENT_CALLBACK,
    ) -> HRESULT;

    /// Returns properties of the Host Compute Service
    pub fn HcsGetServiceProperties(propertyQuery: PCWSTR, result: *mut PWSTR) -> HRESULT;

    /// Modifies settings of the Host Compute Service
    pub fn HcsModifyServiceSettings(settings: PCWSTR, result: *mut PWSTR) -> HRESULT;

    /// Submits a WER report
    pub fn HcsSubmitWerReport(settings: PCWSTR) -> HRESULT;

    /// Creates an empty guest-state file (.vmgs) for a VM.
    pub fn HcsCreateEmptyGuestStateFile(guestStateFilePath: PCWSTR) -> HRESULT;

    /// Creates an empty runtime-state file (.vmrs) for a VM.
    pub fn HcsCreateEmptyRuntimeStateFile(runtimeStateFilePath: PCWSTR) -> HRESULT;

    /// Adds an entry to a file's ACL that grants access for a VM.
    pub fn HcsGrantVmAccess(vmId: PCWSTR, filePath: PCWSTR) -> HRESULT;

    /// Removes an entry to a file's ACL that granted access for a VM.
    pub fn HcsRevokeVmAccess(vmId: PCWSTR, filePath: PCWSTR) -> HRESULT;

    pub fn IsHcsEnumerateComputeSystemsPresent() -> BOOLEAN;

    pub fn IsHcsCreateOperationPresent() -> BOOLEAN;

    pub fn IsHcsCloseOperationPresent() -> BOOLEAN;

    pub fn IsHcsGetOperationContextPresent() -> BOOLEAN;

    pub fn IsHcsSetOperationContextPresent() -> BOOLEAN;

    pub fn IsHcsGetComputeSystemFromOperationPresent() -> BOOLEAN;

    pub fn IsHcsGetProcessFromOperationPresent() -> BOOLEAN;

    pub fn IsHcsGetOperationTypePresent() -> BOOLEAN;

    pub fn IsHcsGetOperationIdPresent() -> BOOLEAN;

    pub fn IsHcsGetOperationResultPresent() -> BOOLEAN;

    pub fn IsHcsGetOperationResultAndProcessInfoPresent() -> BOOLEAN;

    pub fn IsHcsWaitForOperationResultPresent() -> BOOLEAN;

    pub fn IsHcsWaitForOperationResultAndProcessInfoPresent() -> BOOLEAN;

    pub fn IsHcsSetOperationCallbackPresent() -> BOOLEAN;

    pub fn IsHcsCancelOperationPresent() -> BOOLEAN;

    pub fn IsHcsCreateComputeSystemPresent() -> BOOLEAN;

    pub fn IsHcsOpenComputeSystemPresent() -> BOOLEAN;

    pub fn IsHcsCloseComputeSystemPresent() -> BOOLEAN;

    pub fn IsHcsStartComputeSystemPresent() -> BOOLEAN;

    pub fn IsHcsShutDownComputeSystemPresent() -> BOOLEAN;

    pub fn IsHcsTerminateComputeSystemPresent() -> BOOLEAN;

    pub fn IsHcsPauseComputeSystemPresent() -> BOOLEAN;

    pub fn IsHcsResumeComputeSystemPresent() -> BOOLEAN;

    pub fn IsHcsSaveComputeSystemPresent() -> BOOLEAN;

    pub fn IsHcsGetComputeSystemPropertiesPresent() -> BOOLEAN;

    pub fn IsHcsModifyComputeSystemPresent() -> BOOLEAN;

    pub fn IsHcsSetComputeSystemCallbackPresent() -> BOOLEAN;

    pub fn IsHcsCreateProcessPresent() -> BOOLEAN;

    pub fn IsHcsOpenProcessPresent() -> BOOLEAN;

    pub fn IsHcsCloseProcessPresent() -> BOOLEAN;

    pub fn IsHcsTerminateProcessPresent() -> BOOLEAN;

    pub fn IsHcsSignalProcessPresent() -> BOOLEAN;

    pub fn IsHcsGetProcessInfoPresent() -> BOOLEAN;

    pub fn IsHcsGetProcessPropertiesPresent() -> BOOLEAN;

    pub fn IsHcsModifyProcessPresent() -> BOOLEAN;

    pub fn IsHcsSetProcessCallbackPresent() -> BOOLEAN;

    pub fn IsHcsGetServicePropertiesPresent() -> BOOLEAN;

    pub fn IsHcsModifyServiceSettingsPresent() -> BOOLEAN;

    pub fn IsHcsSubmitWerReportPresent() -> BOOLEAN;

    pub fn IsHcsCreateEmptyGuestStateFilePresent() -> BOOLEAN;

    pub fn IsHcsCreateEmptyRuntimeStateFilePresent() -> BOOLEAN;

    pub fn IsHcsGrantVmAccessPresent() -> BOOLEAN;

    pub fn IsHcsRevokeVmAccessPresent() -> BOOLEAN;
}
