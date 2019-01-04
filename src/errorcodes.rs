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
pub enum HypervisorErrorCode {
    /// The hypervisor does not support the operation because the specified hypercall code is not supported.
    ErrorHvInvalidHypercallCode,

    /// The hypervisor does not support the operation because the encoding for the hypercall input register is not supported.
    ErrorHvInvalidHypercallInput,

    /// The hypervisor could not perform the operation because a parameter has an invalid alignment.
    ErrorHvInvalidAlignment,

    /// The hypervisor could not perform the operation because an invalid parameter was specified.
    ErrorHvInvalidParameter,

    /// Access to the specified object was denied.
    ErrorHvAccessDenied,

    /// The hypervisor could not perform the operation because the partition is entering or in an invalid state.
    ErrorHvInvalidPartitionState,

    /// The operation is not allowed in the current state.
    ErrorHvOperationDenied,

    /// The hypervisor does not recognize the specified partition property.
    ErrorHvUnknownProperty,

    /// The specified value of a partition property is out of range or violates an invariant.
    ErrorHvPropertyValueOutOfRange,

    /// There is not enough memory in the hypervisor pool to complete the operation.
    ErrorHvInsufficientMemory,

    /// The maximum partition depth has been exceeded for the partition hierarchy.
    ErrorHvPartitionTooDeep,

    /// A partition with the specified partition Id does not exist.
    ErrorHvInvalidPartitionId,

    /// The hypervisor could not perform the operation because the specified VP index is invalid.
    ErrorHvInvalidVpIndex,

    /// The hypervisor could not perform the operation because the specified port identifier is invalid.
    ErrorHvInvalidPortId,

    /// The hypervisor could not perform the operation because the specified connection identifier is invalid.
    ErrorHvInvalidConnectionId,

    /// Not enough buffers were supplied to send a message.
    ErrorHvInsufficientBuffers,

    /// The previous virtual interrupt has not been acknowledged.
    ErrorHvNotAcknowledged,

    /// A virtual processor is not in the correct state for the indicated operation.
    ErrorHvInvalidVpState,

    /// The previous virtual interrupt has already been acknowledged.
    ErrorHvAcknowledged,

    /// The indicated partition is not in a valid state for saving or restoring.
    ErrorHvInvalidSaveRestoreState,

    /// The hypervisor could not complete the operation because a required feature of the synthetic interrupt controller (SynIC) was disabled.
    ErrorHvInvalidSynicState,

    /// The hypervisor could not perform the operation because the object or value was either already in use or being used for a purpose that would not permit completing the operation.
    ErrorHvObjectInUse,

    /// The proximity domain information is invalid.
    ErrorHvInvalidProximityDomainInfo,

    /// An attempt to retrieve debugging data failed because none was available.
    ErrorHvNoData,

    /// The physical connection being used for debugging has not recorded any receive activity since the last operation.
    ErrorHvInactive,

    /// There are not enough resources to complete the operation.
    ErrorHvNoResources,

    /// A hypervisor feature is not available to the user.
    ErrorHvFeatureUnavailable,

    /// The specified buffer was too small to contain all of the requested data.
    ErrorHvInsufficientBuffer,

    /// The maximum number of domains supported by the platform I/O remapping hardware is currently in use. No domains are available to assign this device to this partition.
    ErrorHvInsufficientDeviceDomains,

    /// Validation of CPUID data of the processor failed.
    ErrorHvCpuidFeatureValidation,

    /// Validation of XSAVE CPUID data of the processor failed.
    ErrorHvCpuidXsaveFeatureValidation,

    /// Processor did not respond within the timeout period.
    ErrorHvProcessorStartupTimeout,

    /// SMX has been enabled in the BIOS.
    ErrorHvSmxEnabled,

    /// The hypervisor could not perform the operation because the specified LP index is invalid.
    ErrorHvInvalidLpIndex,

    /// The supplied register value is invalid.
    ErrorHvInvalidRegisterValue,

    /// The supplied virtual trust level is not in the correct state to perform the requested operation.
    ErrorHvInvalidVtlState,

    /// No execute feature (NX) is not present or not enabled in the BIOS.
    ErrorHvNxNotDetected,

    /// The supplied device ID is invalid.
    ErrorHvInvalidDeviceId,

    /// The operation is not allowed in the current device state.
    ErrorHvInvalidDeviceState,

    /// The device had pending page requests which were discarded.
    ErrorHvPendingPageRequests,

    /// The supplied page request specifies a memory access that the guest does not have permissions to perform.
    ErrorHvPageRequestInvalid,

    /// A CPU group with the specified CPU group Id does not exist.
    ErrorHvInvalidCpuGroupId,

    /// The hypervisor could not perform the operation because the CPU group is entering or in an invalid state.
    ErrorHvInvalidCpuGroupState,

    /// The requested operation failed.
    ErrorHvOperationFailed,

    /// The hypervisor could not perform the operation because it is not allowed with nested virtualization active.
    ErrorHvNotAllowedWithNestedVirtActive,

    /// There is not enough memory in the root partition's pool to complete the operation.
    ErrorHvInsufficientRootMemory,

    /// No hypervisor is present on this system.
    ErrorHvNotPresent,

    UnknownHResult(HResult),
}

#[allow(overflowing_literals)]
pub(crate) fn hresult_to_hypervisor_error_code(hresult: &HResult) -> HypervisorErrorCode {
    match hresult {
        0xC0350002 => HypervisorErrorCode::ErrorHvInvalidHypercallCode,
        0xC0350003 => HypervisorErrorCode::ErrorHvInvalidHypercallInput,
        0xC0350004 => HypervisorErrorCode::ErrorHvInvalidAlignment,
        0xC0350005 => HypervisorErrorCode::ErrorHvInvalidParameter,
        0xC0350006 => HypervisorErrorCode::ErrorHvAccessDenied,
        0xC0350007 => HypervisorErrorCode::ErrorHvInvalidPartitionState,
        0xC0350008 => HypervisorErrorCode::ErrorHvOperationDenied,
        0xC0350009 => HypervisorErrorCode::ErrorHvUnknownProperty,
        0xC035000A => HypervisorErrorCode::ErrorHvPropertyValueOutOfRange,
        0xC035000B => HypervisorErrorCode::ErrorHvInsufficientMemory,
        0xC035000C => HypervisorErrorCode::ErrorHvPartitionTooDeep,
        0xC035000D => HypervisorErrorCode::ErrorHvInvalidPartitionId,
        0xC035000E => HypervisorErrorCode::ErrorHvInvalidVpIndex,
        0xC0350011 => HypervisorErrorCode::ErrorHvInvalidPortId,
        0xC0350012 => HypervisorErrorCode::ErrorHvInvalidConnectionId,
        0xC0350013 => HypervisorErrorCode::ErrorHvInsufficientBuffers,
        0xC0350014 => HypervisorErrorCode::ErrorHvNotAcknowledged,
        0xC0350015 => HypervisorErrorCode::ErrorHvInvalidVpState,
        0xC0350016 => HypervisorErrorCode::ErrorHvAcknowledged,
        0xC0350017 => HypervisorErrorCode::ErrorHvInvalidSaveRestoreState,
        0xC0350018 => HypervisorErrorCode::ErrorHvInvalidSynicState,
        0xC0350019 => HypervisorErrorCode::ErrorHvObjectInUse,
        0xC035001A => HypervisorErrorCode::ErrorHvInvalidProximityDomainInfo,
        0xC035001B => HypervisorErrorCode::ErrorHvNoData,
        0xC035001C => HypervisorErrorCode::ErrorHvInactive,
        0xC035001D => HypervisorErrorCode::ErrorHvNoResources,
        0xC035001E => HypervisorErrorCode::ErrorHvFeatureUnavailable,
        0xC0350033 => HypervisorErrorCode::ErrorHvInsufficientBuffer,
        0xC0350038 => HypervisorErrorCode::ErrorHvInsufficientDeviceDomains,
        0xC035003C => HypervisorErrorCode::ErrorHvCpuidFeatureValidation,
        0xC035003D => HypervisorErrorCode::ErrorHvCpuidXsaveFeatureValidation,
        0xC035003E => HypervisorErrorCode::ErrorHvProcessorStartupTimeout,
        0xC035003F => HypervisorErrorCode::ErrorHvSmxEnabled,
        0xC0350041 => HypervisorErrorCode::ErrorHvInvalidLpIndex,
        0xC0350050 => HypervisorErrorCode::ErrorHvInvalidRegisterValue,
        0xC0350051 => HypervisorErrorCode::ErrorHvInvalidVtlState,
        0xC0350055 => HypervisorErrorCode::ErrorHvNxNotDetected,
        0xC0350057 => HypervisorErrorCode::ErrorHvInvalidDeviceId,
        0xC0350058 => HypervisorErrorCode::ErrorHvInvalidDeviceState,
        0x00350059 => HypervisorErrorCode::ErrorHvPendingPageRequests,
        0xC0350060 => HypervisorErrorCode::ErrorHvPageRequestInvalid,
        0xC035006F => HypervisorErrorCode::ErrorHvInvalidCpuGroupId,
        0xC0350070 => HypervisorErrorCode::ErrorHvInvalidCpuGroupState,
        0xC0350071 => HypervisorErrorCode::ErrorHvOperationFailed,
        0xC0350072 => HypervisorErrorCode::ErrorHvNotAllowedWithNestedVirtActive,
        0xC0350073 => HypervisorErrorCode::ErrorHvInsufficientRootMemory,
        0xC0351000 => HypervisorErrorCode::ErrorHvNotPresent,
        other => HypervisorErrorCode::UnknownHResult(*other),
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
    Hypervisor(HypervisorErrorCode),
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
        other => {
            match hresult_to_hypervisor_error_code(other) {
                HypervisorErrorCode::UnknownHResult(value) => ResultCode::UnknownHResult(value),
                hypervisor_error_code => ResultCode::Hypervisor(hypervisor_error_code),
            }
        }
    }
}