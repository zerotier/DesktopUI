/*
 * Copyright (c) 2016 Boucher, Antoni <bouanto@zoho.com>
 *
 * Permission is hereby granted, free of charge, to any person obtaining a copy of
 * this software and associated documentation files (the "Software"), to deal in
 * the Software without restriction, including without limitation the rights to
 * use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of
 * the Software, and to permit persons to whom the Software is furnished to do so,
 * subject to the following conditions:
 *
 * The above copyright notice and this permission notice shall be included in all
 * copies or substantial portions of the Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS
 * FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR
 * COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER
 * IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN
 * CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.
 */

extern crate glib;
extern crate gio;
extern crate gtk;
extern crate webkit2gtk;

#[cfg(feature = "v2_4")]
use glib::ToVariant;
use gtk::{Inhibit, Window, WindowType, prelude::*};
use webkit2gtk::{traits::{SettingsExt, WebContextExt, WebViewExt}, WebContext, WebView};
#[cfg(feature = "v2_6")]
use webkit2gtk::UserContentManager;

fn main() {
    gtk::init().unwrap();

    let window = Window::new(WindowType::Toplevel);
    let context = WebContext::default().unwrap();
    #[cfg(feature = "v2_4")]
    context.set_web_extensions_initialization_user_data(&"webkit".to_variant());
    context.set_web_extensions_directory("../webkit2gtk-webextension-rs/example/target/debug/");
    #[cfg(feature = "v2_6")]
    let webview = WebView::new_with_context_and_user_content_manager(&context, &UserContentManager::new());
    #[cfg(not(feature = "v2_6"))]
    let webview = WebView::with_context(&context);
    webview.load_uri("https://crates.io/");
    window.add(&webview);

    let settings = WebViewExt::settings(&webview).unwrap();
    settings.set_enable_developer_extras(true);

    /*let inspector = webview.get_inspector().unwrap();
    inspector.show();*/

    window.show_all();

    webview.run_javascript("alert('Hello');", None::<&gio::Cancellable>, |_result| {
    });
    webview.run_javascript("42", None::<&gio::Cancellable>, |result| {
        match result {
            Ok(result) => {
                let context = result.global_context().unwrap();
                let value = result.value().unwrap();
                println!("is_boolean: {}", value.is_boolean(&context));
                println!("is_number: {}", value.is_number(&context));
                println!("{:?}", value.to_number(&context));
                println!("{:?}", value.to_boolean(&context));
            },
            Err(error) => println!("{}", error),
        }
    });

    window.connect_delete_event(|_, _| {
        gtk::main_quit();
        Inhibit(false)
    });

    gtk::main();
}
