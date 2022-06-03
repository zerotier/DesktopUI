#[allow(unused_assignments)]
#[allow(unused_mut)]
fn main() {
    let d = env!("CARGO_MANIFEST_DIR");
    println!("cargo:rustc-link-search=native={}/tray", d);
    println!(
        "cargo:rustc-link-search=native={}/libui-ng/build/meson-out",
        d
    );
    println!("cargo:rustc-link-lib=static=zt_desktop_tray");
    println!("cargo:rustc-link-lib=static=ui");
    #[cfg(target_os = "linux")] {
        println!("cargo:rustc-link-lib=dylib=gtk-3");
        println!("cargo:rustc-link-lib=dylib=gdk-3");
        println!("cargo:rustc-link-lib=dylib=gobject-2.0");
        println!("cargo:rustc-link-lib=dylib=glib-2.0");
        println!("cargo:rustc-link-lib=dylib=appindicator3");
    }
}
