//! This module contains the types definitions for the hypervdevicevirtualization APIs.

use crate::windefs::*;

pub type HdvHost = *const Void;
pub type HdvDevice = *const Void;

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
pub type HdvPciDeviceInitialize = extern "C" fn(deviceContext: *mut Void) -> HResult;
pub type HdvPciDeviceTeardown = extern "C" fn(deviceContext: *mut Void);

pub type HdvPciDeviceSetConfiguration = extern "C" fn(
    deviceContext: *mut Void,
    configurationValueCount: u32,
    configurationValues: *const PCWStr,
) -> HResult;

pub type HdvPciDeviceGetDetails = extern "C" fn(
    deviceContext: *mut Void,
    pnpId: PHdvPciPnpId,
    probedBarsCount: u32,
    probedBars: *mut u32,
) -> HResult;

pub type HdvPciDeviceStart = extern "C" fn(deviceContext: *mut Void) -> HResult;

pub type HdvPciDeviceStop = extern "C" fn(deviceContext: *mut Void);

pub type HdvPciReadConfigSpace =
    extern "C" fn(deviceContext: *mut Void, offset: u32, value: *mut u32) -> HResult;

pub type HdvPciWriteConfigSpace =
    extern "C" fn(deviceContext: *mut Void, offset: u32, value: u32) -> HResult;

pub type HdvPciReadInterceptedMemory = extern "C" fn(
    deviceContext: *mut Void,
    barIndex: HdvPciBarSelector,
    offset: u64,
    length: u64,
    value: *mut Byte,
) -> HResult;

pub type HdvPciWriteInterceptedMemory = extern "C" fn(
    deviceContext: *mut Void,
    barIndex: HdvPciBarSelector,
    offset: u64,
    length: u64,
    value: *const Byte,
) -> HResult;

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum HdvPciInterfaceVersion {
    VersionInvalid = 0,
    Version1 = 1,
}

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
