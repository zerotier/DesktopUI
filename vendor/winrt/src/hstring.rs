use std::ptr;
use std::fmt;
use std::cmp;
use std::mem;
use std::marker::PhantomData;
use std::ops::Deref;

use w::shared::basetsd::UINT32;
use w::shared::ntdef::LPCWSTR;
use w::shared::winerror::S_OK;
use w::winrt::hstring::{HSTRING, HSTRING__, HSTRING_HEADER};
use w::winrt::winstring::{WindowsCreateString, WindowsGetStringLen, WindowsIsStringEmpty,
                          WindowsGetStringRawBuffer, WindowsCreateStringReference,
                          WindowsDuplicateString, WindowsDeleteString, WindowsCompareStringOrdinal};

// For some information about HSTRINGs, see http://ksav.com.np/tech/2016/06/16/raymonds-complete-guide-to-hstring-semantics/

// Some helper functions
#[inline]
fn internal_to_string(hstr: HSTRING) -> String {
    unsafe {
        let mut len = 0;
        let buf = WindowsGetStringRawBuffer(hstr, &mut len);
        let slice: &[u16] = ::std::slice::from_raw_parts(buf, len as usize);
        String::from_utf16_lossy(slice)
    }
}

#[inline]
fn internal_cmp(left: HSTRING, right: HSTRING) -> cmp::Ordering {
    let mut result = 0;
    assert!(unsafe { WindowsCompareStringOrdinal(left, right, &mut result) } == S_OK);
    match result {
        -1 => cmp::Ordering::Less,
        0 => cmp::Ordering::Equal,
        1 => cmp::Ordering::Greater,
        _ => unreachable!()
    }
}

#[inline]
fn zero_header() -> HSTRING_HEADER {
    HSTRING_HEADER {
        Reserved: unsafe { mem::zeroed() }
    }
}

/// A reference to either an `HString`, a `FastHString`, or a raw null-terminated UTF-16 buffer.
#[derive(Copy, Clone)]
pub struct HStringReference<'a>(HSTRING_HEADER, PhantomData<&'a ()>);

impl<'a> HStringReference<'a> {
    #[inline]
    /// Creates a new `HStringReference` from a UTF-16 encoded slice, which must be null-terminated.
    /// This function does not allocate and is the fastest option if you already have UTF-16 encoded
    /// data.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use winrt::HStringReference;
    /// let sparkle_heart = vec![0xD83D, 0xDC96, 0x0];
    /// let r = HStringReference::from_utf16(&sparkle_heart);
    /// assert_eq!("💖", r.to_string());
    /// ```
    pub fn from_utf16(slice: &'a [u16]) -> HStringReference<'a> {
        assert!(slice[slice.len() - 1] == 0, "input must be null-terminated");
        unsafe { HStringReference::from_utf16_unchecked(slice) }
    }

    /// Creates a new `HStringReference` from a UTF-16 encoded slice, which must be null-terminated.
    /// This function is unsafe because the caller must make sure that the data really is null-terminated.
    pub unsafe fn from_utf16_unchecked(slice: &'a [u16]) -> HStringReference<'a> {
        let mut hstrref: HStringReference = HStringReference(zero_header(), PhantomData);
        if slice.len() == 0 { return hstrref; }
        let mut hstr: HSTRING = ptr::null_mut();
        assert_eq!(WindowsCreateStringReference(slice as *const _ as LPCWSTR, slice.len() as u32 - 1, &mut hstrref.0, &mut hstr), S_OK);
        // The returned HSTRING is actually a pointer to the returned HSTRING_HEADER,
        // which we check here and then forget about `hstr`.
        debug_assert_eq!(hstr as *const _, &hstrref.0 as *const _ as *const HSTRING__);
        hstrref
    }

    /// Returns the length of the string in Unicode characters, as specified by `WindowsGetStringLen`.
    #[inline]
    pub fn len(&self) -> u32 {
        unsafe { WindowsGetStringLen(self.as_hstring()) }
    }

    /// Checks whether the string is empty.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use winrt::HString;
    /// let s = HString::empty();
    /// assert!(s.is_empty());
    /// ```
    #[inline]
    pub fn is_empty(&self) -> bool {
        unsafe { WindowsIsStringEmpty(self.as_hstring()) != 0 }
    }

    /// Returns the `HSTRING` that this instance is wrapping.
    #[inline]
    unsafe fn as_hstring(&self) -> HSTRING {
        // Since HSTRING is just a pointer to HSTRING_HEADER in disguise, we can just return
        // a pointer to our wrapper header and cast it accordingly.
        &self.0 as *const HSTRING_HEADER as *mut HSTRING_HEADER as *mut HSTRING__
    }
}

// Common trait impls for HStringReference<'a>
#[cfg(feature = "nightly")]
impl<'a> ToString for HStringReference<'a> {
    fn to_string(&self) -> String {
        internal_to_string(unsafe { self.as_hstring() })
    }
}

impl<'a> fmt::Display for HStringReference<'a> {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        formatter.write_str(&internal_to_string(unsafe { self.as_hstring() }))
    }
}

impl<'a> cmp::PartialOrd for HStringReference<'a> {
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
        Some(unsafe { internal_cmp(self.as_hstring(), other.as_hstring()) })
    }
}

impl<'a> cmp::Ord for HStringReference<'a> {
    #[inline]
    fn cmp(&self, other: &Self) -> cmp::Ordering {
        unsafe { internal_cmp(self.as_hstring(), other.as_hstring()) }
    }
}

impl<'a, 'b> cmp::PartialEq<HStringReference<'a>> for HStringReference<'b> {
    #[inline]
    fn eq(&self, other: &HStringReference) -> bool {
        unsafe { internal_cmp(self.as_hstring(), other.as_hstring()) == cmp::Ordering::Equal }
    }
}

impl<'a> cmp::Eq for HStringReference<'a> {}

impl<'a> Deref for HStringReference<'a> {
    type Target = HStringArg;

    #[inline]
    fn deref(&self) -> &HStringArg {
        unsafe { &*(self as *const HStringReference as *const HStringArg) }
    }
}

/// A string type that should be used to create strings that can be passed to Windows Runtime
/// functions. Creating a new `FastHString` is faster than creating an instance of `HString`
/// because it eliminates an additional allocation. Furthermore, obtaining a `HStringArg` from
/// a `FastHString` is basically free, which is not the case for `HString`.
pub struct FastHString(HSTRING_HEADER);

impl FastHString {
    /// Creates a new `FastHString` from a Rust string. `FastHString` uses the Rust allocator to
    /// create a storage buffer for its contents, where the string is stored in UTF-16 encoding.
    /// The buffer is freed on when the `FastHString` is dropped.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use winrt::FastHString;
    /// let s = FastHString::new("hello");
    /// assert_eq!("hello", s.to_string());
    /// ```
    pub fn new(s: &str) -> FastHString {
        let mut hstrref: FastHString = FastHString(zero_header());
        if s.is_empty() {
            return hstrref;
        }
        // Every UTF-8 byte results in either 1 or 2 UTF-16 bytes and we need one
        // more for the null terminator. This size expectation is correct in most cases,
        // so the vector doesn't need to reallocate.
        let mut s16: Vec<u16> = Vec::with_capacity(s.len() + 1);
        for c in s.encode_utf16() {
            s16.push(c);
        }
        let len = s16.len();
        s16.push(0x0u16); // add null-terminator
        s16.shrink_to_fit();
        {
            // Prevent double allocation by directly creating a reference into the memory allocated by the Vec.
            // Then mem::forget the Vec, we can reassemble it in Drop.
            let slice: &[u16] = &s16;
            let mut hstr: HSTRING = ptr::null_mut();
            assert_eq!(unsafe { WindowsCreateStringReference(slice as *const _ as LPCWSTR, len as UINT32, &mut hstrref.0, &mut hstr) }, S_OK);
            // The returned HSTRING is actually a pointer to the returned HSTRING_HEADER,
            // which we check here and then forget about `hstr`.
            assert_eq!(hstr as *const _, &hstrref.0 as *const _ as *const HSTRING__);
        }
        mem::forget(s16);
        hstrref
    }

    /// Creates an empty `FastHString`.
    #[inline]
    pub fn empty() -> FastHString {
        FastHString(zero_header())
    }
    
    /// Returns the `HSTRING` that this instance is wrapping.
    #[inline]
    unsafe fn as_hstring(&self) -> HSTRING {
        // Since HSTRING is just a pointer to HSTRING_HEADER in disguise, we can just return
        // a pointer to our wrapped header and cast it accordingly.
        &self.0 as *const HSTRING_HEADER as *mut HSTRING_HEADER as *mut HSTRING__
    }

    /// Returns the length of the string in Unicode characters, as specified by `WindowsGetStringLen`.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use winrt::FastHString;
    /// let s = FastHString::new("hello");
    /// assert_eq!(5, s.len());
    /// ```
    #[inline]
    pub fn len(&self) -> u32 {
        unsafe { WindowsGetStringLen(self.as_hstring()) }
    }

    /// Checks whether the string is empty.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use winrt::FastHString;
    /// let s = FastHString::empty();
    /// assert!(s.is_empty());
    /// ```
    #[inline]
    pub fn is_empty(&self) -> bool {
        unsafe { WindowsIsStringEmpty(self.as_hstring()) != 0 }
    }

    /// Creates an `HStringReference` that points to the contents of this `FastHString`. 
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use winrt::FastHString;
    /// let s = FastHString::new("hello");
    /// let r = s.make_reference();  
    /// assert_eq!("hello", r.to_string());
    /// ```   
    #[inline]
    pub fn make_reference(&self) -> HStringReference {
        // Creating another reference is basically free,
        // since we're already managing our own memory.
        HStringReference(self.0, PhantomData)
    }
}

impl Drop for FastHString {
    fn drop(&mut self) {
        let mut len = 0;
        let buf = unsafe { WindowsGetStringRawBuffer(self.as_hstring(), &mut len) };
        unsafe { Vec::from_raw_parts(buf as *mut u16, len as usize + 1, len as usize + 1) };
        // The Vec will be dropped now, which frees the HString's backing memory
    }
}


impl<'a> From<&'a str> for FastHString {
    #[inline]
    fn from(s: &'a str) -> Self {
        FastHString::new(s)
    }
}

// Common trait impls for FastHString
#[cfg(feature = "nightly")]
impl ToString for FastHString {
    fn to_string(&self) -> String {
        internal_to_string(unsafe { self.as_hstring() })
    }
}

impl fmt::Display for FastHString {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        formatter.write_str(&internal_to_string(unsafe { self.as_hstring() }))
    }
}

impl cmp::PartialOrd for FastHString {
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
        Some(unsafe { internal_cmp(self.as_hstring(), other.as_hstring()) })
    }
}

impl cmp::Ord for FastHString {
    #[inline]
    fn cmp(&self, other: &Self) -> cmp::Ordering {
        unsafe { internal_cmp(self.as_hstring(), other.as_hstring()) }
    }
}

impl cmp::PartialEq<FastHString> for FastHString {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        unsafe { internal_cmp(self.as_hstring(), other.as_hstring()) == cmp::Ordering::Equal }
    }
}

impl cmp::Eq for FastHString {}

impl<'a> Deref for FastHString {
    type Target = HStringArg;

    #[inline]
    fn deref(&self) -> &HStringArg {
        unsafe { &*(self as *const FastHString as *const HStringArg) }
    }
}

/// References of this type are passed to WinRT functions. You can not create a value of
/// this type, only references can exist and are obtained via (automatic) dereferencing of
/// `FastHString` or `HStringReference`.
pub struct HStringArg(HSTRING_HEADER);

impl HStringArg {
    #[inline]
    pub unsafe fn get(&self) -> HSTRING {
        // Since HSTRING is just a pointer to HSTRING_HEADER in disguise, we can just return
        // a pointer to our wrapper header and cast it accordingly.
        &self.0 as *const HSTRING_HEADER as *mut HSTRING_HEADER as *mut HSTRING__
    }
}

/// A wrapper over an `HSTRING` whose memory is managed by the Windows Runtime.
/// This is what you get as return values when calling WinRT methods that return strings.
/// Note that dereferencing to `&HStringArg` is not implemented for this, because
/// the containing `HSTRING` might be null (empty string), and null references
/// are not allowed. In order to obtain an `&HStringArg` from an `HString`,
/// first create an `HStringReference` using `make_reference()`.
pub struct HString(HSTRING);

impl HString {
    /// Creates a new `HString` whose memory is managed by the Windows Runtime.
    /// This allocates twice (once for the conversion to UTF-16, and again within `WindowsCreateString`),
    /// therefore this should not be used. Use `FastHString::new()` instead.
    pub fn new<'a>(s: &'a str) -> HString {
        // Every UTF-8 byte results in either 1 or 2 UTF-16 bytes and we need one
        // more for the null terminator. This size expectation is correct in most cases,
        // so the vector doesn't need to reallocate.
        let mut s16: Vec<u16> = Vec::with_capacity(s.len() + 1);
        for c in s.encode_utf16() {
            s16.push(c);
        }
        let len = s16.len();
        s16.push(0x0u16); // add null-terminator
        let mut hstr = HString(ptr::null_mut());
        let slice: &[u16] = &s16;
        let res = unsafe { WindowsCreateString(slice as *const _ as LPCWSTR, len as UINT32, &mut hstr.0) };
        assert!(res == S_OK);
        hstr
    }
    
    /// Wraps an existing `HSTRING` inside this `HString`. This is only safe
    /// when the `HSTRING` was allocated by the Windows Runtime.
    #[inline]
    pub unsafe fn wrap(hstr: HSTRING) -> HString {
        HString(hstr)
    }
    
    /// Creates an empty `HString`.
    #[inline]
    pub fn empty() -> HString {
        HString(ptr::null_mut()) // an empty HSTRING is represented by a null-pointer
    }
    
    /// Returns the length of the string in Unicode characters, as specified by `WindowsGetStringLen`.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use winrt::HString;
    /// let s = HString::new("hello");
    /// assert_eq!(5, s.len());
    /// ```
    #[inline]
    pub fn len(&self) -> u32 {
        // This is okay even if pointer is null (returns 0)
        unsafe { WindowsGetStringLen(self.0) }
    }
    
    /// Checks whether the string is empty.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use winrt::HString;
    /// let s = HString::empty();
    /// assert!(s.is_empty());
    /// ```
    #[inline]
    pub fn is_empty(&self) -> bool {
        unsafe { WindowsIsStringEmpty(self.0) != 0 }
    }

    /// Returns the `HSTRING` that this instance is wrapping.
    #[inline]
    unsafe fn as_hstring(&self) -> HSTRING {
        self.0
    }

    /// Creates an `HStringReference` that points to the contents of this `HString`. 
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use winrt::HString;
    /// let s = HString::new("hello");
    /// let r = s.make_reference();  
    /// assert_eq!(s.to_string(), r.to_string());
    /// ```    
    #[inline]
    pub fn make_reference<'a>(&'a self) -> HStringReference<'a> {
        let mut len = 0;
        let buf = unsafe { WindowsGetStringRawBuffer(self.0, &mut len) };
        unsafe { HStringReference::from_utf16_unchecked(::std::slice::from_raw_parts(buf, len as usize + 1)) }
    }
}

impl Drop for HString {
    #[inline]
    fn drop(&mut self) {
        // This is okay even if the pointer is null
        unsafe { WindowsDeleteString(self.0) };
    }
}

impl ::std::clone::Clone for HString {
    #[inline]
    fn clone(&self) -> Self {
        let mut clone = HString::empty();
        let hres = unsafe { WindowsDuplicateString(self.0, &mut clone.0) };
        assert!(hres == S_OK);
        clone
    }
}

// Common trait impls for HString
#[cfg(feature = "nightly")]
impl ToString for HString {
    fn to_string(&self) -> String {
        internal_to_string(unsafe { self.as_hstring() })
    }
}

impl fmt::Display for HString {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        formatter.write_str(&internal_to_string(unsafe { self.as_hstring() }))
    }
}

impl cmp::PartialOrd for HString {
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
        Some(unsafe { internal_cmp(self.as_hstring(), other.as_hstring()) })
    }
}

impl cmp::Ord for HString {
    #[inline]
    fn cmp(&self, other: &Self) -> cmp::Ordering {
        unsafe { internal_cmp(self.as_hstring(), other.as_hstring()) }
    }
}

impl cmp::PartialEq<HString> for HString {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        unsafe { internal_cmp(self.as_hstring(), other.as_hstring()) == cmp::Ordering::Equal }
    }
}

impl cmp::Eq for HString {}

// PartialEq impls for comparison of different types
impl<'a> cmp::PartialEq<HString> for HStringReference<'a> {
    #[inline]
    fn eq(&self, other: &HString) -> bool {
        unsafe { internal_cmp(self.as_hstring(), other.as_hstring()) == cmp::Ordering::Equal }
    }
}
impl<'a> cmp::PartialEq<HStringReference<'a>> for HString {
    #[inline]
    fn eq(&self, other: &HStringReference) -> bool {
        unsafe { internal_cmp(self.as_hstring(), other.as_hstring()) == cmp::Ordering::Equal }
    }
}
impl<'a> cmp::PartialEq<FastHString> for HStringReference<'a> {
    #[inline]
    fn eq(&self, other: &FastHString) -> bool {
        unsafe { internal_cmp(self.as_hstring(), other.as_hstring()) == cmp::Ordering::Equal }
    }
}
impl<'a> cmp::PartialEq<HStringReference<'a>> for FastHString {
    #[inline]
    fn eq(&self, other: &HStringReference) -> bool {
        unsafe { internal_cmp(self.as_hstring(), other.as_hstring()) == cmp::Ordering::Equal }
    }
}
impl cmp::PartialEq<FastHString> for HString {
    #[inline]
    fn eq(&self, other: &FastHString) -> bool {
        unsafe { internal_cmp(self.as_hstring(), other.as_hstring()) == cmp::Ordering::Equal }
    }
}
// TODO: How to generalize this for all kinds of references?
impl<'a> cmp::PartialEq<&'a FastHString> for HString {
    #[inline]
    fn eq(&self, other: &&'a FastHString) -> bool {
        unsafe { internal_cmp(self.as_hstring(), other.as_hstring()) == cmp::Ordering::Equal }
    }
}
impl cmp::PartialEq<HString> for FastHString {
    #[inline]
    fn eq(&self, other: &HString) -> bool {
        unsafe { internal_cmp(self.as_hstring(), other.as_hstring()) == cmp::Ordering::Equal }
    }
}

#[cfg(test)]
mod tests {
    extern crate test;
    
    use super::*;
    use self::test::Bencher;

    #[test]
    fn check_sizes() {
        use ::std::mem::size_of;
        assert_eq!(size_of::<::HString>(), size_of::<::w::winrt::hstring::HSTRING>());
        assert_eq!(size_of::<&::HStringArg>(), size_of::<::w::winrt::hstring::HSTRING>());
    }

    #[test]
    fn roundtrip() {
        let s = "12345";
        let hstr = HString::new(s);
        assert!(hstr.len() as usize == s.len());
        assert!(s == hstr.to_string());
    }

    #[test]
    fn roundtrip_fast() {
        let s = "12345";
        let hstr = FastHString::new(s);
        assert!(hstr.len() as usize == s.len());
        assert!(s == hstr.to_string());
    }

    #[test]
    fn make_reference() {
        let s1 = HString::new("AAA");
        assert_eq!(s1.make_reference().to_string(), "AAA");
    }
    
    #[test]
    fn empty() {
        let hstr = HString::empty();
        assert!(hstr.len() == 0);
        assert!(hstr.to_string().len() == 0);
    }

    #[test]
    fn empty_fast() {
        let hstr = FastHString::empty();
        assert!(hstr.len() == 0);
        assert!(hstr.to_string().len() == 0);
    }

    #[test]
    fn empty_ref() {
        let hstr = FastHString::empty();
        let hstrref = hstr.make_reference();
        assert!(hstrref.len() == 0);
        assert!(hstrref.to_string().len() == 0);
    }
    
    #[test]
    fn is_empty() {
        let hstr = HString::empty();
        assert!(hstr.is_empty());
        let hstr = HString::new("");
        assert!(hstr.is_empty());
        let hstr = HString::new("\0");
        assert!(!hstr.is_empty());
    }

    #[test]
    fn is_empty_fast() {
        let hstr = FastHString::empty();
        assert!(hstr.is_empty());
        let hstr = FastHString::new("");
        assert!(hstr.is_empty());
        let hstr = FastHString::new("\0");
        assert!(!hstr.is_empty());
    }
    
    #[test]
    fn clone() {
        let s = "123456789";
        let hstr = HString::new(s);
        let clone = hstr.clone();
        assert!(clone.to_string() == s);
        drop(clone);
        assert!(hstr.to_string() == s);
    }
    
    #[test]
    fn cmp() {
        let s1 = HString::new("AAA");
        let s2 = HString::new("BBB");
        let s3 = HString::new("AAA");
        
        assert!(s2 > s1);
        assert!(s2 != s3);
        assert!(s1 == s3);
    }

    #[test]
    fn cmp2() {
        let s1 = HString::new("AAA");
        let s2 = FastHString::new("BBB");
        let s3 = s1.make_reference();
        let s4 = FastHString::new("AAA");
        
        assert!(s2 != s1);
        assert!(s2 != s3);
        assert!(s1 == s3);
        assert!(s1 == s4);
        assert!(s2 != s4);
    }

    #[bench]
    fn bench_create(b: &mut Bencher) {
        let s = "123456789";
        b.iter(|| {
            let _ = HString::new(s);
        });;
    }

    #[bench]
    fn bench_create_fast(b: &mut Bencher) {
        let s = "123456789";
        b.iter(|| {
            let _ = FastHString::new(s);
        });
    }

    #[bench]
    fn bench_make_reference(b: &mut Bencher) {
        let s = HString::new("123456789");
        b.iter(|| {
            let _ = s.make_reference();
        });
    }

    #[bench]
    fn bench_make_reference_fast(b: &mut Bencher) {
        let s = FastHString::new("123456789");
        b.iter(|| {
            let _ = s.make_reference();
        });
    }

    #[bench]
    fn bench_from_utf16(b: &mut Bencher) {
        let utf16: Vec<_> = "This is some test string".encode_utf16().chain(Some(0)).collect();
        b.iter(|| {
            let _ = HStringReference::from_utf16(&utf16);
        });
    }
    
    #[bench]
    fn bench_to_string(b: &mut Bencher) {
        let hstr = FastHString::new("123456789");
        b.iter(|| {
            let _ = hstr.to_string();
        });
    }
}