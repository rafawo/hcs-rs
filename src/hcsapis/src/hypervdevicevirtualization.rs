//! This file contains types definitions and APIs to interact with the Hyper-V device virtualization support.

use computedefs::*;
use winapi::shared::guiddef::GUID;
use winapi::shared::minwindef::{BOOL, BYTE};
use winapi::shared::ntdef::{BOOLEAN, HRESULT, LPCWSTR, PVOID, VOID};

#[allow(non_camel_case_types)]
pub type HDV_HOST = *const VOID;

#[allow(non_camel_case_types)]
pub type HDV_DEVICE = *const VOID;

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum HdvDeviceType {
    Undefined = 0,
    PCI = 1,
}

#[repr(C)] #[derive(Debug)]
pub struct HdvPciPnpId {
    vendor_id: u16,
    device_id: u16,
    revision_id: u8,
    prog_if: u8,
    sub_class: u8,
    base_class: u8,
    sub_vendor_id: u16,
    sub_system_id: u16,
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
pub type HDV_PCI_DEVICE_INITIALIZE = extern "C" fn(deviceContext: *mut VOID) -> HRESULT;

#[allow(non_camel_case_types)]
pub type HDV_PCI_DEVICE_TEARDOWN = extern "C" fn(deviceContext: *mut VOID);

#[allow(non_camel_case_types)]
pub type HDV_PCI_DEVICE_SET_CONFIGURATION = extern "C" fn(
    deviceContext: *mut VOID,
    configurationValueCount: u32,
    configurationValues: *const LPCWSTR,
) -> HRESULT;

#[allow(non_camel_case_types)]
pub type HDV_PCI_DEVICE_GET_DETAILS = extern "C" fn(
    deviceContext: *mut VOID,
    pnpId: PHDV_PCI_PNP_ID,
    probedBarsCount: u32,
    probedBars: *mut u32,
) -> HRESULT;

#[allow(non_camel_case_types)]
pub type HDV_PCI_DEVICE_START = extern "C" fn(deviceContext: *mut VOID) -> HRESULT;

#[allow(non_camel_case_types)]
pub type HDV_PCI_DEVICE_STOP = extern "C" fn(deviceContext: *mut VOID);

#[allow(non_camel_case_types)]
pub type HDV_PCI_READ_CONFIG_SPACE =
    extern "C" fn(deviceContext: *mut VOID, offset: u32, value: *mut u32) -> HRESULT;

#[allow(non_camel_case_types)]
pub type HDV_PCI_WRITE_CONFIG_SPACE =
    extern "C" fn(deviceContext: *mut VOID, offset: u32, value: u32) -> HRESULT;

#[allow(non_camel_case_types)]
pub type HDV_PCI_READ_INTERCEPTED_MEMORY = extern "C" fn(
    deviceContext: *mut VOID,
    barIndex: HdvPciBarSelector,
    offset: u64,
    length: u64,
    value: *mut BYTE,
) -> HRESULT;

#[allow(non_camel_case_types)]
pub type HDV_PCI_WRITE_INTERCEPTED_MEMORY = extern "C" fn(
    deviceContext: *mut VOID,
    barIndex: HdvPciBarSelector,
    offset: u64,
    length: u64,
    value: *const BYTE,
) -> HRESULT;
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum HdvPciInterfaceVersion {
    VersionInvalid = 0,
    Version1 = 1,
}

pub struct HdvPciDeviceInterface {
    version: HdvPciInterfaceVersion,
    initialize: HDV_PCI_DEVICE_INITIALIZE,
    teardown: HDV_PCI_DEVICE_TEARDOWN,
    set_configuration: HDV_PCI_DEVICE_SET_CONFIGURATION,
    get_details: HDV_PCI_DEVICE_GET_DETAILS,
    start: HDV_PCI_DEVICE_START,
    stop: HDV_PCI_DEVICE_STOP,
    read_config_space: HDV_PCI_READ_CONFIG_SPACE,
    write_config_space: HDV_PCI_WRITE_CONFIG_SPACE,
    read_intercepted_memory: HDV_PCI_READ_INTERCEPTED_MEMORY,
    write_intercepted_memory: HDV_PCI_WRITE_INTERCEPTED_MEMORY,
}

#[allow(non_camel_case_types)]
pub type PHDV_PCI_DEVICE_INTERFACE = *mut HdvPciDeviceInterface;

extern "C" {

    pub fn HdvInitializeDeviceHost(
        computeSystem: HCS_SYSTEM,
        deviceHostHandle: *mut HDV_HOST,
    ) -> HRESULT;

    pub fn HdvTeardownDeviceHost(deviceHostHandle: HDV_HOST) -> HRESULT;

    pub fn HdvCreateDeviceInstance(
        deviceHostHandle: HDV_HOST,
        deviceType: HdvDeviceType,
        deviceClassId: *const GUID,
        deviceInstanceId: *const GUID,
        deviceInterface: *const VOID,
        deviceContext: *mut VOID,
        deviceHandle: *mut HDV_DEVICE,
    ) -> HRESULT;

    pub fn HdvReadGuestMemory(
        requestor: HDV_DEVICE,
        guestPhysicalAddress: u64,
        byteCount: u32,
        buffer: *mut BYTE,
    ) -> HRESULT;

    pub fn HdvWriteGuestMemory(
        requestor: HDV_DEVICE,
        guestPhysicalAddress: u64,
        byteCount: u32,
        buffer: *const BYTE,
    ) -> HRESULT;

    pub fn HdvCreateGuestMemoryAperture(
        requestor: HDV_DEVICE,
        guestPhysicalAddress: u64,
        byteCount: u32,
        writeProtected: BOOL,
        mappedAddress: *mut PVOID,
    ) -> HRESULT;

    pub fn HdvDestroyGuestMemoryAperture(requestor: HDV_DEVICE, mappedAddress: PVOID) -> HRESULT;

    pub fn HdvDeliverGuestInterrupt(
        requestor: HDV_DEVICE,
        msiAddress: u64,
        msiData: u32,
    ) -> HRESULT;

    pub fn IsHdvInitializeDeviceHostPresent() -> BOOLEAN;

    pub fn IsHdvTeardownDeviceHostPresent() -> BOOLEAN;

    pub fn IsHdvCreateDeviceInstancePresent() -> BOOLEAN;

    pub fn IsHdvReadGuestMemoryPresent() -> BOOLEAN;

    pub fn IsHdvWriteGuestMemoryPresent() -> BOOLEAN;

    pub fn IsHdvCreateGuestMemoryAperturePresent() -> BOOLEAN;

    pub fn IsHdvDestroyGuestMemoryAperturePresent() -> BOOLEAN;

    pub fn IsHdvDeliverGuestInterruptPresent() -> BOOLEAN;

}
