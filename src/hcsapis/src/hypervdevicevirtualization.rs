//! This file contains types definitions and APIs to interact with the Hyper-V device virtualization support.

use crate::computedefs::*;
use crate::windefs::*;

#[allow(non_camel_case_types)]
pub type HDV_HOST = *const Void;

#[allow(non_camel_case_types)]
pub type HDV_DEVICE = *const Void;

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum HdvDeviceType {
    Undefined = 0,
    PCI = 1,
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

#[allow(non_camel_case_types)]
pub type PHDV_PCI_PNP_ID = *mut HdvPciPnpId;

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

#[allow(non_camel_case_types)]
pub const HDV_PCI_BAR_COUNT: u32 = 6;

#[allow(non_camel_case_types)]
pub type HDV_PCI_DEVICE_INITIALIZE = extern "C" fn(deviceContext: *mut Void) -> HResult;

#[allow(non_camel_case_types)]
pub type HDV_PCI_DEVICE_TEARDOWN = extern "C" fn(deviceContext: *mut Void);

#[allow(non_camel_case_types)]
pub type HDV_PCI_DEVICE_SET_CONFIGURATION = extern "C" fn(
    deviceContext: *mut Void,
    configurationValueCount: u32,
    configurationValues: *const PCWStr,
) -> HResult;

#[allow(non_camel_case_types)]
pub type HDV_PCI_DEVICE_GET_DETAILS = extern "C" fn(
    deviceContext: *mut Void,
    pnpId: PHDV_PCI_PNP_ID,
    probedBarsCount: u32,
    probedBars: *mut u32,
) -> HResult;

#[allow(non_camel_case_types)]
pub type HDV_PCI_DEVICE_START = extern "C" fn(deviceContext: *mut Void) -> HResult;

#[allow(non_camel_case_types)]
pub type HDV_PCI_DEVICE_STOP = extern "C" fn(deviceContext: *mut Void);

#[allow(non_camel_case_types)]
pub type HDV_PCI_READ_CONFIG_SPACE =
    extern "C" fn(deviceContext: *mut Void, offset: u32, value: *mut u32) -> HResult;

#[allow(non_camel_case_types)]
pub type HDV_PCI_WRITE_CONFIG_SPACE =
    extern "C" fn(deviceContext: *mut Void, offset: u32, value: u32) -> HResult;

#[allow(non_camel_case_types)]
pub type HDV_PCI_READ_INTERCEPTED_MEMORY = extern "C" fn(
    deviceContext: *mut Void,
    barIndex: HdvPciBarSelector,
    offset: u64,
    length: u64,
    value: *mut Byte,
) -> HResult;

#[allow(non_camel_case_types)]
pub type HDV_PCI_WRITE_INTERCEPTED_MEMORY = extern "C" fn(
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
    pub initialize: HDV_PCI_DEVICE_INITIALIZE,
    pub teardown: HDV_PCI_DEVICE_TEARDOWN,
    pub set_configuration: HDV_PCI_DEVICE_SET_CONFIGURATION,
    pub get_details: HDV_PCI_DEVICE_GET_DETAILS,
    pub start: HDV_PCI_DEVICE_START,
    pub stop: HDV_PCI_DEVICE_STOP,
    pub read_config_space: HDV_PCI_READ_CONFIG_SPACE,
    pub write_config_space: HDV_PCI_WRITE_CONFIG_SPACE,
    pub read_intercepted_memory: HDV_PCI_READ_INTERCEPTED_MEMORY,
    pub write_intercepted_memory: HDV_PCI_WRITE_INTERCEPTED_MEMORY,
}

#[allow(non_camel_case_types)]
pub type PHDV_PCI_DEVICE_INTERFACE = *mut HdvPciDeviceInterface;

#[link(name = "vmdevicehost")]
extern "C" {

    pub fn HdvInitializeDeviceHost(
        computeSystem: HcsSystem,
        deviceHostHandle: *mut HDV_HOST,
    ) -> HResult;

    pub fn HdvTeardownDeviceHost(deviceHostHandle: HDV_HOST) -> HResult;

    pub fn HdvCreateDeviceInstance(
        deviceHostHandle: HDV_HOST,
        deviceType: HdvDeviceType,
        deviceClassId: *const Guid,
        deviceInstanceId: *const Guid,
        deviceInterface: *const Void,
        deviceContext: *mut Void,
        deviceHandle: *mut HDV_DEVICE,
    ) -> HResult;

    pub fn HdvReadGuestMemory(
        requestor: HDV_DEVICE,
        guestPhysicalAddress: u64,
        ByteCount: u32,
        buffer: *mut Byte,
    ) -> HResult;

    pub fn HdvWriteGuestMemory(
        requestor: HDV_DEVICE,
        guestPhysicalAddress: u64,
        ByteCount: u32,
        buffer: *const Byte,
    ) -> HResult;

    pub fn HdvCreateGuestMemoryAperture(
        requestor: HDV_DEVICE,
        guestPhysicalAddress: u64,
        ByteCount: u32,
        writeProtected: Bool,
        mappedAddress: *mut PVoid,
    ) -> HResult;

    pub fn HdvDestroyGuestMemoryAperture(requestor: HDV_DEVICE, mappedAddress: PVoid) -> HResult;

    pub fn HdvDeliverGuestInterrupt(
        requestor: HDV_DEVICE,
        msiAddress: u64,
        msiData: u32,
    ) -> HResult;

    pub fn IsHdvInitializeDeviceHostPresent() -> Boolean;

    pub fn IsHdvTeardownDeviceHostPresent() -> Boolean;

    pub fn IsHdvCreateDeviceInstancePresent() -> Boolean;

    pub fn IsHdvReadGuestMemoryPresent() -> Boolean;

    pub fn IsHdvWriteGuestMemoryPresent() -> Boolean;

    pub fn IsHdvCreateGuestMemoryAperturePresent() -> Boolean;

    pub fn IsHdvDestroyGuestMemoryAperturePresent() -> Boolean;

    pub fn IsHdvDeliverGuestInterruptPresent() -> Boolean;

}
