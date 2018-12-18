//! Rust abstractions of the computestorage APIs.
//! These APIs provide the basic management of container layers.
//!

use crate::computestorage_bindings::*;
use crate::hresult_to_result_code;
use crate::ResultCode;
use widestring::U16CString;

pub fn import_layer(path: &str, source_folder_path: &str, layer_data: &str) -> ResultCode {
    unsafe {
        hresult_to_result_code(&HcsImportLayer(
            U16CString::from_str(path).unwrap().as_ptr(),
            U16CString::from_str(source_folder_path).unwrap().as_ptr(),
            U16CString::from_str(layer_data).unwrap().as_ptr(),
        ))
    }
}

pub fn export_layer(
    path: &str,
    export_folder_path: &str,
    layer_data: &str,
    options: &str,
) -> ResultCode {
    unsafe {
        hresult_to_result_code(&HcsExportLayer(
            U16CString::from_str(path).unwrap().as_ptr(),
            U16CString::from_str(export_folder_path).unwrap().as_ptr(),
            U16CString::from_str(layer_data).unwrap().as_ptr(),
            U16CString::from_str(options).unwrap().as_ptr(),
        ))
    }
}
