use std::ffi::{c_void, CStr, CString};
use std::mem::zeroed;
use std::os::raw::c_int;
use std::ptr::null_mut;

use crate::libui;

const WINDOW_SIZE_X: c_int = 500;
const WINDOW_SIZE_Y: c_int = 250;

unsafe extern "C" fn on_should_quit(_: *mut c_void) -> c_int {
    std::process::exit(0);
}

unsafe extern "C" fn on_window_close(_: *mut libui::uiWindow, _: *mut c_void) -> c_int {
    on_should_quit(null_mut())
}

unsafe extern "C" fn on_ok_button_clicked(_: *mut libui::uiButton, _: *mut c_void) {
    on_should_quit(null_mut());
}

pub fn about_main(version: &str) {
    unsafe {
        let mut options: libui::uiInitOptions = zeroed();
        let init_err = libui::uiInit(&mut options);
        if !init_err.is_null() {
            let s = CStr::from_ptr(init_err.cast());
            println!("libui init error: {}", s.to_string_lossy());
            panic!();
        }

        libui::uiOnShouldQuit(Some(on_should_quit), null_mut());

        let title = CString::new("About ZeroTier UI").unwrap();
        let main_window = libui::uiNewWindow(title.as_ptr(), WINDOW_SIZE_X, WINDOW_SIZE_Y, 0);
        libui::uiWindowSetMargined(main_window, 0);
        libui::uiWindowOnClosing(main_window, Some(on_window_close), null_mut());

        let vbox = libui::uiNewVerticalBox();
        libui::uiBoxSetPadded(vbox, 0);

        let about_text_content = CString::new(format!(
            "
ZeroTier {}
Desktop System Tray UI
(c)2021-2024 ZeroTier, Inc.

Released under the terms of the Mozilla Public License V2.0 (MPL)
Source URL: https://github.com/zerotier/DesktopUI

This UI application contains the following additional open source software:

 * https://github.com/zserge/tray by Serge Zaitsev (with modifications)
 * https://github.com/libui-ng/libui-ng by Pietro Gagliardi and others",
            version
        ))
        .unwrap();
        let about_text = libui::uiNewMultilineEntry();
        libui::uiMultilineEntrySetReadOnly(about_text, 1);
        libui::uiMultilineEntrySetText(about_text, about_text_content.as_ptr().cast());
        libui::uiBoxAppend(vbox, about_text.cast(), 1);

        let ok = CString::new(" Ok ").unwrap();
        let ok_button = libui::uiNewButton(ok.as_ptr());
        libui::uiButtonOnClicked(ok_button, Some(on_ok_button_clicked), null_mut());
        libui::uiBoxAppend(vbox, ok_button.cast(), 0);

        libui::uiWindowSetChild(main_window, vbox.cast());

        libui::uiControlShow(main_window.cast());
        libui::uiMain();

        libui::uiUninit();
    }
}
