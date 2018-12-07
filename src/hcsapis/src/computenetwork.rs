//! This file contains types definitions and APIs used to interact with the HCN.

use crate::windefs::*;

/// Notifications indicated to callbacks
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum HcsNotifications {
    Invalid = 0x00000000,

    // Notifications for HCN_SERVICE handles
    NetworkPreCreate = 0x00000001,
    NetworkCreate = 0x00000002,
    NetworkPreDelete = 0x00000003,
    NetworkDelete = 0x00000004,

    // Namespace Notifications
    NamespaceCreate = 0x00000005,
    NamespaceDelete = 0x00000006,

    // Common notifications
    ServiceDisconnect = 0x01000000,

    // The upper 4 bits are reserved for flags
    FlagsReserved = 0xF0000000,
}

/// Handle to a callback registered on an hns object
#[allow(non_camel_case_types)]
pub type HCN_CALLBACK = *const Void;

/// Function type for HNS notification callbacks
#[allow(non_camel_case_types)]
pub type HCN_NOTIFICATION_CALLBACK = extern "C" fn(
    notificationType: DWord,
    context: *mut Void,
    notificationStatus: HResult,
    notificationData: PCWStr,
);

/// Context handle referencing a Network in HNS
#[allow(non_camel_case_types)]
pub type HCN_NETWORK = *const Void;

/// Context handle referencing a pointer to a Network in HNS
#[allow(non_camel_case_types)]
pub type PHCN_NETWORK = *mut HCN_NETWORK;

/// Context handle referencing a Namespace in HNS
#[allow(non_camel_case_types)]
pub type HCN_NAMESPACE = *const Void;

/// Context handle referencing a pointer to a Namespace in HNS
#[allow(non_camel_case_types)]
pub type PHCN_NAMESPACE = *mut HCN_NAMESPACE;

/// Context handle referencing an Endpoint in HNS
#[allow(non_camel_case_types)]
pub type HCN_ENDPOINT = *const Void;

/// Context handle referencing a pointer to an Endpoint in HNS
#[allow(non_camel_case_types)]
pub type PHCN_ENDPOINT = *mut HCN_ENDPOINT;

/// Context handle referencing a LoadBalancer in HNS
#[allow(non_camel_case_types)]
pub type HCN_LOADBALANCER = *const Void;

/// Context handle referencing a pointer to a LoadBalancer in HNS
#[allow(non_camel_case_types)]
pub type PHCN_LOADBALANCER = *mut HCN_LOADBALANCER;

/// Context handle referencing the HNS service
#[allow(non_camel_case_types)]
pub type HCN_SERVICE = *const Void;

/// Context handle referencing the HNS service
#[allow(non_camel_case_types)]
pub type PHCN_SERVICE = *mut HCN_SERVICE;

#[link(name = "computenetwork")]
extern "C" {

    /// Return a list of existing Networks
    pub fn HcnEnumerateNetworks(
        query: PCWStr,
        networks: *mut PWStr,
        rrrorRecord: *mut PWStr,
    ) -> HResult;

    /// Create a Network
    pub fn HcnCreateNetwork(
        id: *const Guid,
        settings: PCWStr,
        network: PHCN_NETWORK,
        errorRecord: *mut PWStr,
    ) -> HResult;

    /// Lookup an existing network
    pub fn HcnOpenNetwork(
        id: *const Guid,
        network: PHCN_NETWORK,
        errorRecord: *mut PWStr,
    ) -> HResult;

    /// Modify the settings of a Network
    pub fn HcnModifyNetwork(
        network: HCN_NETWORK,
        settings: PCWStr,
        errorRecord: *mut PWStr,
    ) -> HResult;

    /// Query Network settings
    pub fn HcnQueryNetworkProperties(
        network: HCN_NETWORK,
        query: PCWStr,
        properties: *mut PWStr,
        errorRecord: *mut PWStr,
    ) -> HResult;

    /// Delete a Network
    pub fn HcnDeleteNetwork(id: *const Guid, errorRecord: *mut PWStr) -> HResult;

    /// Close a handle to a Network
    pub fn HcnCloseNetwork(network: HCN_NETWORK) -> HResult;

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
        namespace: PHCN_NAMESPACE,
        errorRecord: *mut PWStr,
    ) -> HResult;

    /// Lookup an existing Namespace
    pub fn HcnOpenNamespace(
        id: *const Guid,
        namespace: PHCN_NAMESPACE,
        errorRecord: *mut PWStr,
    ) -> HResult;

    /// Modify the settings of a Namespace
    pub fn HcnModifyNamespace(
        namespace: HCN_NAMESPACE,
        settings: PCWStr,
        errorRecord: *mut PWStr,
    ) -> HResult;

    /// Query Namespace settings
    pub fn HcnQueryNamespaceProperties(
        namespace: HCN_NAMESPACE,
        query: PCWStr,
        properties: *mut PWStr,
        errorRecord: *mut PWStr,
    ) -> HResult;

    /// Delete a Namespace
    pub fn HcnDeleteNamespace(id: *const Guid, errorRecord: *mut PWStr) -> HResult;

    /// Close a handle to a Namespace
    pub fn HcnCloseNamespace(namespace: HCN_NAMESPACE) -> HResult;

    /// Return a list of existing Endpoints
    pub fn HcnEnumerateEndpoints(
        query: PCWStr,
        endpoints: *mut PWStr,
        errorRecord: *mut PWStr,
    ) -> HResult;

    /// Create an Endpoint
    pub fn HcnCreateEndpoint(
        network: HCN_NETWORK,
        id: *const Guid,
        settings: PCWStr,
        endpoint: PHCN_ENDPOINT,
        errorRecord: *mut PWStr,
    ) -> HResult;

    /// Lookup an existing Endpoint
    pub fn HcnOpenEndpoint(
        id: *const Guid,
        endpoint: PHCN_ENDPOINT,
        errorRecord: *mut PWStr,
    ) -> HResult;

    /// Modify the settings of an Endpoint
    pub fn HcnModifyEndpoint(
        endpoint: HCN_ENDPOINT,
        settings: PCWStr,
        errorRecord: *mut PWStr,
    ) -> HResult;

    /// Query Endpoint properties
    pub fn HcnQueryEndpointProperties(
        endpoint: HCN_ENDPOINT,
        query: PCWStr,
        properties: *mut PWStr,
        errorRecord: *mut PWStr,
    ) -> HResult;

    /// Delete an Endpoint
    pub fn HcnDeleteEndpoint(id: *const Guid, errorRecord: *mut PWStr) -> HResult;

    /// Close a handle to an Endpoint
    pub fn HcnCloseEndpoint(endpoint: HCN_ENDPOINT) -> HResult;

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
        loadBalancer: PHCN_LOADBALANCER,
        errorRecord: *mut PWStr,
    ) -> HResult;

    /// Lookup an existing LoadBalancer
    pub fn HcnOpenLoadBalancer(
        id: *const Guid,
        loadBalancer: PHCN_LOADBALANCER,
        errorRecord: *mut PWStr,
    ) -> HResult;

    /// Modify the settings of a PolicyList
    pub fn HcnModifyLoadBalancer(
        loadBalancer: HCN_LOADBALANCER,
        settings: PCWStr,
        errorRecord: *mut PWStr,
    ) -> HResult;

    /// Query PolicyList settings
    pub fn HcnQueryLoadBalancerProperties(
        loadBalancer: HCN_LOADBALANCER,
        query: PCWStr,
        properties: *mut PWStr,
        errorRecord: *mut PWStr,
    ) -> HResult;

    /// Delete a LoadBalancer
    pub fn HcnDeleteLoadBalancer(id: *const Guid, errorRecord: *mut PWStr) -> HResult;

    /// Close a handle to a LoadBalancer
    pub fn HcnCloseLoadBalancer(loadBalancer: HCN_LOADBALANCER) -> HResult;

    /// Registers a callback function to receive notifications of service-wide events
    pub fn HcnRegisterServiceCallback(
        callback: HCN_NOTIFICATION_CALLBACK,
        context: *const Void,
        callbackHandle: *mut HCN_CALLBACK,
    ) -> HResult;

    /// Unregisters from service-wide notifications
    pub fn HcnUnregisterServiceCallback(callbackHandle: *const HCN_CALLBACK) -> HResult;

    pub fn IsHcnEnumerateNetworksPresent() -> Boolean;

    pub fn IsHcnCreateNetworkPresent() -> Boolean;

    pub fn IsHcnOpenNetworkPresent() -> Boolean;

    pub fn IsHcnModifyNetworkPresent() -> Boolean;

    pub fn IsHcnQueryNetworkPropertiesPresent() -> Boolean;

    pub fn IsHcnDeleteNetworkPresent() -> Boolean;

    pub fn IsHcnCloseNetworkPresent() -> Boolean;

    pub fn IsHcnEnumerateNamespacesPresent() -> Boolean;

    pub fn IsHcnCreateNamespacePresent() -> Boolean;

    pub fn IsHcnOpenNamespacePresent() -> Boolean;

    pub fn IsHcnModifyNamespacePresent() -> Boolean;

    pub fn IsHcnQueryNamespacePropertiesPresent() -> Boolean;

    pub fn IsHcnDeleteNamespacePresent() -> Boolean;

    pub fn IsHcnCloseNamespacePresent() -> Boolean;

    pub fn IsHcnEnumerateEndpointsPresent() -> Boolean;

    pub fn IsHcnCreateEndpointPresent() -> Boolean;

    pub fn IsHcnOpenEndpointPresent() -> Boolean;

    pub fn IsHcnModifyEndpointPresent() -> Boolean;

    pub fn IsHcnQueryEndpointPropertiesPresent() -> Boolean;

    pub fn IsHcnDeleteEndpointPresent() -> Boolean;

    pub fn IsHcnCloseEndpointPresent() -> Boolean;

    pub fn IsHcnEnumerateLoadBalancersPresent() -> Boolean;

    pub fn IsHcnCreateLoadBalancerPresent() -> Boolean;

    pub fn IsHcnOpenLoadBalancerPresent() -> Boolean;

    pub fn IsHcnModifyLoadBalancerPresent() -> Boolean;

    pub fn IsHcnQueryLoadBalancerPropertiesPresent() -> Boolean;

    pub fn IsHcnDeleteLoadBalancerPresent() -> Boolean;

    pub fn IsHcnCloseLoadBalancerPresent() -> Boolean;

    pub fn IsHcnRegisterServiceCallbackPresent() -> Boolean;

    pub fn IsHcnUnregisterServiceCallbackPresent() -> Boolean;

}
