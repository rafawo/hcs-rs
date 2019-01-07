// Copyright © rafawo (rafawo1@hotmail.com). All rights reserved.
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.
// All files in the project carrying such notice may not be copied, modified, or distributed
// except according to those terms.
// THE SOURCE CODE IS AVAILABLE UNDER THE ABOVE CHOSEN LICENSE "AS IS", WITH NO WARRANTIES.

use super::bindings::*;

pub fn import_layer_is_present() -> bool {
    unsafe { IsHcsImportLayerPresent() != 0 }
}

pub fn export_layer_is_present() -> bool {
    unsafe { IsHcsExportLayerPresent() != 0 }
}

pub fn export_legacy_writable_layer_is_present() -> bool {
    unsafe { IsHcsExportLegacyWritableLayerPresent() != 0 }
}

pub fn destroy_layer_is_present() -> bool {
    unsafe { IsHcsDestroyLayerPresent() != 0 }
}

pub fn setup_base_os_layer_is_present() -> bool {
    unsafe { IsHcsSetupBaseOSLayerPresent() != 0 }
}

pub fn initialize_writable_layer_is_present() -> bool {
    unsafe { IsHcsInitializeWritableLayerPresent() != 0 }
}

pub fn initialize_legacy_writable_layer_is_present() -> bool {
    unsafe { IsHcsInitializeLegacyWritableLayerPresent() != 0 }
}

pub fn attach_layer_storage_filter_is_present() -> bool {
    unsafe { IsHcsAttachLayerStorageFilterPresent() != 0 }
}

pub fn detach_layer_storage_filter_is_present() -> bool {
    unsafe { IsHcsDetachLayerStorageFilterPresent() != 0 }
}

pub fn format_writable_layer_vhd_is_present() -> bool {
    unsafe { IsHcsFormatWritableLayerVhdPresent() != 0 }
}

pub fn get_layer_vhd_mount_path_is_present() -> bool {
    unsafe { IsHcsGetLayerVhdMountPathPresent() != 0 }
}