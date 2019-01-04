// Copyright © rafawo (rafawo1@hotmail.com). All rights reserved.
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.
// All files in the project carrying such notice may not be copied, modified, or distributed
// except according to those terms.
// THE SOURCE CODE IS AVAILABLE UNDER THE ABOVE CHOSEN LICENSE "AS IS", WITH NO WARRANTIES.

use winutils_rs::windefs::HResult;

/// Hypervisor error codes
#[derive(Debug, PartialEq)]
pub enum HvErrorCode {
    /// The hypervisor does not support the operation because the specified hypercall code is not supported.
    HvInvalidHypercallCode,

    /// The hypervisor does not support the operation because the encoding for the hypercall input register is not supported.
    HvInvalidHypercallInput,

    /// The hypervisor could not perform the operation because a parameter has an invalid alignment.
    HvInvalidAlignment,

    /// The hypervisor could not perform the operation because an invalid parameter was specified.
    HvInvalidParameter,

    /// Access to the specified object was denied.
    HvAccessDenied,

    /// The hypervisor could not perform the operation because the partition is entering or in an invalid state.
    HvInvalidPartitionState,

    /// The operation is not allowed in the current state.
    HvOperationDenied,

    /// The hypervisor does not recognize the specified partition property.
    HvUnknownProperty,

    /// The specified value of a partition property is out of range or violates an invariant.
    HvPropertyValueOutOfRange,

    /// There is not enough memory in the hypervisor pool to complete the operation.
    HvInsufficientMemory,

    /// The maximum partition depth has been exceeded for the partition hierarchy.
    HvPartitionTooDeep,

    /// A partition with the specified partition Id does not exist.
    HvInvalidPartitionId,

    /// The hypervisor could not perform the operation because the specified VP index is invalid.
    HvInvalidVpIndex,

    /// The hypervisor could not perform the operation because the specified port identifier is invalid.
    HvInvalidPortId,

    /// The hypervisor could not perform the operation because the specified connection identifier is invalid.
    HvInvalidConnectionId,

    /// Not enough buffers were supplied to send a message.
    HvInsufficientBuffers,

    /// The previous virtual interrupt has not been acknowledged.
    HvNotAcknowledged,

    /// A virtual processor is not in the correct state for the indicated operation.
    HvInvalidVpState,

    /// The previous virtual interrupt has already been acknowledged.
    HvAcknowledged,

    /// The indicated partition is not in a valid state for saving or restoring.
    HvInvalidSaveRestoreState,

    /// The hypervisor could not complete the operation because a required feature of the synthetic interrupt controller (SynIC) was disabled.
    HvInvalidSynicState,

    /// The hypervisor could not perform the operation because the object or value was either already in use or being used for a purpose that would not permit completing the operation.
    HvObjectInUse,

    /// The proximity domain information is invalid.
    HvInvalidProximityDomainInfo,

    /// An attempt to retrieve debugging data failed because none was available.
    HvNoData,

    /// The physical connection being used for debugging has not recorded any receive activity since the last operation.
    HvInactive,

    /// There are not enough resources to complete the operation.
    HvNoResources,

    /// A hypervisor feature is not available to the user.
    HvFeatureUnavailable,

    /// The specified buffer was too small to contain all of the requested data.
    HvInsufficientBuffer,

    /// The maximum number of domains supported by the platform I/O remapping hardware is currently in use. No domains are available to assign this device to this partition.
    HvInsufficientDeviceDomains,

    /// Validation of CPUID data of the processor failed.
    HvCpuidFeatureValidation,

    /// Validation of XSAVE CPUID data of the processor failed.
    HvCpuidXsaveFeatureValidation,

    /// Processor did not respond within the timeout period.
    HvProcessorStartupTimeout,

    /// SMX has been enabled in the BIOS.
    HvSmxEnabled,

    /// The hypervisor could not perform the operation because the specified LP index is invalid.
    HvInvalidLpIndex,

    /// The supplied register value is invalid.
    HvInvalidRegisterValue,

    /// The supplied virtual trust level is not in the correct state to perform the requested operation.
    HvInvalidVtlState,

    /// No execute feature (NX) is not present or not enabled in the BIOS.
    HvNxNotDetected,

    /// The supplied device ID is invalid.
    HvInvalidDeviceId,

    /// The operation is not allowed in the current device state.
    HvInvalidDeviceState,

    /// The device had pending page requests which were discarded.
    HvPendingPageRequests,

    /// The supplied page request specifies a memory access that the guest does not have permissions to perform.
    HvPageRequestInvalid,

    /// A CPU group with the specified CPU group Id does not exist.
    HvInvalidCpuGroupId,

    /// The hypervisor could not perform the operation because the CPU group is entering or in an invalid state.
    HvInvalidCpuGroupState,

    /// The requested operation failed.
    HvOperationFailed,

    /// The hypervisor could not perform the operation because it is not allowed with nested virtualization active.
    HvNotAllowedWithNestedVirtActive,

    /// There is not enough memory in the root partition's pool to complete the operation.
    HvInsufficientRootMemory,

    /// No hypervisor is present on this system.
    HvNotPresent,

    UnknownHResult(HResult),
}

#[allow(overflowing_literals)]
pub(crate) fn hresult_to_hv_error_code(hresult: &HResult) -> HvErrorCode {
    match hresult {
        0xC0350002 => HvErrorCode::HvInvalidHypercallCode,
        0xC0350003 => HvErrorCode::HvInvalidHypercallInput,
        0xC0350004 => HvErrorCode::HvInvalidAlignment,
        0xC0350005 => HvErrorCode::HvInvalidParameter,
        0xC0350006 => HvErrorCode::HvAccessDenied,
        0xC0350007 => HvErrorCode::HvInvalidPartitionState,
        0xC0350008 => HvErrorCode::HvOperationDenied,
        0xC0350009 => HvErrorCode::HvUnknownProperty,
        0xC035000A => HvErrorCode::HvPropertyValueOutOfRange,
        0xC035000B => HvErrorCode::HvInsufficientMemory,
        0xC035000C => HvErrorCode::HvPartitionTooDeep,
        0xC035000D => HvErrorCode::HvInvalidPartitionId,
        0xC035000E => HvErrorCode::HvInvalidVpIndex,
        0xC0350011 => HvErrorCode::HvInvalidPortId,
        0xC0350012 => HvErrorCode::HvInvalidConnectionId,
        0xC0350013 => HvErrorCode::HvInsufficientBuffers,
        0xC0350014 => HvErrorCode::HvNotAcknowledged,
        0xC0350015 => HvErrorCode::HvInvalidVpState,
        0xC0350016 => HvErrorCode::HvAcknowledged,
        0xC0350017 => HvErrorCode::HvInvalidSaveRestoreState,
        0xC0350018 => HvErrorCode::HvInvalidSynicState,
        0xC0350019 => HvErrorCode::HvObjectInUse,
        0xC035001A => HvErrorCode::HvInvalidProximityDomainInfo,
        0xC035001B => HvErrorCode::HvNoData,
        0xC035001C => HvErrorCode::HvInactive,
        0xC035001D => HvErrorCode::HvNoResources,
        0xC035001E => HvErrorCode::HvFeatureUnavailable,
        0xC0350033 => HvErrorCode::HvInsufficientBuffer,
        0xC0350038 => HvErrorCode::HvInsufficientDeviceDomains,
        0xC035003C => HvErrorCode::HvCpuidFeatureValidation,
        0xC035003D => HvErrorCode::HvCpuidXsaveFeatureValidation,
        0xC035003E => HvErrorCode::HvProcessorStartupTimeout,
        0xC035003F => HvErrorCode::HvSmxEnabled,
        0xC0350041 => HvErrorCode::HvInvalidLpIndex,
        0xC0350050 => HvErrorCode::HvInvalidRegisterValue,
        0xC0350051 => HvErrorCode::HvInvalidVtlState,
        0xC0350055 => HvErrorCode::HvNxNotDetected,
        0xC0350057 => HvErrorCode::HvInvalidDeviceId,
        0xC0350058 => HvErrorCode::HvInvalidDeviceState,
        0x00350059 => HvErrorCode::HvPendingPageRequests,
        0xC0350060 => HvErrorCode::HvPageRequestInvalid,
        0xC035006F => HvErrorCode::HvInvalidCpuGroupId,
        0xC0350070 => HvErrorCode::HvInvalidCpuGroupState,
        0xC0350071 => HvErrorCode::HvOperationFailed,
        0xC0350072 => HvErrorCode::HvNotAllowedWithNestedVirtActive,
        0xC0350073 => HvErrorCode::HvInsufficientRootMemory,
        0xC0351000 => HvErrorCode::HvNotPresent,
        other => HvErrorCode::UnknownHResult(*other),
    }
}

/// Virtualization  codes
/// These error codes are used by the Virtualization Infrastructure Driver (VID) and other components
/// of the virtualization stack.

#[derive(Debug, PartialEq)]
pub enum VidErrorCode {
    /// The handler for the virtualization infrastructure driver is already registered. Restarting the virtual machine may fix the problem. If the problem persists, try restarting the physical computer.
    VidDuplicateHandler,

    /// The number of registered handlers for the virtualization infrastructure driver exceeded the maximum. Restarting the virtual machine may fix the problem. If the problem persists, try restarting the physical computer.
    VidTooManyHandlers,

    /// The message queue for the virtualization infrastructure driver is full and cannot accept new messages. Restarting the virtual machine may fix the problem. If the problem persists, try restarting the physical computer.
    VidQueueFull,

    /// No handler exists to handle the message for the virtualization infrastructure driver. Restarting the virtual machine may fix the problem. If the problem persists, try restarting the physical computer.
    VidHandlerNotPresent,

    /// The name of the partition or message queue for the virtualization infrastructure driver is invalid. Restarting the virtual machine may fix the problem. If the problem persists, try restarting the physical computer.
    VidInvalidObjectName,

    /// The partition name of the virtualization infrastructure driver exceeds the maximum.
    VidPartitionNameTooLong,

    /// The message queue name of the virtualization infrastructure driver exceeds the maximum.
    VidMessageQueueNameTooLong,

    /// Cannot create the partition for the virtualization infrastructure driver because another partition with the same name already exists.
    VidPartitionAlreadyExists,

    /// The virtualization infrastructure driver has encountered an . The requested partition does not exist. Restarting the virtual machine may fix the problem. If the problem persists, try restarting the physical computer.
    VidPartitionDoesNotExist,

    /// The virtualization infrastructure driver has encountered an . Could not find the requested partition. Restarting the virtual machine may fix the problem. If the problem persists, try restarting the physical computer.
    VidPartitionNameNotFound,

    /// A message queue with the same name already exists for the virtualization infrastructure driver.
    VidMessageQueueAlreadyExists,

    /// The memory block page for the virtualization infrastructure driver cannot be mapped because the page map limit has been reached. Restarting the virtual machine may fix the problem. If the problem persists, try restarting the physical computer.
    VidExceededMbpEntryMapLimit,

    /// The memory block for the virtualization infrastructure driver is still being used and cannot be destroyed.
    VidMbStillReferenced,

    /// Cannot unlock the page array for the guest operating system memory address because it does not match a previous lock request. Restarting the virtual machine may fix the problem. If the problem persists, try restarting the physical computer.
    VidChildGpaPageSetCorrupted,

    /// The non-uniform memory access (NUMA) node settings do not match the system NUMA topology. In order to start the virtual machine, you will need to modify the NUMA configuration.
    VidInvalidNumaSettings,

    /// The non-uniform memory access (NUMA) node index does not match a valid index in the system NUMA topology.
    VidInvalidNumaNodeIndex,

    /// The memory block for the virtualization infrastructure driver is already associated with a message queue.
    VidNotificationQueueAlreadyAssociated,

    /// The handle is not a valid memory block handle for the virtualization infrastructure driver.
    VidInvalidMemoryBlockHandle,

    /// The request exceeded the memory block page limit for the virtualization infrastructure driver. Restarting the virtual machine may fix the problem. If the problem persists, try restarting the physical computer.
    VidPageRangeOverflow,

    /// The handle is not a valid message queue handle for the virtualization infrastructure driver.
    VidInvalidMessageQueueHandle,

    /// The handle is not a valid page range handle for the virtualization infrastructure driver.
    VidInvalidGpaRangeHandle,

    /// Cannot install client notifications because no message queue for the virtualization infrastructure driver is associated with the memory block.
    VidNoMemoryBlockNotificationQueue,

    /// The request to lock or map a memory block page failed because the virtualization infrastructure driver memory block limit has been reached. Restarting the virtual machine may fix the problem. If the problem persists, try restarting the physical computer.
    VidMemoryBlockLockCountExceeded,

    /// The handle is not a valid parent partition mapping handle for the virtualization infrastructure driver.
    VidInvalidPpmHandle,

    /// Notifications cannot be created on the memory block because it is use.
    VidMbpsAreLocked,

    /// The message queue for the virtualization infrastructure driver has been closed. Restarting the virtual machine may fix the problem. If the problem persists, try restarting the physical computer.
    VidMessageQueueClosed,

    /// Cannot add a virtual processor to the partition because the maximum has been reached.
    VidVirtualProcessorLimitExceeded,

    /// Cannot stop the virtual processor immediately because of a pending intercept.
    VidStopPending,

    /// Invalid state for the virtual processor. Restarting the virtual machine may fix the problem. If the problem persists, try restarting the physical computer.
    VidInvalidProcessorState,

    /// The maximum number of kernel mode clients for the virtualization infrastructure driver has been reached. Restarting the virtual machine may fix the problem. If the problem persists, try restarting the physical computer.
    VidExceededKmContextCountLimit,

    /// This kernel mode interface for the virtualization infrastructure driver has already been initialized. Restarting the virtual machine may fix the problem. If the problem persists, try restarting the physical computer.
    VidKmInterfaceAlreadyInitialized,

    /// Cannot set or reset the memory block property more than once for the virtualization infrastructure driver. Restarting the virtual machine may fix the problem. If the problem persists, try restarting the physical computer.
    VidMbPropertyAlreadySetReset,

    /// The memory mapped I/O for this page range no longer exists. Restarting the virtual machine may fix the problem. If the problem persists, try restarting the physical computer.
    VidMmioRangeDestroyed,

    /// The lock or unlock request uses an invalid guest operating system memory address. Restarting the virtual machine may fix the problem. If the problem persists, try restarting the physical computer.
    VidInvalidChildGpaPageSet,

    /// Cannot destroy or reuse the reserve page set for the virtualization infrastructure driver because it is in use. Restarting the virtual machine may fix the problem. If the problem persists, try restarting the physical computer.
    VidReservePageSetIsBeingUsed,

    /// The reserve page set for the virtualization infrastructure driver is too small to use in the lock request. Restarting the virtual machine may fix the problem. If the problem persists, try restarting the physical computer.
    VidReservePageSetTooSmall,

    /// Cannot lock or map the memory block page for the virtualization infrastructure driver because it has already been locked using a reserve page set page. Restarting the virtual machine may fix the problem. If the problem persists, try restarting the physical computer.
    VidMbpAlreadyLockedUsingReservedPage,

    /// Cannot create the memory block for the virtualization infrastructure driver because the requested number of pages exceeded the limit. Restarting the virtual machine may fix the problem. If the problem persists, try restarting the physical computer.
    VidMbpCountExceededLimit,

    /// Cannot restore this virtual machine because the saved state data cannot be read. Delete the saved state data and then try to start the virtual machine.
    VidSavedStateCorrupt,

    /// Cannot restore this virtual machine because an item read from the saved state data is not recognized. Delete the saved state data and then try to start the virtual machine.
    VidSavedStateUnrecognizedItem,

    /// Cannot restore this virtual machine to the saved state because of hypervisor incompatibility. Delete the saved state data and then try to start the virtual machine.
    VidSavedStateIncompatible,

    /// The specified VTL does not have the permission to access the resource.
    VidVtlAccessDenied,

    UnknownHResult(HResult),
}

#[allow(overflowing_literals)]
pub(crate) fn hresult_to_vid_error_code(hresult: &HResult) -> VidErrorCode {
    match hresult {
        0xC0370001 => VidErrorCode::VidDuplicateHandler,
        0xC0370002 => VidErrorCode::VidTooManyHandlers,
        0xC0370003 => VidErrorCode::VidQueueFull,
        0xC0370004 => VidErrorCode::VidHandlerNotPresent,
        0xC0370005 => VidErrorCode::VidInvalidObjectName,
        0xC0370006 => VidErrorCode::VidPartitionNameTooLong,
        0xC0370007 => VidErrorCode::VidMessageQueueNameTooLong,
        0xC0370008 => VidErrorCode::VidPartitionAlreadyExists,
        0xC0370009 => VidErrorCode::VidPartitionDoesNotExist,
        0xC037000A => VidErrorCode::VidPartitionNameNotFound,
        0xC037000B => VidErrorCode::VidMessageQueueAlreadyExists,
        0xC037000C => VidErrorCode::VidExceededMbpEntryMapLimit,
        0xC037000D => VidErrorCode::VidMbStillReferenced,
        0xC037000E => VidErrorCode::VidChildGpaPageSetCorrupted,
        0xC037000F => VidErrorCode::VidInvalidNumaSettings,
        0xC0370010 => VidErrorCode::VidInvalidNumaNodeIndex,
        0xC0370011 => VidErrorCode::VidNotificationQueueAlreadyAssociated,
        0xC0370012 => VidErrorCode::VidInvalidMemoryBlockHandle,
        0xC0370013 => VidErrorCode::VidPageRangeOverflow,
        0xC0370014 => VidErrorCode::VidInvalidMessageQueueHandle,
        0xC0370015 => VidErrorCode::VidInvalidGpaRangeHandle,
        0xC0370016 => VidErrorCode::VidNoMemoryBlockNotificationQueue,
        0xC0370017 => VidErrorCode::VidMemoryBlockLockCountExceeded,
        0xC0370018 => VidErrorCode::VidInvalidPpmHandle,
        0xC0370019 => VidErrorCode::VidMbpsAreLocked,
        0xC037001A => VidErrorCode::VidMessageQueueClosed,
        0xC037001B => VidErrorCode::VidVirtualProcessorLimitExceeded,
        0xC037001C => VidErrorCode::VidStopPending,
        0xC037001D => VidErrorCode::VidInvalidProcessorState,
        0xC037001E => VidErrorCode::VidExceededKmContextCountLimit,
        0xC037001F => VidErrorCode::VidKmInterfaceAlreadyInitialized,
        0xC0370020 => VidErrorCode::VidMbPropertyAlreadySetReset,
        0xC0370021 => VidErrorCode::VidMmioRangeDestroyed,
        0xC0370022 => VidErrorCode::VidInvalidChildGpaPageSet,
        0xC0370023 => VidErrorCode::VidReservePageSetIsBeingUsed,
        0xC0370024 => VidErrorCode::VidReservePageSetTooSmall,
        0xC0370025 => VidErrorCode::VidMbpAlreadyLockedUsingReservedPage,
        0xC0370026 => VidErrorCode::VidMbpCountExceededLimit,
        0xC0370027 => VidErrorCode::VidSavedStateCorrupt,
        0xC0370028 => VidErrorCode::VidSavedStateUnrecognizedItem,
        0xC0370029 => VidErrorCode::VidSavedStateIncompatible,
        0xC037002A => VidErrorCode::VidVtlAccessDenied,
        other => VidErrorCode::UnknownHResult(*other),
    }
}

/// Common result codes and error codes that are specific to virtualization,
/// that can be returned by the HCS APIs.
#[derive(Debug, PartialEq)]
pub enum ResultCode {
    Success,
    OutOfMemory,
    FileNotFound,
    Fail,
    InvalidArgument,
    Unexpected,
    Hypervisor(HvErrorCode),
    VirtualizationIntrastructureDriver(VidErrorCode),
    UnknownHResult(HResult),
}

#[allow(overflowing_literals)]
pub(crate) fn hresult_to_result_code(hresult: &HResult) -> ResultCode {
    match hresult {
        0 => ResultCode::Success,
        0x8007000E => ResultCode::OutOfMemory,
        0x80070002 => ResultCode::FileNotFound,
        0x80004005 => ResultCode::Fail,
        0x80070057 => ResultCode::InvalidArgument,
        0x8000FFFF => ResultCode::Unexpected,
        other => match hresult_to_hv_error_code(other) {
            HvErrorCode::UnknownHResult(hv_error_code) => {
                match hresult_to_vid_error_code(&hv_error_code) {
                    VidErrorCode::UnknownHResult(vid_error_code) => {
                        ResultCode::UnknownHResult(vid_error_code)
                    }
                    vid_error_code => {
                        ResultCode::VirtualizationIntrastructureDriver(vid_error_code)
                    }
                }
            }
            hv_error_code => ResultCode::Hypervisor(hv_error_code),
        },
    }
}
