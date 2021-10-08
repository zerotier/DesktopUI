//! Capture preview with the PrintSc key. The preview will be shown in another window.

use native_windows_gui::{self as nwg, ImageDecoder, ImageFrame, ImageSource, Window};
use once_cell::unsync::OnceCell;
use std::cell::Cell;
use std::mem;
use std::rc::Rc;
use webview2::{Controller, Stream};
use winapi::um::winuser::*;

fn show_preview(stream: Stream) {
    use com::ComPtr;
    use std::ptr;

    let mut preview_window = Window::default();

    Window::builder()
        .title("Preview")
        .position((CW_USEDEFAULT, CW_USEDEFAULT))
        .size((1600, 900))
        .build(&mut preview_window)
        .unwrap();

    let source = unsafe {
        let factory = ImageDecoder::new().unwrap();

        // ImageDecoder::from_stream does not really work.
        //
        // And we already have an IStream, so just use
        // `CreateDecoderFromStream`.
        let mut decoder = ptr::null_mut();
        (&*factory.factory).CreateDecoderFromStream(
            ComPtr::from(stream.into_inner()).as_raw() as *const _,
            ptr::null(),
            0,
            &mut decoder as *mut _,
        );

        ImageSource { decoder }
    };

    let bitmap = source.frame(0).unwrap().as_bitmap().unwrap();

    let mut preview_image_frame = ImageFrame::default();
    ImageFrame::builder()
        .size((1600, 900))
        .parent(&preview_window)
        .bitmap(Some(&bitmap))
        .build(&mut preview_image_frame)
        .unwrap();

    let h = preview_window.handle;
    let handles = Cell::new(Some((preview_window, preview_image_frame)));

    // Drop the handles when the window is closed.
    nwg::bind_event_handler(&h, &h, move |e, _, _| {
        if e == nwg::Event::OnWindowClose {
            handles.replace(None);
        }
    });
}

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
        // CW_USEDEFAULT incidentally works, because it's actually i32::MIN, and
        // after saturating mul_div, it's still i32::MIN.
        .position((CW_USEDEFAULT, CW_USEDEFAULT))
        .size((1600, 900))
        .build(&mut window)
        .unwrap();

    let window_handle = window.handle;
    let hwnd = window_handle.hwnd().unwrap();

    let controller: Rc<OnceCell<Controller>> = Rc::new(OnceCell::new());
    let controller_clone = controller.clone();

    let result = webview2::Environment::builder().build(move |env| {
        env.unwrap().create_controller(hwnd, move |c| {
            let c = c.unwrap();

            unsafe {
                let mut rect = mem::zeroed();
                GetClientRect(hwnd, &mut rect);
                c.put_bounds(rect).unwrap();
            }

            let webview = c.get_webview().unwrap();
            webview.navigate("https://wikipedia.org").unwrap();

            c.move_focus(webview2::MoveFocusReason::Programmatic)
                .unwrap();
            c.add_accelerator_key_pressed(move |_, e| {
                let k = e.get_virtual_key()?;
                if k == VK_SNAPSHOT as u32 {
                    let mut stream = webview2::Stream::from_bytes(&[]);
                    webview.capture_preview(
                        webview2::CapturePreviewImageFormat::PNG,
                        stream.clone(),
                        move |r| {
                            use std::io::{Seek, SeekFrom};

                            r?;
                            stream.seek(SeekFrom::Start(0)).unwrap();
                            show_preview(stream);
                            Ok(())
                        },
                    )?;
                    e.put_handled(true)
                } else {
                    Ok(())
                }
            })
            .unwrap();

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

    nwg::bind_raw_event_handler(&window_handle, 0xffff + 1, move |_, msg, _, _| {
        match msg {
            // Always move focus to the webview, or sometimes it won't get
            // accelerator key messages.
            WM_SETFOCUS => {
                if let Some(controller) = controller.get() {
                    controller
                        .move_focus(webview2::MoveFocusReason::Programmatic)
                        .unwrap();
                }
            }
            WM_MOVE => {
                if let Some(controller) = controller.get() {
                    controller.notify_parent_window_position_changed().unwrap();
                }
            }
            WM_SIZE => {
                if let Some(controller) = controller.get() {
                    unsafe {
                        let mut rect = mem::zeroed();
                        GetClientRect(hwnd, &mut rect);
                        controller.put_bounds(rect).unwrap();
                    }
                }
            }
            WM_CLOSE => nwg::stop_thread_dispatch(),
            _ => {}
        }
        None
    })
    .unwrap();

    nwg::dispatch_thread_events();
}
