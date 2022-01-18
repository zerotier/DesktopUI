// Copyright 2013-2017, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

extern crate libc;

use libc::{c_char, c_double, c_void};

pub type JSCValue = *mut c_void;
pub type JSCContext = *mut c_void;
pub type JSGlobalContextRef = *mut c_void;
pub type JSValueRef = *mut c_void;
pub type JSStringRef = *mut c_void;

extern "C" {
    pub fn JSValueIsBoolean(ctx: JSGlobalContextRef, value: JSValueRef) -> u8;
    pub fn JSValueIsNull(ctx: JSGlobalContextRef, value: JSValueRef) -> u8;
    pub fn JSValueIsUndefined(ctx: JSGlobalContextRef, value: JSValueRef) -> u8;
    pub fn JSValueIsNumber(ctx: JSGlobalContextRef, value: JSValueRef) -> u8;
    pub fn JSValueIsString(ctx: JSGlobalContextRef, value: JSValueRef) -> u8;
    pub fn JSValueIsObject(ctx: JSGlobalContextRef, value: JSValueRef) -> u8;
    pub fn JSValueIsArray(ctx: JSGlobalContextRef, value: JSValueRef) -> u8;
    pub fn JSValueIsDate(ctx: JSGlobalContextRef, value: JSValueRef) -> u8;
    pub fn JSValueToNumber(ctx: JSGlobalContextRef, value: JSValueRef, exception: *mut JSValueRef) -> c_double;
    pub fn JSValueToBoolean(ctx: JSGlobalContextRef, value: JSValueRef) -> u8;
    pub fn JSValueToStringCopy(ctx: JSGlobalContextRef, value: JSValueRef, exception: *mut JSValueRef) -> JSStringRef;
    pub fn JSStringRelease(string: JSStringRef);
    pub fn JSStringGetMaximumUTF8CStringSize(string: JSStringRef) -> usize;
    pub fn JSStringGetUTF8CString(string: JSStringRef, buffer: *mut c_char, buffer_size: usize) -> usize;
}
