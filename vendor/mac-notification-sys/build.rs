extern crate cc;

fn main() {
    if cfg!(target_os = "macos") {
        cc::Build::new()
            .file("objc/notify.m")
            .flag("-fmodules")
            .warnings(false)
            .compile("notify");
    }
}
