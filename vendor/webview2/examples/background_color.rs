//! Example for setting background color.

use once_cell::unsync::OnceCell;
use std::mem;
use std::ptr;
use std::rc::Rc;
use webview2::Controller;
use winapi::{
    shared::minwindef::*, shared::windef::*, um::libloaderapi::GetModuleHandleW, um::winuser::*,
};

fn main() {
    if webview2::get_available_browser_version_string(None).is_err() {
        use std::io::Write;
        use std::os::windows::process::CommandExt;

        // Run a powershell script to install the WebView2 runtime.
        //
        // Use powershell instead of a rust http library like ureq because using
        // the latter makes the executable file a lot bigger (~500KiB).
        let mut p = std::process::Command::new("powershell.exe")
            .arg("-Command")
            .arg("-")
            // Let powershell open its own console window.
            .creation_flags(/*CREATE_NEW_CONSOLE*/ 0x00000010)
            .stdin(std::process::Stdio::piped())
            .spawn()
            .unwrap();
        let mut stdin = p.stdin.take().unwrap();
        stdin
            .write_all(include_bytes!("download-and-run-bootstrapper.ps1"))
            .unwrap();
        drop(stdin);
        let r = p.wait().unwrap();
        if !r.success() {
            return;
        }
    }

    let width = 600;
    let height = 400;

    unsafe {
        // High DPI support.
        SetThreadDpiAwarenessContext(DPI_AWARENESS_CONTEXT_PER_MONITOR_AWARE_V2);
    }

    let controller = Rc::new(OnceCell::<Controller>::new());
    let controller_clone = controller.clone();

    // Window procedure.
    let wnd_proc = move |hwnd, msg, w_param, l_param| match msg {
        WM_SIZE => {
            if let Some(c) = controller.get() {
                let mut r = unsafe { mem::zeroed() };
                unsafe {
                    GetClientRect(hwnd, &mut r);
                }
                c.put_bounds(r).unwrap();
            }
            0
        }
        WM_MOVE => {
            if let Some(c) = controller.get() {
                let _ = c.notify_parent_window_position_changed();
            }
            0
        }
        // Optimization: don't render the webview when the window is minimized.
        WM_SYSCOMMAND if w_param == SC_MINIMIZE => {
            if let Some(c) = controller.get() {
                c.put_is_visible(false).unwrap();
            }
            unsafe { DefWindowProcW(hwnd, msg, w_param, l_param) }
        }
        WM_SYSCOMMAND if w_param == SC_RESTORE => {
            if let Some(c) = controller.get() {
                c.put_is_visible(true).unwrap();
            }
            unsafe { DefWindowProcW(hwnd, msg, w_param, l_param) }
        }
        // High DPI support.
        WM_DPICHANGED => unsafe {
            let rect = *(l_param as *const RECT);
            SetWindowPos(
                hwnd,
                ptr::null_mut(),
                rect.left,
                rect.top,
                rect.right - rect.left,
                rect.bottom - rect.top,
                SWP_NOZORDER | SWP_NOACTIVATE,
            );
            0
        },
        _ => unsafe { DefWindowProcW(hwnd, msg, w_param, l_param) },
    };

    // Register window class. (Standard windows GUI boilerplate).
    let class_name = utf_16_null_terminiated("WebView2 Win32 Class");
    let h_instance = unsafe { GetModuleHandleW(ptr::null()) };
    let class = WNDCLASSW {
        style: CS_HREDRAW | CS_VREDRAW,
        hCursor: unsafe { LoadCursorW(ptr::null_mut(), IDC_ARROW) },
        lpfnWndProc: Some(unsafe { wnd_proc_helper::as_global_wnd_proc(wnd_proc) }),
        lpszClassName: class_name.as_ptr(),
        hInstance: h_instance,
        hbrBackground: (COLOR_WINDOW + 1) as HBRUSH,
        ..unsafe { mem::zeroed() }
    };
    unsafe {
        if RegisterClassW(&class) == 0 {
            message_box(
                ptr::null_mut(),
                &format!("RegisterClassW failed: {}", std::io::Error::last_os_error()),
                "Error",
                MB_ICONERROR | MB_OK,
            );
            return;
        }
    }

    // Create window. (Standard windows GUI boilerplate).
    let window_title = utf_16_null_terminiated("WebView2 Background Color");
    let dpi = unsafe { GetDpiForSystem() } as i32;
    let hwnd = unsafe {
        CreateWindowExW(
            0,
            class_name.as_ptr(),
            window_title.as_ptr(),
            WS_OVERLAPPEDWINDOW,
            CW_USEDEFAULT,
            CW_USEDEFAULT,
            width * dpi / USER_DEFAULT_SCREEN_DPI,
            height * dpi / USER_DEFAULT_SCREEN_DPI,
            ptr::null_mut(),
            ptr::null_mut(),
            h_instance,
            ptr::null_mut(),
        )
    };
    if hwnd.is_null() {
        message_box(
            ptr::null_mut(),
            &format!(
                "CreateWindowExW failed: {}",
                std::io::Error::last_os_error()
            ),
            "Error",
            MB_ICONERROR | MB_OK,
        );
        return;
    }
    unsafe {
        ShowWindow(hwnd, SW_SHOW);
        UpdateWindow(hwnd);
    }

    // Create the webview.
    let r = webview2::Environment::builder().build(move |env| {
        env.unwrap().create_controller(hwnd, move |c| {
            let c = c.unwrap();

            if let Ok(c2) = c.get_controller2() {
                c2.put_default_background_color(webview2_sys::Color {
                    r: 0,
                    g: 255,
                    b: 255,
                    a: 255,
                })
                .unwrap();
            } else {
                eprintln!("failed to get interface to controller2");
            }

            let mut r = unsafe { mem::zeroed() };
            unsafe {
                GetClientRect(hwnd, &mut r);
            }
            c.put_bounds(r).unwrap();

            controller_clone.set(c).unwrap();
            Ok(())
        })
    });
    if let Err(e) = r {
        message_box(
            ptr::null_mut(),
            &format!("Creating WebView2 Environment failed: {}\n", e),
            "Error",
            MB_ICONERROR | MB_OK,
        );
        return;
    }

    // Message loop. (Standard windows GUI boilerplate).
    let mut msg: MSG = unsafe { mem::zeroed() };
    while unsafe { GetMessageW(&mut msg, ptr::null_mut(), 0, 0) } > 0 {
        unsafe {
            TranslateMessage(&msg);
            DispatchMessageW(&msg);
        }
    }
}

fn utf_16_null_terminiated(x: &str) -> Vec<u16> {
    x.encode_utf16().chain(std::iter::once(0)).collect()
}

fn message_box(hwnd: HWND, text: &str, caption: &str, _type: u32) -> i32 {
    let text = utf_16_null_terminiated(text);
    let caption = utf_16_null_terminiated(caption);

    unsafe { MessageBoxW(hwnd, text.as_ptr(), caption.as_ptr(), _type) }
}

mod wnd_proc_helper {
    use super::*;
    use std::cell::UnsafeCell;

    struct UnsafeSyncCell<T> {
        inner: UnsafeCell<T>,
    }

    impl<T> UnsafeSyncCell<T> {
        const fn new(t: T) -> UnsafeSyncCell<T> {
            UnsafeSyncCell {
                inner: UnsafeCell::new(t),
            }
        }
    }

    impl<T: Copy> UnsafeSyncCell<T> {
        unsafe fn get(&self) -> T {
            self.inner.get().read()
        }

        unsafe fn set(&self, v: T) {
            self.inner.get().write(v)
        }
    }

    unsafe impl<T: Copy> Sync for UnsafeSyncCell<T> {}

    static GLOBAL_F: UnsafeSyncCell<usize> = UnsafeSyncCell::new(0);

    /// Use a closure as window procedure.
    ///
    /// The closure will be boxed and stored in a global variable. It will be
    /// released upon WM_DESTROY. (It doesn't get to handle WM_DESTROY.)
    pub unsafe fn as_global_wnd_proc<F: Fn(HWND, UINT, WPARAM, LPARAM) -> isize + 'static>(
        f: F,
    ) -> unsafe extern "system" fn(hwnd: HWND, msg: UINT, w_param: WPARAM, l_param: LPARAM) -> isize
    {
        let f_ptr = Box::into_raw(Box::new(f));
        GLOBAL_F.set(f_ptr as usize);

        unsafe extern "system" fn wnd_proc<F: Fn(HWND, UINT, WPARAM, LPARAM) -> isize + 'static>(
            hwnd: HWND,
            msg: UINT,
            w_param: WPARAM,
            l_param: LPARAM,
        ) -> isize {
            let f_ptr = GLOBAL_F.get() as *mut F;

            if msg == WM_DESTROY {
                Box::from_raw(f_ptr);
                GLOBAL_F.set(0);
                PostQuitMessage(0);
                return 0;
            }

            if !f_ptr.is_null() {
                let f = &*f_ptr;

                f(hwnd, msg, w_param, l_param)
            } else {
                DefWindowProcW(hwnd, msg, w_param, l_param)
            }
        }

        wnd_proc::<F>
    }
}
