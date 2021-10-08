// Copyright 2013-2017, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

extern crate glib;
extern crate javascriptcore_sys;

use std::ptr;

use glib::translate::{FromGlibPtrFull, FromGlibPtrNone};
use javascriptcore_sys::*;

pub struct GlobalContextRef {
    raw: JSGlobalContextRef,
}

pub struct ValueRef {
    raw: JSValueRef,
}

pub struct Value {
    raw: JSCValue,
}

impl ValueRef {
    pub fn is_boolean(&self, context: &GlobalContextRef) -> bool {
        unsafe { JSValueIsBoolean(context.raw, self.raw) != 0 }
    }

    pub fn is_null(&self, context: &GlobalContextRef) -> bool {
        unsafe { JSValueIsNull(context.raw, self.raw) != 0 }
    }

    pub fn is_undefined(&self, context: &GlobalContextRef) -> bool {
        unsafe { JSValueIsUndefined(context.raw, self.raw) != 0 }
    }

    pub fn is_number(&self, context: &GlobalContextRef) -> bool {
        unsafe { JSValueIsNumber(context.raw, self.raw) != 0 }
    }

    pub fn is_string(&self, context: &GlobalContextRef) -> bool {
        unsafe { JSValueIsString(context.raw, self.raw) != 0 }
    }

    pub fn is_object(&self, context: &GlobalContextRef) -> bool {
        unsafe { JSValueIsObject(context.raw, self.raw) != 0 }
    }

    pub fn is_array(&self, context: &GlobalContextRef) -> bool {
        unsafe { JSValueIsArray(context.raw, self.raw) != 0 }
    }

    pub fn is_date(&self, context: &GlobalContextRef) -> bool {
        unsafe { JSValueIsDate(context.raw, self.raw) != 0 }
    }

    pub fn to_number(&self, context: &GlobalContextRef) -> Option<f64> {
        let mut exception = ptr::null_mut();
        let result = unsafe { JSValueToNumber(context.raw, self.raw, &mut exception) };
        if exception.is_null() {
            Some(result)
        } else {
            None
        }
    }

    pub fn to_boolean(&self, context: &GlobalContextRef) -> bool {
        unsafe { JSValueToBoolean(context.raw, self.raw) != 0}
    }

    pub fn to_string(&self, context: &GlobalContextRef) -> Option<String> {
        unsafe {
            let mut exception = ptr::null_mut();
            let jsstring = JSValueToStringCopy(context.raw, self.raw, &mut exception);

            if exception.is_null() {
                let cap = JSStringGetMaximumUTF8CStringSize(jsstring);
                let mut buf = Vec::<u8>::with_capacity(cap);
                let len = JSStringGetUTF8CString(jsstring, buf.as_mut_ptr() as _, cap);
                JSStringRelease(jsstring);
                buf.set_len(len - 1);
                String::from_utf8(buf).ok()
            } else {
                None
            }
        }
    }
}

impl FromGlibPtrNone<JSValueRef> for ValueRef {
    unsafe fn from_glib_none(ptr: JSValueRef) -> Self {
        ValueRef {
            raw: ptr,
        }
    }
}

impl FromGlibPtrFull<JSValueRef> for ValueRef {
    unsafe fn from_glib_full(ptr: JSValueRef) -> Self {
        ValueRef {
            raw: ptr,
        }
    }
}

impl FromGlibPtrNone<*mut JSCValue> for Value {
    unsafe fn from_glib_none(ptr: *mut JSCValue) -> Self {
        assert!(ptr != ptr::null_mut());
        Value {
            raw: *ptr,
        }
    }
}

impl FromGlibPtrNone<JSGlobalContextRef> for GlobalContextRef {
    unsafe fn from_glib_none(ptr: JSGlobalContextRef) -> Self {
        GlobalContextRef {
            raw: ptr,
        }
    }
}

impl FromGlibPtrFull<JSGlobalContextRef> for GlobalContextRef {
    unsafe fn from_glib_full(ptr: JSGlobalContextRef) -> Self {
        GlobalContextRef {
            raw: ptr,
        }
    }
}
