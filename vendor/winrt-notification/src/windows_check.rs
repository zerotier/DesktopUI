#[cfg(target_env = "msvc")]
mod internal {
    #[link(name = "ntdll")]
    extern "system" {
        pub fn RtlGetNtVersionNumbers(major: *mut u32, minor: *mut u32, build: *mut u32);
    }
}

// The gnu target doesn't have access to this method at build time without installing extra things
// but we know it's in ntdll which will always be present at runtime.
#[cfg(target_env = "gnu")]
mod internal {
    use windows::Win32::System::LibraryLoader::{GetModuleHandleA, GetProcAddress};

    #[allow(non_upper_case_globals)]
    static mut CacheRtlGetNtVersionNumbers: Option<unsafe extern "system" fn() -> isize> = None;

    #[allow(non_snake_case)]
    pub unsafe fn RtlGetNtVersionNumbers(major: *mut u32, minor: *mut u32, build: *mut u32) {
        if CacheRtlGetNtVersionNumbers.is_none() {
            CacheRtlGetNtVersionNumbers = GetProcAddress(GetModuleHandleA("ntdll.dll"), "RtlGetNtVersionNumbers");
        }

        if let Some(RtlGetNtVersionNumbers_FUNCTION) = CacheRtlGetNtVersionNumbers {
            std::intrinsics::transmute::<_, extern "system" fn(*mut u32, *mut u32, *mut u32)>(RtlGetNtVersionNumbers_FUNCTION)(
                major, minor, build,
            )
        } else {
            // RtlGetNtVersionNumbers goes as far back as xp.
            // Assume something else has gone wrong and guess we are newer than windows 8.1
            *major = 7;
        }
    }
}

pub fn is_newer_than_windows81() -> bool {
    let mut major = 0u32;
    let mut minor = 0u32;
    let mut build = 0u32;

    unsafe {
        internal::RtlGetNtVersionNumbers(&mut major, &mut minor, &mut build);
    }
    major > 6
}
