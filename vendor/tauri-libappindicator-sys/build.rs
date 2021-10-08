fn main() {
    println!("cargo:rustc-link-lib=appindicator3");

    if let Err(_) = pkg_config::probe_library("appindicator3") {
        if let Err(_) = pkg_config::probe_library("appindicator3-0.1") {
            panic!("libappindicator3 library not found!");
        }
    }
}

