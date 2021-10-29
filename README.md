ZeroTier Desktop Tray Application and User Interface
======

This is (as of ZeroTier 1.8) the system tray application and user interface for controlling a local ZeroTier service on Windows, macOS, and (soon) Linux systems.

It runs a very tiny tray application that displays service status, network status, and allows some basic configuration operations, and then launches a system web view container when the user wishes to display a full control panel. The web view dies when this window is closed, so unlike Electron-based or other web technology based system tray apps it does not hog memory while idle.

Plumbing and tray application glue code is written in Rust. UI control panels are written using [React](https://reactjs.org) and [ElasticUI](https://elastic.github.io/eui/#/).

# Building

Only macOS and Windows are currently supported. Linux may build but there are still outstanding issues. We're working on full Linux support at least for popular Linux desktop environments that support relatively standard tray application APIs.

Building the full HTML/JS/CSS UI bundle is a bit cumbersome but is only required if you make changes to the source code or themes under the [ui](ui/) directory. A pre-built copy of the UI is shipped in the repository to make simple builds easier and significantly faster.

## macOS

#### Prerequisites

 * Rust (and cargo) with targets `x86_64-apple-darwin` and `aarch64-apple-darwin` installed to enable universal binaries to be built.
 * Xcode with standard command line tools and SDKs.
 * We ship the JavaScript UI bundle pre-built. If you want to modify the JavaScript parts of the UI you need:
   * NodeJS (we use the Node package from Homebrew)
   * The [yarn](https://yarnpkg.com) package manager for NodeJS.

To build on macOS you should just be able to type `make` from the project root. If all the necessary dependencies are present it should build a `ZeroTier.app` application in the current directory.

To rebuild the UI, change into `ui/` and type `yarn install` (this is needed only once) and `yarn build`. As discussed above this is optional and is only needed if you make control panel UI changes.

## Windows

#### Prerequisites

 * Rust (and cargo) with targets `x86_64-pc-windows-msvc` and `i686-pc-windows-msvc` installed.
 * [Microsoft Visual Studio](https://visualstudio.microsoft.com/vs/) (we use 2019) with both 32-bit and 64-bit X86 targets and with appropriate desktop application SDKs.
 * [GCC/G++](https://nuwen.net/mingw.html) with support for both 64-bit and 32-bit builds (we use the linked distribution). Yes, we need both Visual Studio and GCC with GNU make.
 
To build native applications for Windows, just type `make`. This assumes that GNU make, GCC, and Cargo are in your path. The result will be two native EXEs in `target\x86_64-pc-windows-msvc\release` and `target\i686-pc-windows-msvc\release`. We plan to add native support for Windows on ARM64 soon, both for this UI application and for ZeroTier itself.

Building the web bundle part of the UI on Windows has never been done and those tool chains tend to be unfriendly to Windows. We recommend doing `yarn build` stuff in Linux or macOS. It should work in the Linux subsystem for Windows. As mentioned above we ship this pre-built to make builds easy if you don't need to modify the JavaScript code.

## Linux, FreeBSD, Other Open Source Desktops

*Coming soon.*

# Directly Incorporated Third Party Code

The ZeroTier desktop UI uses forked and slightly modified versions of the following third party code:

 * [Tray](https://github.com/zserge/tray) by [Serge Zaitsev](https://github.com/zserge), forked to slightly modify behavior in regard to loop timeouts and Mac application settings. We also manually applied a pull request that fixes builds on ARM64 macOS. (Retains the MIT license.)
 * [Rust web-view](https://github.com/Boscop/web-view) by multiple contributors, forked to modify Mac application behavior and add some custom functionality around copy/paste. (Retains the MIT license.)

Other third party dependencies are included in the normal way. See [Cargo.toml](Cargo.toml) and [ui/package.json](ui/package.json) for these.

# License

The main UI code in [src](src/) and [ui](ui/) is copyright ZeroTier, Inc. and is licensed under the Mozilla Public License (MPL) version 2.0.

Code in [tray](tray/) and [web-view](web-view/) is licensed under the MIT license, which is the original license used by the upstream projects.
