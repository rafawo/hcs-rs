// Copyright © rafawo (rafawo1@hotmail.com). All rights reserved.
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.
// All files in the project carrying such notice may not be copied, modified, or distributed
// except according to those terms.
// THE SOURCE CODE IS AVAILABLE UNDER THE ABOVE CHOSEN LICENSE "AS IS", WITH NO WARRANTIES.

use super::bindings::*;

pub fn enumerate_compute_systems() -> bool {
    unsafe { IsHcsEnumerateComputeSystemsPresent() != 0 }
}

pub fn create_operation() -> bool {
    unsafe { IsHcsCreateOperationPresent() != 0 }
}

pub fn close_operation() -> bool {
    unsafe { IsHcsCloseOperationPresent() != 0 }
}

pub fn get_operation_context() -> bool {
    unsafe { IsHcsGetOperationContextPresent() != 0 }
}

pub fn set_operation_context() -> bool {
    unsafe { IsHcsSetOperationContextPresent() != 0 }
}

pub fn get_compute_system_from_operation() -> bool {
    unsafe { IsHcsGetComputeSystemFromOperationPresent() != 0 }
}

pub fn get_process_from_operation() -> bool {
    unsafe { IsHcsGetProcessFromOperationPresent() != 0 }
}

pub fn get_operation_type() -> bool {
    unsafe { IsHcsGetOperationTypePresent() != 0 }
}

pub fn get_operation_id() -> bool {
    unsafe { IsHcsGetOperationIdPresent() != 0 }
}

pub fn get_operation_result() -> bool {
    unsafe { IsHcsGetOperationResultPresent() != 0 }
}

pub fn get_operation_result_and_process_info() -> bool {
    unsafe { IsHcsGetOperationResultAndProcessInfoPresent() != 0 }
}

pub fn wait_for_operation_result() -> bool {
    unsafe { IsHcsWaitForOperationResultPresent() != 0 }
}

pub fn wait_for_operation_result_and_process_info() -> bool {
    unsafe { IsHcsWaitForOperationResultAndProcessInfoPresent() != 0 }
}

pub fn set_operation_callback() -> bool {
    unsafe { IsHcsSetOperationCallbackPresent() != 0 }
}

pub fn cancel_operation() -> bool {
    unsafe { IsHcsCancelOperationPresent() != 0 }
}

pub fn create_compute_system() -> bool {
    unsafe { IsHcsCreateComputeSystemPresent() != 0 }
}

pub fn open_compute_system() -> bool {
    unsafe { IsHcsOpenComputeSystemPresent() != 0 }
}

pub fn close_compute_system() -> bool {
    unsafe { IsHcsCloseComputeSystemPresent() != 0 }
}

pub fn start_compute_system() -> bool {
    unsafe { IsHcsStartComputeSystemPresent() != 0 }
}

pub fn shut_down_compute_system() -> bool {
    unsafe { IsHcsShutDownComputeSystemPresent() != 0 }
}

pub fn terminate_compute_system() -> bool {
    unsafe { IsHcsTerminateComputeSystemPresent() != 0 }
}

pub fn pause_compute_system() -> bool {
    unsafe { IsHcsPauseComputeSystemPresent() != 0 }
}

pub fn resume_compute_system() -> bool {
    unsafe { IsHcsResumeComputeSystemPresent() != 0 }
}

pub fn save_compute_system() -> bool {
    unsafe { IsHcsSaveComputeSystemPresent() != 0 }
}

pub fn get_compute_system_properties() -> bool {
    unsafe { IsHcsGetComputeSystemPropertiesPresent() != 0 }
}

pub fn modify_compute_system() -> bool {
    unsafe { IsHcsModifyComputeSystemPresent() != 0 }
}

pub fn set_compute_system_callback() -> bool {
    unsafe { IsHcsSetComputeSystemCallbackPresent() != 0 }
}

pub fn create_process() -> bool {
    unsafe { IsHcsCreateProcessPresent() != 0 }
}

pub fn open_process() -> bool {
    unsafe { IsHcsOpenProcessPresent() != 0 }
}

pub fn close_process() -> bool {
    unsafe { IsHcsCloseProcessPresent() != 0 }
}

pub fn terminate_process() -> bool {
    unsafe { IsHcsTerminateProcessPresent() != 0 }
}

pub fn signal_process() -> bool {
    unsafe { IsHcsSignalProcessPresent() != 0 }
}

pub fn get_process_info() -> bool {
    unsafe { IsHcsGetProcessInfoPresent() != 0 }
}

pub fn get_process_properties() -> bool {
    unsafe { IsHcsGetProcessPropertiesPresent() != 0 }
}

pub fn modify_process() -> bool {
    unsafe { IsHcsModifyProcessPresent() != 0 }
}

pub fn set_process_callback() -> bool {
    unsafe { IsHcsSetProcessCallbackPresent() != 0 }
}

pub fn get_service_properties() -> bool {
    unsafe { IsHcsGetServicePropertiesPresent() != 0 }
}

pub fn modify_service_settings() -> bool {
    unsafe { IsHcsModifyServiceSettingsPresent() != 0 }
}

pub fn submit_wer_report() -> bool {
    unsafe { IsHcsSubmitWerReportPresent() != 0 }
}

pub fn create_empty_guest_state_file() -> bool {
    unsafe { IsHcsCreateEmptyGuestStateFilePresent() != 0 }
}

pub fn create_empty_runtime_state_file() -> bool {
    unsafe { IsHcsCreateEmptyRuntimeStateFilePresent() != 0 }
}

pub fn grant_vm_access() -> bool {
    unsafe { IsHcsGrantVmAccessPresent() != 0 }
}

pub fn revoke_vm_access() -> bool {
    unsafe { IsHcsRevokeVmAccessPresent() != 0 }
}
