// Copyright © rafawo (rafawo1@hotmail.com). All rights reserved.
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.
// All files in the project carrying such notice may not be copied, modified, or distributed
// except according to those terms.
// THE SOURCE CODE IS AVAILABLE UNDER THE ABOVE CHOSEN LICENSE "AS IS", WITH NO WARRANTIES.

//! Rust abstractions of the computestorage APIs.
//! These are only the Rust idiomatic equivalents of the C bindings.

pub(crate) mod bindings;
pub mod ispresent;

use crate::compute::errorcodes::hresult_to_result_code;
use crate::computestorage::bindings::*;
use crate::HcsResult;
use widestring::WideCString;
use winutils_rs::utilities::CoTaskMemWString;
use winutils_rs::windefs::*;

pub fn import_layer(path: &str, source_folder_path: &str, layer_data: &str) -> HcsResult<()> {
    unsafe {
        match HcsImportLayer(
            WideCString::from_str(path).unwrap().as_ptr(),
            WideCString::from_str(source_folder_path).unwrap().as_ptr(),
            WideCString::from_str(layer_data).unwrap().as_ptr(),
        ) {
            0 => Ok(()),
            hresult => Err(hresult_to_result_code(&hresult)),
        }
    }
}

pub fn export_layer(
    path: &str,
    export_folder_path: &str,
    layer_data: &str,
    options: &str,
) -> HcsResult<()> {
    unsafe {
        match HcsExportLayer(
            WideCString::from_str(path).unwrap().as_ptr(),
            WideCString::from_str(export_folder_path).unwrap().as_ptr(),
            WideCString::from_str(layer_data).unwrap().as_ptr(),
            WideCString::from_str(options).unwrap().as_ptr(),
        ) {
            0 => Ok(()),
            hresult => Err(hresult_to_result_code(&hresult)),
        }
    }
}

pub fn export_legacy_writable_layer(
    mount_path: &str,
    folder_path: &str,
    export_folder_path: &str,
    layer_data: &str,
) -> HcsResult<()> {
    unsafe {
        match HcsExportLegacyWritableLayer(
            WideCString::from_str(mount_path).unwrap().as_ptr(),
            WideCString::from_str(folder_path).unwrap().as_ptr(),
            WideCString::from_str(export_folder_path).unwrap().as_ptr(),
            WideCString::from_str(layer_data).unwrap().as_ptr(),
        ) {
            0 => Ok(()),
            hresult => Err(hresult_to_result_code(&hresult)),
        }
    }
}

pub fn destroy_layer(layer_path: &str) -> HcsResult<()> {
    unsafe {
        match HcsDestroyLayer(WideCString::from_str(layer_path).unwrap().as_ptr()) {
            0 => Ok(()),
            hresult => Err(hresult_to_result_code(&hresult)),
        }
    }
}

pub fn setup_base_os_layer(layer_path: &str, vhd_handle: Handle, options: &str) -> HcsResult<()> {
    unsafe {
        match HcsSetupBaseOSLayer(
            WideCString::from_str(layer_path).unwrap().as_ptr(),
            vhd_handle,
            WideCString::from_str(options).unwrap().as_ptr(),
        ) {
            0 => Ok(()),
            hresult => Err(hresult_to_result_code(&hresult)),
        }
    }
}

pub fn initialize_writable_layer(
    layer_path: &str,
    layer_data: &str,
    options: &str,
) -> HcsResult<()> {
    unsafe {
        match HcsInitializeWritableLayer(
            WideCString::from_str(layer_path).unwrap().as_ptr(),
            WideCString::from_str(layer_data).unwrap().as_ptr(),
            WideCString::from_str(options).unwrap().as_ptr(),
        ) {
            0 => Ok(()),
            hresult => Err(hresult_to_result_code(&hresult)),
        }
    }
}

pub fn initialize_legacy_writable_layer(
    mount_path: &str,
    folder_path: &str,
    layer_data: &str,
    options: &str,
) -> HcsResult<()> {
    unsafe {
        match HcsInitializeLegacyWritableLayer(
            WideCString::from_str(mount_path).unwrap().as_ptr(),
            WideCString::from_str(folder_path).unwrap().as_ptr(),
            WideCString::from_str(layer_data).unwrap().as_ptr(),
            WideCString::from_str(options).unwrap().as_ptr(),
        ) {
            0 => Ok(()),
            hresult => Err(hresult_to_result_code(&hresult)),
        }
    }
}

pub fn attach_layer_storage_filter(layer_path: &str, layer_data: &str) -> HcsResult<()> {
    unsafe {
        match HcsAttachLayerStorageFilter(
            WideCString::from_str(layer_path).unwrap().as_ptr(),
            WideCString::from_str(layer_data).unwrap().as_ptr(),
        ) {
            0 => Ok(()),
            hresult => Err(hresult_to_result_code(&hresult)),
        }
    }
}

pub fn detach_layer_storage_filter(layer_path: &str) -> HcsResult<()> {
    unsafe {
        match HcsDetachLayerStorageFilter(WideCString::from_str(layer_path).unwrap().as_ptr()) {
            0 => Ok(()),
            hresult => Err(hresult_to_result_code(&hresult)),
        }
    }
}

pub fn format_writable_layer_vhd(vhd_handle: Handle) -> HcsResult<()> {
    unsafe {
        match HcsFormatWritableLayerVhd(vhd_handle) {
            0 => Ok(()),
            hresult => Err(hresult_to_result_code(&hresult)),
        }
    }
}

pub fn get_layer_vhd_mount_path(vhd_handle: Handle) -> HcsResult<String> {
    unsafe {
        let mount_path = CoTaskMemWString::new();

        match HcsGetLayerVhdMountPath(vhd_handle, mount_path.ptr) {
            0 => Ok(mount_path.to_string()),
            hresult => Err(hresult_to_result_code(&hresult)),
        }
    }
}
