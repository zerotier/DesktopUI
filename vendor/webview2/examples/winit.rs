//! A demo using native-windows-gui for window creation and event handling.

use once_cell::unsync::OnceCell;
use std::mem;
use std::rc::Rc;
use webview2::Controller;
use winapi::shared::windef::*;
use winapi::um::winuser::*;
use winit::dpi::Size;
use winit::event::{Event, WindowEvent};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::platform::windows::WindowExtWindows;
use winit::window::WindowBuilder;

fn main() {
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_title("WebView2 - winit")
        .with_inner_size(Size::Logical((1600, 900).into()))
        .build(&event_loop)
        .unwrap();

    let controller: Rc<OnceCell<Controller>> = Rc::new(OnceCell::new());

    let create_result = {
        let controller_clone = controller.clone();
        let hwnd = window.hwnd() as HWND;

        webview2::Environment::builder().build(move |env| {
            env.expect("env")
                .create_controller(hwnd, move |controller| {
                    let controller = controller.expect("create host");
                    let w = controller.get_webview().expect("get_webview");

                    let _ = w.get_settings().map(|settings| {
                        let _ = settings.put_is_status_bar_enabled(false);
                        let _ = settings.put_are_default_context_menus_enabled(false);
                        let _ = settings.put_is_zoom_control_enabled(false);
                    });

                    unsafe {
                        let mut rect = mem::zeroed();
                        GetClientRect(hwnd, &mut rect);
                        controller.put_bounds(rect).expect("put_bounds");
                    }

                    w.navigate("https://wikipedia.org").expect("navigate");

                    controller_clone.set(controller).unwrap();
                    Ok(())
                })
        })
    };
    if let Err(e) = create_result {
        eprintln!(
            "Failed to create webview environment: {}. Is the new edge browser installed?",
            e
        );
    }

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;

        match event {
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::CloseRequested => {
                    if let Some(webview_host) = controller.get() {
                        webview_host.close().expect("close");
                    }
                    *control_flow = ControlFlow::Exit;
                }
                // Notify the webview when the parent window is moved.
                WindowEvent::Moved(_) => {
                    if let Some(webview_host) = controller.get() {
                        let _ = webview_host.notify_parent_window_position_changed();
                    }
                }
                // Update webview bounds when the parent window is resized.
                WindowEvent::Resized(new_size) => {
                    if let Some(webview_host) = controller.get() {
                        let r = RECT {
                            left: 0,
                            top: 0,
                            right: new_size.width as i32,
                            bottom: new_size.height as i32,
                        };
                        webview_host.put_bounds(r).expect("put_bounds");
                    }
                }
                _ => {}
            },
            Event::MainEventsCleared => {
                // Application update code.

                // Queue a RedrawRequested event.
                window.request_redraw();
            }
            Event::RedrawRequested(_) => {}
            _ => (),
        }
    });
}
