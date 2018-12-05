//! This file contains types definitions and APIs used to interact with the HCN.

use winapi::shared::guiddef::REFGUID;
use winapi::shared::minwindef::DWORD;
use winapi::shared::ntdef::{BOOLEAN, HRESULT, PCWSTR, PWSTR, VOID};

/// Notifications indicated to callbacks
#[repr(C)]
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
pub type HCN_CALLBACK = *const VOID;

/// Function type for HNS notification callbacks
#[allow(non_camel_case_types)]
pub type HCN_NOTIFICATION_CALLBACK = extern "C" fn(
    notificationType: DWORD,
    context: *mut VOID,
    notificationStatus: HRESULT,
    notificationData: PCWSTR,
);

/// Context handle referencing a Network in HNS
#[allow(non_camel_case_types)]
pub type HCN_NETWORK = *const VOID;

/// Context handle referencing a pointer to a Network in HNS
#[allow(non_camel_case_types)]
pub type PHCN_NETWORK = *mut HCN_NETWORK;

/// Context handle referencing a Namespace in HNS
#[allow(non_camel_case_types)]
pub type HCN_NAMESPACE = *const VOID;

/// Context handle referencing a pointer to a Namespace in HNS
#[allow(non_camel_case_types)]
pub type PHCN_NAMESPACE = *mut HCN_NAMESPACE;

/// Context handle referencing an Endpoint in HNS
#[allow(non_camel_case_types)]
pub type HCN_ENDPOINT = *const VOID;

/// Context handle referencing a pointer to an Endpoint in HNS
#[allow(non_camel_case_types)]
pub type PHCN_ENDPOINT = *mut HCN_ENDPOINT;

/// Context handle referencing a LoadBalancer in HNS
#[allow(non_camel_case_types)]
pub type HCN_LOADBALANCER = *const VOID;

/// Context handle referencing a pointer to a LoadBalancer in HNS
#[allow(non_camel_case_types)]
pub type PHCN_LOADBALANCER = *mut HCN_LOADBALANCER;

/// Context handle referencing the HNS service
#[allow(non_camel_case_types)]
pub type HCN_SERVICE = *const VOID;

/// Context handle referencing the HNS service
#[allow(non_camel_case_types)]
pub type PHCN_SERVICE = *mut HCN_SERVICE;

extern "C" {

    /// Return a list of existing Networks
    pub fn HcnEnumerateNetworks(
        query: PCWSTR,
        networks: *mut PWSTR,
        rrrorRecord: *mut PWSTR,
    ) -> HRESULT;

    /// Create a Network
    pub fn HcnCreateNetwork(
        id: REFGUID,
        settings: PCWSTR,
        network: PHCN_NETWORK,
        errorRecord: *mut PWSTR,
    ) -> HRESULT;

    /// Lookup an existing network
    pub fn HcnOpenNetwork(id: REFGUID, network: PHCN_NETWORK, errorRecord: *mut PWSTR) -> HRESULT;

    /// Modify the settings of a Network
    pub fn HcnModifyNetwork(
        network: HCN_NETWORK,
        settings: PCWSTR,
        errorRecord: *mut PWSTR,
    ) -> HRESULT;

    /// Query Network settings
    pub fn HcnQueryNetworkProperties(
        network: HCN_NETWORK,
        query: PCWSTR,
        properties: *mut PWSTR,
        errorRecord: *mut PWSTR,
    ) -> HRESULT;

    /// Delete a Network
    pub fn HcnDeleteNetwork(id: REFGUID, errorRecord: *mut PWSTR) -> HRESULT;

    /// Close a handle to a Network
    pub fn HcnCloseNetwork(network: HCN_NETWORK) -> HRESULT;

    /// Return a list of existing Namespaces
    pub fn HcnEnumerateNamespaces(
        query: PCWSTR,
        namespaces: *mut PWSTR,
        errorRecord: *mut PWSTR,
    ) -> HRESULT;

    /// Create a Namespace
    pub fn HcnCreateNamespace(
        id: REFGUID,
        settings: PCWSTR,
        namespace: PHCN_NAMESPACE,
        errorRecord: *mut PWSTR,
    ) -> HRESULT;

    /// Lookup an existing Namespace
    pub fn HcnOpenNamespace(
        id: REFGUID,
        namespace: PHCN_NAMESPACE,
        errorRecord: *mut PWSTR,
    ) -> HRESULT;

    /// Modify the settings of a Namespace
    pub fn HcnModifyNamespace(
        namespace: HCN_NAMESPACE,
        settings: PCWSTR,
        errorRecord: *mut PWSTR,
    ) -> HRESULT;

    /// Query Namespace settings
    pub fn HcnQueryNamespaceProperties(
        namespace: HCN_NAMESPACE,
        query: PCWSTR,
        properties: *mut PWSTR,
        errorRecord: *mut PWSTR,
    ) -> HRESULT;

    /// Delete a Namespace
    pub fn HcnDeleteNamespace(id: REFGUID, errorRecord: *mut PWSTR) -> HRESULT;

    /// Close a handle to a Namespace
    pub fn HcnCloseNamespace(namespace: HCN_NAMESPACE) -> HRESULT;

    /// Return a list of existing Endpoints
    pub fn HcnEnumerateEndpoints(
        query: PCWSTR,
        endpoints: *mut PWSTR,
        errorRecord: *mut PWSTR,
    ) -> HRESULT;

    /// Create an Endpoint
    pub fn HcnCreateEndpoint(
        network: HCN_NETWORK,
        id: REFGUID,
        settings: PCWSTR,
        endpoint: PHCN_ENDPOINT,
        errorRecord: *mut PWSTR,
    ) -> HRESULT;

    /// Lookup an existing Endpoint
    pub fn HcnOpenEndpoint(
        id: REFGUID,
        endpoint: PHCN_ENDPOINT,
        errorRecord: *mut PWSTR,
    ) -> HRESULT;

    /// Modify the settings of an Endpoint
    pub fn HcnModifyEndpoint(
        endpoint: HCN_ENDPOINT,
        settings: PCWSTR,
        errorRecord: *mut PWSTR,
    ) -> HRESULT;

    /// Query Endpoint properties
    pub fn HcnQueryEndpointProperties(
        endpoint: HCN_ENDPOINT,
        query: PCWSTR,
        properties: *mut PWSTR,
        errorRecord: *mut PWSTR,
    ) -> HRESULT;

    /// Delete an Endpoint
    pub fn HcnDeleteEndpoint(id: REFGUID, errorRecord: *mut PWSTR) -> HRESULT;

    /// Close a handle to an Endpoint
    pub fn HcnCloseEndpoint(endpoint: HCN_ENDPOINT) -> HRESULT;

    /// Return a list of existing LoadBalancers
    pub fn HcnEnumerateLoadBalancers(
        query: PCWSTR,
        loadBalancer: *mut PWSTR,
        errorRecord: *mut PWSTR,
    ) -> HRESULT;

    /// Create a LoadBalancer
    pub fn HcnCreateLoadBalancer(
        id: REFGUID,
        settings: PCWSTR,
        loadBalancer: PHCN_LOADBALANCER,
        errorRecord: *mut PWSTR,
    ) -> HRESULT;

    /// Lookup an existing LoadBalancer
    pub fn HcnOpenLoadBalancer(
        id: REFGUID,
        loadBalancer: PHCN_LOADBALANCER,
        errorRecord: *mut PWSTR,
    ) -> HRESULT;

    /// Modify the settings of a PolcyList
    pub fn HcnModifyLoadBalancer(
        loadBalancer: HCN_LOADBALANCER,
        settings: PCWSTR,
        errorRecord: *mut PWSTR,
    ) -> HRESULT;

    /// Query PolcyList settings
    pub fn HcnQueryLoadBalancerProperties(
        loadBalancer: HCN_LOADBALANCER,
        query: PCWSTR,
        properties: *mut PWSTR,
        errorRecord: *mut PWSTR,
    ) -> HRESULT;

    /// Delete a LoadBalancer
    pub fn HcnDeleteLoadBalancer(id: REFGUID, errorRecord: *mut PWSTR) -> HRESULT;

    /// Close a handle to a LoadBalancer
    pub fn HcnCloseLoadBalancer(loadBalancer: HCN_LOADBALANCER) -> HRESULT;

    /// Registers a callback function to receive notifications of service-wide events
    pub fn HcnRegisterServiceCallback(
        callback: HCN_NOTIFICATION_CALLBACK,
        context: *const VOID,
        callbackHandle: *mut HCN_CALLBACK,
    ) -> HRESULT;

    /// Unregisters from service-wide notifications
    pub fn HcnUnregisterServiceCallback(callbackHandle: *const HCN_CALLBACK) -> HRESULT;

    pub fn IsHcnEnumerateNetworksPresent() -> BOOLEAN;

    pub fn IsHcnCreateNetworkPresent() -> BOOLEAN;

    pub fn IsHcnOpenNetworkPresent() -> BOOLEAN;

    pub fn IsHcnModifyNetworkPresent() -> BOOLEAN;

    pub fn IsHcnQueryNetworkPropertiesPresent() -> BOOLEAN;

    pub fn IsHcnDeleteNetworkPresent() -> BOOLEAN;

    pub fn IsHcnCloseNetworkPresent() -> BOOLEAN;

    pub fn IsHcnEnumerateNamespacesPresent() -> BOOLEAN;

    pub fn IsHcnCreateNamespacePresent() -> BOOLEAN;

    pub fn IsHcnOpenNamespacePresent() -> BOOLEAN;

    pub fn IsHcnModifyNamespacePresent() -> BOOLEAN;

    pub fn IsHcnQueryNamespacePropertiesPresent() -> BOOLEAN;

    pub fn IsHcnDeleteNamespacePresent() -> BOOLEAN;

    pub fn IsHcnCloseNamespacePresent() -> BOOLEAN;

    pub fn IsHcnEnumerateEndpointsPresent() -> BOOLEAN;

    pub fn IsHcnCreateEndpointPresent() -> BOOLEAN;

    pub fn IsHcnOpenEndpointPresent() -> BOOLEAN;

    pub fn IsHcnModifyEndpointPresent() -> BOOLEAN;

    pub fn IsHcnQueryEndpointPropertiesPresent() -> BOOLEAN;

    pub fn IsHcnDeleteEndpointPresent() -> BOOLEAN;

    pub fn IsHcnCloseEndpointPresent() -> BOOLEAN;

    pub fn IsHcnEnumerateLoadBalancersPresent() -> BOOLEAN;

    pub fn IsHcnCreateLoadBalancerPresent() -> BOOLEAN;

    pub fn IsHcnOpenLoadBalancerPresent() -> BOOLEAN;

    pub fn IsHcnModifyLoadBalancerPresent() -> BOOLEAN;

    pub fn IsHcnQueryLoadBalancerPropertiesPresent() -> BOOLEAN;

    pub fn IsHcnDeleteLoadBalancerPresent() -> BOOLEAN;

    pub fn IsHcnCloseLoadBalancerPresent() -> BOOLEAN;

    pub fn IsHcnRegisterServiceCallbackPresent() -> BOOLEAN;

    pub fn IsHcnUnregisterServiceCallbackPresent() -> BOOLEAN;

}
