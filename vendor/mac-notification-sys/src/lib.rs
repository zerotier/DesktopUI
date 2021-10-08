//! A very thin wrapper around NSNotifications
#![deny(
    missing_docs, trivial_casts, trivial_numeric_casts, unused_import_braces, unused_qualifications
)]
#![cfg(target_os = "macos")]
#![allow(improper_ctypes)]

extern crate chrono;
extern crate objc_foundation;
pub mod error;

use chrono::offset::*;
use error::{ ApplicationError, NotificationError, NotificationResult };
use objc_foundation::{INSString, NSString};
use std::ops::Deref;
use std::path::PathBuf;

static mut APPLICATION_SET: bool = false;

mod sys {
    use objc_foundation::NSString;
    #[link(name = "notify")]
    extern "C" {
        pub fn scheduleNotification(
            title: *const NSString,
            subtitle: *const NSString,
            message: *const NSString,
            sound: *const NSString,
            deliveryDate: f64,
        ) -> bool;
        pub fn sendNotification(
            title: *const NSString,
            subtitle: *const NSString,
            message: *const NSString,
            sound: *const NSString,
        ) -> bool;
        pub fn setApplication(newbundleIdentifier: *const NSString) -> bool;
        pub fn getBundleIdentifier(appName: *const NSString) -> *const NSString;
    }
}

/// Schedules a new notification in the NotificationCenter
///
/// Returns a `NotificationError` if a notification could not be scheduled
/// or is scheduled in the past
///
/// # Example:
///
/// ```ignore
/// extern crate chrono;
/// # use mac_notification_sys::*;
/// use chrono::offset::*;
///
/// // schedule a notification in 5 seconds
/// let _ = schedule_notification("Title", &None, "This is the body", &Some("Ping"),
///                               Utc::now().timestamp() as f64 + 5.).unwrap();
/// ```
pub fn schedule_notification(
    title: &str,
    subtitle: &Option<&str>,
    message: &str,
    sound: &Option<&str>,
    delivery_date: f64,
) -> NotificationResult<()> {
    ensure!(
        delivery_date >= Utc::now().timestamp() as f64,
        NotificationError::ScheduleInThePast
    );

    let use_sound = match sound {
        Some(sound) if check_sound(sound) => sound,
        _ => "_mute",
    };
    unsafe {
        ensure!(
            sys::scheduleNotification(
                NSString::from_str(title).deref(),
                NSString::from_str(subtitle.unwrap_or("")).deref(),
                NSString::from_str(message).deref(),
                NSString::from_str(use_sound).deref(),
                delivery_date,
            ),
            NotificationError::UnableToSchedule
        );
        Ok(())
    }
}

/// Delivers a new notification
///
/// Returns a `NotificationError` if a notification could not be delivered
///
/// # Example:
///
/// ```no_run
/// # use mac_notification_sys::*;
/// // daliver a silent notification
/// let _ = send_notification("Title", &None, "This is the body", &None).unwrap();
/// ```
pub fn send_notification(
    title: &str,
    subtitle: &Option<&str>,
    message: &str,
    sound: &Option<&str>,
) -> NotificationResult<()> {
    let use_sound = match sound {
        Some(sound) if check_sound(sound) => sound,
        _ => "_mute",
    };

    unsafe {
        ensure!(
            sys::sendNotification(
                NSString::from_str(title).deref(),
                NSString::from_str(subtitle.unwrap_or("")).deref(),
                NSString::from_str(message).deref(),
                NSString::from_str(use_sound).deref()
            ),
            NotificationError::UnableToDeliver
        );
        Ok(())
    }
}

/// Search for a possible BundleIdentifier of a given appname.
/// Defaults to "com.apple.Terminal" if no BundleIdentifier is found.
pub fn get_bundle_identifier_or_default(app_name: &str) -> String {
    get_bundle_identifier(app_name).unwrap_or_else(|| "com.apple.Terminal".to_string())
}

/// Search for a BundleIdentifier of an given appname.
pub fn get_bundle_identifier(app_name: &str) -> Option<String> {
    unsafe {
        sys::getBundleIdentifier(NSString::from_str(app_name).deref()) // *const NSString
            .as_ref() // Option<NSString>
            .map(NSString::as_str)
            .map(String::from)
    }
}

/// Set the application which delivers or schedules a notification
pub fn set_application(bundle_ident: &str) -> NotificationResult<()> {
    unsafe {
        ensure!(!APPLICATION_SET, ApplicationError::AlreadySet(bundle_ident.into()));
        APPLICATION_SET = true;
        ensure!(
            sys::setApplication(NSString::from_str(bundle_ident).deref()),
            ApplicationError::CouldNotSet(bundle_ident.into())
        );
        Ok(())
    }
}

fn check_sound(sound_name: &str) -> bool {
    dirs::home_dir()
        .map(|path| path.join("/Library/Sounds/"))
        .into_iter()
        .chain(
            [
                "/Library/Sounds/",
                "/Network/Library/Sounds/",
                "/System/Library/Sounds/",
            ].iter()
                .map(PathBuf::from),
        )
        .map(|sound_path| sound_path.join(format!("{}.aiff", sound_name)))
        .any(|some_path| some_path.exists())
}
