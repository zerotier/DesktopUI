//! A rust library for single instance application.
//!
//! single-instance provides a single API to check if there are any other running instance.
//!
//! ## Detail
//! On windows, init `SingleInstance` will create a mutex named by given `&str` then check error code by calling `GetLastError`.
//! On linux init will bind abstract unix domain socket with given name . On macos, init will create or open a file which path is given `&str`,
//! then call `flock` to apply an advisory lock on the open file.
//!
//! ### Examples
//! ```rust
//! extern crate single_instance;
//!
//! use std::thread;
//! use single_instance::SingleInstance;
//!
//! fn main() {
//!     let instance = SingleInstance::new("whatever").unwrap();
//!     assert!(instance.is_single());
//! }
//! ```

pub mod error;

#[cfg(target_os = "macos")]
extern crate libc;
#[cfg(target_os = "linux")]
extern crate nix;
extern crate thiserror;
#[cfg(target_os = "windows")]
extern crate widestring;
#[cfg(target_os = "windows")]
extern crate winapi;

pub use self::inner::*;

#[cfg(target_os = "windows")]
mod inner {
    use error::{Result, SingleInstanceError};
    use std::ptr;
    use widestring::WideCString;
    use winapi::shared::winerror::{ERROR_ALREADY_EXISTS, ERROR_INVALID_HANDLE};
    use winapi::um::errhandlingapi::GetLastError;
    use winapi::um::handleapi::CloseHandle;
    use winapi::um::synchapi::CreateMutexW;
    use winapi::um::winnt::HANDLE;

    /// A struct representing one running instance.
    pub struct SingleInstance {
        handle: Option<HANDLE>,
    }

    unsafe impl Send for SingleInstance {}
    unsafe impl Sync for SingleInstance {}

    impl SingleInstance {
        /// Returns a new SingleInstance object.
        pub fn new(name: &str) -> Result<Self> {
            let name = WideCString::from_str(name)?;
            unsafe {
                let handle = CreateMutexW(ptr::null_mut(), 0, name.as_ptr());
                let last_error = GetLastError();

                // https://docs.microsoft.com/en-us/windows/win32/api/synchapi/nf-synchapi-createmutexexw
                if handle.is_null() || handle == ERROR_INVALID_HANDLE as _ {
                    Err(SingleInstanceError::MutexError(last_error))
                } else if last_error == ERROR_ALREADY_EXISTS {
                    CloseHandle(handle);
                    Ok(SingleInstance { handle: None })
                } else {
                    Ok(SingleInstance {
                        handle: Some(handle),
                    })
                }
            }
        }

        /// Returns whether this instance is single.
        pub fn is_single(&self) -> bool {
            self.handle.is_some()
        }
    }

    impl Drop for SingleInstance {
        fn drop(&mut self) {
            if let Some(handle) = self.handle.take() {
                unsafe {
                    CloseHandle(handle);
                }
            }
        }
    }
}

#[cfg(target_os = "linux")]
mod inner {
    use error::Result;
    use nix::errno::Errno;
    use nix::sys::socket::{self, UnixAddr};
    use nix::unistd;
    use std::os::unix::prelude::RawFd;

    /// A struct representing one running instance.
    pub struct SingleInstance {
        maybe_sock: Option<RawFd>,
    }

    impl SingleInstance {
        /// Returns a new SingleInstance object.
        pub fn new(name: &str) -> Result<Self> {
            let addr = UnixAddr::new_abstract(name.as_bytes())?;
            let sock = socket::socket(
                socket::AddressFamily::Unix,
                socket::SockType::Stream,
                socket::SockFlag::empty(),
                None,
            )?;

            let maybe_sock = match socket::bind(sock, &socket::SockAddr::Unix(addr)) {
                Ok(()) => Some(sock),
                Err(nix::Error::Sys(Errno::EADDRINUSE)) => None,
                Err(e) => return Err(e.into()),
            };

            Ok(Self { maybe_sock })
        }

        /// Returns whether this instance is single.
        pub fn is_single(&self) -> bool {
            self.maybe_sock.is_some()
        }
    }

    impl Drop for SingleInstance {
        fn drop(&mut self) {
            if let Some(sock) = self.maybe_sock {
                // Intentionally discard any close errors.
                let _ = unistd::close(sock);
            }
        }
    }
}

#[cfg(target_os = "macos")]
mod inner {
    use error::Result;
    use libc::{__error, flock, EWOULDBLOCK, LOCK_EX, LOCK_NB};
    use std::fs::File;
    use std::os::unix::io::AsRawFd;
    use std::path::Path;

    /// A struct representing one running instance.
    pub struct SingleInstance {
        _file: File,
        is_single: bool,
    }

    impl SingleInstance {
        /// Returns a new SingleInstance object.
        pub fn new(name: &str) -> Result<Self> {
            let path = Path::new(name);
            let file = if path.exists() {
                File::open(path)?
            } else {
                File::create(path)?
            };
            unsafe {
                let rc = flock(file.as_raw_fd(), LOCK_EX | LOCK_NB);
                let is_single = rc == 0 || EWOULDBLOCK != *__error();
                Ok(Self {
                    _file: file,
                    is_single,
                })
            }
        }

        /// Returns whether this instance is single.
        pub fn is_single(&self) -> bool {
            self.is_single
        }
    }
}

#[test]
fn test_single_instance() {
    {
        let instance_a = SingleInstance::new("aa2d0258-ffe9-11e7-ba89-0ed5f89f718b").unwrap();
        assert!(instance_a.is_single());
        let instance_b = SingleInstance::new("aa2d0258-ffe9-11e7-ba89-0ed5f89f718b").unwrap();
        assert!(!instance_b.is_single());
    }
    let instance_c = SingleInstance::new("aa2d0258-ffe9-11e7-ba89-0ed5f89f718b").unwrap();
    assert!(instance_c.is_single());
}
