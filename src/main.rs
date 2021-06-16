// (c)2021 ZeroTier, Inc.

mod tray;
mod serviceclient;

use std::process::{Child, Command, Stdio};
use std::sync::{Mutex, Arc, MutexGuard};
use std::time::Duration;
use std::io::Write;
use std::cmp::Ordering;
use std::sync::atomic::AtomicBool;
use std::os::raw::{c_int, c_char};
use std::ffi::CString;

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
}

#[cfg(target_os = "macos")]
extern "C" {
    pub fn c_set_this_thread_to_background_priority();
    pub fn c_set_this_thread_to_foreground_priority();
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
}

#[cfg(all(unix, not(target_os = "macos")))]
fn tray_icon_name() -> &'static str {
    if is_dark_mode() { "generic-dark.png" } else { "generic-light.png" }
}

#[cfg(windows)]
fn tray_icon_name() -> &'static str {
    "windows.ico"
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

#[cfg(windows)]
fn copy_to_clipboard(s: &str) {
    CString::new(s).map(|s| {
        unsafe { c_windows_post_to_clipboard(s.as_ptr()) };
    });
}

/*******************************************************************************************************************/

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

#[cfg(not(target_os = "macos"))]
#[inline(always)]
fn get_web_ui_blob() -> String {
    let css = if is_dark_mode() { include_str!("../ui/dist/dark.css") } else { include_str!("../ui/dist/light.css") };
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
        let ch = Command::new(std::env::current_exe().unwrap()).arg("window").arg(ui_mode).arg(width.to_string()).arg(height.to_string()).spawn();
        if ch.is_ok() {
            let _ = w.replace(ch.unwrap());
        }
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
        .frameless(WEBVIEW_WINDOW_FRAMELESS)
        .debug(false)
        .user_data(())
        .invoke_handler(move |wv, arg| {
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

    //set_thread_to_background_priority();

    let mut icon_name = tray_icon_name();
    let mut tray: Option<Tray> = None;
    let exit_flag = Arc::new(AtomicBool::new(false));

    let (client, dirty_flag) = start_client(vec!["status", "network"], 500, 20);

    let main_window: Arc<Mutex<Option<Child>>> = Arc::new(Mutex::new(None));
    let join_network_window: Arc<Mutex<Option<Child>>> = Arc::new(Mutex::new(None));
    let about_window: Arc<Mutex<Option<Child>>> = Arc::new(Mutex::new(None));

    loop {
        if dirty_flag.swap(false, std::sync::atomic::Ordering::Relaxed) {
            let (main_window2, main_window3) = (main_window.clone(), main_window.clone());
            let (join_network_window2, join_network_window3) = (join_network_window.clone(), join_network_window.clone());
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
                                items: managed_routes_menu,
                            });
                        }
                        network_menu.push(TrayMenuItem::Separator);
                        let (nwid, client2) = ((*network).0.clone(), client.clone());
                        network_menu.push(TrayMenuItem::Text {
                            text: "Leave Network ".into(),
                            checked: false,
                            disabled: false,
                            handler: Some(Box::new(move || client2.lock().unwrap().enqueue_delete(format!("network/{}", nwid)))),
                        });

                        menu.push(TrayMenuItem::Submenu {
                            text: format!("{}\t{}  ", (*network).0, nw_obj.get("name").map_or("", |n| n.as_str().unwrap_or(""))),
                            items: network_menu,
                        });
                    });
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
                let exit_flag2 = exit_flag.clone();
                menu.push(TrayMenuItem::Text {
                    text: "Quit ZeroTier UI ".into(),
                    checked: false,
                    disabled: false,
                    handler: Some(Box::new(move || {
                        for w in [&main_window3, &join_network_window3, &about_window3].iter() {
                            kill_window_subprocess(w.lock().unwrap());
                        }
                        exit_flag2.store(true, std::sync::atomic::Ordering::Relaxed);
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
            } else if exit_flag.load(std::sync::atomic::Ordering::Relaxed) {
                break;
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
