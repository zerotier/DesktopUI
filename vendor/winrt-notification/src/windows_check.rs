// Lifted from mattmccarty's work in os_info
use std::mem::zeroed;
use std::mem::size_of;
use winapi::shared::ntdef::NTSTATUS;
use winapi::shared::minwindef::DWORD;
use winapi::shared::ntstatus::STATUS_SUCCESS;

#[cfg(target_arch = "x86")]
use winapi::um::winnt::OSVERSIONINFOEXA;

#[cfg(not(target_arch = "x86"))]
use winapi::um::winnt::OSVERSIONINFOEXW;

#[cfg(target_arch = "x86")]
type OSVERSIONINFOEX = OSVERSIONINFOEXA;

#[cfg(not(target_arch = "x86"))]
type OSVERSIONINFOEX = OSVERSIONINFOEXW;

#[cfg_attr(target_env = "gnu", link(name = "winapi_ntdll"))]
#[cfg_attr(target_env = "msvc", link(name = "ntdll"))]
extern "system" {
    pub fn RtlGetVersion(lpVersionInformation: &mut OSVERSIONINFOEX) -> NTSTATUS;
}

pub fn is_newer_than_windows81() -> bool {
    unsafe {
        let mut info: OSVERSIONINFOEX = { zeroed() };
        info.dwOSVersionInfoSize = size_of::<OSVERSIONINFOEX>() as DWORD;

        if RtlGetVersion(&mut info) == STATUS_SUCCESS {
            info.dwMajorVersion > 6
        } else {
            false
        }
    }
}
