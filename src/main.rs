/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 *
 * (c)2021 ZeroTier, Inc.
 * https://www.zerotier.com/
 */

// Without this Windows will pop up a cmd.exe window when this is launched.
#![windows_subsystem = "windows"]

use std::cmp::Ordering;
#[allow(unused)]
use std::collections::{HashMap, LinkedList};
#[allow(unused)]
use std::ffi::CString;
#[allow(unused)]
use std::io::{Read, Write};
#[allow(unused)]
use std::os::raw::{c_char, c_int, c_uint};
use std::path::Path;
#[allow(unused)]
use std::process::{Child, Command, Stdio};
use std::sync::Arc;
use std::sync::atomic::*;
use std::time::{Duration, SystemTime, Instant};

use parking_lot::Mutex;
use serde_json::Value;

use crate::serviceclient::*;
use crate::tray::*;

mod tray;
mod serviceclient;

/// The string in the HTML blob to replace with the right CSS for this platform and light/dark mode.
/// It's a bit weird so web app bundlers don't optimize it out.
const CSS_PLACEHOLDER: &'static str = ".XXXthis_is_replaced_by_css_in_the_rust_codeXXX{border:0}";

const MAIN_WINDOW_WIDTH: i32 = 1350;
const MAIN_WINDOW_HEIGHT: i32 = 600;

pub(crate) static mut APPLICATION_PATH: String = String::new();
pub(crate) static mut APPLICATION_HOME: String = String::new();
pub(crate) static mut NETWORK_CACHE_PATH: String = String::new();
pub(crate) static mut START_ON_LOGIN: bool = false;

#[cfg(target_os = "macos")]
pub(crate) const GLOBAL_SERVICE_HOME_V2: &'static str = "/Library/Application Support/ZeroTier";

#[cfg(windows)]
pub(crate) const GLOBAL_SERVICE_HOME_V2: &'static str = "\\ProgramData\\ZeroTier";

#[cfg(any(target_os = "freebsd", target_os = "dragonfly", target_os = "openbsd", target_os = "netbsd"))]
pub(crate) const GLOBAL_SERVICE_HOME_V2: &'static str = "/var/db/zerotier";

#[cfg(target_os = "linux")]
pub(crate) const GLOBAL_SERVICE_HOME_V2: &'static str = "/var/lib/zerotier";

#[cfg(target_os = "macos")]
pub(crate) const GLOBAL_SERVICE_HOME_V1: &'static str = "/Library/Application Support/ZeroTier/One";

#[cfg(windows)]
pub(crate) const GLOBAL_SERVICE_HOME_V1: &'static str = "\\ProgramData\\ZeroTier\\One";

#[cfg(any(target_os = "freebsd", target_os = "dragonfly", target_os = "openbsd", target_os = "netbsd"))]
pub(crate) const GLOBAL_SERVICE_HOME_V1: &'static str = "/var/db/zerotier-one";

#[cfg(target_os = "linux")]
pub(crate) const GLOBAL_SERVICE_HOME_V1: &'static str = "/var/lib/zerotier-one";

/*******************************************************************************************************************/
/* OS-specific functions and C externs */

#[cfg(windows)]
extern "C" {
    pub fn c_windows_is_dark_theme() -> c_int;
    pub fn c_windows_post_to_clipboard(data: *const c_char);
    pub fn c_windows_get_from_clipboard(buf: *mut c_char) -> c_uint;
}

#[cfg(target_os = "macos")]
extern "C" {
    pub fn c_set_this_thread_to_background_priority();
    pub fn c_set_this_thread_to_foreground_priority();
}

extern "C" {
    pub fn c_lock_down_file(path: *const c_char, is_dir: c_int);
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

/// Quick and dirty recursive parser for decoded plist files from the old GUI on Mac... this is just for transition.
#[cfg(target_os = "macos")]
fn parse_mac_network_plist(base_array: &Vec<plist::Value>, data: &plist::Value, networks: &mut HashMap<String, String>, nwid: &mut Option<u64>, name: &mut Option<String>) {
    if nwid.as_ref().map_or(false, |x| *x != 0) && name.as_ref().map_or(false, |x| !x.eq("\0\0\0\0")) {
        networks.insert(format!("{:0>16x}", nwid.take().unwrap()), name.take().unwrap());
    }
    match data {
        plist::Value::Array(arr) => {
            for v in arr.iter() {
                parse_mac_network_plist(base_array, v, networks, nwid, name);
            }
        },
        plist::Value::Dictionary(dict) => {
            for kv in dict.iter() {
                if kv.0.eq("nwid") {
                    nwid.replace(0);
                    parse_mac_network_plist(base_array, kv.1, networks, nwid, name);
                } else if kv.0.eq("name") {
                    name.replace("\0\0\0\0".into());
                    parse_mac_network_plist(base_array, kv.1, networks, nwid, name);
                }
            }
        },
        plist::Value::String(s) => {
            if name.as_ref().map_or(false, |x| x.eq("\0\0\0\0")) {
                name.replace(s.clone());
            }
        },
        plist::Value::Integer(i) => {
            if nwid.as_ref().map_or(false, |x| *x == 0) {
                nwid.replace(i.as_unsigned().unwrap_or(0));
            }
        },
        plist::Value::Uid(uid) => {
            let _ = base_array.get(uid.get() as usize).map(|v| parse_mac_network_plist(base_array, v, networks, nwid, name));
        },
        _ => {}
    }
}

/*******************************************************************************************************************/
/* Refresh the state of START_ON_LOGIN */

#[cfg(target_os = "macos")]
fn refresh_mac_start_on_login() {
    // osascript -e 'tell application "System Events" to get the name of every login item'
    let out = Command::new("/usr/bin/osascript").arg("-e").arg("tell application \"System Events\" to get the name of every login item").output();
    unsafe {
        START_ON_LOGIN = out.map_or(false, |app_list| String::from_utf8(app_list.stdout.to_ascii_lowercase()).map_or(false, |app_list| app_list.contains("zerotier")));
    }
}

#[cfg(target_os = "windows")]
fn refresh_windows_start_on_login() {
    let hkcu = winreg::RegKey::predef(winreg::enums::HKEY_CURRENT_USER);
    let startup = hkcu.open_subkey("SOFTWARE\\Microsoft\\Windows\\CurrentVersion\\Run");
    let enabled = startup.map_or(false, |startup| startup.get_value::<String, &str>("ZeroTierUI").is_ok());
    unsafe {
        START_ON_LOGIN = enabled;
    }
}

/*******************************************************************************************************************/
/* Check if we are in dark mode */

#[cfg(target_os = "macos")]
fn is_dark_mode() -> bool {
    Command::new("/usr/bin/defaults").arg("read").arg("-g").arg("AppleInterfaceStyle").output().map_or(false, |mode| String::from_utf8(mode.stdout.to_ascii_lowercase()).map_or(false, |mode| mode.contains("dark")))
}

#[cfg(all(unix, not(target_os = "macos")))]
fn is_dark_mode() -> bool {
    false
}

#[cfg(windows)]
fn is_dark_mode() -> bool {
    unsafe { c_windows_is_dark_theme() != 0 }
}

/*******************************************************************************************************************/
/* Get the tray icon path for the current theme / color scheme / OS */

#[cfg(target_os = "macos")]
fn tray_icon_name() -> &'static str {
    "trayIconTemplate.pdf"
} // must be in resources folder of app bundle

#[cfg(not(target_os = "macos"))]
fn tray_icon_name() -> String {
    let icon_path = std::env::temp_dir().join("zerotier-tray-icon.ico");
    if std::fs::metadata(&icon_path).is_err() {
        #[cfg(windows)] {
            let _ = std::fs::write(&icon_path, include_bytes!("../icon.ico"));
        }
        #[cfg(not(windows))] {
            let _ = std::fs::write(&icon_path, include_bytes!("../icon.png"));
        }
    }
    icon_path.to_str().unwrap().into()
}

/*******************************************************************************************************************/
/* Clipboard functions for different OSes */

#[cfg(target_os = "macos")]
fn copy_to_clipboard(s: &str) {
    let _ = Command::new("/usr/bin/pbcopy").stdin(Stdio::piped()).stdout(Stdio::inherit()).stderr(Stdio::inherit()).spawn().map(|mut c| {
        c.stdin.take().map(|mut stdin| {
            let _ = stdin.write_all(s.as_bytes());
        });
        let _ = c.wait();
    });
}

#[cfg(target_os = "macos")]
fn read_from_clipboard() -> String {
    Command::new("/usr/bin/pbpaste").output().map_or_else(|_| String::new(), |out| String::from_utf8(out.stdout).map_or_else(|_| String::new(), |s| s))
}

#[cfg(windows)]
fn copy_to_clipboard(s: &str) {
    let _ = CString::new(s).map(|s| {
        unsafe { c_windows_post_to_clipboard(s.as_ptr()) };
    });
}

#[cfg(windows)]
fn read_from_clipboard() -> String {
    let mut buf = [0_u8; 1024];
    unsafe {
        if c_windows_get_from_clipboard(buf.as_mut_ptr().cast()) > 0 {
            return CString::from_raw(buf.as_mut_ptr().cast()).to_str().map_or_else(|_| String::new(), |s| String::from(s));
        }
    }
    String::new()
}

/*******************************************************************************************************************/
/* Get the web UI HTML/CSS/JS blob for the right color scheme */

#[cfg(target_os = "macos")]
#[inline(always)]
fn get_web_ui_blob(dark: bool) -> String {
    let css = if dark { "dark.css" } else { "light.css" };
    let resources_path = std::env::current_exe().unwrap().parent().unwrap().parent().unwrap().join("Resources");
    std::fs::read_to_string(resources_path.join("ui.html")).map_or_else(|_| {
        "<html><body>Error: unable to load ui.html from application bundle Resources.<script>window.zt_ui_render = function(window_type) {}; setTimeout(function() { external.invoke('{ \"cmd\": \"ready\" }'); }, 1);</script></body></html>".into()
    }, |ui| {
        ui.replace(CSS_PLACEHOLDER, std::fs::read_to_string(resources_path.join(css)).unwrap_or(String::new()).as_str())
    })
}

#[cfg(not(target_os = "macos"))]
#[inline(always)]
fn get_web_ui_blob(dark: bool) -> String {
    let css = if dark { include_str!("../ui/dist/dark.css") } else { include_str!("../ui/dist/light.css") };
    include_str!("../ui/dist/index.html").replace(CSS_PLACEHOLDER, css)
}

/*******************************************************************************************************************/

/// Start the service client background thread, returning the client and a flag set when the data changes.
fn start_client(refresh_base_paths: Vec<&'static str>, tick_period_ms: u64, refresh_period_ticks: u64) -> (Arc<Mutex<ServiceClient>>, Arc<AtomicBool>) {
    let (mut client, dirty_flag) = ServiceClient::new(refresh_base_paths);
    client.sync();

    let client = Arc::new(Mutex::new(client));

    let thread_client = client.clone();
    let _ = std::thread::spawn(move || {
        set_thread_to_background_priority();
        let mut k = 0_u64;
        loop {
            if thread_client.lock().do_posts() {
                k = refresh_period_ticks - 1;
            }
            k += 1;
            if k >= refresh_period_ticks {
                k = 0;
                thread_client.lock().sync();
            }
            std::thread::sleep(Duration::from_millis(tick_period_ms));
        }
    });

    (client, dirty_flag)
}

/// Returns true if a process has exited, and also replaces the supplied Option with None.
fn did_process_exit(w: &mut Option<Child>) -> bool {
    if w.is_some() {
        let res = w.as_mut().unwrap().try_wait();
        if res.is_ok() && res.ok().unwrap().is_some() {
            let _ = w.take();
            true
        } else {
            false
        }
    } else {
        true
    }
}

/// Kill a subproces and wait for zombie to be collected (on Unix systems).
fn kill_process(w: &mut Option<Child>) {
    did_process_exit(w);
    w.as_mut().map(|w| {
        let _ = w.kill();
        let _ = w.wait();
    });
}

/// Create a background thread that listens for 'r' from STDIN and sets a boolean flag if received.
fn create_raise_window_listener_thread() -> Arc<AtomicBool> {
    let raise_window = Arc::new(AtomicBool::new(false));
    let raise_window2 = raise_window.clone();
    let _ = std::thread::spawn(move || {
        set_thread_to_background_priority();
        loop {
            let mut buf: [u8; 1] = [0_u8];
            let _ = std::io::stdin().read_exact(&mut buf);
            if buf[0] == ('r' as u8) {
                raise_window2.store(true, std::sync::atomic::Ordering::Relaxed);
            }
        }
    });
    raise_window
}

/// Opens SSO authentication window, which in turn runs sso_auth_window_main().
fn open_sso_auth_window_subprocess(w: &mut Option<Child>, width: i32, height: i32, param: &[&str]) {
    did_process_exit(w);
    if w.is_none() {
        let ch = Command::new(std::env::current_exe().unwrap()).arg("auth").arg(width.to_string()).arg(height.to_string()).args(param).stdin(Stdio::piped()).spawn();
        if ch.is_ok() {
            let _ = w.replace(ch.unwrap());
        }
    }
}

#[cfg(target_os = "macos")]
fn sso_auth_window_main(args: &Vec<String>) {
    let raise_window = create_raise_window_listener_thread();
    let title = format!("Remote Network Login: {}", args[4].as_str());
    let mut wv = web_view::builder()
        .title(title.as_str())
        .content(web_view::Content::Url(args[5].as_str()))
        .size(i32::from_str_radix(args[2].as_str(), 10).unwrap_or(1024), i32::from_str_radix(args[3].as_str(), 10).unwrap_or(768))
        .visible(false)
        .frameless(false)
        .hide_instead_of_close(false)
        .debug(false)
        .user_data(())
        .invoke_handler(move |_, _| Ok(()))
        .build()
        .unwrap();
    loop {
        if wv.step().is_none() {
            break;
        }
        if raise_window.load(std::sync::atomic::Ordering::Relaxed) {
            wv.set_visible(true);
        }
    }
}

/// Main function for SSO authentication webview popup windows.
#[cfg(not(target_os = "macos"))]
#[allow(unused_mut)]
fn sso_auth_window_main(args: &Vec<String>) {
    let raise_window = create_raise_window_listener_thread();
    set_thread_to_background_priority();
    let mut event_loop = EventLoop::new();
    let window = wry::window::WindowBuilder::new()
        .with_visible(false)
        .with_title(format!("Remote Network Login: {}", args[4].as_str()))
        .with_inner_size(wry::application::dpi::LogicalSize::new(i32::from_str_radix(args[2].as_str(), 10).unwrap_or(1024), i32::from_str_radix(args[3].as_str(), 10).unwrap_or(768)))
        .with_resizable(true)
        .build(&event_loop).unwrap();
    let webview = wry::WebViewBuilder::new(window).unwrap()
        .with_url(args[5].as_str()).unwrap()
        .build().unwrap();
    event_loop.run(move |event, _, control_flow| {
        *control_flow = wry::application::event_loop::ControlFlow::WaitUntil(Instant::now() + Duration::from_secs(1));
        match event {
            wry::application::event::Event::WindowEvent {event: wry::application::event::WindowEvent::CloseRequested, ..} => *control_flow = wry::application::event_loop::ControlFlow::Exit,
            _ => {}
        }
        if raise_window.load(std::sync::atomic::Ordering::Relaxed) {
            set_thread_to_foreground_priority();
            webview.window().set_visible(true);
            webview.window().set_focus();
        }
    });
}

/// Opens a UI window subprocess, which in turn runs control_panel_window_main().
fn open_ui_window_subprocess(w: &mut Option<Child>, ui_mode: &str, width: i32, height: i32) {
    did_process_exit(w);
    if w.is_none() {
        let ch = Command::new(std::env::current_exe().unwrap()).arg("window").arg(ui_mode).arg(width.to_string()).arg(height.to_string()).stdin(Stdio::piped()).spawn();
        if ch.is_ok() {
            let _ = w.replace(ch.unwrap());
        }
    } else {
        // Sending 'r' causes subprocess to raise the window at the first opportunity.
        let _ = w.as_mut().unwrap().stdin.as_mut().unwrap().write_all(&[b'r']);
    }
}

/// Main function for control panel webview windows.
#[allow(unused_mut)]
fn control_panel_window_main(args: &Vec<String>) {
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

    set_thread_to_foreground_priority();

    let (client, dirty_flag) = start_client(vec!["status", "network", "peer"], 100, 5);
    let raise_window = create_raise_window_listener_thread();

    let mut event_loop = EventLoop::new();
    let window = wry::window::WindowBuilder::new()
        .with_title("ZeroTier Control Panel")
        .with_inner_size(wry::dpi::LogicalSize::new(i32::from_str_radix(args[3].as_str(), 10).unwrap_or(1024), i32::from_str_radix(args[4].as_str(), 10).unwrap_or(768)))
        .with_resizable(true)
        .with_visible(true)
        .build(&event_loop)
        .unwrap();

    let ui_mode = args[2].clone();
    let ui_client = client.clone();

    let webview = wry::WebViewBuilder::new(window).unwrap()
        .with_html(get_web_ui_blob(is_dark_mode())).unwrap()
        .with_rpc_handler(move |window: &wry::window::Window, req: wry::webview::RpcRequest| -> Option<wry::webview::RpcResponse> {
            let arg = req.params.map_or(Value::Null, |p| p.as_array().map_or(Value::Null, |p| {
                if p.is_empty() {
                    Value::Null
                } else {
                    p.first().unwrap().clone()
                }
            }));
            match req.method.as_str() {
                "ready" => {
                    Some(wry::webview::RpcResponse::new_result(req.id.clone(), Some(Value::from(ui_mode.as_str()))))
                },
                "log" => {
                    println!("> {}", arg.to_string());
                    None
                }
                "post" => {
                    let _ = arg.as_array().map(|p| {
                        if p.len() == 2 {
                            let path = p.get(0).map_or("", |s| s.as_str().unwrap_or(""));
                            let data = p.get(1).map_or("", |s| s.as_str().unwrap_or(""));
                            if !path.is_empty() && !data.is_empty() {
                                ui_client.lock().enqueue_post(path.to_string(), data.to_string());
                            }
                        }
                    });
                    None
                },
                "delete" => {
                    arg.as_str().map(|path| ui_client.lock().enqueue_delete(path.to_string()));
                    None
                },
                "remember_network" => {
                    let _ = arg.as_array().map(|p| {
                        if p.len() == 3 {
                            let nwid = p.get(0).map_or("", |s| s.as_str().unwrap_or(""));
                            let name = p.get(1).map_or("", |s| s.as_str().unwrap_or(""));
                            let settings = p.get(2).map_or("", |s| s.as_str().unwrap_or(""));
                            ui_client.lock().remember_network(nwid.to_string(), name.to_string(), settings.to_string());
                        }
                    });
                    None
                },
                "forget_network" => {
                    arg.as_str().map(|path| ui_client.lock().forget_network(&path.to_string()));
                    None
                },
                "copy_to_clipboard" => {
                    arg.as_str().map(|path| copy_to_clipboard(path));
                    None
                },
                "paste_from_clipboard" => {
                    Some(wry::webview::RpcResponse::new_result(req.id.clone(), Some(Value::from(read_from_clipboard()))))
                },
                "raise" => {
                    window.set_visible(true);
                    window.set_focus();
                    None
                },
                "poll" => {
                    if dirty_flag.swap(false, std::sync::atomic::Ordering::Relaxed) {
                        Some(wry::webview::RpcResponse::new_result(req.id.clone(), Some(Value::from(ui_client.lock().get_all_json()))))
                    } else {
                        Some(wry::webview::RpcResponse::new_result(req.id.clone(), Some(Value::Null)))
                    }
                },
                "quit" => {
                    std::process::exit(0);
                }
                _ => None
            }
        })
        .build()
        .unwrap();

    event_loop.run(move |event, _, control_flow| {
        *control_flow = wry::event_loop::ControlFlow::WaitUntil(Instant::now() + Duration::from_secs(1));
        match event {
            wry::event::Event::WindowEvent {event: wry::application::event::WindowEvent::CloseRequested, ..} => *control_flow = wry::event_loop::ControlFlow::Exit,
            _ => {}
        }
        if raise_window.load(std::sync::atomic::Ordering::Relaxed) {
            webview.window().set_visible(true);
            webview.window().set_focus();
        }
    });
}

#[cfg(windows)]
fn notify(text: &str) {
    let ico: String = tray_icon_name();
    let _ = notify_rust::Notification::new().icon(ico.as_str()).body(text).appname("ZeroTier").show();
}

#[cfg(not(windows))]
fn notify(text: &str) {
    let _ = notify_rust::Notification::new().body(text).appname("ZeroTier").show();
}

/// System tray main function.
fn tray_main() {
    /*
     * System tray process
     *
     * If the ZT app is run with no arguments, it enters system tray mode.
     * This mode runs as a single thread with minimum priority. When the
     * user wants to really do something, it launches another instance of
     * this binary in webview mode.
     */

    /*
    let single = single_instance::SingleInstance::new(std::env::temp_dir().join("ZeroTierUI_InstanceLock").to_str().unwrap());
    if single.is_err() || !single.unwrap().is_single() {
        println!("FATAL: another instance of the ZeroTier UI is already running.");
        std::process::exit(1);
    }
    */

    // On Apple Silicon Macs this tells the OS to use efficiency cores for the tray app. Perceptible performance is still great.
    // Don't do it on Intel Macs because it seems to make stuff really slow with little power use benefit. For the genuine background
    // threads we always do it because if those a bit slow it's fine.
    #[cfg(all(target_arch = "aarch64", target_os = "macos"))]
    set_thread_to_background_priority();

    // Automatically populated with a path to the right icon for e.g. light/dark mode.
    let mut icon_name = tray_icon_name();

    // Set to true from anywhere to cause app to exit on next event loop.
    let exit_flag = Arc::new(AtomicBool::new(false));

    let (client, dirty_flag) = start_client(vec!["status", "network"], 250, 10);

    let main_window: Arc<Mutex<Option<Child>>> = Arc::new(Mutex::new(None));
    let about_window: Arc<Mutex<Option<Child>>> = Arc::new(Mutex::new(None));
    let auth_windows: Arc<Mutex<HashMap<String, (String, Mutex<Option<Child>>)>>> = Arc::new(Mutex::new(HashMap::new()));

    // This closure builds a new menu for display in the app icon.
    let refresh = || {
        let mut menu: Vec<TrayMenuItem> = Vec::new();
        if client.lock().is_online() {
            let address = client.lock().get_str(&["status", "address"]);
            let address2 = address.clone();
            menu.push(TrayMenuItem::Text {
                text: format!("My Address:  {} ", address),
                checked: false,
                disabled: false,
                handler: Some(Box::new(move || {
                    copy_to_clipboard(address2.as_str());
                    notify("Copied this node's ZeroTier address to clipboard.");
                })),
            });

            menu.push(TrayMenuItem::Separator);

            let main_window2 = main_window.clone();
            menu.push(TrayMenuItem::Text {
                text: "Open Control Panel... ".into(),
                checked: false,
                disabled: false,
                handler: Some(Box::new(move || open_ui_window_subprocess(&mut *main_window2.lock(), "Main", MAIN_WINDOW_WIDTH, MAIN_WINDOW_HEIGHT)))
            });

            let networks = client.lock().networks();
            if !networks.is_empty() {
                menu.push(TrayMenuItem::Separator);
                for network in networks.iter() {
                    let nw_obj = &((*network).1);

                    let assigned_addrs = nw_obj.get("assignedAddresses").unwrap_or(&Value::Null);
                    let managed_routes = nw_obj.get("routes").unwrap_or(&Value::Null);
                    let device_name = nw_obj.get("portDeviceName").map_or("", |s| s.as_str().unwrap_or(""));
                    let allow_managed_addresses = nw_obj.get("allowManaged").map_or(false, |b| b.as_bool().unwrap_or(false));
                    let allow_global_ips = nw_obj.get("allowGlobal").map_or(false, |b| b.as_bool().unwrap_or(false));
                    let allow_default = nw_obj.get("allowDefault").map_or(false, |b| b.as_bool().unwrap_or(false));
                    let allow_dns = nw_obj.get("allowDNS").map_or(false, |b| b.as_bool().unwrap_or(false));
                    let status = nw_obj.get("status").map_or("REQUESTING_CONFIGURATION", |s| s.as_str().unwrap_or("REQUESTING_CONFIGURATION"));

                    let mut network_menu: Vec<TrayMenuItem> = Vec::new();

                    let nwid = (*network).0.clone();
                    network_menu.push(TrayMenuItem::Text {
                        text: format!("Network ID:\t  {}", (*network).0),
                        checked: false,
                        disabled: false,
                        handler: Some(Box::new(move || {
                            copy_to_clipboard(nwid.as_str());
                            notify("Copied network ID to clipboard.");
                        })),
                    });

                    network_menu.push(TrayMenuItem::Separator);

                    let (nwid, client2) = ((*network).0.clone(), client.clone());
                    network_menu.push(TrayMenuItem::Text {
                        text: format!("Allow Managed Addresses"),
                        checked: allow_managed_addresses,
                        disabled: false,
                        handler: Some(Box::new(move || client2.lock().enqueue_post(format!("network/{}", nwid), format!("{{ \"allowManaged\": {} }}", !allow_managed_addresses)))),
                    });
                    let (nwid, client2) = ((*network).0.clone(), client.clone());
                    network_menu.push(TrayMenuItem::Text {
                        text: format!("Allow Assignment of Global IPs"),
                        checked: allow_global_ips,
                        disabled: false,
                        handler: Some(Box::new(move || client2.lock().enqueue_post(format!("network/{}", nwid), format!("{{ \"allowGlobal\": {} }}", !allow_global_ips)))),
                    });
                    let (nwid, client2) = ((*network).0.clone(), client.clone());
                    network_menu.push(TrayMenuItem::Text {
                        text: format!("Allow Default Router Override"),
                        checked: allow_default,
                        disabled: false,
                        handler: Some(Box::new(move || client2.lock().enqueue_post(format!("network/{}", nwid), format!("{{ \"allowDefault\": {} }}", !allow_default)))),
                    });
                    let (nwid, client2) = ((*network).0.clone(), client.clone());
                    network_menu.push(TrayMenuItem::Text {
                        text: format!("Allow DNS Configuration"),
                        checked: allow_dns,
                        disabled: false,
                        handler: Some(Box::new(move || client2.lock().enqueue_post(format!("network/{}", nwid), format!("{{ \"allowDNS\": {} }}", !allow_dns)))),
                    });

                    network_menu.push(TrayMenuItem::Separator);

                    nw_obj.get("mac").map(|a| a.as_str().map(|a| {
                        network_menu.push(TrayMenuItem::Text {
                            text: format!("Ethernet:\t\t\t  {}", a),
                            checked: false,
                            disabled: true,
                            handler: None,
                        });
                    }));
                    if !device_name.is_empty() {
                        network_menu.push(TrayMenuItem::Text {
                            text: format!("Device:\t\t\t  {}", device_name),
                            checked: false,
                            disabled: true,
                            handler: None,
                        });
                    }
                    nw_obj.get("type").map(|a| a.as_str().map(|a| {
                        network_menu.push(TrayMenuItem::Text {
                            text: format!("Type:\t\t\t  {}", a),
                            checked: false,
                            disabled: true,
                            handler: None,
                        });
                    }));
                    network_menu.push(TrayMenuItem::Text {
                        text: format!("Status:\t\t\t  {}", status),
                        checked: false,
                        disabled: true,
                        handler: None,
                    });
                    if status == "OK" {
                        nw_obj.get("authenticationExpiryTime").map(|auth_exp_time| {
                            auth_exp_time.as_i64().map(|auth_exp_time| {
                                let auth_exp_time = auth_exp_time;
                                if auth_exp_time > 0 {
                                    let auth_exp_time = SystemTime::UNIX_EPOCH.checked_add(std::time::Duration::from_millis(auth_exp_time as u64)).unwrap();
                                    let auth_exp_time = chrono::DateTime::<chrono::Local>::from(auth_exp_time);
                                    network_menu.push(TrayMenuItem::Text {
                                        text: format!("Auth Expire:\t\t  {}", auth_exp_time.format("%Y-%m-%d %H:%M:%S").to_string()),
                                        checked: false,
                                        disabled: true,
                                        handler: None,
                                    });
                                }
                            })
                        });
                    }

                    let mut added_managed_separator = false;
                    assigned_addrs.as_array().map(|assigned_addrs| {
                        if !assigned_addrs.is_empty() {
                            if !added_managed_separator {
                                added_managed_separator = true;
                                network_menu.push(TrayMenuItem::Separator);
                            }
                            let mut assigned_addrs_menu: Vec<TrayMenuItem> = Vec::new();
                            for a in assigned_addrs.iter() {
                                a.as_str().map(|a| {
                                    let a_copy = String::from(a);
                                    assigned_addrs_menu.push(TrayMenuItem::Text {
                                        text: String::from(a),
                                        checked: false,
                                        disabled: false,
                                        handler: Some(Box::new(move || {
                                            copy_to_clipboard(a_copy.split_once('/').map_or(a_copy.as_str(), |a| a.0));
                                            notify("Copied address to clipboard.");
                                        })),
                                    });
                                });
                            }
                            network_menu.push(TrayMenuItem::Submenu {
                                text: "Managed Addresses ".into(),
                                checked: false,
                                items: assigned_addrs_menu,
                            });
                        }
                    });

                    let mut managed_routes2: Vec<(String, String)> = Vec::new();
                    managed_routes.as_array().map(|managed_routes| {
                        if !managed_routes.is_empty() {
                            if !added_managed_separator {
                                added_managed_separator = true;
                                network_menu.push(TrayMenuItem::Separator);
                            }
                            for r in managed_routes.iter() {
                                r.as_object().map(|r| {
                                    let via = r.get("via").map_or("", |s| s.as_str().unwrap_or(""));
                                    r.get("target").map(|target| target.as_str().map(|target| {
                                        managed_routes2.push((target.into(), (if via.is_empty() { device_name } else { via }).into()));
                                    }));
                                });
                            }
                        }
                    });
                    if !managed_routes2.is_empty() {
                        managed_routes2.sort_by(|a, b| {
                            let aa = &((*a).1);
                            let bb = &((*b).1);
                            if aa.contains('.') || aa.contains(':') {
                                if bb.contains('.') || bb.contains(':') {
                                    aa.cmp(bb)
                                } else {
                                    Ordering::Greater
                                }
                            } else {
                                if bb.contains('.') || bb.contains(':') {
                                    Ordering::Less
                                } else {
                                    aa.cmp(bb)
                                }
                            }
                        });
                        let mut managed_routes_menu: Vec<TrayMenuItem> = Vec::new();
                        for r in managed_routes2.iter() {
                            let s = format!("{} via {}", (*r).0, (*r).1);
                            managed_routes_menu.push(TrayMenuItem::Text {
                                text: s.clone(),
                                checked: false,
                                disabled: false,
                                handler: Some(Box::new(move || {
                                    copy_to_clipboard(s.as_str());
                                    notify("Copied managed route to clipboard.");
                                })),
                            });
                        }
                        network_menu.push(TrayMenuItem::Submenu {
                            text: "Managed Routes ".into(),
                            checked: false,
                            items: managed_routes_menu,
                        });
                    }

                    network_menu.push(TrayMenuItem::Separator);

                    let (nwid, client2) = ((*network).0.clone(), client.clone());
                    let network_name: String = nw_obj.get("name").map_or("", |v| v.as_str().unwrap_or("")).into();
                    let settings = serde_json::to_string(&nw_obj).unwrap_or(String::new());
                    network_menu.push(TrayMenuItem::Text {
                        text: "Disconnect ".into(),
                        checked: false,
                        disabled: false,
                        handler: Some(Box::new(move || {
                            let mut c = client2.lock();
                            c.enqueue_delete(format!("network/{}", nwid));
                            c.remember_network(nwid.clone(), network_name.clone(), settings.clone());
                        })),
                    });

                    menu.push(TrayMenuItem::Submenu {
                        text: format!("{}\t{}  ", (*network).0, nw_obj.get("name").map_or("", |n| n.as_str().unwrap_or(""))),
                        checked: true,
                        items: network_menu,
                    });
                }
            }

            menu.push(TrayMenuItem::Separator);

            let saved_networks = client.lock().saved_networks();
            if !saved_networks.is_empty() {
                let mut saved_networks_empty = true;
                for nw in saved_networks.iter() {
                    if !networks.iter().any(|x| (*x).0.eq(&(*nw).0)) {
                        let (client2, client3) = (client.clone(), client.clone());
                        let (nwid2, nwid3, settings) = ((*nw).0.clone(), (*nw).0.clone(), (*nw).2.clone());
                        menu.push(TrayMenuItem::Submenu {
                            text: format!("{}\t{}  ", (*nw).0, (*nw).1),
                            checked: false,
                            items: vec![
                                TrayMenuItem::Text {
                                    text: "Reconnect".into(),
                                    checked: false,
                                    disabled: false,
                                    handler: Some(Box::new(move || client2.lock().enqueue_post(format!("network/{}", nwid2), settings.clone()))),
                                },
                                TrayMenuItem::Text {
                                    text: "Forget".into(),
                                    checked: false,
                                    disabled: false,
                                    handler: Some(Box::new(move || client3.lock().forget_network(&nwid3))),
                                }
                            ],
                        });
                        saved_networks_empty = false;
                    }
                }
                if !saved_networks_empty {
                    menu.push(TrayMenuItem::Separator);
                }
            }

            #[cfg(target_os = "macos")] {
                let dirty_flag2 = dirty_flag.clone();
                menu.push(TrayMenuItem::Text {
                    text: "Start UI at Login ".into(),
                    checked: unsafe { START_ON_LOGIN },
                    disabled: false,
                    handler: Some(Box::new(move || {
                        refresh_mac_start_on_login();
                        if unsafe { START_ON_LOGIN } {
                            // osascript -e 'tell application "System Events" to get the name of every login item'
                            let out = Command::new("/usr/bin/osascript").arg("-e").arg("tell application \"System Events\" to get the name of every login item").output();
                            let _ = out.map(|app_list| {
                                String::from_utf8(app_list.stdout).map(|app_list| {
                                    app_list.split('\n').for_each(|app| {
                                        if app.to_ascii_lowercase().contains("zerotier") {
                                            // osascript -e 'tell application "System Events" to delete login item "itemname"'
                                            let _ = Command::new("/usr/bin/osascript").arg("-e").arg(format!("tell application \"System Events\" to delete login item \"{}\"", app.trim())).output();
                                        }
                                    });
                                })
                            });
                        } else {
                            // osascript -e 'tell application "System Events" to make login item at end with properties {path:"PATH_TO_APP", hidden:false}'
                            let _ = Command::new("/usr/bin/osascript").arg("-e").arg(format!("tell application \"System Events\" to make login item at end with properties {{path:\"{}\", hidden:false}}", unsafe { &APPLICATION_PATH })).output();
                        }
                        refresh_mac_start_on_login();
                        dirty_flag2.store(true, std::sync::atomic::Ordering::Relaxed);
                    }))
                });
            }

            #[cfg(windows)] {
                let dirty_flag2 = dirty_flag.clone();
                menu.push(TrayMenuItem::Text {
                    text: "Start UI at Login ".into(),
                    checked: unsafe { START_ON_LOGIN },
                    disabled: false,
                    handler: Some(Box::new(move || {
                        refresh_windows_start_on_login();
                        let hkcu = winreg::RegKey::predef(winreg::enums::HKEY_CURRENT_USER);
                        let startup = hkcu.create_subkey("SOFTWARE\\Microsoft\\Windows\\CurrentVersion\\Run"); // opens read/write if exists
                        if startup.is_ok() {
                            let startup = startup.unwrap().0;
                            if unsafe { START_ON_LOGIN } {
                                let _ = startup.delete_value("ZeroTierUI");
                            } else {
                                let exe = String::from(std::env::current_exe().unwrap().to_str().unwrap());
                                let _ = startup.set_value("ZeroTierUI", &exe);
                            }
                        }
                        refresh_windows_start_on_login();
                        dirty_flag2.store(true, std::sync::atomic::Ordering::Relaxed);
                    }))
                });
            }

            let about_window2 = about_window.clone();
            menu.push(TrayMenuItem::Text {
                text: "About ".into(),
                checked: false,
                disabled: false,
                handler: Some(Box::new(move || open_ui_window_subprocess(&mut *about_window2.lock(), "About", 800, 600)))
            });
        } else {
            menu.push(TrayMenuItem::Text {
                text: "Waiting for ZeroTier system service...".into(),
                checked: false,
                disabled: true,
                handler: None,
            });
        }

        let exit_flag2 = exit_flag.clone();
        menu.push(TrayMenuItem::Text {
            text: "Quit ZeroTier UI ".into(),
            checked: false,
            disabled: false,
            handler: Some(Box::new(move || exit_flag2.store(true, std::sync::atomic::Ordering::SeqCst)))
        });

        menu
    };

    // Start a background thread to supervise windows and reap dead processes. This also
    // handles launching SSO login sessions when needed.
    let client2 = client.clone();
    let auth_windows2 = auth_windows.clone();
    let main_window2 = main_window.clone();
    let about_window2 = about_window.clone();
    std::thread::spawn(move || {
        set_thread_to_background_priority();

        let client = client2;
        let auth_windows = auth_windows2;
        let main_window = main_window2;
        let about_window = about_window2;

        loop {
            std::thread::sleep(Duration::from_secs(5));

            let mut auth_windows = auth_windows.lock();
            let auth_needed_networks = client.lock().sso_auth_needed_networks(20000);

            for network in auth_needed_networks.iter() { // network is a tuple of (ID, URL, remaining ms)
                let nwid = &(*network).0;
                let auth_url = &(*network).1;
                let status = (*network).2.as_str();

                if auth_windows.get(nwid).map_or(false, |w| {
                    if (*w).0 != *auth_url {
                        kill_process(&mut *(*w).1.lock());
                        true
                    } else {
                        false
                    }
                }) {
                    auth_windows.remove(nwid);
                }

                let auth_window = auth_windows.get(nwid);
                if auth_window.is_some() {
                    if status == "AUTHENTICATION_REQUIRED" {
                        auth_window.map(|w| {
                            (*w).1.lock().as_mut().map(|c| {
                                c.stdin.as_mut().map(|c| {
                                    let _ = c.write_all(&[b'r']);
                                });
                            });
                        });
                    }
                } else {
                    let _ = auth_windows.insert(nwid.clone(), (auth_url.clone(), Mutex::new(None))); // KEY -> (URL, WINDOW)
                    open_sso_auth_window_subprocess(&mut *(*auth_windows.get(nwid).unwrap()).1.lock(), 1024, 768, &[nwid.as_str(), (*network).1.as_str()]);
                }
            }

            auth_windows.retain(|nwid, window| {
                let w = &mut *(*window).1.lock();
                if did_process_exit(w) {
                    false
                } else if !auth_needed_networks.iter().any(|network| (*network).0.eq(nwid)) {
                    kill_process(w);
                    false
                } else {
                    true
                }
            });

            for w in [&main_window, &about_window].iter() {
                did_process_exit(&mut *w.lock());
            }
        }
    });

    let tray = Tray::init(icon_name.as_ref(), refresh());
    loop {
        // Refresh menu if data has changed.
        if dirty_flag.swap(false, std::sync::atomic::Ordering::Relaxed) {
            let new_icon = tray_icon_name();
            if new_icon != icon_name {
                icon_name = new_icon;
                tray.update(Some(icon_name.as_ref()), refresh());
            } else {
                tray.update(None, refresh());
            }
        }

        // Execute next tray event loop, which will (as per modifications made to tray.h) return after a second or two if
        // nothing happens. This allows periodic polling for changes to happen in this code.
        if !tray.poll() || exit_flag.load(std::sync::atomic::Ordering::Relaxed) {
            break;
        }
    }

    for w in [&main_window, &about_window].iter() {
        kill_process(&mut *w.lock());
    }
    for w in auth_windows.lock().iter() {
        kill_process(&mut *(*w.1).1.lock());
    }
}

fn main() {
    #[cfg(target_os = "macos")] {
        let p = std::env::current_exe().unwrap();
        for pp in p.ancestors() {
            let pps = pp.to_str().unwrap();
            if pps.ends_with(".app") {
                unsafe {
                    APPLICATION_PATH = String::from(pps);
                }
                break;
            }
        }
        unsafe {
            if APPLICATION_PATH.is_empty() {
                APPLICATION_PATH = String::from(p.to_str().unwrap());
            }
            APPLICATION_HOME = format!("{}/Library/Application Support/ZeroTier", std::env::var("HOME").unwrap_or(String::from("/tmp")));
        }
        refresh_mac_start_on_login();
    }

    #[cfg(not(target_os = "macos"))] {
        unsafe {
            APPLICATION_PATH = String::from(std::env::current_exe().unwrap().to_str().unwrap());
        }
        #[cfg(windows)] {
            refresh_windows_start_on_login();
            unsafe {
                APPLICATION_HOME = format!("{}\\AppData\\Local\\ZeroTier", std::env::var("USERPROFILE").unwrap_or(std::env::var("HOMEPATH").unwrap_or(String::from("C:\\"))));
            }
        }
        #[cfg(not(windows))] {
            unsafe {
                let _ = std::env::var("HOME").map_or_else(|_| {
                    APPLICATION_HOME = String::from("/tmp/zerotier_ui");
                }, |home_dir| {
                    APPLICATION_HOME = format!("{}/.zerotier_ui", home_dir);
                });
            }
        }
    }

    unsafe {
        let _ = std::fs::create_dir_all(APPLICATION_HOME.as_str());
        NETWORK_CACHE_PATH = String::from(Path::new(&APPLICATION_HOME).join("saved_networks.json").to_str().unwrap());
    }

    // Import old network list info from old Mac UI.
    #[cfg(target_os = "macos")] {
        if !Path::new(unsafe { NETWORK_CACHE_PATH.as_str() }).is_file() {
            let mut nwid: Option<u64> = None;
            let mut name: Option<String> = None;
            let mut networks: HashMap<String, String> = HashMap::new();
            let _ = plist::Value::from_file(format!("{}/One/networkinfo.dat", unsafe { &APPLICATION_HOME })).map(|old_plist| {
                old_plist.as_dictionary().map(|old_plist| {
                    old_plist.get("$objects").map(|old_plist| {
                        old_plist.as_array().map(|old_plist| {
                            for v in old_plist.iter() {
                                parse_mac_network_plist(old_plist, v, &mut networks, &mut nwid, &mut name);
                            }
                        });
                    });
                });
            });
            let mut networks_json: HashMap<String, HashMap<String, String>> = HashMap::new();
            for kv in networks.iter() {
                let mut nw: HashMap<String, String> = HashMap::new();
                nw.insert(String::from("id"), kv.0.clone());
                nw.insert(String::from("name"), kv.1.clone());
                networks_json.insert(kv.0.clone(), nw);
            }
            let _ = std::fs::write(unsafe { NETWORK_CACHE_PATH.as_str() }, serde_json::to_vec(&networks_json).unwrap());
        }
    }

    // Import old network list info from old Windows UI
    #[cfg(windows)] {
        if !Path::new(unsafe { NETWORK_CACHE_PATH.as_str() }).is_file() {
            let _ = std::env::var("USERPROFILE").map(|uprof| std::fs::read(format!("{}\\AppData\\Local\\ZeroTier\\One\\networks.dat", uprof)).map(|windows_networks_bin| {
                let mut networks_json: HashMap<String, HashMap<String, String>> = HashMap::new();
                let mut hex_str = String::with_capacity(16);
                for b in windows_networks_bin.iter() {
                    if (b"0123456789abcdef").contains(b) {
                        hex_str.push(*b as char);
                        if hex_str.len() == 16 {
                            let mut nw: HashMap<String, String> = HashMap::new();
                            nw.insert(String::from("id"), hex_str.clone());
                            networks_json.insert(hex_str.clone(), nw);
                            hex_str.clear();
                        }
                    } else {
                        hex_str.clear();
                    }
                }
                let _ = std::fs::write(unsafe { NETWORK_CACHE_PATH.as_str() }, serde_json::to_vec(&networks_json).unwrap());
            }));
        }
    }

    let args: Vec<String> = std::env::args().collect();
    if args.len() >= 2 {
        match args[1].as_str() {
            "window" => { // invoked to open webview GUI windows
                if args.len() >= 3 {
                    control_panel_window_main(&args);
                } else {
                    println!("FATAL: window requires arguments: ui_mode [width hint] [height hint]");
                }
            },
            "auth" => { // invoked to open a window to an SSO login endpoint
                if args.len() >= 5 {
                    sso_auth_window_main(&args);
                } else {
                    println!("FATAL: window requires arguments: ui_mode [width hint] [height hint] [url]");
                }
            },
            "copy_authtoken" => { // invoked with elevated permissions to get the auth token and copy it locally
                let _ = serviceclient::get_auth_token_and_port(false);
            }
            _ => println!("FATAL: unrecognized mode: {}", args[1])
        }
    } else {
        tray_main();
    }
    std::process::exit(0);
}
