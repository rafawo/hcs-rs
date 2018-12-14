//! This module contains the Rust FFI bindings for the APIs to interact with storage management in HCS.

use crate::windefs::*;

#[link(name = "computestorage")]
extern "C" {
    /// Imports a container layer.
    pub fn HcsImportLayer(
        layerPath: PCWStr,
        sourceFolderPath: PCWStr,
        layerData: PCWStr,
    ) -> HResult;

    /// Exports a container layer.
    pub fn HcsExportLayer(
        layerPath: PCWStr,
        exportFolderPath: PCWStr,
        layerData: PCWStr,
        options: PCWStr,
    ) -> HResult;

    /// Exports a legacy container writable layer.
    pub fn HcsExportLegacyWritableLayer(
        writableLayerMountPath: PCWStr,
        writableLayerFolderPath: PCWStr,
        exportFolderPath: PCWStr,
        layerData: PCWStr,
    ) -> HResult;

    /// Deletes a container layer.
    pub fn HcsDestroyLayer(layerPath: PCWStr) -> HResult;

    /// Sets up a layer that contains a base OS for a container.
    pub fn HcsSetupBaseOSLayer(layerPath: PCWStr, vhdHandle: Handle, options: PCWStr) -> HResult;

    /// Initializes a writable layer for a container.
    pub fn HcsInitializeWritableLayer(
        writableLayerPath: PCWStr,
        layerData: PCWStr,
        options: PCWStr,
    ) -> HResult;

    /// Initializes a writable layer for a container using the legacy hive folder format.
    pub fn HcsInitializeLegacyWritableLayer(
        writableLayerMountPath: PCWStr,
        writableLayerFolderPath: PCWStr,
        layerData: PCWStr,
        options: PCWStr,
    ) -> HResult;

    /// Sets up the layer storage filter on a writable container layer.
    pub fn HcsAttachLayerStorageFilter(layerPath: PCWStr, layerData: PCWStr) -> HResult;

    /// Detaches the layer storage filter from a writable container layer.
    pub fn HcsDetachLayerStorageFilter(layerPath: PCWStr) -> HResult;

    /// Formats a virtual disk for the use as a writable container layer.
    pub fn HcsFormatWritableLayerVhd(vhdHandle: Handle) -> HResult;

    /// Returns the volume path for a virtual disk of a writable container layer.
    pub fn HcsGetLayerVhdMountPath(vhdHandle: Handle, mountPath: *mut PWStr) -> HResult;

    pub fn IsHcsImportLayerPresent() -> Boolean;

    pub fn IsHcsExportLayerPresent() -> Boolean;

    pub fn IsHcsExportLegacyWritableLayerPresent() -> Boolean;

    pub fn IsHcsDestroyLayerPresent() -> Boolean;

    pub fn IsHcsSetupBaseOSLayerPresent() -> Boolean;

    pub fn IsHcsInitializeWritableLayerPresent() -> Boolean;

    pub fn IsHcsInitializeLegacyWritableLayerPresent() -> Boolean;

    pub fn IsHcsAttachLayerStorageFilterPresent() -> Boolean;

    pub fn IsHcsDetachLayerStorageFilterPresent() -> Boolean;

    pub fn IsHcsFormatWritableLayerVhdPresent() -> Boolean;

    pub fn IsHcsGetLayerVhdMountPathPresent() -> Boolean;
}
