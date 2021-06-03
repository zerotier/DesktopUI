// (c)2021 ZeroTier, Inc.

mod tray;

use serde::{Deserialize, Serialize};
use tray::*;
use std::process::{Child, Command};
use std::sync::{Mutex, Arc, MutexGuard};

#[derive(Serialize, Deserialize)]
pub struct CommandFromWebView {
    pub cmd: String
}

fn check_main_menu_exit(mw: &mut MutexGuard<Option<Child>>) {
    if mw.is_some() {
        let res = mw.as_mut().unwrap().try_wait();
        if res.is_ok() && res.ok().unwrap().is_some() {
            let _ = mw.take();
        }
    }
}

#[cfg(target_os = "macos")]
fn get_web_ui_blob() -> String {
    std::fs::read_to_string(std::env::current_exe().unwrap().parent().unwrap().parent().unwrap().join("Resources").join("ui.html")).unwrap()
    //include_str!("../ui/dist/index.html").into()
}

fn main() {
    let exe = std::env::current_exe();
    if exe.is_err() {
        println!("FATAL: unable to get exe path: {}", exe.err().unwrap().to_string());
        std::process::exit(1);
    }
    let exe = exe.ok().unwrap();
    let args = std::env::args();

    if std::env::args().len() == 2 {
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
                    _ => {}
                }
                Ok(())
            })
            .run()
            .unwrap();
    } else {
        let main_window: Arc<Mutex<Option<Child>>> = Arc::new(Mutex::new(None));

        let main_window2 = main_window.clone();
        let main_window3 = main_window.clone();
        let t = Tray::init("mac-dark.png", vec![
            TrayMenuItem::Text {
                text: "Node ID: ".into(),
                checked: false,
                disabled: false,
                handler: None,
            },
            TrayMenuItem::Separator,
            TrayMenuItem::Text {
                text: "Join Network...".into(),
                checked: false,
                disabled: false,
                handler: None,
            },
            TrayMenuItem::Text {
                text: "Open Console...".into(),
                checked: false,
                disabled: false,
                handler: Some(Box::new(move || {
                    let mut mw = main_window2.lock().unwrap();
                    check_main_menu_exit(&mut mw);
                    if mw.is_none() {
                        let ch = Command::new(exe.clone()).arg("MainWindow").spawn();
                        if ch.is_ok() {
                            let _ = mw.replace(ch.unwrap());
                        }
                    }
                })),
            },
            TrayMenuItem::Separator,
            TrayMenuItem::Text {
                text: "About".into(),
                checked: false,
                disabled: false,
                handler: None,
            },
            TrayMenuItem::Text {
                text: "Quit ZeroTier UI".into(),
                checked: false,
                disabled: false,
                handler: Some(Box::new(move || {
                    let mut mw = main_window3.lock().unwrap();
                    if mw.is_some() {
                        let _ = mw.as_mut().unwrap().kill();
                    }
                    std::process::exit(0);
                })),
            }]);

        //let mut i = 0;
        loop {
            //println!("loop {}", i);
            //i += 1;
            if t.poll() {
                let mut mw = main_window.lock().unwrap();
                check_main_menu_exit(&mut mw);
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
