mod tray;
mod window;

use tray::*;
use std::process::{Child, Command};
use std::sync::{Mutex, Arc, MutexGuard};

fn check_main_menu_exit(mw: &mut MutexGuard<Option<Child>>) {
    if mw.is_some() {
        let res = mw.as_mut().unwrap().try_wait();
        if res.is_ok() && res.ok().unwrap().is_some() {
            let _ = mw.take();
        }
    }
}

fn main() {
    let exe = std::env::current_exe();
    if exe.is_err() {
        println!("FATAL: unable to get exe path: {}", exe.err().unwrap().to_string());
        std::process::exit(1);
    }
    let exe = exe.ok().unwrap();
    let args = std::env::args();

    if args.len() == 2 && args.last().unwrap() == "mainwindow" {
        let html_content = "<html><body><h1>Hello, World!</h1></body></html>";
        web_view::builder()
            .title("My Project")
            .content(web_view::Content::Html(html_content))
            .size(320, 480)
            .resizable(false)
            .debug(false)
            .user_data(())
            .invoke_handler(|_webview, _arg| Ok(()))
            .run()
            .unwrap();
    } else {
        let main_window: Arc<Mutex<Option<Child>>> = Arc::new(Mutex::new(None));
        let main_window2 = main_window.clone();

        let t = Tray::init("icon.png", vec![MenuItem::Text{
            text: "foo".into(),
            checked: false,
            disabled: false,
            handler: Some(Box::new(move || {
                let mut mw = main_window2.lock().unwrap();
                check_main_menu_exit(&mut mw);
                if mw.is_none() {
                    let ch = Command::new(exe.clone()).arg("mainwindow").spawn();
                    if ch.is_ok() {
                        let _ = mw.replace(ch.unwrap());
                    }
                }
            })),
        }]);

        loop {
            if !t.poll() {
                break;
            } else {
                let mut mw = main_window.lock().unwrap();
                check_main_menu_exit(&mut mw);
            }
        }

        let _ = main_window.lock().unwrap().take().map(|mut mw| {
            let _ = mw.kill();
            let _ = mw.wait();
        });
    }
}
