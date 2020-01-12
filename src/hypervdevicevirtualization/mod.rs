// Copyright (c) 2019-2020 Rafael Alcaraz Mercado. All rights reserved.
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.
// All files in the project carrying such notice may not be copied, modified, or distributed
// except according to those terms.
// THE SOURCE CODE IS AVAILABLE UNDER THE ABOVE CHOSEN LICENSE "AS IS", WITH NO WARRANTIES.

//! Rust abstractions of the hypervdevicevirtualization APIs.
//! These are Rust idiomatic equivalents of the C bindings.

#[cfg(not(feature = "bindings"))]
pub(crate) mod bindings;
#[cfg(feature = "bindings")]
pub mod bindings;

#[cfg(feature = "utilities")]
pub mod utilities;

pub mod defs;

use crate::compute::defs::*;
use crate::compute::errorcodes::hresult_to_result_code;
use crate::hypervdevicevirtualization::bindings::*;
use crate::hypervdevicevirtualization::defs::*;
use crate::HcsResult;
use winutils_rs::windefs::*;

/// Initializes the device emulator host in the caller's process and associates it
/// with the specified compute system. This function should be called after the compute system
/// has been created and before it has been started. The compute system's configuration must
/// indicate that an external device host for the compute system will be present, by means
/// of specifying the identity (SID) of the user account under which the device host will execute.
/// If the device host has not been initialized by the time the compute system starts,
/// the start operation fails.
/// A successfull call to `initialize_device_host` must be balanced by a call to `teardown_device_host`
/// on the same thread.
///
/// Returns a handle to the created device host.
///
/// # Arguments
/// * `compute_system` - Handle to the compute system to which the device host will be associated.
pub fn initialize_device_host(compute_system: HcsSystemHandle) -> HcsResult<HdvHostHandle> {
    let mut device_host_handle: HdvHostHandle = std::ptr::null_mut();

    unsafe {
        match HdvInitializeDeviceHost(compute_system, &mut device_host_handle) {
            0 => Ok(device_host_handle),
            hresult => Err(hresult_to_result_code(&hresult)),
        }
    }
}

/// Tears down a device emulator host in the caller's process. All device instances
/// associated to the device host become non-functional. This function should be called
/// after the compute system to which the device host is associated has been terminated.
///
/// # Arguments
/// * `device_host_handle` - Handle to the device host to tear down.
pub fn teardown_device_host(device_host_handle: HdvHostHandle) -> HcsResult<()> {
    unsafe {
        match HdvTeardownDeviceHost(device_host_handle) {
            0 => Ok(()),
            hresult => Err(hresult_to_result_code(&hresult)),
        }
    }
}

/// Creates a device instance in the specified host and associates it with a device emulation
/// interface and context.
///
/// Returns a handle to the created device instance.
///
/// # Arguments
/// * `device_host_handle` - Identifies the device host in which the device instance will be created.
/// * `device_type` - Specifies the type of the device instance to create.
/// * `device_class_id` - Supplies the client-defined class ID of the device instance to create.
/// * `device_interface` - Supplies the client-defined instance ID of the device instance to create.
/// * `device_interface` - Supplies a function table representing the interface expsed by the device instance. The actual type of this parameter is implied by the DeviceType parameter.
/// * `device_context` - An optional opaque context pointer that will be supplied to the device interface callbacks.
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

/// Reads guest primary memory (RAM) contents into the supplied buffer.
///
/// # Arguments
/// * `requestor` - Handle to the device requesting memory access.
/// * `guest_physical_address` - Guest physical address at which the read operation starts.
/// * `byte_array` - Byte array where the memory will be read to.
pub fn read_guest_memory(
    requestor: HdvDeviceHandle,
    guest_physical_address: u64,
    byte_array: &mut [Byte],
) -> HcsResult<()> {
    unsafe {
        match HdvReadGuestMemory(
            requestor,
            guest_physical_address,
            byte_array.len() as u32,
            byte_array.as_mut_ptr(),
        ) {
            0 => Ok(()),
            hresult => Err(hresult_to_result_code(&hresult)),
        }
    }
}

/// Writes the contents of the supplied buffer to guest primary memory (RAM).
///
/// # Arguments
/// * `requestor` - Handle to the device requesting memory access.
/// * `guest_physical_address` - Guest physical address at which the write operation starts.
/// * `byte_array` - Source byte buffer for the write operation.
pub fn write_guest_memory(
    requestor: HdvDeviceHandle,
    guest_physical_address: u64,
    byte_array: &[Byte],
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

/// Creates a guest RAM aperture into the address space of the calling process.
///
/// Returns the virtual address (in the calling process) at which the requested memory region has been mapped.
///
/// # Arguments
/// * `requestor` - Handle to the device requesting memory access.
/// * `guest_physical_address` - Base physical address at which the aperture starts.
/// * `byte_count` - Size of the aperture in bytes.
/// * `write_protected` - If `true`, the process is only granted read access to the mapped memory.
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

/// Destroys the guest RAM aperture mapped at the supplied process address.
///
/// # Arguments
/// * `requestor` - Handle to the device requesting memory access.
/// * `mapped_address` - The virtual address (in the calling process) at which the aperture to destroy was mapped.
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

/// Delivers a message signalled interrupt (MSI) to the guest partition.
///
/// # Arguments
/// * `requestor` - Handle to the device requesting the interrupt.
/// * `msi_address` - The guest address to which the interrupt message is written.
/// * `msi_data` - The data to write at MsiAddress.
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

#[cfg(any(feature = "19h1"))]
/// Registers a guest PFN to trigger an event on writes. The value of the write
/// will be discarded.
///
/// #Arguments
/// * `requestor` - Handle to the device requesting the doorbell.
/// * `bar_index` - The index of the BAR containing the page to register.
/// * `page_index` - The page of the BAR to register.
/// * `doorbell_event` - Handle to the event to signal.
pub fn register_doorbell_page(
    requestor: HdvDeviceHandle,
    bar_index: HdvPciBarSelector,
    page_index: u64,
    doorbell_event: Handle,
) -> HcsResult<()> {
    unsafe {
        match HdvRegisterDoorbellPage(requestor, bar_index, page_index, doorbell_event) {
            0 => Ok(()),
            hresult => Err(hresult_to_result_code(&hresult)),
        }
    }
}

#[cfg(any(feature = "19h1"))]
/// Unregisters a guest physical page registered via `register_doorbell_page`.
///
/// # Arguments
/// * `requestor` - Handle to the device requesting the interrupt.
/// * `bar_index` - The index of the BAR containing the registered page.
/// * `page_index` - The registered page of the BAR.
pub fn unregister_doorbell_page(
    requestor: HdvDeviceHandle,
    bar_index: HdvPciBarSelector,
    page_index: u64,
) -> HcsResult<()> {
    unsafe {
        match HdvUnregisterDoorbellPage(requestor, bar_index, page_index) {
            0 => Ok(()),
            hresult => Err(hresult_to_result_code(&hresult)),
        }
    }
}
