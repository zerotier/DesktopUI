//! A demo using native-windows-gui for window creation and event handling.

use native_windows_gui::{self as nwg, Window};
use once_cell::unsync::OnceCell;
use std::mem;
use std::rc::Rc;
use webview2::Controller;
use winapi::um::winuser::*;

fn main() {
    // native-windows-gui has some basic high DPI support with the high-dpi
    // feature. It supports the "System DPI Awareness" mode, but not the more
    // advanced Per-Monitor (v2) DPI Awareness modes.
    //
    // Use an application manifest to get rid of this deprecated warning.
    #[allow(deprecated)]
    unsafe {
        nwg::set_dpi_awareness()
    };

    nwg::init().unwrap();

    let mut window = Window::default();

    Window::builder()
        .title("WebView2 - NWG")
        .size((1600, 900))
        .build(&mut window)
        .unwrap();

    let window_handle = window.handle;

    let controller: Rc<OnceCell<Controller>> = Rc::new(OnceCell::new());
    let controller_clone = controller.clone();

    let result = webview2::Environment::builder().build(move |env| {
        env.unwrap()
            .create_controller(window_handle.hwnd().unwrap(), move |c| {
                let c = c.unwrap();

                unsafe {
                    let mut rect = mem::zeroed();
                    GetClientRect(window_handle.hwnd().unwrap(), &mut rect);
                    c.put_bounds(rect).unwrap();
                }

                let webview = c.get_webview().unwrap();
                webview.navigate("https://wikipedia.org").unwrap();

                controller_clone.set(c).unwrap();
                Ok(())
            })
    });
    if let Err(e) = result {
        nwg::modal_fatal_message(
            &window_handle,
            "Failed to Create WebView2 Environment",
            &format!("{}", e),
        );
    }

    let window_handle = window.handle;

    // There lacks an OnWindowRestored event for SC_RESTORE in
    // native-windows-gui, so we use raw events.
    nwg::bind_raw_event_handler(&window_handle, 0xffff + 1, move |_, msg, w, _| {
        match (msg, w as usize) {
            (WM_SIZE, _) => {
                if let Some(controller) = controller.get() {
                    unsafe {
                        let mut rect = mem::zeroed();
                        GetClientRect(window_handle.hwnd().unwrap(), &mut rect);
                        controller.put_bounds(rect).unwrap();
                    }
                }
            }
            (WM_MOVE, _) => {
                if let Some(controller) = controller.get() {
                    controller.notify_parent_window_position_changed().unwrap();
                }
            }
            (WM_SYSCOMMAND, SC_MINIMIZE) => {
                if let Some(controller) = controller.get() {
                    controller.put_is_visible(false).unwrap();
                }
            }
            (WM_SYSCOMMAND, SC_RESTORE) => {
                if let Some(controller) = controller.get() {
                    controller.put_is_visible(true).unwrap();
                }
            }
            (WM_CLOSE, _) => nwg::stop_thread_dispatch(),
            _ => {}
        }
        None
    })
    .unwrap();

    nwg::dispatch_thread_events();
}
