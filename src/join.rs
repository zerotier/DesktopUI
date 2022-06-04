use std::ffi::{c_void, CStr, CString};
use std::mem::zeroed;
use std::os::raw::c_int;
use std::ptr::null_mut;

use crate::libui;

const WINDOW_SIZE_X: c_int = 350;
const WINDOW_SIZE_Y: c_int = 100;
pub const NETWORK_ID_LEN: usize = 16;

#[allow(non_upper_case_globals)]
static mut main_window: *mut libui::uiWindow = null_mut();

#[allow(non_upper_case_globals)]
static mut network_id_input: *mut libui::uiEntry = null_mut();

#[allow(non_upper_case_globals)]
static mut ok_button: *mut libui::uiButton = null_mut();

unsafe fn get_network_id_entered() -> (String, String) {
    if !network_id_input.is_null() {
        let c_id = libui::uiEntryText(network_id_input);
        if !c_id.is_null() {
            let id = CStr::from_ptr(c_id.cast()).to_string_lossy().to_string();
            let id_entered = id.clone();
            let mut id = id.to_lowercase();
            id.retain(|c| "0123456789abcdef".contains(c));
            while id.len() > NETWORK_ID_LEN {
                let _ = id.pop();
            }
            return (id_entered, id);
        }
    }
    return ("".into(), "".into());
}

unsafe extern "C" fn on_should_quit(_: *mut c_void) -> c_int {
    std::process::exit(0);
}

unsafe extern "C" fn on_window_close(_: *mut libui::uiWindow, _: *mut c_void) -> c_int {
    on_should_quit(null_mut())
}

unsafe extern "C" fn on_cancel_button_clicked(_: *mut libui::uiButton, _: *mut c_void) {
    on_should_quit(null_mut());
}

unsafe extern "C" fn on_ok_button_clicked(_: *mut libui::uiButton, _: *mut c_void) {
    if !network_id_input.is_null() {
        let (_, id) = get_network_id_entered();
        if id.len() == NETWORK_ID_LEN {
            println!("!!!JOIN{}", id); // parent scans stdout for this string to pick up ID
            on_should_quit(null_mut());
        }
    }
}

unsafe extern "C" fn on_network_id_input_changed(_: *mut libui::uiEntry, _: *mut c_void) {
    if !network_id_input.is_null() {
        let (id_entered, id) = get_network_id_entered();
        if id.len() == NETWORK_ID_LEN {
            libui::uiControlEnable(ok_button.cast());
        } else {
            libui::uiControlDisable(ok_button.cast());
        }
        if !id.eq(&id_entered) {
            let c_id = CString::new(id).unwrap();
            libui::uiEntrySetText(network_id_input, c_id.as_ptr());
        }
    }
}

unsafe extern "C" fn on_timer(_: *mut c_void) -> c_int {
    //if !main_window.is_null() {}
    1
}

pub fn join_main() {
    unsafe {
        let mut options: libui::uiInitOptions = zeroed();
        assert!(libui::uiInit(&mut options).is_null());

        #[cfg(target_os = "macos")]
        {
            let edit = libui::uiNewMenu("Edit\0".as_bytes().as_ptr().cast());
            libui::uiMenuAppendItem(edit, "@macCut\0".as_bytes().as_ptr().cast());
            libui::uiMenuAppendItem(edit, "@macCopy\0".as_bytes().as_ptr().cast());
            libui::uiMenuAppendItem(edit, "@macPaste\0".as_bytes().as_ptr().cast());
            libui::uiMenuAppendItem(edit, "@macSelectAll\0".as_bytes().as_ptr().cast());
        }

        libui::uiOnShouldQuit(Some(on_should_quit), null_mut());

        main_window = libui::uiNewWindow(
            "Join ZeroTier Network\0".as_bytes().as_ptr().cast(),
            WINDOW_SIZE_X,
            WINDOW_SIZE_Y,
            1,
        );
        libui::uiWindowSetMargined(main_window, 1);
        libui::uiWindowOnClosing(main_window, Some(on_window_close), null_mut());

        let vbox = libui::uiNewVerticalBox();
        libui::uiBoxSetPadded(vbox, 1);

        let input_title = CString::new("Enter 16-digit Network ID to Join:").unwrap();
        let input_title_label = libui::uiNewLabel(input_title.as_ptr());
        libui::uiBoxAppend(vbox, input_title_label.cast(), 0);
        network_id_input = libui::uiNewEntry();
        libui::uiEntryOnChanged(
            network_id_input,
            Some(on_network_id_input_changed),
            null_mut(),
        );
        libui::uiEntrySetReadOnly(network_id_input, 0);
        libui::uiBoxAppend(vbox, network_id_input.cast(), 0);

        let button_box = libui::uiNewHorizontalBox();
        libui::uiBoxSetPadded(button_box, 1);
        let ok = CString::new(" Join ").unwrap();
        let cancel = CString::new(" Cancel ").unwrap();
        ok_button = libui::uiNewButton(ok.as_ptr());
        libui::uiButtonOnClicked(ok_button, Some(on_ok_button_clicked), null_mut());
        libui::uiControlDisable(ok_button.cast());
        let cancel_button = libui::uiNewButton(cancel.as_ptr());
        libui::uiButtonOnClicked(cancel_button, Some(on_cancel_button_clicked), null_mut());
        libui::uiBoxAppend(button_box, libui::uiNewHorizontalBox().cast(), 1);
        libui::uiBoxAppend(button_box, cancel_button.cast(), 0);
        libui::uiBoxAppend(button_box, ok_button.cast(), 0);
        libui::uiBoxAppend(vbox, button_box.cast(), 0);

        libui::uiWindowSetChild(main_window, vbox.cast());

        libui::uiTimer(250, Some(on_timer), null_mut());

        libui::uiControlShow(main_window.cast());
        libui::uiMain();

        libui::uiUninit();
    }
}
