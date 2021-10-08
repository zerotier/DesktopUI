use std::ffi::OsStr;
use std::io;
use std::mem;
use std::os::raw::c_ushort;
use std::os::windows::ffi::OsStrExt;
use std::process::ExitStatus;

use crate::Command;

extern "C" {
    fn rust_win_runas(cmd: *const c_ushort, args: *const c_ushort, show: i32) -> u32;
}

pub fn runas_impl(cmd: &Command) -> io::Result<ExitStatus> {
    let mut params = String::new();
    for arg in cmd.args.iter() {
        let arg = arg.to_string_lossy();
        params.push(' ');
        if arg.len() == 0 {
            params.push_str("\"\"");
        } else if arg.find(&[' ', '\t', '"'][..]).is_none() {
            params.push_str(&arg);
        } else {
            params.push('"');
            for c in arg.chars() {
                match c {
                    '\\' => params.push_str("\\\\"),
                    '"' => params.push_str("\\\""),
                    c => params.push(c),
                }
            }
            params.push('"');
        }
    }

    let file = OsStr::new(&cmd.command)
        .encode_wide()
        .chain(Some(0))
        .collect::<Vec<_>>();
    let params = OsStr::new(&params)
        .encode_wide()
        .chain(Some(0))
        .collect::<Vec<_>>();

    unsafe {
        let show = if cmd.hide { 0 } else { 1 };
        Ok(mem::transmute(rust_win_runas(
            file.as_ptr(),
            params.as_ptr(),
            show,
        )))
    }
}
