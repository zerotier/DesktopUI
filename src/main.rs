// (c)2021 ZeroTier, Inc.

mod tray;
mod serviceclient;

use serde::{Deserialize, Serialize};
use tray::*;
use std::process::{Child, Command};
use std::sync::{Mutex, Arc, MutexGuard};

#[derive(Serialize, Deserialize)]
pub struct CommandFromWebView {
    pub cmd: String
}

fn check_window_subprocess_exit(w: &mut MutexGuard<Option<Child>>) {
    if w.is_some() {
        let res = w.as_mut().unwrap().try_wait();
        if res.is_ok() && res.ok().unwrap().is_some() {
            let _ = w.take();
        }
    }
}

fn kill_window_subprocess(mut w: MutexGuard<Option<Child>>) {
    if w.is_some() {
        let _ = w.as_mut().unwrap().kill();
    }
}

fn open_window_subprocess(mut w: MutexGuard<Option<Child>>, ui_mode: &str) {
    check_window_subprocess_exit(&mut w);
    if w.is_none() {
        let ch = Command::new(std::env::current_exe().unwrap()).arg(ui_mode).spawn();
        if ch.is_ok() {
            let _ = w.replace(ch.unwrap());
        }
    }
}

const CSS_PLACEHOLDER: &'static str = "/* XXXthis_is_replaced_by_css_in_the_rust_codeXXX */";

#[cfg(target_os = "macos")]
#[inline(always)]
fn get_web_ui_blob() -> String {
    let resources_path = std::env::current_exe().unwrap().parent().unwrap().parent().unwrap().join("Resources");
    let ui = std::fs::read_to_string(resources_path.join("ui.html")).map_or_else(|_| {
        "<html><body>Error: unable to load ui.html from application bundle Resources.<script>window.zt_ui_render = function(window_type) {}; setTimeout(function() { external.invoke('{ \"cmd\": \"ready\" }'); }, 1);</script></body></html>".into()
    }, |ui| {
        ui.replace(CSS_PLACEHOLDER, std::fs::read_to_string(resources_path.join("dark.css")).unwrap_or(String::new()).into())
    });
}

#[cfg(all(unix, not(target_os = "macos")))]
fn get_web_ui_blob() -> String {
    include_str!("../ui/dist/index.html").replace(CSS_PLACEHOLDER, include_str!("../ui/node_modules/@elastic/eui/dist/eui_theme_amsterdam_dark.css"))
}

fn main() {
    if std::env::args().len() == 2 {
        // If there's an argument, enter window mode and pop up a UI. The
        // argument is passed through and is used to determine which UI to
        // display.

        let ui_mode = std::env::args().last().unwrap();
        let _ = web_view::builder()
            .title("ZeroTier")
            .content(web_view::Content::Html(get_web_ui_blob()))
            .size(800, 600)
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

        let main_window: Arc<Mutex<Option<Child>>> = Arc::new(Mutex::new(None));
        let join_network_window: Arc<Mutex<Option<Child>>> = Arc::new(Mutex::new(None));
        let about_window: Arc<Mutex<Option<Child>>> = Arc::new(Mutex::new(None));

        let main_window2 = main_window.clone();
        let main_window3 = main_window.clone();
        let join_network_window2 = join_network_window.clone();
        let join_network_window3 = join_network_window.clone();
        let about_window2 = about_window.clone();
        let about_window3 = about_window.clone();
        let t = Tray::init("mac-dark.png", vec![
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
                    open_window_subprocess(join_network_window2.lock().unwrap(), "JoinNetwork");
                })),
            },
            TrayMenuItem::Text {
                text: "Control Panel... ".into(),
                checked: false,
                disabled: false,
                handler: Some(Box::new(move || {
                    open_window_subprocess(main_window2.lock().unwrap(), "MainWindow");
                })),
            },
            TrayMenuItem::Separator,
            TrayMenuItem::Text {
                text: "About ".into(),
                checked: false,
                disabled: false,
                handler: Some(Box::new(move || {
                    open_window_subprocess(about_window2.lock().unwrap(), "About");
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
            }]);

        loop {
            if t.poll() {
                let mut mw = main_window.lock().unwrap();
                check_window_subprocess_exit(&mut mw);
            } else {
                break;
            }
        }

        let _ = main_window.lock().unwrap().take().map(|mut mw| {
            let _ = mw.kill();
            let _ = mw.wait();
        });

    }
}
