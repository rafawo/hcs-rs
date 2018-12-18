pub mod computedefs;
pub mod computenetworkdefs;
pub mod hypervdevicevirtualizationdefs;

pub mod computestorage;

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
    use libc;

    pub type Bool = libc::c_int;
    pub type Boolean = libc::c_uchar;
    pub type Byte = libc::c_uchar;
    pub type DWord = libc::c_ulong;
    pub type Handle = *mut libc::c_void;
    pub type HResult = libc::c_long;
    pub type PCWStr = *const libc::wchar_t;
    pub type PWStr = *mut libc::wchar_t;
    pub type PSId = *mut Void;
    pub type PVoid = *mut Void;
    pub type SecurityDescriptorControl = libc::c_ushort;
    pub type Word = libc::c_ushort;
    pub type Void = libc::c_void;

    #[repr(C)]
    #[derive(Debug, Copy, Clone)]
    pub struct Guid {
        pub data1: libc::c_ulong,
        pub data2: libc::c_ushort,
        pub data3: libc::c_ushort,
        pub data4: [libc::c_uchar; 8],
    }

    #[repr(C)]
    #[derive(Debug, Copy, Clone)]
    pub struct Acl {
        pub acl_revision: Byte,
        pub sbz1: Byte,
        pub acl_size: Word,
        pub ace_count: Word,
        pub sbz2: Word,
    }

    #[repr(C)]
    #[derive(Debug, Copy, Clone)]
    pub struct SecurityDescriptor {
        pub revision: Byte,
        pub sbz1: Byte,
        pub control: SecurityDescriptorControl,
        pub owner: PSId,
        pub group: PSId,
        pub sacl: *mut Acl,
        pub dacl: *mut Acl,
    }
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
