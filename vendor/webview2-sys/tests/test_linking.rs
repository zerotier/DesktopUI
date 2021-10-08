use core::ptr::{null, null_mut};

// Just make sure that we can actually call a function from the SDK, i.e. linking is successful.
#[test]
fn test_linking() {
    unsafe { webview2_sys::CompareBrowserVersions(null(), null(), null_mut()) };
}
