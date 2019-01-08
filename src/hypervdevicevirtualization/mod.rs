// Copyright © rafawo (rafawo1@hotmail.com). All rights reserved.
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.
// All files in the project carrying such notice may not be copied, modified, or distributed
// except according to those terms.
// THE SOURCE CODE IS AVAILABLE UNDER THE ABOVE CHOSEN LICENSE "AS IS", WITH NO WARRANTIES.

pub(crate) mod bindings;
pub mod defs;
pub mod ispresent;

use crate::compute::defs::*;
use crate::compute::errorcodes::hresult_to_result_code;
use crate::hypervdevicevirtualization::bindings::*;
use crate::hypervdevicevirtualization::defs::*;
use crate::HcsResult;
use winutils_rs::windefs::*;

pub fn initialize_device_host(compute_system: HcsSystemHandle) -> HcsResult<HdvHostHandle> {
    let mut device_host_handle: HdvHostHandle = std::ptr::null_mut();

    unsafe {
        match HdvInitializeDeviceHost(compute_system, &mut device_host_handle) {
            0 => Ok(device_host_handle),
            hresult => Err(hresult_to_result_code(&hresult)),
        }
    }
}

pub fn teardown_device_host(device_host_handle: HdvHostHandle) -> HcsResult<()> {
    unsafe {
        match HdvTeardownDeviceHost(device_host_handle) {
            0 => Ok(()),
            hresult => Err(hresult_to_result_code(&hresult)),
        }
    }
}

pub fn create_device_instance(
    device_host_handle: HdvHostHandle,
    device_type: HdvDeviceType,
    device_class_id: &Guid,
    device_instance_id: &Guid,
    device_interface: *const Void,
    device_context: *mut Void,
) -> HcsResult<HdvDeviceHandle> {
    let mut device_handle: HdvHostHandle = std::ptr::null_mut();

    unsafe {
        match HdvCreateDeviceInstance(
            device_host_handle,
            device_type,
            device_class_id,
            device_instance_id,
            device_interface,
            device_context,
            &mut device_handle,
        ) {
            0 => Ok(device_handle),
            hresult => Err(hresult_to_result_code(&hresult)),
        }
    }
}

pub fn read_guest_memory(
    requestor: HdvDeviceHandle,
    guest_physical_address: u64,
    byte_count: u32,
) -> HcsResult<Vec<Byte>> {
    let mut byte_array: Vec<Byte> = Vec::new();
    byte_array.resize(byte_count as usize, 0);

    unsafe {
        match HdvReadGuestMemory(
            requestor,
            guest_physical_address,
            byte_count,
            byte_array.as_mut_ptr(),
        ) {
            0 => Ok(byte_array),
            hresult => Err(hresult_to_result_code(&hresult)),
        }
    }
}

pub fn write_guest_memory(
    requestor: HdvDeviceHandle,
    guest_physical_address: u64,
    byte_array: &Vec<Byte>,
) -> HcsResult<()> {
    unsafe {
        match HdvWriteGuestMemory(
            requestor,
            guest_physical_address,
            byte_array.len() as u32,
            byte_array.as_ptr(),
        ) {
            0 => Ok(()),
            hresult => Err(hresult_to_result_code(&hresult)),
        }
    }
}

pub fn create_guest_memory_aperture(
    requestor: HdvDeviceHandle,
    guest_physical_address: u64,
    byte_count: u32,
    write_protected: bool,
) -> HcsResult<PVoid> {
    let mut mapped_address: PVoid = std::ptr::null_mut();

    unsafe {
        match HdvCreateGuestMemoryAperture(
            requestor,
            guest_physical_address,
            byte_count,
            match write_protected {
                true => 1,
                false => 0,
            },
            &mut mapped_address,
        ) {
            0 => Ok(mapped_address),
            hresult => Err(hresult_to_result_code(&hresult)),
        }
    }
}

pub fn destroy_guest_memory_aperture(
    requestor: HdvDeviceHandle,
    mapped_address: PVoid,
) -> HcsResult<()> {
    unsafe {
        match HdvDestroyGuestMemoryAperture(requestor, mapped_address) {
            0 => Ok(()),
            hresult => Err(hresult_to_result_code(&hresult)),
        }
    }
}

pub fn deliver_guest_interrupt(
    requestor: HdvDeviceHandle,
    msi_address: u64,
    msi_data: u32,
) -> HcsResult<()> {
    unsafe {
        match HdvDeliverGuestInterrupt(requestor, msi_address, msi_data) {
            0 => Ok(()),
            hresult => Err(hresult_to_result_code(&hresult)),
        }
    }
}
