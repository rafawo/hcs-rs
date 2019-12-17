// Copyright © rafawo (rafawo1@hotmail.com). All rights reserved.
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.
// All files in the project carrying such notice may not be copied, modified, or distributed
// except according to those terms.
// THE SOURCE CODE IS AVAILABLE UNDER THE ABOVE CHOSEN LICENSE "AS IS", WITH NO WARRANTIES.

//! Rust types that provide convenient functionality built on top of the hypervdevicevirtualiation APIs.

use crate::compute::defs::HcsSystemHandle;
use crate::compute::errorcodes::{result_code_to_hresult, ResultCode};
use crate::hypervdevicevirtualization;
use crate::hypervdevicevirtualization::defs::*;
use crate::HcsResult;
use std::sync::{Arc, RwLock, RwLockReadGuard, RwLockWriteGuard};
use winutils_rs::windefs::*;

/// Trait definition that lists the functions required by an HDV PCI device
/// to implement to properly handle the device interface callbacks.
/// Because these are called from within C-style callbacks, do not make them panic
/// and instead return an `HcsResult<()>`.
///
/// Here's an example on how to utilize this trait and the rest of the utilities
/// in this module to create/implement a HDV PCI device:
/// ```rust,ignore
/// use std::sync::{Arc, RwLock};
/// use hcs_rs::hypervdevicevirtualization::utilities::*;
///
/// struct ExampleDevice {
///     base_wrapper: HdvPciDeviceBaseWrapper,
/// }
///
/// impl HdvPciDevice for ExampleDevice {
///     fn new() -> ExampleDevice {
///         ExampleDevice {
///             base_wrapper: HdvPciDeviceBaseWrapper::new()
///         }
///     }
///
///     fn assign_base(&mut self, base: Arc<RwLock<HdvPciDeviceBase>>) {
///         self.base_wrapper.assign_base(base);
///     }
///
///     fn initialize(&mut self) -> HcsResult<()> {
///         let data: u64 = 0;
///         self.base_wrapper.device_base_mut()?.write_guest_memory(0, &data)?;
///         // Do something
///         Ok(())
///     }
///
///     // The rest of the HdvPciDevice interface...
/// }
///
/// fn main() {
///     let system = create_new_system().unwrap(); // Assume code that creates an HcsSystem
///     let hdv = system.initialize_device_host().unwrap();
///     let device = Arc::new(RwLock::new(ExampleDevice::new()));
///     let mut device = device as Arc<RwLock<dyn HdvPciDevice>>;
///     unsafe {
///         // By ensuring that &mut device outlives the device's lifetime the C callbacks
///         // will work correctly.
///         HdvPciDeviceBase::hook_device_interface_callbacks(
///             hdv.handle(),
///             &some_guid, // Assume GUID object exists
///             &some_guid, // Assume GUID object exists
///             &mut device,
///         )
///     }
///     .unwrap();
///
///     // Start system
///     // Do stuff
///     // Stop system
///     // hdv will get dropped here and teardown the device host
///     // System handle is closed here after the system object gets dropped
/// }
/// ```
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

/// Safe wrapper of an HDV Host handle.
/// When dropped, the underlying handle is closed from the HCS API.
pub struct HdvHost {
    handle: HdvHostHandle,
}

impl std::ops::Drop for HdvHost {
    fn drop(&mut self) {
        if self.handle != std::ptr::null() {
            hypervdevicevirtualization::teardown_device_host(self.handle)
                .expect("Failed to close operation HDV host handle");
        }
    }
}

impl HdvHost {
    /// Initializes the device emulator host in the caller's process and associates it
    /// with the specified compute system. This function should be called after the compute system
    /// has been created and before it has been started. The compute system's configuration must
    /// indicate that an external device host for the compute system will be present, by means
    /// of specifying the identity (SID) of the user account under which the device host will execute.
    /// If the device host has not been initialized by the time the compute system starts,
    /// the start operation fails.
    pub fn new(system_handle: HcsSystemHandle) -> HcsResult<HdvHost> {
        Ok(HdvHost {
            handle: hypervdevicevirtualization::initialize_device_host(system_handle)?,
        })
    }

    /// Returns the wrapped `HdvHostHandle`.
    pub fn handle(&self) -> HdvHostHandle {
        self.handle
    }
}

/// Wrapper object that abstracts out setting up the C-style callbacks into
/// the hypervdevicevirtualization framework. When such callbacks are fired,
/// it will redirect the function call to the specified `device` trait object
/// supplied through `hook_device_interface_callbacks`.
/// Stores internally a handle to the initialized device and provides convenient
/// methods that utilize such handle under the covers.
#[derive(Clone)]
pub struct HdvPciDeviceBase {
    device_handle: HdvDeviceHandle,
}

impl HdvPciDeviceBase {
    /// Creates a new `HdvPciDeviceBase` object that abstracts out setting up
    /// the C-style callbacks into the hyperdevicevirtualization framework.
    /// It will utilize the supplied `device` as the device context for the callbacks,
    /// so that it can forward the actual function call to the implemented trait object.
    ///
    /// This will also call the device's `assign_base` so that the object gets
    /// a chance to store a clone of this new device base and gain access
    /// to all of its methods that utilize the initialized device handle.
    ///
    /// # Callback context
    /// The callback context will be the supplied mutable reference to the trait
    /// object. THIS REFERENCE MUST NOT CHANGE AFTER HOOKING UP THE INTERFACE
    /// CALLBACKS TO GUARANTEE THEY'LL DO PROPER FUNCTION CALL FORWARDING.
    ///
    /// # Safety
    /// Callers must guarantee the supplied `device` mutable reference outlives
    /// the device base until it's tore down through the device host handle closing.
    pub unsafe fn hook_device_interface_callbacks(
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
        device.write().unwrap().assign_base(device_base);
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

/// Wrapper object on top of an `HdvPciDeviceBase` object.
/// This struct exists to simplify the implementation of structs that
/// implement trait `HdvPciDevice`.
#[derive(Clone)]
pub struct HdvPciDeviceBaseWrapper {
    base: Option<Arc<RwLock<HdvPciDeviceBase>>>,
}

impl HdvPciDeviceBaseWrapper {
    /// Creates a new device base wrapper that has an unassigned device base.
    /// Use method `assign_base` to set it to something.
    pub fn new() -> Self {
        Self { base: None }
    }

    /// Assigns an `HdvPciDeviceBase` object to this wrapper.
    pub fn assign_base(&mut self, base: Arc<RwLock<HdvPciDeviceBase>>) {
        self.base = Some(base);
    }

    /// Gets the base device with a read lock.
    pub fn device_base(&self) -> HcsResult<RwLockReadGuard<HdvPciDeviceBase>> {
        self.base
            .as_ref()
            .ok_or(ResultCode::UnknownHResult(winapi::shared::winerror::E_FAIL))
            .map_err(|hresult| hresult)?
            .read()
            .map_err(|_| ResultCode::UnknownHResult(winapi::shared::winerror::E_FAIL))
    }

    /// Gets the base device with a write lock.
    /// This is not really necessary, but useful nonetheless.
    pub fn device_base_mut(&mut self) -> HcsResult<RwLockWriteGuard<HdvPciDeviceBase>> {
        self.base
            .as_mut()
            .ok_or(ResultCode::UnknownHResult(winapi::shared::winerror::E_FAIL))
            .map_err(|hresult| hresult)?
            .write()
            .map_err(|_| ResultCode::UnknownHResult(winapi::shared::winerror::E_FAIL))
    }
}

pub(crate) mod device_base_interface {
    use super::*;

    unsafe extern "system" fn initialize(device_context: *mut Void) -> HResult {
        match (*(device_context as *mut Arc<RwLock<dyn HdvPciDevice>>)).write() {
            Ok(mut device) => match device.initialize() {
                Ok(_) => winapi::shared::winerror::S_OK,
                Err(err) => result_code_to_hresult(err),
            },
            Err(_) => winapi::shared::winerror::E_FAIL,
        }
    }

    unsafe extern "system" fn teardown(device_context: *mut Void) {
        match (*(device_context as *mut Arc<RwLock<dyn HdvPciDevice>>)).write() {
            Ok(mut device) => device.teardown(),
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
        match (*(device_context as *mut Arc<RwLock<dyn HdvPciDevice>>)).write() {
            Ok(mut device) => match device.set_configuration(config_values) {
                Ok(_) => winapi::shared::winerror::S_OK,
                Err(err) => result_code_to_hresult(err),
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
        match (*(device_context as *mut Arc<RwLock<dyn HdvPciDevice>>)).write() {
            Ok(device) => match device.get_details(&mut *pnp_id, probed_bars) {
                Ok(_) => winapi::shared::winerror::S_OK,
                Err(err) => result_code_to_hresult(err),
            },
            Err(_) => winapi::shared::winerror::E_FAIL,
        }
    }

    unsafe extern "system" fn start(device_context: *mut Void) -> HResult {
        match (*(device_context as *mut Arc<RwLock<dyn HdvPciDevice>>)).write() {
            Ok(mut device) => match device.start() {
                Ok(_) => winapi::shared::winerror::S_OK,
                Err(err) => result_code_to_hresult(err),
            },
            Err(_) => winapi::shared::winerror::E_FAIL,
        }
    }

    unsafe extern "system" fn stop(device_context: *mut Void) {
        match (*(device_context as *mut Arc<RwLock<dyn HdvPciDevice>>)).write() {
            Ok(mut device) => device.stop(),
            Err(_) => {}
        }
    }

    unsafe extern "system" fn read_config_space(
        device_context: *mut Void,
        offset: u32,
        value: *mut u32,
    ) -> HResult {
        match (*(device_context as *mut Arc<RwLock<dyn HdvPciDevice>>)).write() {
            Ok(device) => match device.read_config_space(offset, &mut *value) {
                Ok(_) => winapi::shared::winerror::S_OK,
                Err(err) => result_code_to_hresult(err),
            },
            Err(_) => winapi::shared::winerror::E_FAIL,
        }
    }

    unsafe extern "system" fn write_config_space(
        device_context: *mut Void,
        offset: u32,
        value: u32,
    ) -> HResult {
        match (*(device_context as *mut Arc<RwLock<dyn HdvPciDevice>>)).write() {
            Ok(mut device) => match device.write_config_space(offset, value) {
                Ok(_) => winapi::shared::winerror::S_OK,
                Err(err) => result_code_to_hresult(err),
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
        match (*(device_context as *mut Arc<RwLock<dyn HdvPciDevice>>)).write() {
            Ok(device) => match device.read_intercepted_memory(bar_index, offset, values) {
                Ok(_) => winapi::shared::winerror::S_OK,
                Err(err) => result_code_to_hresult(err),
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
        match (*(device_context as *mut Arc<RwLock<dyn HdvPciDevice>>)).write() {
            Ok(mut device) => match device.write_intercepted_memory(bar_index, offset, values) {
                Ok(_) => winapi::shared::winerror::S_OK,
                Err(err) => result_code_to_hresult(err),
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
