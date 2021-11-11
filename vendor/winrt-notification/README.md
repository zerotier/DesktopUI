# winrt-notification

[![license](https://img.shields.io/crates/l/winrt-notification.svg)](https://crates.io/crates/winrt-notification/)
[![version](https://img.shields.io/crates/v/winrt-notification.svg)](https://crates.io/crates/winrt-notification/)
[![Build Status](https://img.shields.io/appveyor/ci/allenbenz/winrt-notification.svg)](https://ci.appveyor.com/project/allenbenz/winrt-notification)

An incomplete wrapper over the WinRT toast api

Tested in Windows 10 and 8.1. Untested in Windows 8, might work.

[0.5 Documentation](https://allenbenz.github.io/winrt-notification/0_5_0/winrt_notification/index.html)

[0.2 Documentation](https://allenbenz.github.io/winrt-notification/0_2_0/winrt_notification/index.html)

Todo:
* Add support for Adaptive Content
* Add support for Actions

Known Issues:
* Will not work for Windows 7.

Limitations:
* Windows 8.1 only supports a single image, the last image (icon, hero, image) will be the one on the toast

## Usage

```toml
#Cargo.toml
[dependencies]
winrt-notification = "0.5.0"
```

## Examples

```rust
extern crate winrt_notification;
use winrt_notification::{Duration, Sound, Toast};

fn main() {
    Toast::new(Toast::POWERSHELL_APP_ID)
        .title("Look at this flip!")
        .text1("(╯°□°）╯︵ ┻━┻")
        .sound(Some(Sound::SMS))
        .duration(Duration::Short)
        .show()
        .expect("unable to toast");
}
```

```rust
extern crate winrt_notification;
use std::path::Path;
use winrt_notification::{IconCrop, Toast};

fn main() {
    Toast::new("Your AppUserModeId")
        .hero(&Path::new("C:\\absolute\\path\\to\\image.jpeg"), "alt text")
        .icon(
            &Path::new("c:/this/style/works/too/image.png"),
            IconCrop::Circular,
            "alt text",
        )
        .title("Lots of pictures here")
        .text1("One above the text as the hero")
        .text2("One to the left as an icon, and several below")
        .image(&Path::new("c:/photos/sun.png"), "the sun")
        .image(&Path::new("c:/photos/moon.png"), "the moon")
        .sound(None) // will be silent
        .show()
        .expect("unable to toast");
}
```
