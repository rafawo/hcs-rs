// Copyright © rafawo (rafawo1@hotmail.com). All rights reserved.
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.
// All files in the project carrying such notice may not be copied, modified, or distributed
// except according to those terms.
// THE SOURCE CODE IS AVAILABLE UNDER THE ABOVE CHOSEN LICENSE "AS IS", WITH NO WARRANTIES.

//! Rust types that provide convenient functionality built on top of the hypervdevicevirtualiation APIs.

use crate::hypervdevicevirtualization;
use crate::hypervdevicevirtualization::defs::*;
use crate::HcsResult;
use std::sync::{Arc, RwLock};
use winutils_rs::windefs::*;

/// Trait definition that lists the functions required by an HDV PCI device
/// to implement to properly handle the device interface callbacks.
/// Because these are called from within C-style callbacks, do not make them panic
/// and instead return an HRESULT.
pub trait HdvPciDevice {
    fn assign_base(&mut self, base: Arc<RwLock<HdvPciDeviceBase>>);

    fn initialize(&mut self) -> HcsResult<()>;

    fn teardown(&mut self);

    fn set_configuration(&mut self, values: &[PCWStr]) -> HcsResult<()>;

    fn get_details(&self, pnp_id: &mut HdvPciPnpId, probed_bars: &mut [u32]) -> HcsResult<()>;

    fn start(&mut self) -> HcsResult<()>;

    fn stop(&mut self);

    fn read_config_space(&self, offset: u32, value: &mut u32) -> HcsResult<()>;

    fn write_config_space(&mut self, offset: u32, value: u32) -> HcsResult<()>;

    fn read_intercepted_memory(
        &self,
        bar_index: HdvPciBarSelector,
        offset: u64,
        value: &mut [Byte],
    ) -> HcsResult<()>;

    fn write_intercepted_memory(
        &mut self,
        bar_index: HdvPciBarSelector,
        offset: u64,
        value: &[Byte],
    ) -> HcsResult<()>;
}

/// Wrapper object that abstracts out setting up the C-style callbacks into
/// the hypervdevicevirtualization framework. When such callbacks are fired,
/// it will redirect the function call to the stored `device` trait object.
pub struct HdvPciDeviceBase {
    device_handle: HdvDeviceHandle,
}

impl HdvPciDeviceBase {
    /// Creates a new `HdvPciDeviceBase` object that abstracts out setting up
    /// the C-style callbacks into the hyperdevicevirtualization framework.
    /// It will store the supplied cloned `device` internally.
    /// When the actual C callback is fired, it will redirect the function call
    /// to the stored `device` trait object.
    /// This will also call the device's `assign_base` so that the object gets
    /// a chance to store a clone of this new device base and gain access
    /// to all of its methods.
    ///
    /// # Callback context
    /// The callback context will be the supplied mutable reference to the trait
    /// object. THIS REFERENCE MUST NOT CHANGE AFTER HOOKING UP THE INTERFACE
    /// CALLBACKS TO GUARANTEE THEY'LL DO PROPER FUNCTION CALL FORWARDING.
    ///
    /// **NOTE: Creating objects of this type will enforce a circular
    /// dependency between the trait object and this instance. This allows
    /// for both instances to use each other. This is desirable behavior
    /// since usually consumer code will implement a trait object with everything
    /// in it, leveraging the methods exposed through the device base.**
    pub fn hook_device_interface_callbacks(
        device_host_handle: HdvHostHandle,
        device_class_id: &Guid,
        device_instance_id: &Guid,
        device: &mut Arc<RwLock<dyn HdvPciDevice>>,
    ) -> HcsResult<()> {
        let device_base = Arc::new(RwLock::new(HdvPciDeviceBase {
            device_handle: hypervdevicevirtualization::create_device_instance(
                device_host_handle,
                HdvDeviceType::Pci,
                device_class_id,
                device_instance_id,
                &device_base_interface::DEVICE_INTERFACE as *const _ as *const Void,
                device as *mut _ as PVoid,
            )?,
        }));
        (*device).write().unwrap().assign_base(device_base.clone());
        Ok(())
    }

    /// Writes the contents of the supplied buffer to guest primary memory (RAM).
    pub fn write_guest_memory_buffer(
        &self,
        guest_physical_address: u64,
        buffer: &[Byte],
    ) -> HcsResult<()> {
        hypervdevicevirtualization::write_guest_memory(
            self.device_handle,
            guest_physical_address,
            buffer,
        )
    }

    /// Writes the supplied object as a byte buffer to guest primary memory (RAM).
    pub fn write_guest_memory<T>(&self, guest_physical_address: u64, data: &T) -> HcsResult<()>
    where
        T: Sized,
    {
        self.write_guest_memory_buffer(guest_physical_address, unsafe {
            std::slice::from_raw_parts(data as *const _ as *const u8, std::mem::size_of::<T>())
        })
    }

    /// Reads guest primary memory (RAM) contents into the supplied buffer.
    pub fn read_guest_memory_buffer(
        &self,
        guest_physical_address: u64,
        buffer: &mut [u8],
    ) -> HcsResult<()> {
        hypervdevicevirtualization::read_guest_memory(
            self.device_handle,
            guest_physical_address,
            buffer,
        )
    }

    /// Reads guest primary memory (RAM) contents into the supplied object, treating it as a byte buffer.
    pub fn read_guest_memory<T>(&self, guest_physical_address: u64, data: &mut T) -> HcsResult<()>
    where
        T: Sized,
    {
        self.read_guest_memory_buffer(guest_physical_address, unsafe {
            std::slice::from_raw_parts_mut(data as *mut _ as *mut u8, std::mem::size_of::<T>())
        })
    }

    /// Creates a guest RAM aperture into the address space of the calling process.
    pub fn create_guest_memory_aperture(
        &self,
        guest_physical_address: u64,
        byte_count: u32,
        write_protected: bool,
    ) -> HcsResult<PVoid> {
        hypervdevicevirtualization::create_guest_memory_aperture(
            self.device_handle,
            guest_physical_address,
            byte_count,
            write_protected,
        )
    }

    /// Destroys the guest RAM aperture mapped at the supplied process address.
    pub fn destroy_guest_memory_aperture(&self, mapped_address: PVoid) -> HcsResult<()> {
        hypervdevicevirtualization::destroy_guest_memory_aperture(
            self.device_handle,
            mapped_address,
        )
    }

    /// Delivers a message signalled interrupt (MSI) to the guest partition.
    pub fn deliver_guest_interrupt(&self, msi_address: u64, msi_data: u32) -> HcsResult<()> {
        hypervdevicevirtualization::deliver_guest_interrupt(
            self.device_handle,
            msi_address,
            msi_data,
        )
    }

    #[cfg(any(feature = "19h1"))]
    /// Registers a guest PFN to trigger an event on writes. The value of the write
    /// will be discarded.
    pub fn register_doorbell_page(
        &self,
        bar_index: HdvPciBarSelector,
        page_index: u64,
        doorbell_event: Handle,
    ) -> HcsResult<()> {
        hypervdevicevirtualization::register_doorbell_page(
            self.device_handle,
            bar_index,
            page_index,
            doorbell_event,
        )
    }

    #[cfg(any(feature = "19h1"))]
    /// Unregisters a guest physical page registered via `register_doorbell_page`.
    pub fn unregister_doorbell_page(
        &self,
        bar_index: HdvPciBarSelector,
        page_index: u64,
    ) -> HcsResult<()> {
        hypervdevicevirtualization::unregister_doorbell_page(
            self.device_handle,
            bar_index,
            page_index,
        )
    }
}

pub(crate) mod device_base_interface {
    use super::*;

    unsafe extern "system" fn initialize(device_context: *mut Void) -> HResult {
        let lock = (*(device_context as *mut Arc<RwLock<dyn HdvPciDevice>>)).write();
        match lock {
            Ok(mut device) => match (*device).initialize() {
                Ok(_) => winapi::shared::winerror::S_OK,
                Err(err) => crate::compute::errorcodes::result_code_to_hresult(err),
            },
            Err(_) => winapi::shared::winerror::E_FAIL,
        }
    }

    unsafe extern "system" fn teardown(device_context: *mut Void) {
        let lock = (*(device_context as *mut Arc<RwLock<dyn HdvPciDevice>>)).write();
        match lock {
            Ok(mut device) => (*device).teardown(),
            Err(_) => {}
        }
    }

    unsafe extern "system" fn set_configuration(
        device_context: *mut Void,
        configuration_value_count: u32,
        configuration_values: *const PCWStr,
    ) -> HResult {
        let config_values: &[PCWStr] =
            std::slice::from_raw_parts(configuration_values, configuration_value_count as usize);
        let lock = (*(device_context as *mut Arc<RwLock<dyn HdvPciDevice>>)).write();
        match lock {
            Ok(mut device) => match (*device).set_configuration(config_values) {
                Ok(_) => winapi::shared::winerror::S_OK,
                Err(err) => crate::compute::errorcodes::result_code_to_hresult(err),
            },
            Err(_) => winapi::shared::winerror::E_FAIL,
        }
    }

    unsafe extern "system" fn get_details(
        device_context: *mut Void,
        pnp_id: PHdvPciPnpId,
        probed_bars_count: u32,
        probed_bars: *mut u32,
    ) -> HResult {
        let probed_bars: &mut [u32] =
            std::slice::from_raw_parts_mut(probed_bars, probed_bars_count as usize);
        let lock = (*(device_context as *mut Arc<RwLock<dyn HdvPciDevice>>)).write();
        match lock {
            Ok(device) => match (*device).get_details(&mut *pnp_id, probed_bars) {
                Ok(_) => winapi::shared::winerror::S_OK,
                Err(err) => crate::compute::errorcodes::result_code_to_hresult(err),
            },
            Err(_) => winapi::shared::winerror::E_FAIL,
        }
    }

    unsafe extern "system" fn start(device_context: *mut Void) -> HResult {
        let lock = (*(device_context as *mut Arc<RwLock<dyn HdvPciDevice>>)).write();
        match lock {
            Ok(mut device) => match (*device).start() {
                Ok(_) => winapi::shared::winerror::S_OK,
                Err(err) => crate::compute::errorcodes::result_code_to_hresult(err),
            },
            Err(_) => winapi::shared::winerror::E_FAIL,
        }
    }

    unsafe extern "system" fn stop(device_context: *mut Void) {
        let lock = (*(device_context as *mut Arc<RwLock<dyn HdvPciDevice>>)).write();
        match lock {
            Ok(mut device) => (*device).stop(),
            Err(_) => {}
        }
    }

    unsafe extern "system" fn read_config_space(
        device_context: *mut Void,
        offset: u32,
        value: *mut u32,
    ) -> HResult {
        let lock = (*(device_context as *mut Arc<RwLock<dyn HdvPciDevice>>)).write();
        match lock {
            Ok(device) => match (*device).read_config_space(offset, &mut *value) {
                Ok(_) => winapi::shared::winerror::S_OK,
                Err(err) => crate::compute::errorcodes::result_code_to_hresult(err),
            },
            Err(_) => winapi::shared::winerror::E_FAIL,
        }
    }

    unsafe extern "system" fn write_config_space(
        device_context: *mut Void,
        offset: u32,
        value: u32,
    ) -> HResult {
        let lock = (*(device_context as *mut Arc<RwLock<dyn HdvPciDevice>>)).write();
        match lock {
            Ok(mut device) => match (*device).write_config_space(offset, value) {
                Ok(_) => winapi::shared::winerror::S_OK,
                Err(err) => crate::compute::errorcodes::result_code_to_hresult(err),
            },
            Err(_) => winapi::shared::winerror::E_FAIL,
        }
    }

    unsafe extern "system" fn read_intercepted_memory(
        device_context: *mut Void,
        bar_index: HdvPciBarSelector,
        offset: u64,
        length: u64,
        value: *mut Byte,
    ) -> HResult {
        let values: &mut [Byte] = std::slice::from_raw_parts_mut(value, length as usize);
        let lock = (*(device_context as *mut Arc<RwLock<dyn HdvPciDevice>>)).write();
        match lock {
            Ok(device) => match (*device).read_intercepted_memory(bar_index, offset, values) {
                Ok(_) => winapi::shared::winerror::S_OK,
                Err(err) => crate::compute::errorcodes::result_code_to_hresult(err),
            },
            Err(_) => winapi::shared::winerror::E_FAIL,
        }
    }

    unsafe extern "system" fn write_intercepted_memory(
        device_context: *mut Void,
        bar_index: HdvPciBarSelector,
        offset: u64,
        length: u64,
        value: *const Byte,
    ) -> HResult {
        let values: &[Byte] = std::slice::from_raw_parts(value, length as usize);
        let lock = (*(device_context as *mut Arc<RwLock<dyn HdvPciDevice>>)).write();
        match lock {
            Ok(mut device) => match (*device).write_intercepted_memory(bar_index, offset, values) {
                Ok(_) => winapi::shared::winerror::S_OK,
                Err(err) => crate::compute::errorcodes::result_code_to_hresult(err),
            },
            Err(_) => winapi::shared::winerror::E_FAIL,
        }
    }

    /// Global HDV PCI device interface object with all the necessary
    /// callbacks assigned to global unsafe functions that take care
    /// of forwarding the calls to higher-abstract structs.
    pub static DEVICE_INTERFACE: HdvPciDeviceInterface = HdvPciDeviceInterface {
        version: HdvPciInterfaceVersion::Version1,
        initialize: Some(initialize),
        teardown: Some(teardown),
        set_configuration: Some(set_configuration),
        get_details: Some(get_details),
        start: Some(start),
        stop: Some(stop),
        read_config_space: Some(read_config_space),
        write_config_space: Some(write_config_space),
        read_intercepted_memory: Some(read_intercepted_memory),
        write_intercepted_memory: Some(write_intercepted_memory),
    };
}
