use std::env;
use std::ffi::{CString, OsString};
use std::io;
use std::mem;
use std::os::unix::ffi::OsStrExt;
use std::path::{Path, PathBuf};
use std::process::ExitStatus;
use std::ptr;

use crate::impl_unix::runas_impl as runas_sudo_impl;
use crate::Command;

extern "C" {
    fn rust_darwin_gui_runas(cmd: *const i8, argv: *const *const i8) -> u32;
}

fn find_exe<P: AsRef<Path>>(exe_name: P) -> Option<PathBuf> {
    let exe_name = exe_name.as_ref().as_os_str();
    if let Some(exe) = exe_name.to_str() {
        if exe.starts_with("/") || exe.starts_with("./") {
            return Some(PathBuf::from(exe_name));
        }
    }

    env::var_os("PATH").and_then(|paths| {
        env::split_paths(&paths)
            .filter_map(|dir| {
                let full_path = dir.join(&exe_name);
                if full_path.is_file() {
                    Some(full_path)
                } else {
                    None
                }
            })
            .next()
    })
}

macro_rules! make_cstring {
    ($s:expr) => {
        match CString::new($s.as_bytes()) {
            Ok(s) => s,
            Err(_) => {
                return Err(io::Error::new(io::ErrorKind::Other, "null byte in string"));
            }
        }
    };
}

fn runas_gui_impl(cmd: &Command) -> io::Result<ExitStatus> {
    let exe: OsString = match find_exe(&cmd.command) {
        Some(exe) => exe.into(),
        None => unsafe {
            return Ok(mem::transmute(!0));
        },
    };
    let prog = make_cstring!(exe);
    let mut args = vec![];
    for arg in cmd.args.iter() {
        args.push(make_cstring!(arg))
    }
    let mut argv: Vec<_> = args.iter().map(|x| x.as_ptr()).collect();
    argv.push(ptr::null());

    unsafe {
        Ok(mem::transmute(rust_darwin_gui_runas(
            prog.as_ptr(),
            argv.as_ptr(),
        )))
    }
}

pub fn runas_impl(cmd: &Command) -> io::Result<ExitStatus> {
    if cmd.gui {
        runas_gui_impl(&cmd)
    } else {
        runas_sudo_impl(&cmd)
    }
}
