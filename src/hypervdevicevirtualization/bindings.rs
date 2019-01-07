// Copyright © rafawo (rafawo1@hotmail.com). All rights reserved.
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.
// All files in the project carrying such notice may not be copied, modified, or distributed
// except according to those terms.
// THE SOURCE CODE IS AVAILABLE UNDER THE ABOVE CHOSEN LICENSE "AS IS", WITH NO WARRANTIES.

//! This module contains types definitions and Rust FFI bindings APIs to interact with the Hyper-V device virtualization support.

use super::defs::*;
use crate::compute::defs::*;
use winutils_rs::windefs::*;

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
