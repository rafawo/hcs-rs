// Copyright © rafawo (rafawo1@hotmail.com). All rights reserved.
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.
// All files in the project carrying such notice may not be copied, modified, or distributed
// except according to those terms.
// THE SOURCE CODE IS AVAILABLE UNDER THE ABOVE CHOSEN LICENSE "AS IS", WITH NO WARRANTIES.

//! Rust abstractions of the "IsPresent" variants of the hypervdevicevirtualization APIs.
//! These are used to tell if the hosting DLL is present in the system, prior to calling the real API.

use super::bindings::*;

pub fn initialize_device_host() -> bool {
    unsafe { IsHdvInitializeDeviceHostPresent() != 0 }
}

pub fn teardown_device_host() -> bool {
    unsafe { IsHdvTeardownDeviceHostPresent() != 0 }
}

pub fn create_device_instance() -> bool {
    unsafe { IsHdvCreateDeviceInstancePresent() != 0 }
}

pub fn read_guest_memory() -> bool {
    unsafe { IsHdvReadGuestMemoryPresent() != 0 }
}

pub fn write_guest_memory() -> bool {
    unsafe { IsHdvWriteGuestMemoryPresent() != 0 }
}

pub fn create_guest_memory_aperture() -> bool {
    unsafe { IsHdvCreateGuestMemoryAperturePresent() != 0 }
}

pub fn destroy_guest_memory_aperture() -> bool {
    unsafe { IsHdvDestroyGuestMemoryAperturePresent() != 0 }
}

pub fn deliver_guest_interrupt() -> bool {
    unsafe { IsHdvDeliverGuestInterruptPresent() != 0 }
}

pub fn register_doorbell_page() -> bool {
    unsafe { IsHdvRegisterDoorbellPage() != 0 }
}

pub fn unregister_doorbell_page() -> bool {
    unsafe { IsHdvUnregisterDoorbellPage() != 0 }
}

pub fn register_doorbell() -> bool {
    unsafe { IsHdvRegisterDoorbell() != 0 }
}

pub fn unregister_doorbell() -> bool {
    unsafe { IsHdvUnregisterDoorbell() != 0 }
}
