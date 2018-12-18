//! This module contains types definitions and Rust FFI bindings APIs to interact with the Hyper-V device virtualization support.

use crate::computedefs::*;
use crate::hypervdevicevirtualizationdefs::*;
use crate::windefs::*;

#[link(name = "vmdevicehost")]
extern "C" {
    pub fn HdvInitializeDeviceHost(
        computeSystem: HcsSystem,
        deviceHostHandle: *mut HdvHost,
    ) -> HResult;

    pub fn HdvTeardownDeviceHost(deviceHostHandle: HdvHost) -> HResult;

    pub fn HdvCreateDeviceInstance(
        deviceHostHandle: HdvHost,
        deviceType: HdvDeviceType,
        deviceClassId: *const Guid,
        deviceInstanceId: *const Guid,
        deviceInterface: *const Void,
        deviceContext: *mut Void,
        deviceHandle: *mut HdvDevice,
    ) -> HResult;

    pub fn HdvReadGuestMemory(
        requestor: HdvDevice,
        guestPhysicalAddress: u64,
        ByteCount: u32,
        buffer: *mut Byte,
    ) -> HResult;

    pub fn HdvWriteGuestMemory(
        requestor: HdvDevice,
        guestPhysicalAddress: u64,
        ByteCount: u32,
        buffer: *const Byte,
    ) -> HResult;

    pub fn HdvCreateGuestMemoryAperture(
        requestor: HdvDevice,
        guestPhysicalAddress: u64,
        ByteCount: u32,
        writeProtected: Bool,
        mappedAddress: *mut PVoid,
    ) -> HResult;

    pub fn HdvDestroyGuestMemoryAperture(requestor: HdvDevice, mappedAddress: PVoid) -> HResult;

    pub fn HdvDeliverGuestInterrupt(requestor: HdvDevice, msiAddress: u64, msiData: u32)
        -> HResult;

    pub fn IsHdvInitializeDeviceHostPresent() -> Boolean;

    pub fn IsHdvTeardownDeviceHostPresent() -> Boolean;

    pub fn IsHdvCreateDeviceInstancePresent() -> Boolean;

    pub fn IsHdvReadGuestMemoryPresent() -> Boolean;

    pub fn IsHdvWriteGuestMemoryPresent() -> Boolean;

    pub fn IsHdvCreateGuestMemoryAperturePresent() -> Boolean;

    pub fn IsHdvDestroyGuestMemoryAperturePresent() -> Boolean;

    pub fn IsHdvDeliverGuestInterruptPresent() -> Boolean;
}
