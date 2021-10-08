// Take a look at the license at the top of the repository in the LICENSE file.

/*
 * TODO: add Cargo categories.
 * TODO: add all unstable methods.
 */

extern crate bitflags;
extern crate cairo;
extern crate gdk;
extern crate gdk_sys;
extern crate gio;
extern crate gio_sys;
extern crate glib;
extern crate glib_sys;
extern crate gobject_sys;
extern crate gtk;
extern crate gtk_sys;
extern crate javascriptcore as java_script_core;
extern crate libc;

extern crate ffi;

macro_rules! assert_initialized_main_thread {
    () => (
        if !::gtk::is_initialized_main_thread() {
            if ::gtk::is_initialized() {
                panic!("GTK may only be used from the main thread.");
            }
            else {
                panic!("GTK has not been initialized. Call `gtk::init` first.");
            }
        }
    )
}

macro_rules! skip_assert_initialized {
    () => ()
}

mod auto;
mod script_dialog;
mod web_view;
mod web_context;

pub use glib::Error;

pub use auto::*;
pub use script_dialog::*;
pub use web_view::*;
pub use web_context::*;
