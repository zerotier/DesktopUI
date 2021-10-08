# winrt-rust [![crates.io](https://img.shields.io/crates/v/winrt.svg)](https://crates.io/crates/winrt) [![docs.rs](https://docs.rs/winrt/badge.svg)](https://docs.rs/winrt/*/x86_64-pc-windows-msvc/winrt/)

This crate provides type and method definitions to use the *Windows Runtime (WinRT)* APIs from Rust.

## Status
This library is still subject to breaking changes, but it is already possible to use all APIs, including asynchronous ones
(a completion handler can be passed as a closure).
Since we can not yet guarantee the safety of the generated wrappers, all methods are currently marked as `unsafe`.
Creating custom WinRT classes using inheritance is not yet supported, so it is currently not possible to create user interfaces using *XAML*. 

## Prerequisites
Using this crate requires at least Rust 1.20. A compatibility mode that requires only Rust 1.15 can be enabled with the `lang-compat` Cargo feature.
Additional nightly features (e.g. using specialization) can be enabled with the `nightly` Cargo feature.

## Design
All definitions are automatically generated from *WinMD* metadata files.
The module structure of the generated code reflects the namespace structure of the original definitions
starting at `winrt::windows` (if the crate has not been renamed on import) for the `Windows` namespace.

All names have been adjusted to fit the Rust coding style, therefore module names are all in lower case and function names
are converted to `snake_case`.

Since it takes a long time to compile all generated definitions (the [generated files](https://github.com/contextfree/winrt-rust/blob/master/src/rt/gen/) amount to more than 15 MB),
Cargo features have been introduced that correspond to the *WinMD* files. For example, to use the definitions from `Windows.Devices.winmd`, use the feature `windows-devices`.
There is no feature for definitions from `Windows.Foundation.winmd`, these are always available. Whenever a (method) definition references a type from a different *WinMD* file,
it is also not available until you enable the corresponding features for all required type definitions.

With only the definitions from `Windows.Foundation`, this crates takes about 10 seconds to compile. With all features enabled (there is a shortcut feature `all`), compilation can take
as long as 5 minutes, so it is highly recommended to enable features only as you need them.

## Example
```rust
extern crate winrt;

use winrt::*; // import various helper types
use winrt::windows::system::diagnostics::*; // import namespace Windows.System.Diagnostics

fn main() {
    let rt = RuntimeContext::init(); // initialize the Windows Runtime
    let infos = ProcessDiagnosticInfo::get_for_processes().unwrap();
    println!("Currently executed processes ({}):", unsafe { infos.get_size().unwrap() });
    for p in infos.into_iter() {
        let pid = unsafe { p.get_process_id().unwrap() };
        let exe = unsafe { p.get_executable_file_name().unwrap() };
        println!("[{}] {}", pid, exe);
    }
}
```

Because this example uses the `Windows.System` namespace, we have to enable the `windows-system` feature in `Cargo.toml`:
```toml
[dependencies.winrt]
version = "0.3.0"
features = ["windows-system"]
```

Running this example program should result in an output similar to the following:
```
Currently executed processes (132):
[4] System
[392] smss.exe
[520] csrss.exe
[604] wininit.exe
[612] csrss.exe
[708] winlogon.exe
...
```

## WinRT and UWP
The *Windows Runtime (WinRT)* has been introduced in Windows 8 and provides the foundation for building
Windows apps that run on different devices using different programming languages. The *Universal Windows Platform (UWP)* is
an extension of WinRT, introduced in Windows 10, that allows using additional, more platform-specific APIs besides those provided by WinRT (according to
[MSDN](https://msdn.microsoft.com/en-us/windows/uwp/get-started/universal-application-platform-guide)).
*WinRT* is not to be confused with the discontinued flavor of the Windows operating system for ARM devices, *Windows RT*.

## Changelog

#### Version 0.4.0 (2017-12-27)
- [Breaking] Upgrade to winapi 0.3
- [Breaking] Default constructors are now accessible via `RtDefaultConstructible` trait
- [Breaking] Fixed and improved error handling (among other changes, `blocking_get()` now returns `Result`)
- [Breaking] Output array parameters are now passed as mutable slices (`&mut [T]`)
- Provide access to [`IMemoryBufferByteAccess`](https://docs.microsoft.com/en-us/uwp/api/windows.foundation.memorybuffer)
- Add another example (`hexdump`)

#### Version 0.3.0 (2017-07-21)
- [Breaking] The `self` parameter for interface calls is now passed as `&self` instead of `&mut self`
- [Breaking] Remove (empty) contract structs from generated code
- Documentation improvements

#### Version 0.2.1 (2017-04-01)
- Add `blocking_get()` for async operations
- Add toast notification example

#### Version 0.2.0 (2017-03-10)
- Factories and statics now actually work
- [Breaking] Feature names use dash instead of underscore

#### Version 0.1.0 (2016-09-28)
- First release

## License

Licensed under either of

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally
submitted for inclusion in the work by you, as defined in the Apache-2.0
license, shall be dual licensed as above, without any additional terms or
conditions.
