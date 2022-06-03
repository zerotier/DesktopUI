ZeroTier Desktop Tray Application and User Interface
======

This is the system tray application and user interface for controlling a local ZeroTier service on Windows, macOS, and (soon) Linux systems.

# Building

Only macOS and Windows are currently supported. Linux may build but there are still outstanding issues. We're working on full Linux support at least for popular Linux desktop environments that support relatively standard tray application APIs.

## macOS

#### Prerequisites

 * Xcode with standard command line tools and SDKs.
 * Rust (and cargo) with targets `x86_64-apple-darwin` and `aarch64-apple-darwin` installed to enable universal binaries to be built.
 * The [Meson/Ninja](https://mesonbuild.com) build system (for libui-ng).

To build on macOS you should just be able to type `make` from the project root. If all the necessary dependencies are present it should build a `ZeroTier.app` application in the current directory.

## Windows

#### Prerequisites

 * [Microsoft Visual Studio 2022](https://visualstudio.microsoft.com/vs/) with both 32-bit and 64-bit X86 targets and with appropriate desktop application SDKs.
 * Rust (and cargo) with targets `x86_64-pc-windows-msvc` and `i686-pc-windows-msvc` installed.
 * The [Meson/Ninja](https://mesonbuild.com) build system (for libui-ng).
 * [GCC/G++](https://nuwen.net/mingw.html) with support for both 64-bit and 32-bit builds. Yes, we need both Visual Studio and GCC with GNU make.

To build native applications for Windows, just type `make`. This assumes that GNU make, GCC, and Cargo are in your path. The result will be two native EXEs in `target\x86_64-pc-windows-msvc\release` and `target\i686-pc-windows-msvc\release`. We plan to add native support for Windows on ARM64 soon, both for this UI application and for ZeroTier itself.

## Linux, FreeBSD, Other Open Source Desktops

#### Prerequisites

* Rust (and cargo) with 2021 edition support
* gtk-3
* gdk-3
* gobject-2.0
* glib-2.0
* libappindicator3

# Directly Incorporated Third Party Code

The ZeroTier desktop UI uses forked and slightly modified versions of the following third party code:

 * [Tray](https://github.com/zserge/tray) by [Serge Zaitsev](https://github.com/zserge), forked to slightly modify behavior in regard to loop timeouts and Mac application settings. We also manually applied a pull request that fixes builds on ARM64 macOS. (MIT license)
 * [LibUI-ng](https://github.com/libui-ng/libui-ng) by Pietro Gagliardi and others. (MIT license)

Other third party dependencies are included in the normal way. See [Cargo.toml](Cargo.toml) for these.

# License

Licensed under the Mozilla Public License (MPL) version 2.0.
