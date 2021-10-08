extern crate libc;
use libc::{c_char, c_uchar, c_int};
use std::ffi::{CStr, CString};
use std::ptr;

extern {
    fn tinyfd_messageBox (
        aTitle: *const c_char ,
        aMessage: *const c_char,
        aDialogType: *const c_char,
        aIconType: *const c_char,
        aDefaultButton: c_int) -> c_int;

    fn tinyfd_inputBox (
        aTitle: *const c_char ,
        aMessage: *const c_char,
        aDefaultInput: *const c_char) -> *const c_char;

    fn tinyfd_saveFileDialog (
        aTitle: *const c_char,
        aDefaultPathAndFile: *const c_char,
        aNumOfFilterPatterns: c_int,
        aFilterPatterns: *const *const c_char,
        aSingleFilterDescription: *const c_char) -> *const c_char;

    fn tinyfd_openFileDialog (
        aTitle: *const c_char,
        aDefaultPathAndFile: *const c_char,
        aNumOfFilterPatterns: c_int,
        aFilterPatterns: *const *const c_char,
        aSingleFilterDescription: *const c_char,
        aAllowMultipleSelects: c_int) -> *const c_char;

    fn tinyfd_selectFolderDialog (
        aTitle: *const c_char,
        aDefaultPath: *const c_char) -> *const c_char;

    fn tinyfd_colorChooser (
        aTitle: *const c_char,
        aDefaultHexRGB: *const c_char,
        aDefaultRGB: *const c_uchar,
        aoResultRGB: *mut c_uchar) -> *const c_char;
}

fn message_box(title: &str, message: &str, box_type: &str, icon: &str, button: i32) -> i32 {
    let message_box_title          = CString::new(title).unwrap();
    let message_box_message        = CString::new(message).unwrap();
    let message_box_type           = CString::new(box_type).unwrap();
    let message_box_icon           = CString::new(icon).unwrap();

    unsafe {
        tinyfd_messageBox(
            message_box_title.as_ptr(),
            message_box_message.as_ptr(),
            message_box_type.as_ptr(),
            message_box_icon.as_ptr(),
            button)
    }
}

pub enum MessageBoxIcon {
    Info,
    Warning,
    Error,
    Question,
}

impl MessageBoxIcon {
    fn to_str(&self) -> &'static str {
        match *self {
            MessageBoxIcon::Info => "info",
            MessageBoxIcon::Warning => "warning",
            MessageBoxIcon::Error => "error",
            MessageBoxIcon::Question => "question",
        }
    }
}

pub fn message_box_ok(title: &str, message: &str, icon: MessageBoxIcon) {
    let _ = message_box(title, message, "ok", icon.to_str(), 0);
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum OkCancel {
    Cancel = 0,
    Ok,
}

pub fn message_box_ok_cancel(title: &str, message: &str, icon: MessageBoxIcon, default: OkCancel) -> OkCancel {
    match message_box(title, message, "okcancel", icon.to_str(), default as i32) {
        0 => OkCancel::Cancel,
        1 => OkCancel::Ok,
        _ => unreachable!(),
    }
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum YesNo {
    No = 0,
    Yes,
}

pub fn message_box_yes_no(title: &str, message: &str, icon: MessageBoxIcon, default: YesNo) -> YesNo {
    match message_box(title, message, "yesno", icon.to_str(), default as i32) {
        0 => YesNo::No,
        1 => YesNo::Yes,
        _ => unreachable!(),
    }
}

fn input_box_impl(title: &str, message: &str, default: Option<&str>) -> Option<String> {
    let input_box_title   = CString::new(title).unwrap();
    let input_box_message = CString::new(message).unwrap();
    let input_box_default = default.map(|default| CString::new(default).unwrap());

    let c_input = unsafe {
        tinyfd_inputBox(input_box_title.as_ptr(),
                        input_box_message.as_ptr(),
                        input_box_default.as_ref().map(|d| d.as_ptr()).unwrap_or(ptr::null()))
    };

    if !c_input.is_null() {
        unsafe {
            Some(CStr::from_ptr(c_input).to_string_lossy().into_owned())
        }
    } else {
        None
    }
}

pub fn input_box(title: &str, message: &str, default: &str) -> Option<String> {
    input_box_impl(title, message, Some(default))
}

pub fn password_box(title: &str, message: &str) -> Option<String> {
    input_box_impl(title, message, None)
}

fn save_file_dialog_impl(title: &str, path: &str, filter: Option<(&[&str], &str)>) -> Option<String> {
    let save_dialog_title                = CString::new(title).unwrap();
    let save_dialog_path                 = CString::new(path).unwrap();
    let save_dialog_des                  = CString::new(filter.map_or("", |f| f.1)).unwrap();

    let filter_patterns =
        filter.map_or(vec![], |f| f.0.iter().map(|s| CString::new(*s).unwrap()).collect());
    let ptr_filter_patterns = filter_patterns.iter().map(|c| c.as_ptr()).collect::<Vec<*const c_char>>();

    let c_file_name = unsafe {
        tinyfd_saveFileDialog(
            save_dialog_title.as_ptr(),
            save_dialog_path.as_ptr(),
            ptr_filter_patterns.len() as c_int,
            ptr_filter_patterns.as_ptr(),
            save_dialog_des.as_ptr())
    };

    if !c_file_name.is_null() {
        unsafe {
            Some(CStr::from_ptr(c_file_name).to_string_lossy().into_owned())
        }
    } else {
        None
    }
}

pub fn save_file_dialog_with_filter(title: &str,
                                    path: &str,
                                    filter_patterns: &[&str],
                                    description: &str) -> Option<String> {
    save_file_dialog_impl(title, path, Some((filter_patterns, description)))
}

pub fn save_file_dialog(title: &str, path: &str) -> Option<String> {
    save_file_dialog_impl(title, path, None)
}

fn open_file_dialog_impl(title: &str,
                         path: &str,
                         filter: Option<(&[&str], &str)>,
                         multi: bool) -> Option<Vec<String>> {
    let open_dialog_title                = CString::new(title).unwrap();
    let open_dialog_path                 = CString::new(path).unwrap();
    let open_dialog_des                  = CString::new(filter.map_or("", |f| f.1)).unwrap();

    let filter_patterns =
        filter.map_or(vec![], |f| f.0.iter().map(|s| CString::new(*s).unwrap()).collect());
    let ptr_filter_patterns =
        filter_patterns.iter().map(|c| c.as_ptr()).collect::<Vec<*const c_char>>();

    let c_file_name = unsafe {
        tinyfd_openFileDialog(
            open_dialog_title.as_ptr(),
            open_dialog_path.as_ptr(),
            ptr_filter_patterns.len() as c_int,
            ptr_filter_patterns.as_ptr(),
            open_dialog_des.as_ptr(),
            multi as c_int)
    };

    if !c_file_name.is_null() {
        let result = unsafe {
            CStr::from_ptr(c_file_name).to_string_lossy().into_owned()
        };
        Some(if multi {
            result.split('|').map(|s| s.to_owned()).collect()
        } else {
            vec![result]
        })
    } else {
        None
    }
}

pub fn open_file_dialog(title: &str,
                        path: &str,
                        filter: Option<(&[&str], &str)>) -> Option<String> {
    open_file_dialog_impl(title, path, filter, false).and_then(|v| v.into_iter().next())
}

pub fn open_file_dialog_multi(title: &str,
                              path: &str,
                              filter: Option<(&[&str], &str)>) -> Option<Vec<String>> {
    open_file_dialog_impl(title, path, filter, true)
}

pub fn select_folder_dialog(title: &str, path: &str) -> Option<String> {
    let select_folder_title = CString::new(title).unwrap();
    let select_folder_path  = CString::new(path).unwrap();

    let folder = unsafe {
        tinyfd_selectFolderDialog(select_folder_title.as_ptr(), select_folder_path.as_ptr())
    };

    if !folder.is_null() {
        unsafe {
            Some(CStr::from_ptr(folder).to_string_lossy().into_owned())
        }
    } else {
        None
    }
}

pub enum DefaultColorValue<'a> {
    Hex(&'a str),
    RGB(&'a [u8; 3]),
}

pub fn color_chooser_dialog(title: &str, default: DefaultColorValue)
                            -> Option<(String, [u8; 3])> {
    let color_title                      = CString::new(title).unwrap();
    let rubbish = [0, 0, 0];
    let (color_default_hex, color_default_rgb) = match default {
        DefaultColorValue::Hex(hex) => (Some(CString::new(hex).unwrap()), &rubbish),
        DefaultColorValue::RGB(rgb) => (None, rgb),
    };
    let mut color_result_rgb = [0, 0, 0];

    let result = unsafe {
        tinyfd_colorChooser(color_title.as_ptr(),
                            color_default_hex.as_ref().map_or(ptr::null(), |h| h.as_ptr()),
                            color_default_rgb.as_ptr(),
                            color_result_rgb.as_mut_ptr())
    };

    if !result.is_null() {
        unsafe {
            Some((CStr::from_ptr(result).to_string_lossy().into_owned(), color_result_rgb))
        }
    } else {
        None
    }
}
