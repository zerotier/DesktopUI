Rust bindings for
[WebView2](https://docs.microsoft.com/en-us/microsoft-edge/hosting/webview2):

> The Microsoft Edge WebView2 control enables you to embed web technologies
(HTML, CSS, and JavaScript) in your native applications. The WebView2 control
uses Microsoft Edge (Chromium) as the rendering engine to display the web
content in native applications. With WebView2, you may embed web code in
different parts of your native application, or build the entire native
application within a single WebView. For information on how to start building a
WebView2 application, see [Get
Started](https://docs.microsoft.com/en-us/microsoft-edge/webview2/#getting-started).

# API

The `webview2` crate contains high-level, idiomatic wrappers for the raw COM
APIs, which can be found in the `webview2-sys` crate.

The API mapping should be quite straightforward.

The `CreateCoreWebView2EnvironmentWithDetails` function does not have a direct
equivalent. It is replaced with a nicer `EnvironmentBuilder` API. The
`GetAvailableCoreWebView2BrowserVersionString` and `CompareBrowserVersions`
functions are also exposed through the builder.

# Runtime

The Edge browser from beta, dev or canary channels (>= 86.0.622.0) or the
[Evergreen WebView2
Runtime](https://docs.microsoft.com/en-us/microsoft-edge/webview2/concepts/distribution#understand-the-webview2-runtime-and-installer-preview)
need to be installed for this to actually work. Or the
[`build`](struct.EnvironmentBuilder.html#method.build) method will return an
error.

# WebView2Loader

A binary library `WebView2Loader` from the WebView2 SDK need to be used, either
the DLL `WebView2Loader.dll` or the static library `WebView2LoaderStatic.lib`.
This brings some complexities:

* When using the **gnu** toolchain, the static library does not seem to work so
  the `WebView2Loader.dll` DLL is used. You need to make sure that the DLL can
  be loaded at runtime, e.g. by **putting it alongside the built exe files**.

* When using the **msvc** toolchain, the static library is used. Make sure you
  have the **v142** toolset (or **visual studio 2019**), because the static
  library seem to be built with visual studio 2019 and could not be correctly
  linked by earlier versions of the visual studio. See [C++ binary compatibility
  between Visual Studio 2015, 2017, and
  2019](https://docs.microsoft.com/en-us/cpp/porting/binary-compat-2015-2017?view=vs-2019).

# Examples

See the `examples` directory, especially the heavily commented `win32` example.
