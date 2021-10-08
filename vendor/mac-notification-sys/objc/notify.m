#import "notify.h"

// getBundleIdentifier(app_name: &str) -> "com.apple.Terminal"
NSString *getBundleIdentifier(NSString *appName){
        NSString *findString = [NSString stringWithFormat:@"get id of application \"%@\"", appName];
        NSAppleScript *findScript = [[NSAppleScript alloc] initWithSource:findString];
        NSAppleEventDescriptor *resultDescriptor = [findScript executeAndReturnError:nil];
        return [resultDescriptor stringValue];
}

// setApplication(new_bundle_identifier: &str) -> Result<()>
BOOL setApplication(NSString *newbundleIdentifier) {
        if(LSCopyApplicationURLsForBundleIdentifier((CFStringRef)newbundleIdentifier, NULL) != NULL) {
                fakeBundleIdentifier = newbundleIdentifier;
                return YES;
        }
        return NO;
}

// scheduleNotification(title: &str, subtitle: &str message: &str, sound: &str, f64) -> NotificationResult<()>
bool scheduleNotification(NSString *title, NSString *subtitle, NSString *message, NSString *sound, double deliveryDate) {
        @autoreleasepool {
                if (!installNSBundleHook()) {
                        return NO;
                }
                NSDate *scheduleTime = [NSDate dateWithTimeIntervalSince1970:deliveryDate];
                NSUserNotificationCenter *nc = [NSUserNotificationCenter defaultUserNotificationCenter];
                NotificationCenterDelegate *ncDelegate = [[NotificationCenterDelegate alloc] init];
                ncDelegate.keepRunning = YES;
                nc.delegate = ncDelegate;

                NSUserNotification *note = [[NSUserNotification alloc] init];
                note.title = title;
                if (![subtitle isEqualToString:@""]) {
                        note.subtitle = subtitle;
                }
                note.informativeText = message;
                note.deliveryDate = scheduleTime;
                if (![sound isEqualToString:@"_mute"]) {
                        note.soundName = sound;
                }
                [nc scheduleNotification:note];
                [NSThread sleepForTimeInterval:0.1f];
                return YES;
        }
}

// sendNotification(title: &str, subtitle: &str, message: &str, sound: &str) -> NotificationResult<()>
bool sendNotification(NSString *title, NSString *subtitle, NSString *message, NSString *sound) {
        @autoreleasepool {
                if (!installNSBundleHook()) {
                        return NO;
                }

                NSUserNotificationCenter *nc = [NSUserNotificationCenter defaultUserNotificationCenter];
                NotificationCenterDelegate *ncDelegate = [[NotificationCenterDelegate alloc] init];
                ncDelegate.keepRunning = YES;
                nc.delegate = ncDelegate;

                NSUserNotification *note = [[NSUserNotification alloc] init];
                note.title = title;
                if (![subtitle isEqualToString:@""]) {
                        note.subtitle = subtitle;
                }
                note.informativeText = message;
                if (![sound isEqualToString:@"_mute"]) {
                        note.soundName = sound;
                }
                [nc deliverNotification:note];

                [NSThread sleepForTimeInterval:0.1f];
                return YES;
        }
}
