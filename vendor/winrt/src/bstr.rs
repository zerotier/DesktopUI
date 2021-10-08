use ::std::ptr;
use ::std::fmt;

use w::shared::wtypes::BSTR;
use w::shared::basetsd::UINT32;
use w::shared::wtypesbase::OLECHAR;
use w::um::oleauto::{SysStringLen, SysAllocStringLen, SysFreeString};

// TODO: move to separate crate?

/// A wrapper around `BSTR`, a string type used by classic COM. This is usually not needed when
/// working with WinRT, but can be helpful when interacting with other APIs.
pub struct BStr(BSTR);

impl<'a> From<&'a str> for BStr {
    
    fn from(s: &'a str) -> Self {
        // Every UTF-8 byte results in either 1 or 2 UTF-16 bytes. This size expectation is
        // correct in most cases, so the vector doesn't need to reallocate.
        let mut s16: Vec<u16> = Vec::with_capacity(s.len());
        for c in s.encode_utf16() {
            s16.push(c);
        }
        let len = s16.len();
        let slice: &[u16] = &s16;
        let bstr = unsafe { SysAllocStringLen(slice as *const _ as *const OLECHAR, len as UINT32) };
        BStr(bstr)
    }
}

impl BStr {    
    #[inline(always)]
    pub fn get(&self) -> BSTR {
        self.0
    }
    
    #[inline(always)]
    // TODO: maybe remove this
    pub unsafe fn wrap(bstr: BSTR) -> BStr {
        BStr(bstr)
    }
    
    #[inline(always)]
    pub fn empty() -> BStr {
        BStr(ptr::null_mut())
    }
    
    #[inline(always)]
    pub fn get_address(&mut self) -> &mut BSTR {
        &mut self.0
    }
    
    #[inline(always)]
    pub fn len(&self) -> u32 {
        // This is okay even if pointer is null (returns 0)
        unsafe { SysStringLen(self.0) }
    }
    
    #[inline(always)]
    fn internal_to_string(&self) -> String {
        unsafe {
            let len = self.len();
            let slice: &[u16] = ::std::slice::from_raw_parts(self.0, len as usize);
            String::from_utf16_lossy(slice)
        }
    }
}

#[cfg(feature = "nightly")]
impl ToString for BStr {
    fn to_string(&self) -> String {
        self.internal_to_string()
    }
}

impl fmt::Display for BStr {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        formatter.write_str(&self.internal_to_string())
    }
}

impl Drop for BStr {
    #[inline(always)]
    fn drop(&mut self) {
        // This is okay even if the pointer is null
        unsafe { SysFreeString(self.0) };
    }
}

#[cfg(test)]
mod tests {
    extern crate test;
    
    use super::*;
    use self::test::Bencher;

    #[test]
    fn roundtrip() {
        let s = "12345";
        let bstr: BStr = s.into();
        assert!(bstr.len() as usize == s.len());
        assert!(s == bstr.to_string());
    }
    
    #[test]
    fn empty() {
        let bstr = BStr::empty();
        assert!(bstr.len() == 0);
        assert!(bstr.to_string().len() == 0);
    }

    #[bench]
    fn bench_create(b: &mut Bencher) {
        let s = "123456789";
        b.iter(|| {
            let _: BStr = s.into();
        });;
    }
    
    #[bench]
    fn bench_to_string(b: &mut Bencher) {
        let bstr: BStr = "123456789".into();
        b.iter(|| {
            let _ = bstr.to_string();
        });
    }
}