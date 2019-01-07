// Copyright © rafawo (rafawo1@hotmail.com). All rights reserved.
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.
// All files in the project carrying such notice may not be copied, modified, or distributed
// except according to those terms.
// THE SOURCE CODE IS AVAILABLE UNDER THE ABOVE CHOSEN LICENSE "AS IS", WITH NO WARRANTIES.

use super::bindings::*;

pub fn enumerate_networks() -> bool {
    unsafe { IsHcnEnumerateNetworksPresent() != 0 }
}

pub fn create_network() -> bool {
    unsafe { IsHcnCreateNetworkPresent() != 0 }
}

pub fn open_network() -> bool {
    unsafe { IsHcnOpenNetworkPresent() != 0 }
}

pub fn modify_network() -> bool {
    unsafe { IsHcnModifyNetworkPresent() != 0 }
}

pub fn query_network_properties() -> bool {
    unsafe { IsHcnQueryNetworkPropertiesPresent() != 0 }
}

pub fn delete_network() -> bool {
    unsafe { IsHcnDeleteNetworkPresent() != 0 }
}

pub fn close_network() -> bool {
    unsafe { IsHcnCloseNetworkPresent() != 0 }
}

pub fn enumerate_namespaces() -> bool {
    unsafe { IsHcnEnumerateNamespacesPresent() != 0 }
}

pub fn create_namespace() -> bool {
    unsafe { IsHcnCreateNamespacePresent() != 0 }
}

pub fn open_namespace() -> bool {
    unsafe { IsHcnOpenNamespacePresent() != 0 }
}

pub fn modify_namespace() -> bool {
    unsafe { IsHcnModifyNamespacePresent() != 0 }
}

pub fn query_namespace_properties() -> bool {
    unsafe { IsHcnQueryNamespacePropertiesPresent() != 0 }
}

pub fn delete_namespace() -> bool {
    unsafe { IsHcnDeleteNamespacePresent() != 0 }
}

pub fn close_namespace() -> bool {
    unsafe { IsHcnCloseNamespacePresent() != 0 }
}

pub fn enumerate_endpoints() -> bool {
    unsafe { IsHcnEnumerateEndpointsPresent() != 0 }
}

pub fn create_endpoint() -> bool {
    unsafe { IsHcnCreateEndpointPresent() != 0 }
}

pub fn open_endpoint() -> bool {
    unsafe { IsHcnOpenEndpointPresent() != 0 }
}

pub fn modify_endpoint() -> bool {
    unsafe { IsHcnModifyEndpointPresent() != 0 }
}

pub fn query_endpoint_properties() -> bool {
    unsafe { IsHcnQueryEndpointPropertiesPresent() != 0 }
}

pub fn delete_endpoint() -> bool {
    unsafe { IsHcnDeleteEndpointPresent() != 0 }
}

pub fn close_endpoint() -> bool {
    unsafe { IsHcnCloseEndpointPresent() != 0 }
}

pub fn enumerate_load_balancers() -> bool {
    unsafe { IsHcnEnumerateLoadBalancersPresent() != 0 }
}

pub fn create_load_balancer() -> bool {
    unsafe { IsHcnCreateLoadBalancerPresent() != 0 }
}

pub fn open_load_balancer() -> bool {
    unsafe { IsHcnOpenLoadBalancerPresent() != 0 }
}

pub fn modify_load_balancer() -> bool {
    unsafe { IsHcnModifyLoadBalancerPresent() != 0 }
}

pub fn query_load_balancer_properties() -> bool {
    unsafe { IsHcnQueryLoadBalancerPropertiesPresent() != 0 }
}

pub fn delete_load_balancer() -> bool {
    unsafe { IsHcnDeleteLoadBalancerPresent() != 0 }
}

pub fn close_load_balancer() -> bool {
    unsafe { IsHcnCloseLoadBalancerPresent() != 0 }
}

pub fn register_service_callback() -> bool {
    unsafe { IsHcnRegisterServiceCallbackPresent() != 0 }
}

pub fn unregister_service_callback() -> bool {
    unsafe { IsHcnUnregisterServiceCallbackPresent() != 0 }
}
