use std::ffi::{c_void, CString};
use std::mem::zeroed;
use std::os::raw::c_int;
use std::ptr::null_mut;

use crate::libui;

const WINDOW_SIZE_X: c_int = 500;
const WINDOW_SIZE_Y: c_int = 300;

const ABOUT: &'static str = "ZeroTier Desktop UI

(c)2021-2022 ZeroTier, Inc.
Released under the terms of the Mozilla Public License V2.0 (MPL)
Source URL: https://github.com/zerotier/DesktopUI

The following additional open source code was used in this software:

 * https://github.com/zserge/tray by Serge Zaitsev (heavily modified)
 * https://github.com/libui-ng/libui-ng by Pietro Gagliardi and others
";

unsafe extern "C" fn on_should_quit(_: *mut c_void) -> c_int {
    std::process::exit(0);
}

unsafe extern "C" fn on_window_close(_: *mut libui::uiWindow, _: *mut c_void) -> c_int {
    on_should_quit(null_mut())
}

unsafe extern "C" fn on_ok_button_clicked(_: *mut libui::uiButton, _: *mut c_void) {
    on_should_quit(null_mut());
}

pub fn about_main() {
    unsafe {
        let mut options: libui::uiInitOptions = zeroed();
        assert!(libui::uiInit(&mut options).is_null());

        libui::uiOnShouldQuit(Some(on_should_quit), null_mut());

        let title = CString::new("About ZeroTier UI").unwrap();
        let main_window = libui::uiNewWindow(title.as_ptr(), WINDOW_SIZE_X, WINDOW_SIZE_Y, 1);
        libui::uiWindowSetMargined(main_window, 1);
        libui::uiWindowOnClosing(main_window, Some(on_window_close), null_mut());

        let vbox = libui::uiNewVerticalBox();
        libui::uiBoxSetPadded(vbox, 0);

        let about_text_content = CString::new(ABOUT).unwrap();
        let about_text = libui::uiNewLabel(about_text_content.as_ptr());
        libui::uiBoxAppend(vbox, about_text.cast(), 1);

        let button_box = libui::uiNewHorizontalBox();
        let ok = CString::new(" Ok ").unwrap();
        let ok_button = libui::uiNewButton(ok.as_ptr());
        libui::uiBoxAppend(button_box, libui::uiNewHorizontalBox().cast(), 1);
        libui::uiBoxAppend(button_box, ok_button.cast(), 0);
        libui::uiButtonOnClicked(ok_button, Some(on_ok_button_clicked), null_mut());
        libui::uiBoxAppend(button_box, libui::uiNewHorizontalBox().cast(), 1);
        libui::uiBoxAppend(vbox, button_box.cast(), 0);

        libui::uiWindowSetChild(main_window, vbox.cast());

        libui::uiControlShow(main_window.cast());
        libui::uiMain();

        libui::uiUninit();
    }
}
