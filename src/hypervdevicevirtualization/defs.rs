// Copyright © rafawo (rafawo1@hotmail.com). All rights reserved.
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.
// All files in the project carrying such notice may not be copied, modified, or distributed
// except according to those terms.
// THE SOURCE CODE IS AVAILABLE UNDER THE ABOVE CHOSEN LICENSE "AS IS", WITH NO WARRANTIES.

//! This module contains the types definitions for the hypervdevicevirtualization APIs.

use winutils_rs::windefs::*;

pub type HdvHostHandle = *const Void;
pub type HdvDeviceHandle = *const Void;

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum HdvDeviceType {
    Undefined = 0,
    Pci = 1,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct HdvPciPnpId {
    pub vendor_id: u16,
    pub device_id: u16,
    pub revision_id: u8,
    pub prog_if: u8,
    pub sub_class: u8,
    pub base_class: u8,
    pub sub_vendor_id: u16,
    pub sub_system_id: u16,
}

pub type PHdvPciPnpId = *mut HdvPciPnpId;

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum HdvPciBarSelector {
    Bar0 = 0,
    Bar1 = 1,
    Bar2 = 2,
    Bar3 = 3,
    Bar4 = 4,
    Bar5 = 5,
}

pub const HDV_PCI_BAR_COUNT: u32 = 6;

pub type HdvPciDeviceInitialize =
    Option<unsafe extern "system" fn(deviceContext: *mut Void) -> HResult>;

pub type HdvPciDeviceTeardown = Option<unsafe extern "system" fn(deviceContext: *mut Void)>;

pub type HdvPciDeviceSetConfiguration = Option<
    unsafe extern "system" fn(
        deviceContext: *mut Void,
        configurationValueCount: u32,
        configurationValues: *const PCWStr,
    ) -> HResult,
>;

pub type HdvPciDeviceGetDetails = Option<
    unsafe extern "system" fn(
        deviceContext: *mut Void,
        pnpId: PHdvPciPnpId,
        probedBarsCount: u32,
        probedBars: *mut u32,
    ) -> HResult,
>;

pub type HdvPciDeviceStart = Option<unsafe extern "system" fn(deviceContext: *mut Void) -> HResult>;

pub type HdvPciDeviceStop = Option<unsafe extern "system" fn(deviceContext: *mut Void)>;

pub type HdvPciReadConfigSpace = Option<
    unsafe extern "system" fn(deviceContext: *mut Void, offset: u32, value: *mut u32) -> HResult,
>;

pub type HdvPciWriteConfigSpace =
    Option<unsafe extern "system" fn(deviceContext: *mut Void, offset: u32, value: u32) -> HResult>;

pub type HdvPciReadInterceptedMemory = Option<
    unsafe extern "system" fn(
        deviceContext: *mut Void,
        barIndex: HdvPciBarSelector,
        offset: u64,
        length: u64,
        value: *mut Byte,
    ) -> HResult,
>;

pub type HdvPciWriteInterceptedMemory = Option<
    unsafe extern "system" fn(
        deviceContext: *mut Void,
        barIndex: HdvPciBarSelector,
        offset: u64,
        length: u64,
        value: *const Byte,
    ) -> HResult,
>;

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum HdvPciInterfaceVersion {
    VersionInvalid = 0,
    Version1 = 1,
}

#[repr(C)]
pub struct HdvPciDeviceInterface {
    pub version: HdvPciInterfaceVersion,
    pub initialize: HdvPciDeviceInitialize,
    pub teardown: HdvPciDeviceTeardown,
    pub set_configuration: HdvPciDeviceSetConfiguration,
    pub get_details: HdvPciDeviceGetDetails,
    pub start: HdvPciDeviceStart,
    pub stop: HdvPciDeviceStop,
    pub read_config_space: HdvPciReadConfigSpace,
    pub write_config_space: HdvPciWriteConfigSpace,
    pub read_intercepted_memory: HdvPciReadInterceptedMemory,
    pub write_intercepted_memory: HdvPciWriteInterceptedMemory,
}

pub type PHdvPciDeviceInterface = *mut HdvPciDeviceInterface;
