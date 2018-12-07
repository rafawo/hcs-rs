//! This file contains types definitions and APIs used to interact with the HCN.

use crate::windefs::*;

/// Notifications indicated to callbacks
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum HcsNotifications {
    Invalid = 0x00000000,

    // Notifications for HcnService handles
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
pub type HcnCallback = *const Void;

/// Function type for HNS notification callbacks
pub type HcnNotificationCallback = extern "C" fn(
    notificationType: DWord,
    context: *mut Void,
    notificationStatus: HResult,
    notificationData: PCWStr,
);

/// Context handle referencing a Network in HNS
pub type HcnNetwork = *const Void;

/// Context handle referencing a pointer to a Network in HNS
pub type PHcnNetwork = *mut HcnNetwork;

/// Context handle referencing a Namespace in HNS
pub type HcnNamespace = *const Void;

/// Context handle referencing a pointer to a Namespace in HNS
pub type PHcnNamespace = *mut HcnNamespace;

/// Context handle referencing an Endpoint in HNS
pub type HcnEndpoint = *const Void;

/// Context handle referencing a pointer to an Endpoint in HNS
pub type PHcnEndpoint = *mut HcnEndpoint;

/// Context handle referencing a LoadBalancer in HNS
pub type HcnLoadBalancer = *const Void;

/// Context handle referencing a pointer to a LoadBalancer in HNS
pub type PHcnLoadBalancer = *mut HcnLoadBalancer;

/// Context handle referencing the HNS service
pub type HcnService = *const Void;

/// Context handle referencing the HNS service
pub type PHcnService = *mut HcnService;

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
        network: PHcnNetwork,
        errorRecord: *mut PWStr,
    ) -> HResult;

    /// Lookup an existing network
    pub fn HcnOpenNetwork(
        id: *const Guid,
        network: PHcnNetwork,
        errorRecord: *mut PWStr,
    ) -> HResult;

    /// Modify the settings of a Network
    pub fn HcnModifyNetwork(
        network: HcnNetwork,
        settings: PCWStr,
        errorRecord: *mut PWStr,
    ) -> HResult;

    /// Query Network settings
    pub fn HcnQueryNetworkProperties(
        network: HcnNetwork,
        query: PCWStr,
        properties: *mut PWStr,
        errorRecord: *mut PWStr,
    ) -> HResult;

    /// Delete a Network
    pub fn HcnDeleteNetwork(id: *const Guid, errorRecord: *mut PWStr) -> HResult;

    /// Close a handle to a Network
    pub fn HcnCloseNetwork(network: HcnNetwork) -> HResult;

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
        namespace: PHcnNamespace,
        errorRecord: *mut PWStr,
    ) -> HResult;

    /// Lookup an existing Namespace
    pub fn HcnOpenNamespace(
        id: *const Guid,
        namespace: PHcnNamespace,
        errorRecord: *mut PWStr,
    ) -> HResult;

    /// Modify the settings of a Namespace
    pub fn HcnModifyNamespace(
        namespace: HcnNamespace,
        settings: PCWStr,
        errorRecord: *mut PWStr,
    ) -> HResult;

    /// Query Namespace settings
    pub fn HcnQueryNamespaceProperties(
        namespace: HcnNamespace,
        query: PCWStr,
        properties: *mut PWStr,
        errorRecord: *mut PWStr,
    ) -> HResult;

    /// Delete a Namespace
    pub fn HcnDeleteNamespace(id: *const Guid, errorRecord: *mut PWStr) -> HResult;

    /// Close a handle to a Namespace
    pub fn HcnCloseNamespace(namespace: HcnNamespace) -> HResult;

    /// Return a list of existing Endpoints
    pub fn HcnEnumerateEndpoints(
        query: PCWStr,
        endpoints: *mut PWStr,
        errorRecord: *mut PWStr,
    ) -> HResult;

    /// Create an Endpoint
    pub fn HcnCreateEndpoint(
        network: HcnNetwork,
        id: *const Guid,
        settings: PCWStr,
        endpoint: PHcnEndpoint,
        errorRecord: *mut PWStr,
    ) -> HResult;

    /// Lookup an existing Endpoint
    pub fn HcnOpenEndpoint(
        id: *const Guid,
        endpoint: PHcnEndpoint,
        errorRecord: *mut PWStr,
    ) -> HResult;

    /// Modify the settings of an Endpoint
    pub fn HcnModifyEndpoint(
        endpoint: HcnEndpoint,
        settings: PCWStr,
        errorRecord: *mut PWStr,
    ) -> HResult;

    /// Query Endpoint properties
    pub fn HcnQueryEndpointProperties(
        endpoint: HcnEndpoint,
        query: PCWStr,
        properties: *mut PWStr,
        errorRecord: *mut PWStr,
    ) -> HResult;

    /// Delete an Endpoint
    pub fn HcnDeleteEndpoint(id: *const Guid, errorRecord: *mut PWStr) -> HResult;

    /// Close a handle to an Endpoint
    pub fn HcnCloseEndpoint(endpoint: HcnEndpoint) -> HResult;

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
        loadBalancer: PHcnLoadBalancer,
        errorRecord: *mut PWStr,
    ) -> HResult;

    /// Lookup an existing LoadBalancer
    pub fn HcnOpenLoadBalancer(
        id: *const Guid,
        loadBalancer: PHcnLoadBalancer,
        errorRecord: *mut PWStr,
    ) -> HResult;

    /// Modify the settings of a PolicyList
    pub fn HcnModifyLoadBalancer(
        loadBalancer: HcnLoadBalancer,
        settings: PCWStr,
        errorRecord: *mut PWStr,
    ) -> HResult;

    /// Query PolicyList settings
    pub fn HcnQueryLoadBalancerProperties(
        loadBalancer: HcnLoadBalancer,
        query: PCWStr,
        properties: *mut PWStr,
        errorRecord: *mut PWStr,
    ) -> HResult;

    /// Delete a LoadBalancer
    pub fn HcnDeleteLoadBalancer(id: *const Guid, errorRecord: *mut PWStr) -> HResult;

    /// Close a handle to a LoadBalancer
    pub fn HcnCloseLoadBalancer(loadBalancer: HcnLoadBalancer) -> HResult;

    /// Registers a callback function to receive notifications of service-wide events
    pub fn HcnRegisterServiceCallback(
        callback: HcnNotificationCallback,
        context: *const Void,
        callbackHandle: *mut HcnCallback,
    ) -> HResult;

    /// Unregisters from service-wide notifications
    pub fn HcnUnregisterServiceCallback(callbackHandle: *const HcnCallback) -> HResult;

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
