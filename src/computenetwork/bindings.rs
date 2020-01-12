// Copyright (c) 2019-2020 Rafael Alcaraz Mercado. All rights reserved.
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.
// All files in the project carrying such notice may not be copied, modified, or distributed
// except according to those terms.
// THE SOURCE CODE IS AVAILABLE UNDER THE ABOVE CHOSEN LICENSE "AS IS", WITH NO WARRANTIES.

//! This module contains types definitions and Rust FFI bindings for the APIs used to interact with the HCN.

use super::defs::*;
use winutils_rs::windefs::*;

#[link(name = "computenetwork")]
extern "system" {
    /// Return a list of existing Networks
    pub fn HcnEnumerateNetworks(
        query: PCWStr,
        networks: *mut PWStr,
        errorRecord: *mut PWStr,
    ) -> HResult;

    /// Create a Network
    pub fn HcnCreateNetwork(
        id: *const Guid,
        settings: PCWStr,
        network: PHcnNetworkHandle,
        errorRecord: *mut PWStr,
    ) -> HResult;

    /// Lookup an existing network
    pub fn HcnOpenNetwork(
        id: *const Guid,
        network: PHcnNetworkHandle,
        errorRecord: *mut PWStr,
    ) -> HResult;

    /// Modify the settings of a Network
    pub fn HcnModifyNetwork(
        network: HcnNetworkHandle,
        settings: PCWStr,
        errorRecord: *mut PWStr,
    ) -> HResult;

    /// Query Network settings
    pub fn HcnQueryNetworkProperties(
        network: HcnNetworkHandle,
        query: PCWStr,
        properties: *mut PWStr,
        errorRecord: *mut PWStr,
    ) -> HResult;

    /// Delete a Network
    pub fn HcnDeleteNetwork(id: *const Guid, errorRecord: *mut PWStr) -> HResult;

    /// Close a handle to a Network
    pub fn HcnCloseNetwork(network: HcnNetworkHandle) -> HResult;

    /// Return a list of existing Namespaces
    pub fn HcnEnumerateNamespaces(
        query: PCWStr,
        namespaces: *mut PWStr,
        errorRecord: *mut PWStr,
    ) -> HResult;

    /// Create a Namespace
    pub fn HcnCreateNamespace(
        id: *const Guid,
        settings: PCWStr,
        namespace: PHcnNamespaceHandle,
        errorRecord: *mut PWStr,
    ) -> HResult;

    /// Lookup an existing Namespace
    pub fn HcnOpenNamespace(
        id: *const Guid,
        namespace: PHcnNamespaceHandle,
        errorRecord: *mut PWStr,
    ) -> HResult;

    /// Modify the settings of a Namespace
    pub fn HcnModifyNamespace(
        namespace: HcnNamespaceHandle,
        settings: PCWStr,
        errorRecord: *mut PWStr,
    ) -> HResult;

    /// Query Namespace settings
    pub fn HcnQueryNamespaceProperties(
        namespace: HcnNamespaceHandle,
        query: PCWStr,
        properties: *mut PWStr,
        errorRecord: *mut PWStr,
    ) -> HResult;

    /// Delete a Namespace
    pub fn HcnDeleteNamespace(id: *const Guid, errorRecord: *mut PWStr) -> HResult;

    /// Close a handle to a Namespace
    pub fn HcnCloseNamespace(namespace: HcnNamespaceHandle) -> HResult;

    /// Return a list of existing Endpoints
    pub fn HcnEnumerateEndpoints(
        query: PCWStr,
        endpoints: *mut PWStr,
        errorRecord: *mut PWStr,
    ) -> HResult;

    /// Create an Endpoint
    pub fn HcnCreateEndpoint(
        network: HcnNetworkHandle,
        id: *const Guid,
        settings: PCWStr,
        endpoint: PHcnEndpointHandle,
        errorRecord: *mut PWStr,
    ) -> HResult;

    /// Lookup an existing Endpoint
    pub fn HcnOpenEndpoint(
        id: *const Guid,
        endpoint: PHcnEndpointHandle,
        errorRecord: *mut PWStr,
    ) -> HResult;

    /// Modify the settings of an Endpoint
    pub fn HcnModifyEndpoint(
        endpoint: HcnEndpointHandle,
        settings: PCWStr,
        errorRecord: *mut PWStr,
    ) -> HResult;

    /// Query Endpoint properties
    pub fn HcnQueryEndpointProperties(
        endpoint: HcnEndpointHandle,
        query: PCWStr,
        properties: *mut PWStr,
        errorRecord: *mut PWStr,
    ) -> HResult;

    /// Delete an Endpoint
    pub fn HcnDeleteEndpoint(id: *const Guid, errorRecord: *mut PWStr) -> HResult;

    /// Close a handle to an Endpoint
    pub fn HcnCloseEndpoint(endpoint: HcnEndpointHandle) -> HResult;

    /// Return a list of existing LoadBalancers
    pub fn HcnEnumerateLoadBalancers(
        query: PCWStr,
        loadBalancer: *mut PWStr,
        errorRecord: *mut PWStr,
    ) -> HResult;

    /// Create a LoadBalancer
    pub fn HcnCreateLoadBalancer(
        id: *const Guid,
        settings: PCWStr,
        loadBalancer: PHcnLoadBalancerHandle,
        errorRecord: *mut PWStr,
    ) -> HResult;

    /// Lookup an existing LoadBalancer
    pub fn HcnOpenLoadBalancer(
        id: *const Guid,
        loadBalancer: PHcnLoadBalancerHandle,
        errorRecord: *mut PWStr,
    ) -> HResult;

    /// Modify the settings of a PolicyList
    pub fn HcnModifyLoadBalancer(
        loadBalancer: HcnLoadBalancerHandle,
        settings: PCWStr,
        errorRecord: *mut PWStr,
    ) -> HResult;

    /// Query PolicyList settings
    pub fn HcnQueryLoadBalancerProperties(
        loadBalancer: HcnLoadBalancerHandle,
        query: PCWStr,
        properties: *mut PWStr,
        errorRecord: *mut PWStr,
    ) -> HResult;

    /// Delete a LoadBalancer
    pub fn HcnDeleteLoadBalancer(id: *const Guid, errorRecord: *mut PWStr) -> HResult;

    /// Close a handle to a LoadBalancer
    pub fn HcnCloseLoadBalancer(loadBalancer: HcnLoadBalancerHandle) -> HResult;

    /// Registers a callback function to receive notifications of service-wide events
    pub fn HcnRegisterServiceCallback(
        callback: HcnNotificationCallback,
        context: *const Void,
        callbackHandle: *mut HcnCallback,
    ) -> HResult;

    /// Unregisters from service-wide notifications
    pub fn HcnUnregisterServiceCallback(callbackHandle: *const HcnCallback) -> HResult;
}
