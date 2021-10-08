extern crate mac_notification_sys;
extern crate chrono;

use chrono::offset::*;
use mac_notification_sys::*;

#[test]
#[should_panic]
fn dont_schedle_in_past() {
    let stamp = Utc::now().timestamp() as f64 - 5.;
    let _sent = schedule_notification("Danger",
                                      &Some("Will Robinson"),
                                      "Run away as fast as you can",
                                      &Some("Blow"),
                                      stamp)
        .unwrap();
}

