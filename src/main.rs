// (c)2021 ZeroTier, Inc.

mod tray;
mod serviceclient;

#[allow(unused)]
use std::process::{Child, Command, Stdio};
use std::sync::{Mutex, Arc, MutexGuard};
use std::time::Duration;
#[allow(unused)]
use std::io::{Read, Write};
use std::cmp::Ordering;
use std::sync::atomic::AtomicBool;
#[allow(unused)]
use std::os::raw::{c_int, c_char, c_uint};
#[allow(unused)]
use std::ffi::CString;
#[allow(unused)]
use std::collections::HashMap;
use std::path::Path;

use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::tray::*;
use crate::serviceclient::ServiceClient;

/// The string in the HTML blob to replace with the right CSS for this platform and light/dark mode.
/// It's a bit weird so web app bundlers don't optimize it out.
const CSS_PLACEHOLDER: &'static str = ".XXXthis_is_replaced_by_css_in_the_rust_codeXXX{border:0}";

const MAIN_WINDOW_WIDTH: i32 = 1300;
const MAIN_WINDOW_HEIGHT: i32 = 500;

const WEBVIEW_WINDOW_FRAMELESS: bool = false;

pub(crate) static mut APPLICATION_PATH: String = String::new();
pub(crate) static mut APPLICATION_HOME: String = String::new();
pub(crate) static mut NETWORK_CACHE_PATH: String = String::new();
pub(crate) static mut START_ON_LOGIN: bool = false;

#[derive(Serialize, Deserialize)]
pub struct CommandFromWebView {
    #[serde(default)]
    pub cmd: String,
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub data: String,
}

/*******************************************************************************************************************/

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

/*******************************************************************************************************************/

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

#[cfg(target_os = "macos")]
fn is_dark_mode() -> bool {
    let out = Command::new("/usr/bin/defaults").arg("read").arg("-g").arg("AppleInterfaceStyle").output();
    out.map_or(false, |mode| {
        String::from_utf8(mode.stdout.to_ascii_lowercase()).map_or(false, |mode| mode.contains("dark"))
    })
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

/*******************************************************************************************************************/

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
        let ch = Command::new(std::env::current_exe().unwrap()).arg("window").arg(ui_mode).arg(width.to_string()).arg(height.to_string()).stdin(Stdio::piped()).spawn();
        if ch.is_ok() {
            let _ = w.replace(ch.unwrap());
        }
    } else {
        // Sending 'r' causes subprocess to raise the window at the first opportunity.
        let _ = w.as_mut().unwrap().stdin.as_mut().unwrap().write_all(&['r' as u8]);
    }
}

/// This is called once per process to start the client I/O loop.
fn start_client(refresh_base_paths: Vec<&'static str>, tick_period_ms: u64, refresh_period_ticks: u64) -> (Arc<Mutex<ServiceClient>>, Arc<AtomicBool>) {
    let (mut client, dirty_flag) = ServiceClient::new(refresh_base_paths);
    client.sync();

    let client = Arc::new(Mutex::new(client));

    let thread_client = client.clone();
    let _ = std::thread::spawn(move || {
        set_thread_to_background_priority();
        let mut k = 0_u64;
        loop {
            let _ = thread_client.lock().map(|mut c| {
                k += 1;
                if c.do_posts() || k >= refresh_period_ticks {
                    k = 0;
                    c.sync();
                }
            });
            std::thread::sleep(Duration::from_millis(tick_period_ms));
        }
    });

    (client, dirty_flag)
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

    let (client, dirty_flag) = start_client(vec!["status", "network", "peer"], 100, 5);

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

    set_thread_to_foreground_priority();

    let ui_client = client.clone();
    let _ = ui_client.lock().map(|mut c| c.sync());
    let _ = web_view::builder()
        .title("ZeroTier")
        .content(web_view::Content::Html(get_web_ui_blob(is_dark_mode())))
        .size(
            if args.len() >= 5 { i32::from_str_radix(args[3].as_str(), 10).unwrap_or(800) } else { 800 },
            if args.len() >= 5 { i32::from_str_radix(args[4].as_str(), 10).unwrap_or(600) } else { 600 })
        .resizable(true)
        .visible(false)
        .frameless(WEBVIEW_WINDOW_FRAMELESS)
        .debug(false)
        .user_data(())
        .invoke_handler(move |wv, arg| {
            if raise_window.swap(false, std::sync::atomic::Ordering::Relaxed) {
                wv.set_visible(true);
            }

            let _ = serde_json::from_str::<CommandFromWebView>(arg).map(|cmd| {
                match cmd.cmd.as_str() {
                    "ready" => {
                        wv.set_visible(true);
                        let _ = wv.eval(format!("zt_ui_render('{}', {});", args[2], WEBVIEW_WINDOW_FRAMELESS).as_str());
                    },
                    "post" => {
                        let _ = ui_client.lock().map(|mut c| c.enqueue_post(cmd.name, cmd.data));
                    },
                    "delete" => {
                        let _ = ui_client.lock().map(|mut c| c.enqueue_delete(cmd.name));
                    },
                    "copy_to_clipboard" => {
                        copy_to_clipboard(cmd.data.as_str());
                    },
                    "paste_from_clipboard" => {
                        let data = read_from_clipboard();
                        let data: Vec<u16> = data.encode_utf16().collect();
                        let data = format!("zt_paste_from_clipboard_callback({});", serde_json::to_string(data.as_slice()).unwrap());
                        let _ = wv.eval(data.as_str());
                    },
                    "raise" => {
                        wv.set_visible(true);
                    },
                    "poll" => {
                        if dirty_flag.swap(false, std::sync::atomic::Ordering::Relaxed) {
                            let _ = ui_client.lock().map(|ui_client| {
                                let _ = wv.eval(format!("zt_ui_update({});", ui_client.get_all_json()).as_str());
                            });
                        }
                    },
                    "log" => {
                        println!("> {}", cmd.data);
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

    let single = single_instance::SingleInstance::new(std::env::temp_dir().join("ZeroTierUI_InstanceLock").to_str().unwrap()).unwrap();
    if !single.is_single() {
        println!("FATAL: another instance of the ZeroTier UI is already running.");
        std::process::exit(1);
    }

    #[cfg(all(target_arch = "aarch64", target_os = "macos"))]
    set_thread_to_background_priority();

    let mut icon_name = tray_icon_name();
    let mut tray: Option<Tray> = None;
    let exit_flag = Arc::new(AtomicBool::new(false));

    let (client, dirty_flag) = start_client(vec!["status", "network"], 500, 10);

    let main_window: Arc<Mutex<Option<Child>>> = Arc::new(Mutex::new(None));
    let about_window: Arc<Mutex<Option<Child>>> = Arc::new(Mutex::new(None));

    while !exit_flag.load(std::sync::atomic::Ordering::Relaxed) {
        if dirty_flag.swap(false, std::sync::atomic::Ordering::Relaxed) {
            let (main_window2, main_window3) = (main_window.clone(), main_window.clone());
            let (about_window2, about_window3) = (about_window.clone(), about_window.clone());

            let mut menu: Vec<TrayMenuItem> = Vec::new();
            if client.lock().unwrap().is_online() {
                let address = client.lock().unwrap().get_str(&["status", "address"]);
                let address2 = address.clone();
                menu.push(TrayMenuItem::Text {
                    text: format!("Node ID:  {} ", address),
                    checked: false,
                    disabled: false,
                    handler: Some(Box::new(move || copy_to_clipboard(address2.as_str()) )),
                });

                menu.push(TrayMenuItem::Separator);

                menu.push(TrayMenuItem::Text {
                    text: "Open Control Panel... ".into(),
                    checked: false,
                    disabled: false,
                    handler: Some(Box::new(move || {
                        open_window_subprocess(main_window2.lock().unwrap(), "Main", MAIN_WINDOW_WIDTH, MAIN_WINDOW_HEIGHT);
                    })),
                });

                let networks = client.lock().unwrap().networks();
                if !networks.is_empty() {
                    menu.push(TrayMenuItem::Separator);
                    networks.iter().for_each(|network| {
                        let nw_obj = &((*network).1);

                        let assigned_addrs = nw_obj.get("assignedAddresses").unwrap_or(&Value::Null);
                        let managed_routes = nw_obj.get("routes").unwrap_or(&Value::Null);
                        let device_name = nw_obj.get("portDeviceName").map_or("", |s| s.as_str().unwrap_or(""));
                        let allow_managed_addresses = nw_obj.get("allowManaged").map_or(false, |b| b.as_bool().unwrap_or(false));
                        let allow_global_ips = nw_obj.get("allowGlobal").map_or(false, |b| b.as_bool().unwrap_or(false));
                        let allow_default = nw_obj.get("allowDefault").map_or(false, |b| b.as_bool().unwrap_or(false));
                        let allow_dns = nw_obj.get("allowDNS").map_or(false, |b| b.as_bool().unwrap_or(false));

                        let mut network_menu: Vec<TrayMenuItem> = Vec::new();
                        let nwid = (*network).0.clone();
                        network_menu.push(TrayMenuItem::Text {
                            text: format!("Network ID:\t  {}", (*network).0),
                            checked: false,
                            disabled: false,
                            handler: Some(Box::new(move || copy_to_clipboard(nwid.as_str()) )),
                        });
                        network_menu.push(TrayMenuItem::Separator);
                        let (nwid, client2) = ((*network).0.clone(), client.clone());
                        network_menu.push(TrayMenuItem::Text {
                            text: format!("Allow Managed Addresses"),
                            checked: allow_managed_addresses,
                            disabled: false,
                            handler: Some(Box::new(move || client2.lock().unwrap().enqueue_post(format!("network/{}", nwid), format!("{{ \"allowManaged\": {} }}", !allow_managed_addresses)))),
                        });
                        let (nwid, client2) = ((*network).0.clone(), client.clone());
                        network_menu.push(TrayMenuItem::Text {
                            text: format!("Allow Assignment of Global IPs"),
                            checked: allow_global_ips,
                            disabled: false,
                            handler: Some(Box::new(move || client2.lock().unwrap().enqueue_post(format!("network/{}", nwid), format!("{{ \"allowGlobal\": {} }}", !allow_global_ips)))),
                        });
                        let (nwid, client2) = ((*network).0.clone(), client.clone());
                        network_menu.push(TrayMenuItem::Text {
                            text: format!("Allow Default Router Override"),
                            checked: allow_default,
                            disabled: false,
                            handler: Some(Box::new(move || client2.lock().unwrap().enqueue_post(format!("network/{}", nwid), format!("{{ \"allowDefault\": {} }}", !allow_default)))),
                        });
                        let (nwid, client2) = ((*network).0.clone(), client.clone());
                        network_menu.push(TrayMenuItem::Text {
                            text: format!("Allow DNS Configuration"),
                            checked: allow_dns,
                            disabled: false,
                            handler: Some(Box::new(move || client2.lock().unwrap().enqueue_post(format!("network/{}", nwid), format!("{{ \"allowDNS\": {} }}", !allow_dns)))),
                        });
                        network_menu.push(TrayMenuItem::Separator);
                        nw_obj.get("mac").map(|a| a.as_str().map(|a| {
                            network_menu.push(TrayMenuItem::Text {
                                text: format!("Ethernet:\t\t  {}", a),
                                checked: false,
                                disabled: true,
                                handler: None,
                            });
                        }));
                        if !device_name.is_empty() {
                            network_menu.push(TrayMenuItem::Text {
                                text: format!("Device:\t\t  {}", device_name),
                                checked: false,
                                disabled: true,
                                handler: None,
                            });
                        }
                        nw_obj.get("type").map(|a| a.as_str().map(|a| {
                            network_menu.push(TrayMenuItem::Text {
                                text: format!("Type:\t\t  {}", a),
                                checked: false,
                                disabled: true,
                                handler: None,
                            });
                        }));
                        nw_obj.get("status").map(|a| a.as_str().map(|a| {
                            network_menu.push(TrayMenuItem::Text {
                                text: format!("Status:\t\t  {}", a),
                                checked: false,
                                disabled: true,
                                handler: None,
                            });
                        }));
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
                                            handler: Some(Box::new(move || copy_to_clipboard(a_copy.split_once('/').map_or(a_copy.as_str(), |a| a.0)) )),
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
                                    handler: Some(Box::new(move || copy_to_clipboard(s.as_str()))),
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
                        network_menu.push(TrayMenuItem::Text {
                            text: "Disconnect ".into(),
                            checked: false,
                            disabled: false,
                            handler: Some(Box::new(move || client2.lock().unwrap().enqueue_delete(format!("network/{}", nwid)))),
                        });

                        menu.push(TrayMenuItem::Submenu {
                            text: format!("{}\t{}  ", (*network).0, nw_obj.get("name").map_or("", |n| n.as_str().unwrap_or(""))),
                            checked: true,
                            items: network_menu,
                        });
                    });
                }

                menu.push(TrayMenuItem::Separator);

                let saved_networks = client.lock().unwrap().saved_networks();
                if !saved_networks.is_empty() {
                    let mut saved_networks_empty = true;
                    for nw in saved_networks.iter() {
                        if !networks.iter().any(|x| (*x).0.eq(&(*nw).0)) {
                            let (client2, client3) = (client.clone(), client.clone());
                            let (nwid2, nwid3) = ((*nw).0.clone(), (*nw).0.clone());
                            menu.push(TrayMenuItem::Submenu {
                                text: format!("{}\t{}  ", (*nw).0, (*nw).1),
                                checked: false,
                                items: vec![
                                    TrayMenuItem::Text {
                                        text: "Reconnect".into(),
                                        checked: false,
                                        disabled: false,
                                        handler: Some(Box::new(move || client2.lock().unwrap().enqueue_post(format!("network/{}", nwid2), "{}".into()))),
                                    },
                                    TrayMenuItem::Text {
                                        text: "Forget".into(),
                                        checked: false,
                                        disabled: false,
                                        handler: Some(Box::new(move || client3.lock().unwrap().forget_network(&nwid3))),
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

                menu.push(TrayMenuItem::Text {
                    text: "About ".into(),
                    checked: false,
                    disabled: false,
                    handler: Some(Box::new(move || {
                        open_window_subprocess(about_window2.lock().unwrap(), "About", 800, 600);
                    }))
                });

                let exit_flag2 = exit_flag.clone();
                menu.push(TrayMenuItem::Text {
                    text: "Quit ZeroTier UI ".into(),
                    checked: false,
                    disabled: false,
                    handler: Some(Box::new(move || {
                        for w in [&main_window3, &about_window3].iter() {
                            kill_window_subprocess(w.lock().unwrap());
                        }
                        exit_flag2.store(true, std::sync::atomic::Ordering::SeqCst);
                    }))
                });
            } else {
                menu.push(TrayMenuItem::Text {
                    text: "Service unreachable...".into(),
                    checked: false,
                    disabled: true,
                    handler: None,
                });
            }

            if tray.is_none() {
                tray.replace(Tray::init(icon_name.as_ref(), menu));
            } else {
                let new_icon = tray_icon_name();
                if new_icon != icon_name {
                    icon_name = new_icon;
                    tray.as_ref().unwrap().update(Some(icon_name.as_ref()), menu);
                } else {
                    tray.as_ref().unwrap().update(None, menu);
                }

                for w in [&main_window, &about_window].iter() {
                    check_window_subprocess_exit(w.lock().as_mut().unwrap());
                }
            }
        }

        if !tray.as_ref().map_or(false, |tr| tr.poll()) {
            break;
        }
    }

    for w in [&main_window, &about_window].iter() {
        kill_window_subprocess(w.lock().unwrap());
    }
}

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
