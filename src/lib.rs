pub mod computedefs;
pub mod computenetworkdefs;
pub mod hypervdevicevirtualizationdefs;

pub mod computestorage;
pub mod diskutilities;
pub mod vhdutilities;

#[allow(dead_code)]
pub(crate) mod computecore_bindings;

#[allow(dead_code)]
pub(crate) mod computenetwork_bindings;

#[allow(dead_code)]
pub(crate) mod computestorage_bindings;

#[allow(dead_code)]
pub(crate) mod hypervdevicevirtualization_bindings;

#[allow(dead_code)]
pub(crate) mod windefs {
    //! Defines type aliases for Windows Definitions to user Rust naming conventions
    //! throughout the crate.

    pub type Bool = winapi::shared::minwindef::BOOL;
    pub type Boolean = winapi::shared::ntdef::BOOLEAN;
    pub type Byte = winapi::shared::minwindef::BYTE;
    pub type DWord = winapi::shared::minwindef::DWORD;
    pub type Handle = winapi::shared::ntdef::HANDLE;
    pub type HResult = winapi::shared::ntdef::HRESULT;
    pub type PCWStr = winapi::shared::ntdef::PCWSTR;
    pub type PWStr = winapi::shared::ntdef::PWSTR;
    pub type PVoid = winapi::shared::ntdef::PVOID;
    pub type UChar = winapi::shared::ntdef::UCHAR;
    pub type Void = winapi::shared::ntdef::VOID;
    pub type WChar = winapi::shared::ntdef::WCHAR;
    pub type Word = winapi::shared::minwindef::WORD;

    pub type Guid = winapi::shared::guiddef::GUID;
    pub type Acl = winapi::um::winnt::ACL;
    pub type SecurityDescriptor = winapi::um::winnt::SECURITY_DESCRIPTOR;
}

/// Common result codes that can be returned by the HCS APIs
#[derive(Debug, PartialEq)]
pub enum ResultCode {
    Success,
    OutOfMemory,
    FileNotFound,
    Fail,
    InvalidArgument,
    Unexpected,
    WindowsHResult(windefs::HResult),
}

#[allow(overflowing_literals)]
pub(crate) fn hresult_to_result_code(hresult: &windefs::HResult) -> ResultCode {
    match hresult {
        0 => ResultCode::Success,
        0x8007000E => ResultCode::OutOfMemory,
        0x80070002 => ResultCode::FileNotFound,
        0x80004005 => ResultCode::Fail,
        0x80070057 => ResultCode::InvalidArgument,
        0x8000FFFF => ResultCode::Unexpected,
        other => ResultCode::WindowsHResult(other.clone()),
    }
}
