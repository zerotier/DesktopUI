extern crate winrt_notification;
use winrt_notification::{Duration, Sound, Toast};

fn main() {
    let duration = Duration::Short;
    let sound = Some(Sound::SMS);

    let toast = Toast::new(Toast::POWERSHELL_APP_ID)
        .title("first toast")
        .text1("line1")
        .duration(duration)
        .sound(sound);

    toast
        .show()
        // silently consume errors
        .expect("notification failed");

    toast
        .show()
        // silently consume errors
        .expect("notification failed");
}
