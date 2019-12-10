// Copyright © rafawo (rafawo1@hotmail.com). All rights reserved.
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.
// All files in the project carrying such notice may not be copied, modified, or distributed
// except according to those terms.
// THE SOURCE CODE IS AVAILABLE UNDER THE ABOVE CHOSEN LICENSE "AS IS", WITH NO WARRANTIES.

//! Rust abstractions of the computenetwork APIs.
//! These are Rust idiomatic equivalents of the C bindings.

#[cfg(not(feature = "bindings"))]
pub(crate) mod bindings;
#[cfg(feature = "bindings")]
pub mod bindings;

pub mod defs;

use crate::compute::errorcodes::{hresult_to_result_code, ResultCode};
use crate::computenetwork::bindings::*;
use crate::computenetwork::defs::*;
use widestring::WideCString;
use winutils_rs::utilities::CoTaskMemWString;
use winutils_rs::windefs::*;

#[derive(Debug)]
pub struct ErrorResult {
    pub error_record: String,
    pub result_code: ResultCode,
}

impl ErrorResult {
    pub fn new(error_record: String, hresult: HResult) -> ErrorResult {
        ErrorResult {
            error_record,
            result_code: hresult_to_result_code(&hresult),
        }
    }
}

/// Alias used by HCN results, which on error, contain an error record as a JSON object
/// and the underlying returned result code.
pub type HcnResult<T> = Result<T, ErrorResult>;

/// Return a list of existing Networks.
pub fn enumerate_networks(query: &str) -> HcnResult<String> {
    unsafe {
        let networks = CoTaskMemWString::new();
        let error_record = CoTaskMemWString::new();

        match HcnEnumerateNetworks(
            WideCString::from_str(query).unwrap().as_ptr(),
            networks.ptr,
            error_record.ptr,
        ) {
            0 => Ok(networks.to_string()),
            hresult => Err(ErrorResult::new(error_record.to_string(), hresult)),
        }
    }
}

/// Create a network.
pub fn create_network(id: &Guid, settings: &str) -> HcnResult<HcnNetworkHandle> {
    unsafe {
        let mut network_handle: HcnNetworkHandle = std::ptr::null_mut();
        let error_record = CoTaskMemWString::new();

        match HcnCreateNetwork(
            id,
            WideCString::from_str(settings).unwrap().as_ptr(),
            &mut network_handle,
            error_record.ptr,
        ) {
            0 => Ok(network_handle),
            hresult => Err(ErrorResult::new(error_record.to_string(), hresult)),
        }
    }
}

/// Lookup an existing network.
pub fn open_network(id: &Guid) -> HcnResult<HcnNetworkHandle> {
    unsafe {
        let mut network_handle: HcnNetworkHandle = std::ptr::null_mut();
        let error_record = CoTaskMemWString::new();

        match HcnOpenNetwork(id, &mut network_handle, error_record.ptr) {
            0 => Ok(network_handle),
            hresult => Err(ErrorResult::new(error_record.to_string(), hresult)),
        }
    }
}

/// Modify the settings of a Network.
pub fn modify_network(network: HcnNetworkHandle, settings: &str) -> HcnResult<()> {
    unsafe {
        let error_record = CoTaskMemWString::new();

        match HcnModifyNetwork(
            network,
            WideCString::from_str(settings).unwrap().as_ptr(),
            error_record.ptr,
        ) {
            0 => Ok(()),
            hresult => Err(ErrorResult::new(error_record.to_string(), hresult)),
        }
    }
}

/// Query network settings.
pub fn query_network_properties(network: HcnNetworkHandle, query: &str) -> HcnResult<String> {
    unsafe {
        let properties = CoTaskMemWString::new();
        let error_record = CoTaskMemWString::new();

        match HcnQueryNetworkProperties(
            network,
            WideCString::from_str(query).unwrap().as_ptr(),
            properties.ptr,
            error_record.ptr,
        ) {
            0 => Ok(properties.to_string()),
            hresult => Err(ErrorResult::new(error_record.to_string(), hresult)),
        }
    }
}

/// Delete a network.
pub fn delete_network(id: &Guid) -> HcnResult<()> {
    unsafe {
        let error_record = CoTaskMemWString::new();

        match HcnDeleteNetwork(id, error_record.ptr) {
            0 => Ok(()),
            hresult => Err(ErrorResult::new(error_record.to_string(), hresult)),
        }
    }
}

/// Close handle to a Network.
pub fn close_network(network: HcnNetworkHandle) -> HcnResult<()> {
    unsafe {
        match HcnCloseNetwork(network) {
            0 => Ok(()),
            hresult => Err(ErrorResult::new(String::from(""), hresult)),
        }
    }
}

/// Return a list of existing Namespaces.
pub fn enumerate_namespaces(query: &str) -> HcnResult<String> {
    unsafe {
        let namespaces = CoTaskMemWString::new();
        let error_record = CoTaskMemWString::new();

        match HcnEnumerateNamespaces(
            WideCString::from_str(query).unwrap().as_ptr(),
            namespaces.ptr,
            error_record.ptr,
        ) {
            0 => Ok(namespaces.to_string()),
            hresult => Err(ErrorResult::new(error_record.to_string(), hresult)),
        }
    }
}

/// Create a Namespace.
pub fn create_namespace(id: &Guid, settings: &str) -> HcnResult<HcnNamespaceHandle> {
    unsafe {
        let mut namespace_handle: HcnNamespaceHandle = std::ptr::null_mut();
        let error_record = CoTaskMemWString::new();

        match HcnCreateNamespace(
            id,
            WideCString::from_str(settings).unwrap().as_ptr(),
            &mut namespace_handle,
            error_record.ptr,
        ) {
            0 => Ok(namespace_handle),
            hresult => Err(ErrorResult::new(error_record.to_string(), hresult)),
        }
    }
}

/// Lookup an existing Namespace.
pub fn open_namespace(id: &Guid) -> HcnResult<HcnNamespaceHandle> {
    unsafe {
        let mut namespace_handle: HcnNamespaceHandle = std::ptr::null_mut();
        let error_record = CoTaskMemWString::new();

        match HcnOpenNamespace(id, &mut namespace_handle, error_record.ptr) {
            0 => Ok(namespace_handle),
            hresult => Err(ErrorResult::new(error_record.to_string(), hresult)),
        }
    }
}

/// Modify the settings of a Namespace.
pub fn modify_namespace(namespace: HcnNamespaceHandle, settings: &str) -> HcnResult<()> {
    unsafe {
        let error_record = CoTaskMemWString::new();

        match HcnModifyNamespace(
            namespace,
            WideCString::from_str(settings).unwrap().as_ptr(),
            error_record.ptr,
        ) {
            0 => Ok(()),
            hresult => Err(ErrorResult::new(error_record.to_string(), hresult)),
        }
    }
}

/// Query Namespace settings.
pub fn query_namespace_properties(namespace: HcnNamespaceHandle, query: &str) -> HcnResult<String> {
    unsafe {
        let properties = CoTaskMemWString::new();
        let error_record = CoTaskMemWString::new();

        match HcnQueryNamespaceProperties(
            namespace,
            WideCString::from_str(query).unwrap().as_ptr(),
            properties.ptr,
            error_record.ptr,
        ) {
            0 => Ok(properties.to_string()),
            hresult => Err(ErrorResult::new(error_record.to_string(), hresult)),
        }
    }
}

/// Delete a Namespace.
pub fn delete_namespace(id: &Guid) -> HcnResult<()> {
    unsafe {
        let error_record = CoTaskMemWString::new();

        match HcnDeleteNamespace(id, error_record.ptr) {
            0 => Ok(()),
            hresult => Err(ErrorResult::new(error_record.to_string(), hresult)),
        }
    }
}

/// Close a handle to a Namespace.
pub fn close_namespace(namespace: HcnNamespaceHandle) -> HcnResult<()> {
    unsafe {
        match HcnCloseNamespace(namespace) {
            0 => Ok(()),
            hresult => Err(ErrorResult::new(String::from(""), hresult)),
        }
    }
}

/// Return a list of existing Endpoints.
pub fn enumerate_endpoints(query: &str) -> HcnResult<String> {
    unsafe {
        let endpoints = CoTaskMemWString::new();
        let error_record = CoTaskMemWString::new();

        match HcnEnumerateEndpoints(
            WideCString::from_str(query).unwrap().as_ptr(),
            endpoints.ptr,
            error_record.ptr,
        ) {
            0 => Ok(endpoints.to_string()),
            hresult => Err(ErrorResult::new(error_record.to_string(), hresult)),
        }
    }
}

/// Create an Endpoint.
pub fn create_endpoint(
    network: HcnNetworkHandle,
    id: &Guid,
    settings: &str,
) -> HcnResult<HcnEndpointHandle> {
    unsafe {
        let mut endpoint_handle: HcnEndpointHandle = std::ptr::null_mut();
        let error_record = CoTaskMemWString::new();

        match HcnCreateEndpoint(
            network,
            id,
            WideCString::from_str(settings).unwrap().as_ptr(),
            &mut endpoint_handle,
            error_record.ptr,
        ) {
            0 => Ok(endpoint_handle),
            hresult => Err(ErrorResult::new(error_record.to_string(), hresult)),
        }
    }
}

/// Lookup an existing Endpoint.
pub fn open_endpoint(id: &Guid) -> HcnResult<HcnEndpointHandle> {
    unsafe {
        let mut endpoint_handle: HcnEndpointHandle = std::ptr::null_mut();
        let error_record = CoTaskMemWString::new();

        match HcnOpenEndpoint(id, &mut endpoint_handle, error_record.ptr) {
            0 => Ok(endpoint_handle),
            hresult => Err(ErrorResult::new(error_record.to_string(), hresult)),
        }
    }
}

/// Modify the settings of an Endpoint.
pub fn modify_endpoint(endpoint: HcnEndpointHandle, settings: &str) -> HcnResult<()> {
    unsafe {
        let error_record = CoTaskMemWString::new();

        match HcnModifyEndpoint(
            endpoint,
            WideCString::from_str(settings).unwrap().as_ptr(),
            error_record.ptr,
        ) {
            0 => Ok(()),
            hresult => Err(ErrorResult::new(error_record.to_string(), hresult)),
        }
    }
}

/// Query Endpoint properties.
pub fn query_endpoint_properties(endpoint: HcnEndpointHandle, query: &str) -> HcnResult<String> {
    unsafe {
        let properties = CoTaskMemWString::new();
        let error_record = CoTaskMemWString::new();

        match HcnQueryEndpointProperties(
            endpoint,
            WideCString::from_str(query).unwrap().as_ptr(),
            properties.ptr,
            error_record.ptr,
        ) {
            0 => Ok(properties.to_string()),
            hresult => Err(ErrorResult::new(error_record.to_string(), hresult)),
        }
    }
}

/// Delete an Endpoint.
pub fn delete_endpoint(id: &Guid) -> HcnResult<()> {
    unsafe {
        let error_record = CoTaskMemWString::new();

        match HcnDeleteEndpoint(id, error_record.ptr) {
            0 => Ok(()),
            hresult => Err(ErrorResult::new(error_record.to_string(), hresult)),
        }
    }
}

/// Close a handle to an Endpoint.
pub fn close_endpoint(endpoint: HcnEndpointHandle) -> HcnResult<()> {
    unsafe {
        match HcnCloseEndpoint(endpoint) {
            0 => Ok(()),
            hresult => Err(ErrorResult::new(String::from(""), hresult)),
        }
    }
}

/// Return a list of existing LoadBalancers.
pub fn enumerate_load_balancers(query: &str) -> HcnResult<String> {
    unsafe {
        let load_balancers = CoTaskMemWString::new();
        let error_record = CoTaskMemWString::new();

        match HcnEnumerateLoadBalancers(
            WideCString::from_str(query).unwrap().as_ptr(),
            load_balancers.ptr,
            error_record.ptr,
        ) {
            0 => Ok(load_balancers.to_string()),
            hresult => Err(ErrorResult::new(error_record.to_string(), hresult)),
        }
    }
}

/// Create a LoadBalancer.
pub fn create_load_balancer(id: &Guid, settings: &str) -> HcnResult<HcnLoadBalancerHandle> {
    unsafe {
        let mut load_balancer_handle: HcnLoadBalancerHandle = std::ptr::null_mut();
        let error_record = CoTaskMemWString::new();

        match HcnCreateLoadBalancer(
            id,
            WideCString::from_str(settings).unwrap().as_ptr(),
            &mut load_balancer_handle,
            error_record.ptr,
        ) {
            0 => Ok(load_balancer_handle),
            hresult => Err(ErrorResult::new(error_record.to_string(), hresult)),
        }
    }
}

/// Lookup an existing LoadBalancer.
pub fn open_load_balancer(id: &Guid) -> HcnResult<HcnLoadBalancerHandle> {
    unsafe {
        let mut load_balancer_handle: HcnLoadBalancerHandle = std::ptr::null_mut();
        let error_record = CoTaskMemWString::new();

        match HcnOpenLoadBalancer(id, &mut load_balancer_handle, error_record.ptr) {
            0 => Ok(load_balancer_handle),
            hresult => Err(ErrorResult::new(error_record.to_string(), hresult)),
        }
    }
}

/// Modify the settings of a LoadBalancer.
pub fn modify_load_balancer(load_balancer: HcnLoadBalancerHandle, settings: &str) -> HcnResult<()> {
    unsafe {
        let error_record = CoTaskMemWString::new();

        match HcnModifyLoadBalancer(
            load_balancer,
            WideCString::from_str(settings).unwrap().as_ptr(),
            error_record.ptr,
        ) {
            0 => Ok(()),
            hresult => Err(ErrorResult::new(error_record.to_string(), hresult)),
        }
    }
}

/// Query LoadBalancer settings.
pub fn query_load_balancer_properties(
    load_balancer: HcnLoadBalancerHandle,
    query: &str,
) -> HcnResult<String> {
    unsafe {
        let properties = CoTaskMemWString::new();
        let error_record = CoTaskMemWString::new();

        match HcnQueryLoadBalancerProperties(
            load_balancer,
            WideCString::from_str(query).unwrap().as_ptr(),
            properties.ptr,
            error_record.ptr,
        ) {
            0 => Ok(properties.to_string()),
            hresult => Err(ErrorResult::new(error_record.to_string(), hresult)),
        }
    }
}

/// Delete a LoadBalancer.
pub fn delete_load_balancer(id: &Guid) -> HcnResult<()> {
    unsafe {
        let error_record = CoTaskMemWString::new();

        match HcnDeleteLoadBalancer(id, error_record.ptr) {
            0 => Ok(()),
            hresult => Err(ErrorResult::new(error_record.to_string(), hresult)),
        }
    }
}

/// Close a handle to a LoadBalancer.
pub fn close_load_balancer(load_balancer: HcnLoadBalancerHandle) -> HcnResult<()> {
    unsafe {
        match HcnCloseLoadBalancer(load_balancer) {
            0 => Ok(()),
            hresult => Err(ErrorResult::new(String::from(""), hresult)),
        }
    }
}

/// Registers a callback function to receive notifications of service-wide events.
pub fn register_service_callback(
    callback: HcnNotificationCallback,
    context: *const Void,
    callback_handle: *mut HcnCallback,
) -> HcnResult<()> {
    unsafe {
        match HcnRegisterServiceCallback(callback, context, callback_handle) {
            0 => Ok(()),
            hresult => Err(ErrorResult::new(String::from(""), hresult)),
        }
    }
}

/// Unregisters from service-wide notifications.
pub fn unregister_service_callback(callback_handle: *const HcnCallback) -> HcnResult<()> {
    unsafe {
        match HcnUnregisterServiceCallback(callback_handle) {
            0 => Ok(()),
            hresult => Err(ErrorResult::new(String::from(""), hresult)),
        }
    }
}
