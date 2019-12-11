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
use winutils_rs::windefs::*;

pub trait HdvPciDevice {
    fn initialize(&mut self) -> HResult;
    fn teardown(&mut self);
    fn set_configuration(&mut self, values: &[PCWStr]) -> HResult;
    fn get_details(&self, pnpi_id: PHdvPciPnpId, probed_bars: &mut [u32]) -> HResult;
    fn start(&mut self) -> HResult;
    fn stop(&mut self);
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

        hypervdevicevirtualization::create_device_instance(
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

    unsafe extern "system" fn teardown(device_context: *mut Void) {
        let mut device: Box<Box<dyn HdvPciDevice>> = Box::from_raw(device_context as *mut _);
        (*device).teardown();
    }

    unsafe extern "system" fn set_configuration(
        device_context: *mut Void,
        configuration_value_count: u32,
        configuration_values: *const PCWStr,
    ) -> HResult {
        let mut device: Box<Box<dyn HdvPciDevice>> = Box::from_raw(device_context as *mut _);
        let config_values: &[PCWStr] =
            std::slice::from_raw_parts(configuration_values, configuration_value_count as usize);
        (*device).set_configuration(config_values)
    }

    unsafe extern "system" fn get_details(
        device_context: *mut Void,
        pnp_id: PHdvPciPnpId,
        probed_bars_count: u32,
        probed_bars: *mut u32,
    ) -> HResult {
        let device: Box<Box<dyn HdvPciDevice>> = Box::from_raw(device_context as *mut _);
        let probed_bars: &mut [u32] =
            std::slice::from_raw_parts_mut(probed_bars, probed_bars_count as usize);
        (*device).get_details(pnp_id, probed_bars)
    }

    unsafe extern "system" fn start(device_context: *mut Void) -> HResult {
        let mut device: Box<Box<dyn HdvPciDevice>> = Box::from_raw(device_context as *mut _);
        (*device).start()
    }

    unsafe extern "system" fn stop(device_context: *mut Void) {
        let mut device: Box<Box<dyn HdvPciDevice>> = Box::from_raw(device_context as *mut _);
        (*device).stop();
    }

    unsafe extern "system" fn read_config_space(
        device_context: *mut Void,
        offset: u32,
        value: *mut u32,
    ) -> HResult {
        let device: Box<Box<dyn HdvPciDevice>> = Box::from_raw(device_context as *mut _);
        (*device).read_config_space(offset, &mut *value)
    }

    unsafe extern "system" fn write_config_space(
        device_context: *mut Void,
        offset: u32,
        value: u32,
    ) -> HResult {
        let mut device: Box<Box<dyn HdvPciDevice>> = Box::from_raw(device_context as *mut _);
        (*device).write_config_space(offset, value)
    }

    unsafe extern "system" fn read_intercepted_memory(
        device_context: *mut Void,
        bar_index: HdvPciBarSelector,
        offset: u64,
        length: u64,
        value: *mut Byte,
    ) -> HResult {
        let device: Box<Box<dyn HdvPciDevice>> = Box::from_raw(device_context as *mut _);
        let values: &mut [Byte] = std::slice::from_raw_parts_mut(value, length as usize);
        (*device).read_intercepted_memory(bar_index, offset, values)
    }

    unsafe extern "system" fn write_intercepted_memory(
        device_context: *mut Void,
        bar_index: HdvPciBarSelector,
        offset: u64,
        length: u64,
        value: *const Byte,
    ) -> HResult {
        let mut device: Box<Box<dyn HdvPciDevice>> = Box::from_raw(device_context as *mut _);
        let values: &[Byte] = std::slice::from_raw_parts(value, length as usize);
        (*device).write_intercepted_memory(bar_index, offset, values)
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
