// (c)2021 ZeroTier, Inc.

mod tray;
mod serviceclient;

use serde::{Deserialize, Serialize};
use tray::*;
use std::process::{Child, Command};
use std::sync::{Mutex, Arc, MutexGuard};
use std::time::{SystemTime, Duration};

const CSS_PLACEHOLDER: &'static str = ".XXXthis_is_replaced_by_css_in_the_rust_codeXXX{border:0}";

#[derive(Serialize, Deserialize)]
pub struct CommandFromWebView {
    pub cmd: String
}

#[cfg(target_os = "macos")]
fn is_dark_mode() -> bool {
    let out = Command::new("/usr/bin/defaults").arg("read").arg("-g").arg("AppleInterfaceStyle").output();
    out.map_or(false, |mode| {
        String::from_utf8(mode.stdout.to_ascii_lowercase()).map_or(false, |mode| mode.contains("dark"))
    })
}

#[cfg(target_os = "macos")]
fn tray_icon_name() -> &'static str {
    if is_dark_mode() { "mac-dark.png" } else { "mac-light.png" }
}

#[cfg(all(unix, not(target_os = "macos")))]
fn is_dark_mode() -> bool {
    false
}

#[cfg(all(unix, not(target_os = "macos")))]
fn tray_icon_name() -> &'static str {
    if is_dark_mode() { "generic-dark.png" } else { "generic-light.png" }
}

#[cfg(target_os = "macos")]
extern "C" {
    pub fn c_set_this_thread_to_background_priority();
    pub fn c_set_this_thread_to_foreground_priority();
}

#[cfg(target_os = "macos")]
fn set_thread_to_background_priority() {
    unsafe { c_set_this_thread_to_background_priority(); }
}

#[cfg(not(target_os = "macos"))]
fn set_thread_to_background_priority() {}

#[cfg(target_os = "macos")]
fn set_thread_to_foreground_priority() {
    unsafe { c_set_this_thread_to_foreground_priority(); }
}

#[cfg(not(target_os = "macos"))]
fn set_thread_to_foreground_priority() {}

fn check_window_subprocess_exit(w: &mut MutexGuard<Option<Child>>) {
    if w.is_some() {
        let res = w.as_mut().unwrap().try_wait();
        if res.is_ok() && res.ok().unwrap().is_some() {
            let _ = w.take();
        }
    }
}

fn kill_window_subprocess(mut w: MutexGuard<Option<Child>>) {
    check_window_subprocess_exit(&mut w);
    if w.is_some() {
        let _ = w.as_mut().unwrap().kill();
    }
}

fn open_window_subprocess(mut w: MutexGuard<Option<Child>>, ui_mode: &str, width: i32, height: i32) {
    check_window_subprocess_exit(&mut w);
    if w.is_none() {
        let ch = Command::new(std::env::current_exe().unwrap()).arg(ui_mode).arg(width.to_string()).arg(height.to_string()).spawn();
        if ch.is_ok() {
            let _ = w.replace(ch.unwrap());
        }
    }
}

#[cfg(target_os = "macos")]
#[inline(always)]
fn get_web_ui_blob() -> String {
    let css = if is_dark_mode() { "dark.css" } else { "light.css" };
    let resources_path = std::env::current_exe().unwrap().parent().unwrap().parent().unwrap().join("Resources");
    std::fs::read_to_string(resources_path.join("ui.html")).map_or_else(|_| {
        "<html><body>Error: unable to load ui.html from application bundle Resources.<script>window.zt_ui_render = function(window_type) {}; setTimeout(function() { external.invoke('{ \"cmd\": \"ready\" }'); }, 1);</script></body></html>".into()
    }, |ui| {
        ui.replace(CSS_PLACEHOLDER, std::fs::read_to_string(resources_path.join(css)).unwrap_or(String::new()).as_str())
    })
}

#[cfg(all(unix, not(target_os = "macos")))]
#[inline(always)]
fn get_web_ui_blob() -> String {
    include_str!("../ui/dist/index.html").replace(CSS_PLACEHOLDER, include_str!("../ui/node_modules/@elastic/eui/dist/eui_theme_amsterdam_dark.css"))
}

fn main() {
    if std::env::args().len() > 1 {
        // If there's an argument, enter window mode and pop up a UI. The
        // argument is passed through and is used to determine which UI to
        // display.

        set_thread_to_foreground_priority();
        let ui_mode = std::env::args().nth(1).unwrap_or("null".into());
        let _ = web_view::builder()
            .title("ZeroTier")
            .content(web_view::Content::Html(get_web_ui_blob()))
            .size(
                i32::from_str_radix(std::env::args().nth(2).unwrap_or("".into()).as_str(), 10).unwrap_or(800),
                i32::from_str_radix(std::env::args().nth(3).unwrap_or("".into()).as_str(), 10).unwrap_or(600))
            .resizable(true)
            .visible(false)
            .frameless(false)
            .debug(false)
            .user_data(())
            .invoke_handler(|wv, _arg| {
                let cmd: serde_json::Result<CommandFromWebView> = serde_json::from_str(_arg);
                if cmd.is_err() {
                    return Ok(());
                }
                let cmd = cmd.unwrap();
                match cmd.cmd.as_str() {
                    "ready" => {
                        wv.set_visible(true);
                        let _ = wv.eval(format!("zt_ui_render({});", serde_json::to_string(&ui_mode).unwrap()).as_str());
                    },
                    _ => {},
                }
                Ok(())
            })
            .run()
            .unwrap();

    } else {
        // Without an argument, launch the tray app / UI supervisor.
        set_thread_to_background_priority();

        let mut icon_name = tray_icon_name();
        let mut last_refreshed_tray = SystemTime::UNIX_EPOCH;
        let mut tray: Option<Tray> = None;

        let main_window: Arc<Mutex<Option<Child>>> = Arc::new(Mutex::new(None));
        let join_network_window: Arc<Mutex<Option<Child>>> = Arc::new(Mutex::new(None));
        let about_window: Arc<Mutex<Option<Child>>> = Arc::new(Mutex::new(None));

        loop {
            let now = SystemTime::now();
            if now.duration_since(last_refreshed_tray).map_or(true, |since| since > Duration::from_secs(10)) {
                last_refreshed_tray = now;

                let (main_window2, main_window3) = (main_window.clone(), main_window.clone());
                let (join_network_window2, join_network_window3) = (join_network_window.clone(), join_network_window.clone());
                let (about_window2, about_window3) = (about_window.clone(), about_window.clone());
                let menu = vec![
                    TrayMenuItem::Text {
                        text: "Node ID: ".into(),
                        checked: false,
                        disabled: false,
                        handler: None,
                    },
                    TrayMenuItem::Separator,
                    TrayMenuItem::Text {
                        text: "Join Network... ".into(),
                        checked: false,
                        disabled: false,
                        handler: Some(Box::new(move || {
                            open_window_subprocess(join_network_window2.lock().unwrap(), "JoinNetwork", 400, 100);
                        })),
                    },
                    TrayMenuItem::Text {
                        text: "Control Panel... ".into(),
                        checked: false,
                        disabled: false,
                        handler: Some(Box::new(move || {
                            open_window_subprocess(main_window2.lock().unwrap(), "MainWindow", 1000, 400);
                        })),
                    },
                    TrayMenuItem::Separator,
                    TrayMenuItem::Text {
                        text: "About ".into(),
                        checked: false,
                        disabled: false,
                        handler: Some(Box::new(move || {
                            open_window_subprocess(about_window2.lock().unwrap(), "About", 800, 600);
                        })),
                    },
                    TrayMenuItem::Text {
                        text: "Quit ZeroTier UI ".into(),
                        checked: false,
                        disabled: false,
                        handler: Some(Box::new(move || {
                            kill_window_subprocess(main_window3.lock().unwrap());
                            kill_window_subprocess(join_network_window3.lock().unwrap());
                            kill_window_subprocess(about_window3.lock().unwrap());
                            std::process::exit(0);
                        })),
                    }];

                if tray.is_none() {
                    tray.replace(Tray::init(icon_name, menu));
                } else {
                    let new_icon = tray_icon_name();
                    if new_icon != icon_name {
                        icon_name = new_icon;
                        tray.as_ref().unwrap().update(Some(icon_name), menu);
                    } else {
                        tray.as_ref().unwrap().update(None, menu);
                    }
                    {
                        let mut w = main_window.lock().unwrap();
                        check_window_subprocess_exit(&mut w);
                    }
                    {
                        let mut w = join_network_window.lock().unwrap();
                        check_window_subprocess_exit(&mut w);
                    }
                    {
                        let mut w = about_window.lock().unwrap();
                        check_window_subprocess_exit(&mut w);
                    }
                }
            }

            if !tray.as_ref().map_or(false, |tr| tr.poll()) {
                break;
            }
        }

        kill_window_subprocess(main_window.lock().unwrap());
        kill_window_subprocess(join_network_window.lock().unwrap());
        kill_window_subprocess(about_window.lock().unwrap());
        std::process::exit(0);
    }
}
