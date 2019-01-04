// Copyright © rafawo (rafawo1@hotmail.com). All rights reserved.
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
