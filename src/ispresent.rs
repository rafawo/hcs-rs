// Copyright © rafawo (rafawo1@hotmail.com). All rights reserved.
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.
// All files in the project carrying such notice may not be copied, modified, or distributed
// except according to those terms.
// THE SOURCE CODE IS AVAILABLE UNDER THE ABOVE CHOSEN LICENSE "AS IS", WITH NO WARRANTIES.

pub mod computecore {
    use crate::computecore_bindings::*;

    pub fn enumerate_compute_systems_is_present() -> bool {
        unsafe { IsHcsEnumerateComputeSystemsPresent() != 0 }
    }

    pub fn create_operation_is_present() -> bool {
        unsafe { IsHcsCreateOperationPresent() != 0 }
    }

    pub fn close_operation_is_present() -> bool {
        unsafe { IsHcsCloseOperationPresent() != 0 }
    }

    pub fn get_operation_context_is_present() -> bool {
        unsafe { IsHcsGetOperationContextPresent() != 0 }
    }

    pub fn set_operation_context_is_present() -> bool {
        unsafe { IsHcsSetOperationContextPresent() != 0 }
    }

    pub fn get_compute_system_from_operation_is_present() -> bool {
        unsafe { IsHcsGetComputeSystemFromOperationPresent() != 0 }
    }

    pub fn get_process_from_operation_is_present() -> bool {
        unsafe { IsHcsGetProcessFromOperationPresent() != 0 }
    }

    pub fn get_operation_type_is_present() -> bool {
        unsafe { IsHcsGetOperationTypePresent() != 0 }
    }

    pub fn get_operation_id_is_present() -> bool {
        unsafe { IsHcsGetOperationIdPresent() != 0 }
    }

    pub fn get_operation_result_is_present() -> bool {
        unsafe { IsHcsGetOperationResultPresent() != 0 }
    }

    pub fn get_operation_result_and_process_info_is_present() -> bool {
        unsafe { IsHcsGetOperationResultAndProcessInfoPresent() != 0 }
    }

    pub fn wait_for_operation_result_is_present() -> bool {
        unsafe { IsHcsWaitForOperationResultPresent() != 0 }
    }

    pub fn wait_for_operation_result_and_process_info_is_present() -> bool {
        unsafe { IsHcsWaitForOperationResultAndProcessInfoPresent() != 0 }
    }

    pub fn set_operation_callback_is_present() -> bool {
        unsafe { IsHcsSetOperationCallbackPresent() != 0 }
    }

    pub fn cancel_operation_is_present() -> bool {
        unsafe { IsHcsCancelOperationPresent() != 0 }
    }

    pub fn create_compute_system_is_present() -> bool {
        unsafe { IsHcsCreateComputeSystemPresent() != 0 }
    }

    pub fn open_compute_system_is_present() -> bool {
        unsafe { IsHcsOpenComputeSystemPresent() != 0 }
    }

    pub fn close_compute_system_is_present() -> bool {
        unsafe { IsHcsCloseComputeSystemPresent() != 0 }
    }

    pub fn start_compute_system_is_present() -> bool {
        unsafe { IsHcsStartComputeSystemPresent() != 0 }
    }

    pub fn shut_down_compute_system_is_present() -> bool {
        unsafe { IsHcsShutDownComputeSystemPresent() != 0 }
    }

    pub fn terminate_compute_system_is_present() -> bool {
        unsafe { IsHcsTerminateComputeSystemPresent() != 0 }
    }

    pub fn pause_compute_system_is_present() -> bool {
        unsafe { IsHcsPauseComputeSystemPresent() != 0 }
    }

    pub fn resume_compute_system_is_present() -> bool {
        unsafe { IsHcsResumeComputeSystemPresent() != 0 }
    }

    pub fn save_compute_system_is_present() -> bool {
        unsafe { IsHcsSaveComputeSystemPresent() != 0 }
    }

    pub fn get_compute_system_properties_is_present() -> bool {
        unsafe { IsHcsGetComputeSystemPropertiesPresent() != 0 }
    }

    pub fn modify_compute_system_is_present() -> bool {
        unsafe { IsHcsModifyComputeSystemPresent() != 0 }
    }

    pub fn set_compute_system_callback_is_present() -> bool {
        unsafe { IsHcsSetComputeSystemCallbackPresent() != 0 }
    }

    pub fn create_process_is_present() -> bool {
        unsafe { IsHcsCreateProcessPresent() != 0 }
    }

    pub fn open_process_is_present() -> bool {
        unsafe { IsHcsOpenProcessPresent() != 0 }
    }

    pub fn close_process_is_present() -> bool {
        unsafe { IsHcsCloseProcessPresent() != 0 }
    }

    pub fn terminate_process_is_present() -> bool {
        unsafe { IsHcsTerminateProcessPresent() != 0 }
    }

    pub fn signal_process_is_present() -> bool {
        unsafe { IsHcsSignalProcessPresent() != 0 }
    }

    pub fn get_process_info_is_present() -> bool {
        unsafe { IsHcsGetProcessInfoPresent() != 0 }
    }

    pub fn get_process_properties_is_present() -> bool {
        unsafe { IsHcsGetProcessPropertiesPresent() != 0 }
    }

    pub fn modify_process_is_present() -> bool {
        unsafe { IsHcsModifyProcessPresent() != 0 }
    }

    pub fn set_process_callback_is_present() -> bool {
        unsafe { IsHcsSetProcessCallbackPresent() != 0 }
    }

    pub fn get_service_properties_is_present() -> bool {
        unsafe { IsHcsGetServicePropertiesPresent() != 0 }
    }

    pub fn modify_service_settings_is_present() -> bool {
        unsafe { IsHcsModifyServiceSettingsPresent() != 0 }
    }

    pub fn submit_wer_report_is_present() -> bool {
        unsafe { IsHcsSubmitWerReportPresent() != 0 }
    }

    pub fn create_empty_guest_state_file_is_present() -> bool {
        unsafe { IsHcsCreateEmptyGuestStateFilePresent() != 0 }
    }

    pub fn create_empty_runtime_state_file_is_present() -> bool {
        unsafe { IsHcsCreateEmptyRuntimeStateFilePresent() != 0 }
    }

    pub fn grant_vm_access_is_present() -> bool {
        unsafe { IsHcsGrantVmAccessPresent() != 0 }
    }

    pub fn revoke_vm_access_is_present() -> bool {
        unsafe { IsHcsRevokeVmAccessPresent() != 0 }
    }
}

pub mod computenetwork {
    use crate::computenetwork_bindings::*;

    pub fn enumerate_networks_is_present() -> bool {
        unsafe { IsHcnEnumerateNetworksPresent() != 0 }
    }

    pub fn create_network_is_present() -> bool {
        unsafe { IsHcnCreateNetworkPresent() != 0 }
    }

    pub fn open_network_is_present() -> bool {
        unsafe { IsHcnOpenNetworkPresent() != 0 }
    }

    pub fn modify_network_is_present() -> bool {
        unsafe { IsHcnModifyNetworkPresent() != 0 }
    }

    pub fn query_network_properties_is_present() -> bool {
        unsafe { IsHcnQueryNetworkPropertiesPresent() != 0 }
    }

    pub fn delete_network_is_present() -> bool {
        unsafe { IsHcnDeleteNetworkPresent() != 0 }
    }

    pub fn close_network_is_present() -> bool {
        unsafe { IsHcnCloseNetworkPresent() != 0 }
    }

    pub fn enumerate_namespaces_is_present() -> bool {
        unsafe { IsHcnEnumerateNamespacesPresent() != 0 }
    }

    pub fn create_namespace_is_present() -> bool {
        unsafe { IsHcnCreateNamespacePresent() != 0 }
    }

    pub fn open_namespace_is_present() -> bool {
        unsafe { IsHcnOpenNamespacePresent() != 0 }
    }

    pub fn modify_namespace_is_present() -> bool {
        unsafe { IsHcnModifyNamespacePresent() != 0 }
    }

    pub fn query_namespace_properties_is_present() -> bool {
        unsafe { IsHcnQueryNamespacePropertiesPresent() != 0 }
    }

    pub fn delete_namespace_is_present() -> bool {
        unsafe { IsHcnDeleteNamespacePresent() != 0 }
    }

    pub fn close_namespace_is_present() -> bool {
        unsafe { IsHcnCloseNamespacePresent() != 0 }
    }

    pub fn enumerate_endpoints_is_present() -> bool {
        unsafe { IsHcnEnumerateEndpointsPresent() != 0 }
    }

    pub fn create_endpoint_is_present() -> bool {
        unsafe { IsHcnCreateEndpointPresent() != 0 }
    }

    pub fn open_endpoint_is_present() -> bool {
        unsafe { IsHcnOpenEndpointPresent() != 0 }
    }

    pub fn modify_endpoint_is_present() -> bool {
        unsafe { IsHcnModifyEndpointPresent() != 0 }
    }

    pub fn query_endpoint_properties_is_present() -> bool {
        unsafe { IsHcnQueryEndpointPropertiesPresent() != 0 }
    }

    pub fn delete_endpoint_is_present() -> bool {
        unsafe { IsHcnDeleteEndpointPresent() != 0 }
    }

    pub fn close_endpoint_is_present() -> bool {
        unsafe { IsHcnCloseEndpointPresent() != 0 }
    }

    pub fn enumerate_load_balancers_is_present() -> bool {
        unsafe { IsHcnEnumerateLoadBalancersPresent() != 0 }
    }

    pub fn create_load_balancer_is_present() -> bool {
        unsafe { IsHcnCreateLoadBalancerPresent() != 0 }
    }

    pub fn open_load_balancer_is_present() -> bool {
        unsafe { IsHcnOpenLoadBalancerPresent() != 0 }
    }

    pub fn modify_load_balancer_is_present() -> bool {
        unsafe { IsHcnModifyLoadBalancerPresent() != 0 }
    }

    pub fn query_load_balancer_properties_is_present() -> bool {
        unsafe { IsHcnQueryLoadBalancerPropertiesPresent() != 0 }
    }

    pub fn delete_load_balancer_is_present() -> bool {
        unsafe { IsHcnDeleteLoadBalancerPresent() != 0 }
    }

    pub fn close_load_balancer_is_present() -> bool {
        unsafe { IsHcnCloseLoadBalancerPresent() != 0 }
    }

    pub fn register_service_callback_is_present() -> bool {
        unsafe { IsHcnRegisterServiceCallbackPresent() != 0 }
    }

    pub fn unregister_service_callback_is_present() -> bool {
        unsafe { IsHcnUnregisterServiceCallbackPresent() != 0 }
    }
}

pub mod hypervdevicevirtualization {
    use crate::hypervdevicevirtualization_bindings::*;

    pub fn initialize_device_host_is_present() -> bool {
        unsafe { IsHdvInitializeDeviceHostPresent() != 0 }
    }

    pub fn teardown_device_host_is_present() -> bool {
        unsafe { IsHdvTeardownDeviceHostPresent() != 0 }
    }

    pub fn create_device_instance_is_present() -> bool {
        unsafe { IsHdvCreateDeviceInstancePresent() != 0 }
    }

    pub fn read_guest_memory_is_present() -> bool {
        unsafe { IsHdvReadGuestMemoryPresent() != 0 }
    }

    pub fn write_guest_memory_is_present() -> bool {
        unsafe { IsHdvWriteGuestMemoryPresent() != 0 }
    }

    pub fn create_guest_memory_aperture_is_present() -> bool {
        unsafe { IsHdvCreateGuestMemoryAperturePresent() != 0 }
    }

    pub fn destroy_guest_memory_aperture_is_present() -> bool {
        unsafe { IsHdvDestroyGuestMemoryAperturePresent() != 0 }
    }

    pub fn deliver_guest_interrupt_is_present() -> bool {
        unsafe { IsHdvDeliverGuestInterruptPresent() != 0 }
    }
}
