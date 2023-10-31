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
use std::collections::{HashMap, HashSet, LinkedList};
#[allow(unused)]
use std::ffi::CString;
#[allow(unused)]
use std::io::{Read, Write};
#[allow(unused)]
use std::os::raw::{c_char, c_int, c_uint};
use std::path::Path;
use std::process::{Child, Command, Stdio};
#[allow(unused)]
use std::sync::atomic::*;
use std::sync::Arc;
use std::time::{Duration, SystemTime};

use parking_lot::Mutex;
use serde_json::Value;

use crate::serviceclient::*;
use crate::tray::*;

pub mod about;
pub mod join;
pub mod libui;
pub mod serviceclient;
pub mod tray;

pub(crate) static mut APPLICATION_PATH: String = String::new();
pub(crate) static mut APPLICATION_HOME: String = String::new();
pub(crate) static mut NETWORK_CACHE_PATH: String = String::new();
pub(crate) static mut START_ON_LOGIN: bool = false;

#[cfg(target_os = "macos")]
pub(crate) const GLOBAL_SERVICE_HOME_V2: &'static str = "/Library/Application Support/ZeroTier";

#[cfg(windows)]
pub(crate) const GLOBAL_SERVICE_HOME_V2: &'static str = "\\ProgramData\\ZeroTier";

#[cfg(any(
    target_os = "freebsd",
    target_os = "dragonfly",
    target_os = "openbsd",
    target_os = "netbsd"
))]
pub(crate) const GLOBAL_SERVICE_HOME_V2: &'static str = "/var/db/zerotier";

#[cfg(target_os = "linux")]
pub(crate) const GLOBAL_SERVICE_HOME_V2: &'static str = "/var/lib/zerotier";

#[cfg(target_os = "macos")]
pub(crate) const GLOBAL_SERVICE_HOME_V1: &'static str = "/Library/Application Support/ZeroTier/One";

#[cfg(windows)]
pub(crate) const GLOBAL_SERVICE_HOME_V1: &'static str = "\\ProgramData\\ZeroTier\\One";

#[cfg(any(
    target_os = "freebsd",
    target_os = "dragonfly",
    target_os = "openbsd",
    target_os = "netbsd"
))]
pub(crate) const GLOBAL_SERVICE_HOME_V1: &'static str = "/var/db/zerotier-one";

#[cfg(target_os = "linux")]
pub(crate) const GLOBAL_SERVICE_HOME_V1: &'static str = "/var/lib/zerotier-one";

#[cfg(target_os = "macos")]
pub(crate) const MAC_BUNDLE_ID: &'static str = "com.zerotier.ZeroTier-UI";

/*******************************************************************************************************************/
/* OS-specific functions and C externs */

#[cfg(windows)]
extern "C" {
    pub fn c_windows_post_to_clipboard(data: *const c_char);
    pub fn c_windows_get_from_clipboard(buf: *mut c_char) -> c_uint;
}

#[cfg(target_os = "macos")]
extern "C" {
    pub fn c_set_this_thread_to_background_priority();
    pub fn c_set_this_thread_to_foreground_priority();
    pub fn c_lock_down_file(path: *const c_char, is_dir: c_int);
}

#[allow(unused)]
#[cfg(target_os = "macos")]
fn set_thread_to_background_priority() {
    unsafe {
        c_set_this_thread_to_background_priority();
    }
}

#[allow(unused)]
#[cfg(not(target_os = "macos"))]
fn set_thread_to_background_priority() {}

#[allow(unused)]
#[cfg(target_os = "macos")]
fn set_thread_to_foreground_priority() {
    unsafe {
        c_set_this_thread_to_foreground_priority();
    }
}

#[allow(unused)]
#[cfg(not(target_os = "macos"))]
fn set_thread_to_foreground_priority() {}

/// Quick and dirty recursive parser for decoded plist files from the old GUI on Mac... this is just for transition.
#[cfg(target_os = "macos")]
fn parse_mac_network_plist(
    base_array: &Vec<plist::Value>,
    data: &plist::Value,
    networks: &mut HashMap<String, String>,
    nwid: &mut Option<u64>,
    name: &mut Option<String>,
) {
    if nwid.as_ref().map_or(false, |x| *x != 0)
        && name.as_ref().map_or(false, |x| !x.eq("\0\0\0\0"))
    {
        networks.insert(
            format!("{:0>16x}", nwid.take().unwrap()),
            name.take().unwrap(),
        );
    }
    match data {
        plist::Value::Array(arr) => {
            for v in arr.iter() {
                parse_mac_network_plist(base_array, v, networks, nwid, name);
            }
        }
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
        }
        plist::Value::String(s) => {
            if name.as_ref().map_or(false, |x| x.eq("\0\0\0\0")) {
                name.replace(s.clone());
            }
        }
        plist::Value::Integer(i) => {
            if nwid.as_ref().map_or(false, |x| *x == 0) {
                nwid.replace(i.as_unsigned().unwrap_or(0));
            }
        }
        plist::Value::Uid(uid) => {
            let _ = base_array
                .get(uid.get() as usize)
                .map(|v| parse_mac_network_plist(base_array, v, networks, nwid, name));
        }
        _ => {}
    }
}

/*******************************************************************************************************************/
/* Refresh the state of START_ON_LOGIN */

#[cfg(target_os = "macos")]
fn refresh_mac_start_on_login() {
    // osascript -e 'tell application "System Events" to get the name of every login item'
    let out = Command::new("/usr/bin/osascript")
        .arg("-e")
        .arg("tell application \"System Events\" to get the name of every login item")
        .output();
    unsafe {
        START_ON_LOGIN = out.map_or(false, |app_list| {
            String::from_utf8(app_list.stdout.to_ascii_lowercase())
                .map_or(false, |app_list| app_list.contains("zerotier"))
        });
    }
}

#[cfg(target_os = "windows")]
fn refresh_windows_start_on_login() {
    let hkcu = winreg::RegKey::predef(winreg::enums::HKEY_CURRENT_USER);
    let startup = hkcu.open_subkey("SOFTWARE\\Microsoft\\Windows\\CurrentVersion\\Run");
    let enabled = startup.map_or(false, |startup| {
        startup.get_value::<String, &str>("ZeroTierUI").is_ok()
    });
    unsafe {
        START_ON_LOGIN = enabled;
    }
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
        #[cfg(windows)]
        {
            let _ = std::fs::write(&icon_path, include_bytes!("../icon.ico"));
        }
        #[cfg(not(windows))]
        {
            let _ = std::fs::write(&icon_path, include_bytes!("../icon.png"));
        }
    }
    icon_path.to_str().unwrap().into()
}

/*******************************************************************************************************************/
/* Clipboard functions for different OSes */

#[cfg(target_os = "linux")]
fn copy_to_clipboard(s: &str) {
    let _ = Command::new("/usr/bin/xclip")
        .stdin(Stdio::piped())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .spawn()
        .map(|mut c| {
            c.stdin.take().map(|mut stdin| {
                let _ = stdin.write_all(s.as_bytes());
            });
            let _ = c.wait();
        });
}

#[cfg(target_os = "macos")]
fn copy_to_clipboard(s: &str) {
    let _ = Command::new("/usr/bin/pbcopy")
        .stdin(Stdio::piped())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .spawn()
        .map(|mut c| {
            c.stdin.take().map(|mut stdin| {
                let _ = stdin.write_all(s.as_bytes());
            });
            let _ = c.wait();
        });
}

#[cfg(windows)]
fn copy_to_clipboard(s: &str) {
    let _ = CString::new(s).map(|s| {
        unsafe { c_windows_post_to_clipboard(s.as_ptr()) };
    });
}

/*******************************************************************************************************************/

#[cfg(windows)]
fn notify(text: &str, _: Option<(String, String)>) {
    let ico: String = tray_icon_name();
    let _ = notify_rust::Notification::new()
        .icon(ico.as_str())
        .body(text)
        .appname("ZeroTier")
        .show();
}

#[cfg(not(any(windows, target_os = "macos")))]
fn notify(text: &str, _: Option<(String, String)>) {
    let _ = notify_rust::Notification::new()
        .body(text)
        .appname("ZeroTier")
        .show();
}

#[cfg(target_os = "macos")]
fn notify(text: &str, _: Option<(String, String)>) {
    let _ = mac_notification_sys::set_application(MAC_BUNDLE_ID);
    let mut n = mac_notification_sys::Notification::new();
    n.title("ZeroTier");
    n.message(text);
    n.asynchronous(true);
    let _ = n.send();
    /*
    if let Some(url) = url {
        if let Ok(r) = mac_notification_sys::send_notification(
            "ZeroTier",
            None,
            text,
            Some(&mac_notification_sys::Notification::new().main_button(
                mac_notification_sys::MainButton::SingleAction(url.0.as_str()),
            )),
        ) {
            match r {
                mac_notification_sys::NotificationResponse::Click
                | mac_notification_sys::NotificationResponse::ActionButton(_) => {
                    let _ = webbrowser::open(url.1.as_str());
                }
                _ => {}
            }
        }
    } else {
        let _ = mac_notification_sys::send_notification("ZeroTier", None, text, None);
    }
    */
}

/*******************************************************************************************************************/

/// Start the service client background thread, returning the client and a flag set when the data changes.
fn start_client(
    refresh_base_paths: Vec<&'static str>,
    tick_period_ms: u64,
    refresh_period_ticks: u64,
) -> (Arc<Mutex<ServiceClient>>, Arc<AtomicBool>) {
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
    let about_child: Arc<Mutex<Option<Child>>> = Arc::new(Mutex::new(None));
    let joining: Arc<Mutex<Vec<String>>> = Arc::new(Mutex::new(Vec::new()));
    let join_window_open = Arc::new(AtomicBool::new(false));
    let sso_notification_shown: Arc<Mutex<HashSet<String>>> = Arc::new(Mutex::new(HashSet::new()));

    // This closure builds a new menu for display in the app icon.
    let refresh = || {
        let mut menu: Vec<TrayMenuItem> = Vec::new();
        if client.lock().is_online() {
            let address = client.lock().get_str(&["status", "address"]);
            let version = client.lock().get_str(&["status", "version"]);

            let address2 = address.clone();
            menu.push(TrayMenuItem::Text {
                text: format!("My Address:  {} ", address),
                checked: false,
                disabled: false,
                handler: Some(Box::new(move || {
                    copy_to_clipboard(address2.as_str());
                    notify("Copied this node's ZeroTier address to clipboard.", None);
                })),
            });

            let client2 = client.clone();
            let joining2 = joining.clone();
            let join_window_open2 = join_window_open.clone();
            menu.push(TrayMenuItem::Text {
                text: "Join New Network...".into(),
                checked: false,
                disabled: false,
                handler: Some(Box::new(move || {
                    if !join_window_open2.load(std::sync::atomic::Ordering::Relaxed) {
                        join_window_open2.store(true, std::sync::atomic::Ordering::Relaxed);
                        let client3 = client2.clone();
                        let joining3 = joining2.clone();
                        let join_window_open3 = join_window_open2.clone();
                        std::thread::spawn(move || {
                            let ch = Command::new(std::env::current_exe().unwrap())
                                .arg("join_prompt")
                                .stdout(Stdio::piped())
                                .output();
                            join_window_open3.store(false, std::sync::atomic::Ordering::Relaxed);
                            if ch.is_ok() {
                                let ch = ch.unwrap();
                                if let Some((_, nwid)) =
                                    String::from_utf8_lossy(ch.stdout.as_slice())
                                        .split_once("!!!JOIN")
                                {
                                    if nwid.len() >= 16 {
                                        let (nwid, _) = nwid.split_at(crate::join::NETWORK_ID_LEN);
                                        client3.lock().enqueue_post(
                                            format!("network/{}", nwid),
                                            "{}".into(),
                                        );
                                        joining3.lock().push(nwid.into());
                                    }
                                }
                            }
                        });
                    }
                })),
            });

            menu.push(TrayMenuItem::Separator);

            let mut networks_empty = true;
            let mut login_needed_networks = Vec::new();

            let networks = client.lock().networks();
            if !networks.is_empty() {
                menu.push(TrayMenuItem::Separator);
                for network in networks.iter() {
                    networks_empty = false;
                    let nw_obj = &((*network).1);

                    let assigned_addrs = nw_obj.get("assignedAddresses").unwrap_or(&Value::Null);
                    let managed_routes = nw_obj.get("routes").unwrap_or(&Value::Null);
                    let device_name = nw_obj
                        .get("portDeviceName")
                        .map_or("", |s| s.as_str().unwrap_or(""));
                    let allow_managed_addresses = nw_obj
                        .get("allowManaged")
                        .map_or(false, |b| b.as_bool().unwrap_or(false));
                    let allow_global_ips = nw_obj
                        .get("allowGlobal")
                        .map_or(false, |b| b.as_bool().unwrap_or(false));
                    let allow_default = nw_obj
                        .get("allowDefault")
                        .map_or(false, |b| b.as_bool().unwrap_or(false));
                    let allow_dns = nw_obj
                        .get("allowDNS")
                        .map_or(false, |b| b.as_bool().unwrap_or(false));
                    let status = nw_obj
                        .get("status")
                        .map_or("REQUESTING_CONFIGURATION", |s| {
                            s.as_str().unwrap_or("REQUESTING_CONFIGURATION")
                        });

                    let mut network_menu: Vec<TrayMenuItem> = Vec::new();

                    let nwid = (*network).0.clone();
                    joining.lock().retain(|j| !j.eq(nwid.as_str()));
                    network_menu.push(TrayMenuItem::Text {
                        text: format!("Network ID:\t\t  {}", (*network).0),
                        checked: false,
                        disabled: false,
                        handler: Some(Box::new(move || {
                            copy_to_clipboard(nwid.as_str());
                            notify("Copied network ID to clipboard.", None);
                        })),
                    });

                    network_menu.push(TrayMenuItem::Separator);

                    let (nwid, client2) = ((*network).0.clone(), client.clone());
                    network_menu.push(TrayMenuItem::Text {
                        text: format!("Allow Managed Addresses"),
                        checked: allow_managed_addresses,
                        disabled: false,
                        handler: Some(Box::new(move || {
                            client2.lock().enqueue_post(
                                format!("network/{}", nwid),
                                format!("{{ \"allowManaged\": {} }}", !allow_managed_addresses),
                            )
                        })),
                    });
                    let (nwid, client2) = ((*network).0.clone(), client.clone());
                    network_menu.push(TrayMenuItem::Text {
                        text: format!("Allow Assignment of Global IPs"),
                        checked: allow_global_ips,
                        disabled: false,
                        handler: Some(Box::new(move || {
                            client2.lock().enqueue_post(
                                format!("network/{}", nwid),
                                format!("{{ \"allowGlobal\": {} }}", !allow_global_ips),
                            )
                        })),
                    });
                    let (nwid, client2) = ((*network).0.clone(), client.clone());
                    network_menu.push(TrayMenuItem::Text {
                        text: format!("Allow Default Route Override"),
                        checked: allow_default,
                        disabled: false,
                        handler: Some(Box::new(move || {
                            client2.lock().enqueue_post(
                                format!("network/{}", nwid),
                                format!("{{ \"allowDefault\": {} }}", !allow_default),
                            )
                        })),
                    });
                    let (nwid, client2) = ((*network).0.clone(), client.clone());
                    network_menu.push(TrayMenuItem::Text {
                        text: format!("Allow DNS Configuration"),
                        checked: allow_dns,
                        disabled: false,
                        handler: Some(Box::new(move || {
                            client2.lock().enqueue_post(
                                format!("network/{}", nwid),
                                format!("{{ \"allowDNS\": {} }}", !allow_dns),
                            )
                        })),
                    });

                    network_menu.push(TrayMenuItem::Separator);

                    nw_obj.get("mac").map(|a| {
                        a.as_str().map(|a| {
                            network_menu.push(TrayMenuItem::Text {
                                text: format!("Ethernet:\t\t\t  {}", a),
                                checked: false,
                                disabled: true,
                                handler: None,
                            });
                        })
                    });

                    if !device_name.is_empty() {
                        network_menu.push(TrayMenuItem::Text {
                            text: format!("Device:\t\t\t  {}", device_name),
                            checked: false,
                            disabled: true,
                            handler: None,
                        });
                    }

                    nw_obj.get("type").map(|a| {
                        a.as_str().map(|a| {
                            network_menu.push(TrayMenuItem::Text {
                                text: format!("Type:\t\t\t  {}", a),
                                checked: false,
                                disabled: true,
                                handler: None,
                            });
                        })
                    });

                    if let Some(dns) = nw_obj.get("dns") {
                        if let Some(dns) = dns.as_object() {
                            if let Some(domain) = dns.get("domain") {
                                if let Some(domain) = domain.as_str() {
                                    if !domain.is_empty() {
                                        network_menu.push(TrayMenuItem::Text {
                                            text: format!("DNS Domain:\t\t  {}", domain),
                                            checked: false,
                                            disabled: true,
                                            handler: None,
                                        });
                                    }
                                }
                            }
                        }
                    }

                    network_menu.push(TrayMenuItem::Separator);

                    network_menu.push(TrayMenuItem::Text {
                        text: format!("Status:\t\t\t  {}", status),
                        checked: false,
                        disabled: true,
                        handler: None,
                    });

                    if status == "OK" {
                        let _ = sso_notification_shown.lock().remove(&(*network).0);

                        nw_obj.get("authenticationExpiryTime").map(|auth_exp_time| {
                            auth_exp_time.as_i64().map(|auth_exp_time| {
                                let auth_exp_time = auth_exp_time;
                                if auth_exp_time > 0 {
                                    let auth_exp_time = SystemTime::UNIX_EPOCH
                                        .checked_add(std::time::Duration::from_millis(
                                            auth_exp_time as u64,
                                        ))
                                        .unwrap();
                                    let auth_exp_time =
                                        chrono::DateTime::<chrono::Local>::from(auth_exp_time);
                                    network_menu.push(TrayMenuItem::Text {
                                        text: format!(
                                            "Auth Expire:\t\t  {}",
                                            auth_exp_time.format("%Y-%m-%d %H:%M:%S").to_string()
                                        ),
                                        checked: false,
                                        disabled: true,
                                        handler: None,
                                    });
                                }
                            })
                        });
                    } else if status == "AUTHENTICATION_REQUIRED" {
                        nw_obj.get("authenticationURL").map(|auth_url| {
                            let auth_url = auth_url.as_str();
                            if auth_url.is_some() {
                                let auth_url = auth_url.unwrap().to_string();
                                login_needed_networks
                                    .push(((*network).0.clone(), auth_url.clone()));
                                network_menu.push(TrayMenuItem::Text {
                                    text: "Open SSO Login URL...".into(),
                                    checked: false,
                                    disabled: false,
                                    handler: Some(Box::new(move || {
                                        let _ = webbrowser::open(&auth_url);
                                    })),
                                });
                            }
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
                                            copy_to_clipboard(
                                                a_copy
                                                    .split_once('/')
                                                    .map_or(a_copy.as_str(), |a| a.0),
                                            );
                                            notify("Copied address to clipboard.", None);
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
                                    r.get("target").map(|target| {
                                        target.as_str().map(|target| {
                                            managed_routes2.push((
                                                target.into(),
                                                (if via.is_empty() { device_name } else { via })
                                                    .into(),
                                            ));
                                        })
                                    });
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
                                    notify("Copied managed route to clipboard.", None);
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
                    let network_name: String = nw_obj
                        .get("name")
                        .map_or("", |v| v.as_str().unwrap_or(""))
                        .into();
                    let settings = serde_json::to_string(&nw_obj).unwrap_or(String::new());
                    network_menu.push(TrayMenuItem::Text {
                        text: "Disconnect ".into(),
                        checked: false,
                        disabled: false,
                        handler: Some(Box::new(move || {
                            let mut c = client2.lock();
                            c.enqueue_delete(format!("network/{}", nwid));
                            c.remember_network(
                                nwid.clone(),
                                network_name.clone(),
                                settings.clone(),
                            );
                        })),
                    });

                    menu.push(TrayMenuItem::Submenu {
                        text: format!(
                            "{}\t{}  ",
                            (*network).0,
                            nw_obj.get("name").map_or("", |n| n.as_str().unwrap_or(""))
                        ),
                        checked: true,
                        items: network_menu,
                    });
                }
            }

            for j in joining.lock().iter() {
                networks_empty = false;
                menu.push(TrayMenuItem::Text {
                    text: j.clone(),
                    checked: false,
                    disabled: true,
                    handler: None,
                });
            }

            if networks_empty {
                menu.push(TrayMenuItem::Text {
                    text: "(no networks joined)".into(),
                    checked: false,
                    disabled: true,
                    handler: None,
                });
            }

            menu.push(TrayMenuItem::Separator);

            let saved_networks = client.lock().saved_networks();
            if !saved_networks.is_empty() {
                let mut saved_networks_empty = true;
                for nw in saved_networks.iter() {
                    if !networks.iter().any(|x| (*x).0.eq(&(*nw).0)) {
                        let (client2, client3) = (client.clone(), client.clone());
                        let (nwid2, nwid3, settings) =
                            ((*nw).0.clone(), (*nw).0.clone(), (*nw).2.clone());
                        menu.push(TrayMenuItem::Submenu {
                            text: format!("{}\t{}  ", (*nw).0, (*nw).1),
                            checked: false,
                            items: vec![
                                TrayMenuItem::Text {
                                    text: "Reconnect".into(),
                                    checked: false,
                                    disabled: false,
                                    handler: Some(Box::new(move || {
                                        client2.lock().enqueue_post(
                                            format!("network/{}", nwid2),
                                            settings.clone(),
                                        )
                                    })),
                                },
                                TrayMenuItem::Text {
                                    text: "Forget".into(),
                                    checked: false,
                                    disabled: false,
                                    handler: Some(Box::new(move || {
                                        client3.lock().forget_network(&nwid3)
                                    })),
                                },
                            ],
                        });
                        saved_networks_empty = false;
                    }
                }
                if !saved_networks_empty {
                    menu.push(TrayMenuItem::Separator);
                }
            }

            if !login_needed_networks.is_empty() {
                login_needed_networks.sort_unstable();
                for (nwid, auth_url) in login_needed_networks.drain(..) {
                    menu.push(TrayMenuItem::Text {
                        text: format!("{}\tOpen SSO Login URL... ", nwid),
                        checked: false,
                        disabled: false,
                        handler: Some(Box::new(move || {
                            let _ = webbrowser::open(&auth_url);
                        })),
                    });
                }
                menu.push(TrayMenuItem::Separator);
            }

            #[cfg(target_os = "macos")]
            {
                let dirty_flag2 = dirty_flag.clone();
                menu.push(TrayMenuItem::Text {
                    text: "Start UI at Login ".into(),
                    checked: unsafe { START_ON_LOGIN },
                    disabled: false,
                    handler: Some(Box::new(move || {
                        for _ in 0..2 {
                            refresh_mac_start_on_login();
                            let previous_start_on_login = unsafe { START_ON_LOGIN };
                            if previous_start_on_login {
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
                            if previous_start_on_login != unsafe { START_ON_LOGIN } {
                                break;
                            } else {
                                // If toggling failed, re-request permission to send apple events as this is required to toggle.
                                let _ = Command::new("/usr/bin/tccutil").arg("reset").arg("AppleEvents").arg("com.zerotier.ZeroTier-UI").output();
                            }
                        }
                        dirty_flag2.store(true, std::sync::atomic::Ordering::Relaxed);
                    }))
                });
            }

            #[cfg(windows)]
            {
                let dirty_flag2 = dirty_flag.clone();
                menu.push(TrayMenuItem::Text {
                    text: "Start UI at Login ".into(),
                    checked: unsafe { START_ON_LOGIN },
                    disabled: false,
                    handler: Some(Box::new(move || {
                        refresh_windows_start_on_login();
                        let hkcu = winreg::RegKey::predef(winreg::enums::HKEY_CURRENT_USER);
                        let startup =
                            hkcu.create_subkey("SOFTWARE\\Microsoft\\Windows\\CurrentVersion\\Run"); // opens read/write if exists
                        if startup.is_ok() {
                            let startup = startup.unwrap().0;
                            if unsafe { START_ON_LOGIN } {
                                let _ = startup.delete_value("ZeroTierUI");
                            } else {
                                let exe = String::from(
                                    std::env::current_exe().unwrap().to_str().unwrap(),
                                );
                                let _ = startup.set_value("ZeroTierUI", &exe);
                            }
                        }
                        refresh_windows_start_on_login();
                        dirty_flag2.store(true, std::sync::atomic::Ordering::Relaxed);
                    })),
                });
            }

            let about_child2 = about_child.clone();
            menu.push(TrayMenuItem::Text {
                text: "About ".into(),
                checked: false,
                disabled: false,
                handler: Some(Box::new(move || {
                    let mut child = about_child2.lock();
                    if child.is_none() {
                        let ch = Command::new(std::env::current_exe().unwrap())
                            .arg("about")
                            .arg(version.as_str())
                            .spawn();
                        if ch.is_ok() {
                            let _ = child.insert(ch.unwrap());
                        }
                    }
                })),
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
            handler: Some(Box::new(move || {
                exit_flag2.store(true, std::sync::atomic::Ordering::SeqCst)
            })),
        });

        menu
    };

    let tray = Tray::init(icon_name.as_ref(), refresh());

    loop {
        // Reap subprocesses if finished.
        for child in [&about_child].iter() {
            let mut child = child.lock();
            if child.is_some() {
                if child.as_mut().unwrap().try_wait().is_ok() {
                    let _ = child.take();
                }
            }
        }

        // Check for authentication required networks and notify.
        let auth_required_networks = client.lock().sso_auth_needed_networks();
        for (nwid, auth_url, status) in auth_required_networks.iter() {
            if status == "AUTHENTICATION_REQUIRED"
                && sso_notification_shown.lock().insert(nwid.clone())
            {
                notify(
                    format!("ZeroTier network {} requires SSO authentication. Select 'Open SSL Login URL' to proceed.", nwid).as_str(),
                    Some(("Open SSL Login URL...".into(), auth_url.clone()))
                );
            }
        }

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

    for child in [&about_child].iter() {
        let mut child = child.lock();
        child.take().map(|mut c| c.kill());
    }
}

/*******************************************************************************************************************/

fn main() {
    #[cfg(target_os = "macos")]
    {
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
            APPLICATION_HOME = format!(
                "{}/Library/Application Support/ZeroTier",
                std::env::var("HOME").unwrap_or(String::from("/tmp"))
            );
        }
        refresh_mac_start_on_login();
    }

    #[cfg(not(target_os = "macos"))]
    {
        unsafe {
            APPLICATION_PATH = String::from(std::env::current_exe().unwrap().to_str().unwrap());
        }
        #[cfg(windows)]
        {
            refresh_windows_start_on_login();
            unsafe {
                APPLICATION_HOME = format!(
                    "{}\\AppData\\Local\\ZeroTier",
                    std::env::var("USERPROFILE")
                        .unwrap_or(std::env::var("HOMEPATH").unwrap_or(String::from("C:\\")))
                );
            }
        }
        #[cfg(not(windows))]
        {
            unsafe {
                let _ = std::env::var("HOME").map_or_else(
                    |_| {
                        APPLICATION_HOME = String::from("/tmp/zerotier_ui");
                    },
                    |home_dir| {
                        APPLICATION_HOME = format!("{}/.zerotier_ui", home_dir);
                    },
                );
            }
        }
    }

    unsafe {
        let _ = std::fs::create_dir_all(APPLICATION_HOME.as_str());
        NETWORK_CACHE_PATH = String::from(
            Path::new(&APPLICATION_HOME)
                .join("saved_networks.json")
                .to_str()
                .unwrap(),
        );
    }

    // Import old network list info from old Mac UI.
    #[cfg(target_os = "macos")]
    {
        if !Path::new(unsafe { NETWORK_CACHE_PATH.as_str() }).is_file() {
            let mut nwid: Option<u64> = None;
            let mut name: Option<String> = None;
            let mut networks: HashMap<String, String> = HashMap::new();
            let _ = plist::Value::from_file(format!("{}/One/networkinfo.dat", unsafe {
                &APPLICATION_HOME
            }))
            .map(|old_plist| {
                old_plist.as_dictionary().map(|old_plist| {
                    old_plist.get("$objects").map(|old_plist| {
                        old_plist.as_array().map(|old_plist| {
                            for v in old_plist.iter() {
                                parse_mac_network_plist(
                                    old_plist,
                                    v,
                                    &mut networks,
                                    &mut nwid,
                                    &mut name,
                                );
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
            let _ = std::fs::write(
                unsafe { NETWORK_CACHE_PATH.as_str() },
                serde_json::to_vec(&networks_json).unwrap(),
            );
        }
    }

    // Import old network list info from old Windows UI
    #[cfg(windows)]
    {
        if !Path::new(unsafe { NETWORK_CACHE_PATH.as_str() }).is_file() {
            let _ = std::env::var("USERPROFILE").map(|uprof| {
                std::fs::read(format!(
                    "{}\\AppData\\Local\\ZeroTier\\One\\networks.dat",
                    uprof
                ))
                .map(|windows_networks_bin| {
                    let mut networks_json: HashMap<String, HashMap<String, String>> =
                        HashMap::new();
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
                    let _ = std::fs::write(
                        unsafe { NETWORK_CACHE_PATH.as_str() },
                        serde_json::to_vec(&networks_json).unwrap(),
                    );
                })
            });
        }
    }

    let args: Vec<String> = std::env::args().collect();
    if args.len() >= 2 {
        match args[1].as_str() {
            "about" => {
                let version = if args.len() >= 3 {
                    args[2].as_str()
                } else {
                    "???"
                };
                about::about_main(version)
            }
            "join_prompt" => join::join_main(),
            "copy_authtoken" => {
                // invoked with elevated permissions to get the auth token and copy it locally
                if args.len() < 3 {
                    println!("FATAL: copy_authtoken requires additional argument");
                    std::process::exit(1);
                }
                let _ = serviceclient::get_auth_token_and_port(false, &Ok(args[2].clone()));
            }
            _ => println!("FATAL: unrecognized mode: {}", args[1]),
        }
    } else {
        drop(args);
        tray_main();
    }
    std::process::exit(0);
}
