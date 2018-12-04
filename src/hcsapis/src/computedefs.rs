//! This file contains Rust abstractions for the public types and definitions used by the Host Compute APIs.
extern crate libc;

pub type VOID = libc::c_void;
pub type DWORD = libc::c_ulong;
pub type HANDLE = *mut VOID;
pub type HRESULT = libc::c_long;
pub type WCHAR = libc::wchar_t;
pub type PCWSTR = *const WCHAR;

/// Handle to a compute system
pub type HcsSystem = HANDLE;

/// Handle to a process running in a compute system
pub type HcsProcess = HANDLE;

/// Handle to an operation on a compute system
pub type HcsOperation = HANDLE;

/// Handle to a callback registered on a compute system or process handle.
pub type HcsCallback = HANDLE;

/// Type of an operation. These correspond to the functions that invoke the operation.
pub enum HcsOperationType {
    None = -1,
    Enumerate = 0,
    Create = 1,
    Start = 2,
    Shutdown = 3,
    Pause = 4,
    Resume = 5,
    Save = 6,
    Terminate = 7,
    Modify = 8,
    GetProperties = 9,
    CreateProcess = 10,
    SignalProcess = 11,
    GetProcessInfo = 12,
    GetProcessProperties = 13,
    ModifyProcess = 14,
}

pub const HCS_INVALID_OPERATION_ID: i32 = -1;

/// Function type for the completion callback of an operation.
pub type HcsOperationCompletion = extern "C" fn(operation: HcsOperation, context: *mut VOID);

/// Events indicated to callbacks registered by HcsRegisterComputeSystemCallback or
/// HcsRegisterProcessCallback (since Windows 1809).
pub enum HcsEventType {
    Invalid = 0x00000000,

    /// Events for HCS_SYSTEM handles
    SystemExited = 0x00000001,
    SystemCrashInitiated = 0x00000002,
    SystemCrashReport = 0x00000003,
    SystemRdpEnhancedModeStateChanged = 0x00000004,
    SystemSiloJobCreated = 0x00000005,
    SystemGuestConnectionClosed = 0x00000006,

    /// Events for HCS_PROCESS handles
    ProcessExited = 0x00010000,

    /// Common Events
    OperationCallback = 0x01000000,
    ServiceDisconnect = 0x02000000,
}

/// Provides information about an event that occurred on a compute system or process.
pub struct HcsEvent {
    /// Type of Event (see HcsEventType)
    pub event_type: HcsEventType,

    /// Provides additional data for the event.
    pub event_data: PCWSTR,

    /// Handle to a completed operation (if Type is HcsEventType::OperationCallback).
    pub operation: HcsOperation,
}

/// Options for an event callback registration
pub enum HcsEventOptions {
    None = 0x00000000,
    EnableOperationCallbacks = 0x00000001,
}

/// Function type for compute system event callbacks
pub type HcsEventCallback = extern "C" fn(event: *const HcsEvent, context: *mut VOID);

/// Flags applicable to HcsNotifications
pub enum HcsNotificationFlag {
    Success = 0x00000000,
    Failure = 0x80000000,
}

/// Notifications indicated to callbacks registered by HcsRegisterComputeSystemCallback or
/// HcsRegisterProcessCallback (until Windows 1803).
pub enum HcsNotifications {
    Invalid = 0x00000000,

    /// Notifications for HCS_SYSTEM handles
    SystemExited = 0x00000001,
    SystemCreateCompleted = 0x00000002,
    SystemStartCompleted = 0x00000003,
    SystemPauseCompleted = 0x00000004,
    SystemResumeCompleted = 0x00000005,
    SystemCrashReport = 0x00000006,
    SystemSiloJobCreated = 0x00000007,
    SystemSaveCompleted = 0x00000008,
    SystemRdpEnhancedModeStateChanged = 0x00000009,
    SystemShutdownFailed = 0x0000000A,
    SystemGetPropertiesCompleted = 0x0000000B,
    SystemModifyCompleted = 0x0000000C,
    SystemCrashInitiated = 0x0000000D,
    SystemGuestConnectionClosed = 0x0000000E,

    /// Notifications for HCS_PROCESS handles
    ProcessExited = 0x00010000,

    /// Common notifications
    ServiceDisconnect = 0x01000000,

    /// The upper 4 bits are reserved.
    FlagsReserved = 0xF0000000,
}

/// Function type for compute system notification callbacks
pub type HcsNotificationCallback = extern "C" fn(
    notification_type: DWORD,
    context: *mut VOID,
    notifications_status: HRESULT,
    notification_data: PCWSTR,
);

/// Struct containing information about a process created by HcsStartProcessInComputeSystem
pub struct HcsProcessInformation {
    /// Identifier of the created process
    pub process_id: DWORD,
    reserved: DWORD,

    /// If created, standard input handle of the process
    pub std_input: HANDLE,

    /// If created, standard output handle of the process
    pub std_output: HANDLE,

    /// If created, standard error handle of the process
    pub std_error: HANDLE,
}
