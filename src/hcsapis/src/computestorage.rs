//! This file contains the types definitions and APIs to interact with storage management in HCS.

use winapi::shared::ntdef::{BOOLEAN, HANDLE, HRESULT, PCWSTR, PWSTR};

extern "C" {

    /// Imports a container layer.
    pub fn HcsImportLayer(
        layerPath: PCWSTR,
        sourceFolderPath: PCWSTR,
        layerData: PCWSTR,
    ) -> HRESULT;

    /// Exports a container layer.
    pub fn HcsExportLayer(
        layerPath: PCWSTR,
        exportFolderPath: PCWSTR,
        layerData: PCWSTR,
        options: PCWSTR,
    ) -> HRESULT;

    /// Exports a legacy container writable layer.
    pub fn HcsExportLegacyWritableLayer(
        writableLayerMountPath: PCWSTR,
        writableLayerFolderPath: PCWSTR,
        exportFolderPath: PCWSTR,
        layerData: PCWSTR,
    ) -> HRESULT;

    /// Deletes a container layer.
    pub fn HcsDestroyLayer(layerPath: PCWSTR) -> HRESULT;

    /// Sets up a layer that contains a base OS for a container.
    pub fn HcsSetupBaseOSLayer(layerPath: PCWSTR, vhdHandle: HANDLE, options: PCWSTR) -> HRESULT;

    /// Initializes a writable layer for a container.
    pub fn HcsInitializeWritableLayer(
        writableLayerPath: PCWSTR,
        layerData: PCWSTR,
        options: PCWSTR,
    ) -> HRESULT;

    /// Initializes a writable layer for a container using the legacy hive folder format.
    pub fn HcsInitializeLegacyWritableLayer(
        writableLayerMountPath: PCWSTR,
        writableLayerFolderPath: PCWSTR,
        layerData: PCWSTR,
        options: PCWSTR,
    ) -> HRESULT;

    /// Sets up the layer storage filter on a writable container layer.
    pub fn HcsAttachLayerStorageFilter(layerPath: PCWSTR, layerData: PCWSTR) -> HRESULT;

    /// Detaches the layer storage filter from a writable container layer.
    pub fn HcsDetachLayerStorageFilter(layerPath: PCWSTR) -> HRESULT;

    /// Formats a virtual disk for the use as a writable container layer.
    pub fn HcsFormatWritableLayerVhd(vhdHandle: HANDLE) -> HRESULT;

    /// Returns the volume path for a virtual disk of a writable container layer.
    pub fn HcsGetLayerVhdMountPath(vhdHandle: HANDLE, mountPath: *mut PWSTR) -> HRESULT;

    pub fn IsHcsImportLayerPresent() -> BOOLEAN;

    pub fn IsHcsExportLayerPresent() -> BOOLEAN;

    pub fn IsHcsExportLegacyWritableLayerPresent() -> BOOLEAN;

    pub fn IsHcsDestroyLayerPresent() -> BOOLEAN;

    pub fn IsHcsSetupBaseOSLayerPresent() -> BOOLEAN;

    pub fn IsHcsInitializeWritableLayerPresent() -> BOOLEAN;

    pub fn IsHcsInitializeLegacyWritableLayerPresent() -> BOOLEAN;

    pub fn IsHcsAttachLayerStorageFilterPresent() -> BOOLEAN;

    pub fn IsHcsDetachLayerStorageFilterPresent() -> BOOLEAN;

    pub fn IsHcsFormatWritableLayerVhdPresent() -> BOOLEAN;

    pub fn IsHcsGetLayerVhdMountPathPresent() -> BOOLEAN;

}
