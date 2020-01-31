// Copyright (c) 2019-2020 Rafael Alcaraz Mercado. All rights reserved.
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.
// All files in the project carrying such notice may not be copied, modified, or distributed
// except according to those terms.
// THE SOURCE CODE IS AVAILABLE UNDER THE ABOVE CHOSEN LICENSE "AS IS", WITH NO WARRANTIES.

//! Type definitions used by the computenetwork APIs.

use winutils_rs::windefs::*;

/// Notifications indicated to callbacks
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum HcnNotifications {
    Invalid = 0x00000000,

    // Notifications for HcnServiceHandle handles
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
pub type HcnNotificationCallback = Option<
    unsafe extern "system" fn(
        notificationType: DWord,
        context: *mut Void,
        notificationStatus: HResult,
        notificationData: PCWStr,
    ),
>;

/// Context handle referencing a Network in HNS
pub type HcnNetworkHandle = *const Void;

/// Context handle referencing a pointer to a Network in HNS
pub type PHcnNetworkHandle = *mut HcnNetworkHandle;

/// Context handle referencing a Namespace in HNS
pub type HcnNamespaceHandle = *const Void;

/// Context handle referencing a pointer to a Namespace in HNS
pub type PHcnNamespaceHandle = *mut HcnNamespaceHandle;

/// Context handle referencing an Endpoint in HNS
pub type HcnEndpointHandle = *const Void;

/// Context handle referencing a pointer to an Endpoint in HNS
pub type PHcnEndpointHandle = *mut HcnEndpointHandle;

/// Context handle referencing a LoadBalancer in HNS
pub type HcnLoadBalancerHandle = *const Void;

/// Context handle referencing a pointer to a LoadBalancer in HNS
pub type PHcnLoadBalancerHandle = *mut HcnLoadBalancerHandle;

/// Context handle referencing the HNS service
pub type HcnServiceHandle = *const Void;

/// Context handle referencing the HNS service
pub type PHcnServiceHandle = *mut HcnServiceHandle;
