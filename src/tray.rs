/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 *
 * (c)2021 ZeroTier, Inc.
 * https://www.zerotier.com/
 */

use std::ffi::CString;
use std::os::raw::{c_char, c_int, c_void};
use std::pin::Pin;
use std::ptr::{null, null_mut};
use std::sync::Mutex;

#[repr(C)]
#[derive(Clone)]
struct CTrayMenu {
    text: *const c_char,
    wtext: *const u16, // only used on Windows
    disabled: c_int,
    checked: c_int,
    cb: unsafe extern "C" fn(*const CTrayMenu),
    context: *mut c_void,
    submenu: *const CTrayMenu,
}

#[repr(C)]
struct CTray {
    icon: *const c_char,
    tray_menu: *const CTrayMenu,
}

#[allow(unused)]
struct CTrayMenuContainer {
    c_text: Option<Pin<Box<CString>>>,
    c_text16: Option<Pin<Box<[u16]>>>,
    items: Vec<CTrayMenuContainer>,
    c_items: Option<Pin<Box<[CTrayMenu]>>>,
    c_tray_menu: CTrayMenu,
}

impl Drop for CTrayMenuContainer {
    fn drop(&mut self) {
        if !self.c_tray_menu.context.is_null() {
            unsafe {
                let b: Box<Box<dyn FnMut()>> = Box::from_raw(self.c_tray_menu.context.cast());
                drop(b);
            }
        }
    }
}

#[allow(dead_code)]
pub enum TrayMenuItem {
    Text {
        text: String,
        checked: bool,
        disabled: bool,
        handler: Option<Box<dyn FnMut()>>,
    },
    Separator,
    Submenu {
        text: String,
        checked: bool,
        items: Vec<TrayMenuItem>,
    },
}

/// Cross-platform system tray menu (Rust glue to C code).
/// WARNING: Only one instance of Tray can currently be created per process.
#[allow(unused)]
pub struct Tray {
    current: Mutex<Vec<CTrayMenuContainer>>,
    c_current: Mutex<Pin<Box<[CTrayMenu]>>>,
    icon_path: Mutex<Pin<CString>>,
    tray_initialized: bool,
}

#[cfg(target_os = "linux")]
extern "C" {
    fn tray_init(tray: *const CTray) -> c_int;
    fn tray_loop(blocking: c_int) -> c_int;
    fn tray_update(tray: *const CTray);
    fn tray_exit();
}

#[cfg(target_os = "macos")]
#[link(name = "Cocoa", kind = "framework")]
extern "C" {
    fn tray_init(tray: *const CTray) -> c_int;
    fn tray_loop(blocking: c_int) -> c_int;
    fn tray_update(tray: *const CTray);
    fn tray_exit();
}

#[cfg(windows)]
extern "C" {
    fn tray_init(tray: *const CTray) -> c_int;
    fn tray_loop(blocking: c_int) -> c_int;
    fn tray_update(tray: *const CTray);
    fn tray_exit();
}

unsafe extern "C" fn tray_handler_callback(item: *const CTrayMenu) {
    if !item.is_null() {
        let f: *mut Box<dyn FnMut()> = (*item).context.cast();
        if !f.is_null() {
            (*f)();
        }
    }
}

const C_DASH: [c_char; 2] = [45, 0]; // "-"
const WC_DASH: [u16; 2] = [45, 0]; // "-" in wchar_t

impl Tray {
    fn tray_create_c_structs(menu: Vec<TrayMenuItem>) -> Vec<CTrayMenuContainer> {
        let mut v: Vec<CTrayMenuContainer> = Vec::new();
        menu.into_iter().for_each(|mi: TrayMenuItem| {
            match mi {
                TrayMenuItem::Text {
                    text,
                    checked,
                    disabled,
                    handler,
                } => {
                    #[cfg(windows)]
                    {
                        let mut c_text16: Vec<u16> = text.encode_utf16().collect();
                        c_text16.push(0);
                        let c_text16 = Pin::new(c_text16.into_boxed_slice());
                        let c_text16_ptr = c_text16.as_ptr();
                        v.push(CTrayMenuContainer {
                            c_text: None,
                            c_text16: Some(c_text16),
                            items: Vec::new(),
                            c_items: None,
                            c_tray_menu: CTrayMenu {
                                text: null(),
                                wtext: c_text16_ptr,
                                disabled: disabled as c_int,
                                checked: checked as c_int,
                                cb: tray_handler_callback,
                                context: handler
                                    .map_or(null_mut(), |h| Box::into_raw(Box::new(h)).cast()), // freed in CTrayMenuContainer drop()
                                submenu: null(),
                            },
                        });
                    }
                    #[cfg(not(windows))]
                    {
                        let c_text = Box::pin(CString::new(text.as_str()).unwrap());
                        let c_text_ptr = c_text.as_ptr();
                        v.push(CTrayMenuContainer {
                            c_text: Some(c_text),
                            c_text16: None,
                            items: Vec::new(),
                            c_items: None,
                            c_tray_menu: CTrayMenu {
                                text: c_text_ptr,
                                wtext: null(),
                                disabled: disabled as c_int,
                                checked: checked as c_int,
                                cb: tray_handler_callback,
                                context: handler
                                    .map_or(null_mut(), |h| Box::into_raw(Box::new(h)).cast()), // freed in CTrayMenuContainer drop()
                                submenu: null(),
                            },
                        });
                    }
                }
                TrayMenuItem::Separator => {
                    v.push(CTrayMenuContainer {
                        c_text: None,
                        c_text16: None,
                        items: Vec::new(),
                        c_items: None,
                        c_tray_menu: CTrayMenu {
                            text: C_DASH.as_ptr(),
                            wtext: WC_DASH.as_ptr(),
                            disabled: 0,
                            checked: 0,
                            cb: tray_handler_callback,
                            context: null_mut(),
                            submenu: null(),
                        },
                    });
                }
                TrayMenuItem::Submenu {
                    text,
                    checked,
                    items,
                } => {
                    if !items.is_empty() {
                        #[cfg(windows)]
                        {
                            let mut c_text16: Vec<u16> = text.encode_utf16().collect();
                            c_text16.push(0);
                            let c_text16 = Pin::new(c_text16.into_boxed_slice());
                            let c_text16_ptr = c_text16.as_ptr();
                            v.push(CTrayMenuContainer {
                                c_text: None,
                                c_text16: Some(c_text16),
                                items: Self::tray_create_c_structs(items),
                                c_items: None,
                                c_tray_menu: CTrayMenu {
                                    text: null(),
                                    wtext: c_text16_ptr,
                                    disabled: 0,
                                    checked: checked as c_int,
                                    cb: tray_handler_callback,
                                    context: null_mut(),
                                    submenu: null(),
                                },
                            });
                        }
                        #[cfg(not(windows))]
                        {
                            let c_text = Box::pin(CString::new(text.as_str()).unwrap());
                            let c_text_ptr: *const c_char = c_text.as_ptr();
                            v.push(CTrayMenuContainer {
                                c_text: Some(c_text),
                                c_text16: None,
                                items: Self::tray_create_c_structs(items),
                                c_items: None,
                                c_tray_menu: CTrayMenu {
                                    text: c_text_ptr,
                                    wtext: null(),
                                    disabled: 0,
                                    checked: checked as c_int,
                                    cb: tray_handler_callback,
                                    context: null_mut(),
                                    submenu: null(),
                                },
                            });
                        }

                        let c = v.last_mut().unwrap();
                        let mut c_items: Vec<CTrayMenu> = Vec::new();
                        for i in c.items.iter() {
                            c_items.push(i.c_tray_menu.clone());
                        }
                        c_items.push(CTrayMenu {
                            text: null(),
                            wtext: null(),
                            disabled: 0,
                            checked: 0,
                            cb: tray_handler_callback,
                            context: null_mut(),
                            submenu: null(),
                        });
                        c.c_items.replace(Pin::new(c_items.into_boxed_slice()));
                        c.c_tray_menu.submenu = c.c_items.as_ref().unwrap().as_ptr();
                    }
                }
            }
        });
        v
    }

    fn make_menu(menu: Vec<TrayMenuItem>) -> (Vec<CTrayMenuContainer>, Pin<Box<[CTrayMenu]>>) {
        let menu = Self::tray_create_c_structs(menu);
        let mut c_menu_items: Vec<CTrayMenu> = Vec::new();
        for i in menu.iter() {
            c_menu_items.push(i.c_tray_menu.clone());
        }
        c_menu_items.push(CTrayMenu {
            text: null(),
            wtext: null(),
            disabled: 0,
            checked: 0,
            cb: tray_handler_callback,
            context: null_mut(),
            submenu: null(),
        });
        (menu, Pin::from(c_menu_items.into_boxed_slice()))
    }

    pub fn init(icon_path: &str, menu: Vec<TrayMenuItem>) -> Tray {
        let c_icon_path = Pin::new(CString::new(icon_path).unwrap());
        let (menu, c_menu_items) = Self::make_menu(menu);
        let c_tray = CTray {
            icon: c_icon_path.as_ptr(),
            tray_menu: c_menu_items.as_ptr(),
        };
        if unsafe { tray_init(&c_tray as *const CTray) } == 0 {
            Tray {
                current: Mutex::new(menu),
                c_current: Mutex::new(c_menu_items),
                icon_path: Mutex::new(c_icon_path),
                tray_initialized: true,
            }
        } else {
            panic!("tray_init() failed, unable to create system tray!");
        }
    }

    pub fn update(&self, icon_path: Option<&str>, menu: Vec<TrayMenuItem>) {
        let mut ip = self.icon_path.lock().unwrap();
        let mut ip_ptr = ip.as_ptr();
        if icon_path.is_some() {
            let icon_path = icon_path.unwrap();
            if ip.to_str().map_or(false, |s| s != icon_path) {
                *ip = Pin::new(CString::new(icon_path).unwrap());
                ip_ptr = ip.as_ptr();
            }
        }

        let (menu, c_menu_items) = Self::make_menu(menu);
        let c_tray = CTray {
            icon: ip_ptr,
            tray_menu: c_menu_items.as_ptr(),
        };

        unsafe {
            tray_update(&c_tray as *const CTray);
        }

        *self.current.lock().unwrap() = menu;
        *self.c_current.lock().unwrap() = c_menu_items;
    }

    #[inline(always)]
    pub fn poll(&self) -> bool {
        unsafe { tray_loop(1) == 0 }
    }
}

impl Drop for Tray {
    fn drop(&mut self) {
        if self.tray_initialized {
            unsafe {
                tray_exit();
            }
        }
    }
}
