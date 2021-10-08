use std::path::PathBuf;

const WEBVIEW_SDK_VERSION: &str = "1.0.961.33";

fn main() {
    println!("cargo:rerun-if-changed=build.rs");

    if std::env::var("CARGO_CFG_TARGET_OS").unwrap() != "windows" {
        return;
    }
    let is_msvc = std::env::var("CARGO_CFG_TARGET_ENV").map_or(false, |env| env == "msvc");

    let manifest_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap();
    let mut manifest_dir = PathBuf::from(manifest_dir);
    manifest_dir.push("Microsoft.Web.WebView2.".to_owned() + WEBVIEW_SDK_VERSION);
    manifest_dir.push("build");
    manifest_dir.push("native");
    let arch = match std::env::var("CARGO_CFG_TARGET_ARCH").unwrap().as_str() {
        "x86_64" => "x64",
        "x86" => "x86",
        "aarch64" => "arm64",
        arch => panic!("Unsupported target arch: {}", arch),
    };
    manifest_dir.push(arch);
    let lib_dir = manifest_dir;
    // Link to the static library if using an MSVC toolchain, or to the DLL if using a GNU toolchain.
    println!(
        "cargo:rustc-link-lib={}",
        if is_msvc {
            "WebView2LoaderStatic"
        } else {
            "WebView2Loader.dll"
        }
    );
    println!("cargo:rustc-link-search={}", lib_dir.display());
}
