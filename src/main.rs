// (c)2021 ZeroTier, Inc.

mod tray;
mod serviceclient;

use serde::{Deserialize, Serialize};
use tray::*;
use std::process::{Child, Command};
use std::sync::{Mutex, Arc, MutexGuard};
use std::time::{SystemTime, Duration};

use crate::serviceclient::ServiceClient;

const CSS_PLACEHOLDER: &'static str = ".XXXthis_is_replaced_by_css_in_the_rust_codeXXX{border:0}";

#[derive(Serialize, Deserialize)]
pub struct CommandFromWebView {
    #[serde(default)]
    pub cmd: String,
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub data: String,
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
        let ch = Command::new(std::env::current_exe().unwrap()).arg("window").arg(ui_mode).arg(width.to_string()).arg(height.to_string()).spawn();
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
    let css = if is_dark_mode() { include_str!("../ui/node_modules/@elastic/eui/dist/eui_theme_amsterdam_dark.css") } else { include_str!("../ui/node_modules/@elastic/eui/dist/eui_theme_amsterdam_light.css") };
    include_str!("../ui/dist/index.html").replace(CSS_PLACEHOLDER, css)
}

fn window(args: &Vec<String>) {
    /*
     * Web UI subprocess
     *
     * ZT app argument 'window' runs the GUI app in webview window mode.
     * The argument itself is passed on to the JS code to determine what
     * UI mode to enter.
     *
     * Once the JS has initialized, it sends back a message with command
     * 'ready'. This shows the window and calls a 'ready' command.
     *
     * The service's JSON API is not queried directly in the web UI thread
     * to prevent UI freezes. Instead it posts from a queue and refreshes
     * continuously from a background thread.
     *
     * Webview data flow to/from Rust must be done via callbacks. For
     * example the 'get' command returns its results via a callback called
     * 'zt_get_callback' defined in JS code. The 'post' command enqueues
     * data to be posted at the next refresh.
     */

    let client = Arc::new(Mutex::new(ServiceClient::new()));

    let thread_client = client.clone();
    let _ = std::thread::spawn(move || {
        set_thread_to_background_priority();

        let mut k = 1_u32;
        loop {
            let mut c = thread_client.lock().unwrap();
            if (k & 63) == 0 { // every ~6.5s
                c.sync_client_config();
            }
            c.do_posts();
            if (k & 3) == 0 { // every ~400ms
                c.sync();
            }
            k += 1;
            std::thread::sleep(Duration::from_millis(100));
        }
    });

    set_thread_to_foreground_priority();

    let ui_client = client.clone();
    let _ = ui_client.lock().map(|mut c| c.sync());
    let _ = web_view::builder()
        .title("ZeroTier")
        .content(web_view::Content::Html(get_web_ui_blob()))
        .size(
            if args.len() >= 5 { i32::from_str_radix(args[3].as_str(), 10).unwrap_or(800) } else { 800 },
            if args.len() >= 5 { i32::from_str_radix(args[4].as_str(), 10).unwrap_or(600) } else { 600 })
        .resizable(true)
        .visible(false)
        .frameless(false)
        .debug(false)
        .user_data(())
        .invoke_handler(move |wv, arg| {
            //println!("{}", arg);
            let _ = serde_json::from_str::<CommandFromWebView>(arg).map(|cmd| {
                match cmd.cmd.as_str() {
                    "ready" => {
                        wv.set_visible(true);
                        let _ = wv.eval(format!("zt_ui_render('{}');", args[2]).as_str());
                    },
                    "get" => {
                        ui_client.lock().unwrap().with(cmd.name.as_str(), |v| {
                            let _ = wv.eval(format!("zt_get_callback('{}', {})", cmd.data, serde_json::to_string(v).unwrap().as_str()).as_str());
                        });
                    },
                    "post" => {
                        ui_client.lock().unwrap().enqueue_post(cmd.name, cmd.data);
                    },
                    "log" => {
                        println!("UI> {}", cmd.data);
                    },
                    "quit" => {
                        wv.exit();
                    },
                    _ => {},
                }
            });
            Ok(())
        })
        .run()
        .unwrap();
}

fn tray() {
    /*
     * System tray process
     *
     * If the ZT app is run with no arguments, it enters system tray mode.
     * This mode runs as a single thread with minimum priority. When the
     * user wants to really do something, it launches another instance of
     * this binary in webview mode.
     */

    set_thread_to_background_priority();

    let mut icon_name = tray_icon_name();
    let mut last_refreshed_tray = SystemTime::UNIX_EPOCH;
    let mut tray: Option<Tray> = None;
    let mut client = ServiceClient::new();

    let main_window: Arc<Mutex<Option<Child>>> = Arc::new(Mutex::new(None));
    let join_network_window: Arc<Mutex<Option<Child>>> = Arc::new(Mutex::new(None));
    let about_window: Arc<Mutex<Option<Child>>> = Arc::new(Mutex::new(None));

    let mut k = 0_u32;
    loop {
        let now = SystemTime::now();
        if now.duration_since(last_refreshed_tray).map_or(true, |since| since > Duration::from_secs(10)) {
            last_refreshed_tray = now;

            if (k & 7) == 0 {
                client.sync_client_config();
            }
            k += 1;
            client.sync();

            let (main_window2, main_window3) = (main_window.clone(), main_window.clone());
            let (join_network_window2, join_network_window3) = (join_network_window.clone(), join_network_window.clone());
            let (about_window2, about_window3) = (about_window.clone(), about_window.clone());

            let mut menu: Vec<TrayMenuItem> = Vec::new();
            if client.is_online() {
                menu.push(TrayMenuItem::Text {
                    text: format!("Node ID: {} ", client.get_str("status/address")),
                    checked: false,
                    disabled: false,
                    handler: None,
                });
                menu.push(TrayMenuItem::Separator);
                menu.push(TrayMenuItem::Text {
                    text: "Join Network... ".into(),
                    checked: false,
                    disabled: false,
                    handler: Some(Box::new(move || {
                        open_window_subprocess(join_network_window2.lock().unwrap(), "Join", 400, 100);
                    })),
                });
                menu.push(TrayMenuItem::Text {
                    text: "Control Panel... ".into(),
                    checked: false,
                    disabled: false,
                    handler: Some(Box::new(move || {
                        open_window_subprocess(main_window2.lock().unwrap(), "Main", 1000, 400);
                    })),
                });
                let networks = client.networks();
                if !networks.is_empty() {
                    menu.push(TrayMenuItem::Separator);
                    networks.iter().for_each(|network| {
                        menu.push(TrayMenuItem::Text {
                            text: format!("{} ({})", (*network).0, (*network).1),
                            checked: false,
                            disabled: false,
                            handler: None,
                        });
                    })
                }
                menu.push(TrayMenuItem::Separator);
                menu.push(TrayMenuItem::Text {
                    text: "About ".into(),
                    checked: false,
                    disabled: false,
                    handler: Some(Box::new(move || {
                        open_window_subprocess(about_window2.lock().unwrap(), "About", 800, 600);
                    }))
                });
                menu.push(TrayMenuItem::Text {
                    text: "Quit ZeroTier UI ".into(),
                    checked: false,
                    disabled: false,
                    handler: Some(Box::new(move || {
                        for w in [&main_window3, &join_network_window3, &about_window3].iter() {
                            kill_window_subprocess(w.lock().unwrap());
                        }
                        std::process::exit(0);
                    }))
                });
            } else {
                menu.push(TrayMenuItem::Text {
                    text: "Service not running.".into(),
                    checked: false,
                    disabled: true,
                    handler: None,
                });
            }

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

                for w in [&main_window, &join_network_window, &about_window].iter() {
                    check_window_subprocess_exit(w.lock().as_mut().unwrap());
                }
            }
        }

        if !tray.as_ref().map_or(false, |tr| tr.poll()) {
            break;
        }
    }

    for w in [&main_window, &join_network_window, &about_window].iter() {
        kill_window_subprocess(w.lock().unwrap());
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() >= 2 {
        match args[1].as_str() {
            "window" => {
                if args.len() >= 3 {
                    window(&args);
                } else {
                    println!("FATAL: window requires arguments: ui_mode [width hint] [height hint]");
                }
            },
            _ => println!("FATAL: unrecognized mode: {}", args[1])
        }
    } else {
        tray();
    }
    std::process::exit(0);
}
