extern crate winrt_notification;
use winrt_notification::{
    Duration,
    Sound,
    Toast,
};

fn main() {
    let duration = Duration::Short;
    let sound = Some(Sound::SMS);

    Toast::new(Toast::POWERSHELL_APP_ID)
        .title("first toast")
        .text1("line1")
        .duration(duration)
        .sound(sound)
        .show()
        // silently consume errors
        .expect("notification failed");

    Toast::new(Toast::POWERSHELL_APP_ID)
        .title("another toast")
        .text1("line1")
        .duration(duration)
        .sound(sound)
        .show()
        // silently consume errors
        .expect("notification failed");
}
