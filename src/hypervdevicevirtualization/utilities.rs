// Copyright © rafawo (rafawo1@hotmail.com). All rights reserved.
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.
// All files in the project carrying such notice may not be copied, modified, or distributed
// except according to those terms.
// THE SOURCE CODE IS AVAILABLE UNDER THE ABOVE CHOSEN LICENSE "AS IS", WITH NO WARRANTIES.

//! Rust types that provide convenient functionality built on top of the hypervdevicevirtualiation APIs.

use crate::hypervdevicevirtualization::defs::*;
use crate::hypervdevicevirtualization::*;
use winutils_rs::windefs::*;

pub trait HdvPciDevice {
    fn initialize(&mut self) -> HResult;
    fn teardown(&mut self) -> HResult;
    fn set_configuration(&mut self, values: &[PCWStr]) -> HResult;
    fn get_details(&self, pnpi_id: PHdvPciPnpId, probed_bars: &mut [u32]) -> HResult;
    fn start(&mut self) -> HResult;
    fn stop(&mut self) -> HResult;
    fn read_config_space(&self, offset: u32, value: &mut u32) -> HResult;
    fn write_config_space(&mut self, offset: u32, value: u32) -> HResult;
    fn read_intercepted_memory(
        &self,
        bar_index: HdvPciBarSelector,
        offset: u64,
        value: &mut [Byte],
    ) -> HResult;
    fn write_intercepted_memory(
        &mut self,
        bar_index: HdvPciBarSelector,
        offset: u64,
        value: &[Byte],
    ) -> HResult;
}

pub struct HdvPciDeviceBase {
    pub device: Box<dyn HdvPciDevice>,
}

impl HdvPciDeviceBase {
    pub fn new(
        device_host_handle: HdvHostHandle,
        device_class_id: &Guid,
        device_instance_id: &Guid,
        device: Box<dyn HdvPciDevice>,
    ) -> HcsResult<HdvPciDeviceBase> {
        let mut device_base = HdvPciDeviceBase { device };

        let boxed_device = Box::new(&mut (*device_base.device) as *mut _);

        create_device_instance(
            device_host_handle,
            HdvDeviceType::Pci,
            device_class_id,
            device_instance_id,
            &device_base_interface::DEVICE_INTERFACE as *const _ as *const Void,
            Box::into_raw(boxed_device) as PVoid,
        )?;

        Ok(device_base)
    }
}

pub(crate) mod device_base_interface {
    use super::*;

    unsafe extern "system" fn initialize(device_context: *mut Void) -> HResult {
        let mut device: Box<Box<dyn HdvPciDevice>> = Box::from_raw(device_context as *mut _);
        (*device).initialize()
    }

    /// Global HDV PCI device interface object with all the necessary
    /// callbacks assigned to global unsafe functions that take care
    /// of forwarding the calls to higher-abstract structs.
    pub static DEVICE_INTERFACE: HdvPciDeviceInterface = HdvPciDeviceInterface {
        version: HdvPciInterfaceVersion::Version1,
        initialize: Some(initialize),
        teardown: None,
        set_configuration: None,
        get_details: None,
        start: None,
        stop: None,
        read_config_space: None,
        write_config_space: None,
        read_intercepted_memory: None,
        write_intercepted_memory: None,
    };
}
