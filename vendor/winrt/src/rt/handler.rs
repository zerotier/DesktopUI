use std::sync::atomic;

use {IUnknown, IAgileObject, ComInterface, ComIid, ComPtr, Guid};

use w::shared::ntdef::{VOID, ULONG};
use w::shared::winerror::S_OK;
use w::shared::winerror::E_NOINTERFACE;
use w::shared::guiddef::REFIID;
use w::um::unknwnbase::IUnknownVtbl;

use result::HRESULT;

#[repr(C)]
pub struct ComRepr<T, Vtbl> {
    vtbl: Box<Vtbl>, // we can't use constant VTables because of generic parameters (constants can't be generic)
    refcount: atomic::AtomicUsize,
    data: T
}

/// This is a reusable implementation of AddRef that works for any ComRepr-based type
pub unsafe extern "system" fn ComRepr_AddRef<T>(this: *mut IUnknown) -> ULONG {
    let this = this as *mut _ as *mut ComRepr<T, IUnknownVtbl>;
    
    // Increment the reference count (count member).
    let old_size = (*this).refcount.fetch_add(1, atomic::Ordering::Relaxed);
    //println!("AddRef: {} -> {}", old_size, old_size  + 1);

    // We're supposed to return the updated count.
    return (old_size + 1) as ULONG;
}

/// This is a reusable implementation of Release that works for any ComRepr-based type
pub unsafe extern "system" fn ComRepr_Release<T>(this: *mut IUnknown) -> ULONG {
    let this = this as *mut _ as *mut ComRepr<T, IUnknownVtbl>;
    
    let old_size = (*this).refcount.fetch_sub(1, atomic::Ordering::Release);
    //println!("Release: {} -> {}", old_size, old_size - 1);
    if old_size != 1 {
        return (old_size - 1) as ULONG; // return the updated count
    }
    
    atomic::fence(atomic::Ordering::Acquire); 
    let b = Box::from_raw(this); // creates a Box which is then dropped
    drop(b); // Arc uses a trick to call this in an inline(never) function
    return 0;
}

pub unsafe extern "system" fn ComReprHandler_QueryInterface<T, I>(this_: *mut IUnknown, vTableGuid: REFIID, ppv: *mut *mut VOID) -> HRESULT
    where T: ComClass<I>, I: ComInterface + ComIid
{
    let this_ = this_ as *mut I;
    let guid: Guid = (*vTableGuid).into();
    //println!("QueryInterface called with GUID {:?}", guid);

    // IAgileObject is only supported for Send objects
    if guid != *IUnknown::iid() && guid != *IAgileObject::iid() && guid != *<I as ComIid>::iid() { 
        *ppv = ::std::ptr::null_mut();
        return E_NOINTERFACE;
    }
    *ppv = this_ as *mut _ as *mut VOID;
    ComRepr_AddRef::<T>(this_ as *mut IUnknown);
    S_OK
}

pub trait ComClass<Interface: ComInterface> where Self: Sized {
    fn get_vtbl() -> Interface::Vtbl;
    unsafe fn from_interface<'a>(thing: *mut Interface) -> &'a mut Self {
        &mut (*(thing as *mut _ as *mut ComRepr<Self, Interface::Vtbl>)).data
    }
    unsafe fn from_unknown<'a>(thing: *mut IUnknown) -> &'a mut Self {
        &mut (*(thing as *mut _ as *mut ComRepr<Self, Interface::Vtbl>)).data
    }
    unsafe fn destroy(thing: *mut IUnknown) {
        Box::from_raw(thing as *mut ComRepr<Self, Interface::Vtbl>);
    }
}

pub trait IntoInterface<Interface: ComInterface> {
    fn into_interface(self) -> ComPtr<Interface>;
}

impl<T, Interface: ComInterface> IntoInterface<Interface> for T where T: ComClass<Interface> + Sized {
    #[inline]
    fn into_interface(self) -> ComPtr<Interface> {
        let com = Box::new(ComRepr {
            vtbl: Box::new(Self::get_vtbl()),
            refcount: ::std::sync::atomic::AtomicUsize::new(1),
            data: self
        });
        unsafe { ComPtr::wrap(Box::into_raw(com) as *mut Interface) }
    }
}