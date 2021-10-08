use std::ptr;

use super::{ComInterface, HString, HStringReference, HStringArg, ComPtr, ComArray, ComIid, Guid};
use HRESULT;

use w::shared::ntdef::{VOID, ULONG};
use w::shared::winerror::{S_OK, S_FALSE, CO_E_NOTINITIALIZED, REGDB_E_CLASSNOTREG};
use w::shared::guiddef::IID;
use w::um::unknwnbase::IUnknownVtbl;
use w::winrt::hstring::HSTRING;
use w::winrt::roapi::{RO_INIT_MULTITHREADED, RoInitialize, RoUninitialize, RoGetActivationFactory};

use self::gen::windows::foundation::collections::{
    IIterable,
    IIterator,
    IVectorView
};

/// Represents a single UTF-16 character. This is the standard character type in WinRT. 
#[derive(Clone, Copy)] #[repr(C)]
pub struct Char(pub ::w::ctypes::wchar_t); // TODO: deref to u16

/// Marker trait for all Windows Runtime interfaces. They must inherit from `IInspectable`.
pub unsafe trait RtInterface: ComInterface {}

/// Marker trait for all interfaces that are not factories or statics.
pub unsafe trait RtClassInterface: RtInterface {}

/// Marker trait for all value types (primitive types, structs, enums) that can be used as
/// generic parameters in Windows Runtime.
pub unsafe trait RtValueType: Copy {}

unsafe impl RtValueType for bool {}
unsafe impl RtValueType for f32 {}
unsafe impl RtValueType for f64 {}
unsafe impl RtValueType for i8 {}
unsafe impl RtValueType for i16 {}
unsafe impl RtValueType for i32 {}
unsafe impl RtValueType for i64 {}
unsafe impl RtValueType for u8 {}
unsafe impl RtValueType for u16 {}
unsafe impl RtValueType for u32 {}
unsafe impl RtValueType for u64 {}
unsafe impl RtValueType for Char {}
unsafe impl RtValueType for Guid {}

/// This is a trait implemented by all types that can be used as generic parameters of parameterized interfaces.
/// `Abi` and `Out` must be binary compatible (i.e. `wrap` must basically be the same as `transmute`)
/// in order to work in `ComArray`.
pub trait RtType {
    type In;
    type Abi;
    type Out;

    unsafe fn unwrap(input: &Self::In) -> Self::Abi;
    unsafe fn uninitialized() -> Self::Abi;
    unsafe fn wrap(abi: Self::Abi) -> Self::Out;
}

impl<'a> RtType for HString {
    type In = HStringArg;
    type Abi = HSTRING;
    type Out = HString;

    #[doc(hidden)] #[inline]
    unsafe fn unwrap(v: &HStringArg) -> Self::Abi {
        v.get()
    }
    #[doc(hidden)] #[inline]
    unsafe fn uninitialized() -> Self::Abi {
        ::std::ptr::null_mut()
    }
    #[doc(hidden)] #[inline]
    unsafe fn wrap(v: Self::Abi) -> Self::Out {
        HString::wrap(v)
    }
}

impl<T> RtType for T where T: RtValueType
{
    type In = T;
    type Abi = T;
    type Out = T;

    #[doc(hidden)] #[inline]
    unsafe fn unwrap(v: &Self::In) -> Self::Abi {
        *v
    }
    #[doc(hidden)] #[inline]
    unsafe fn uninitialized() -> Self::Abi {
        ::std::mem::zeroed()
    }
    #[doc(hidden)] #[inline]
    unsafe fn wrap(v: Self::Abi) -> Self::Out {
        v
    }
}

/// This trait is implemented by all classes that have a name
/// associated with them which can be used at runtime.
pub trait RtNamedClass {
    /// Returns the name of the class encoded as a 16-bit wide string.
    fn name() -> &'static [u16];
}

pub trait RtActivatable<Interface> : RtNamedClass {
    /// Returns a factory object to create instances of this class or to call static methods.
    #[inline]
    fn get_activation_factory() -> ComPtr<Interface> where Interface: RtInterface + ComIid {
        let mut res = ptr::null_mut();
        let class_id = unsafe { HStringReference::from_utf16_unchecked(Self::name()) };
        let hr = unsafe { RoGetActivationFactory(class_id.get(), <Interface as ComIid>::iid().as_ref(), &mut res as *mut *mut _ as *mut *mut VOID) };
        if hr == S_OK {
            unsafe { ComPtr::wrap(res) }
        } else if hr == CO_E_NOTINITIALIZED {
            panic!("WinRT is not initialized")
        } else if hr == REGDB_E_CLASSNOTREG {
            let name = Self::name();
            panic!("WinRT class \"{}\" not registered", String::from_utf16_lossy(&name[0..name.len()-1]))
        } else {
            panic!("RoGetActivationFactory failed with error code 0x{:X}", hr as u32)
        }     
    }
}

pub trait RtDefaultConstructible {
    /// Uses the default constructor to create an instance of this class.
    fn new() -> ComPtr<Self> where Self: Sized + RtActivatable<IActivationFactory> + ComInterface;
}

impl<T> RtDefaultConstructible for T where T: RtActivatable<IActivationFactory> {
    #[inline]
    fn new() -> ComPtr<Self> where Self: Sized + RtActivatable<IActivationFactory> + ComInterface {
        let factory: ComPtr<IActivationFactory> = Self::get_activation_factory();
        unsafe { factory.activate_instance().into_unchecked() }
    }
}

// We can also implement IntoIterator for IIterable<T> and IVectorView<T>
// TODO: This should be extended to more types (at least IVector, IMap, IMapView, IObservableVector, IObservableMap)
impl<'a, T> IntoIterator for &'a IIterable<T> where T: RtType
{
    type Item = <T as RtType>::Out;
    type IntoIter = ComPtr<IIterator<T>>;
    #[inline] fn into_iter(self) -> Self::IntoIter {
        unsafe { self.first().unwrap() }
    }
}

impl<'a, T> IntoIterator for &'a IVectorView<T> where T: RtType, IIterable<T>: ComIid
{
    type Item = <T as RtType>::Out;
    type IntoIter = ComPtr<IIterator<T>>;
    #[inline] fn into_iter(self) -> Self::IntoIter {
        ::comptr::query_interface::<_, IIterable<T>>(self).unwrap().into_iter()
    }
}

// TODO: also implement IndexMove for IVectorView etc once that exists (Index or IndexMut won't work since we can't return a reference)

impl<T> Iterator for ComPtr<IIterator<T>> where T: RtType
{
    type Item = <T as RtType>::Out;

    // TODO: This could potentially be made faster by using the output of MoveNext instead of calling HasCurrent
    //       in every iteration. That would require a wrapper struct with a boolean flag.
    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        let has_next = unsafe { self.get_has_current().unwrap() };
        if has_next {
            unsafe {
                let current = self.get_current().unwrap();
                assert!(self.move_next().is_ok());
                Some(current)
            }
        } else {
            None
        }
    }
}

macro_rules! RT_INTERFACE {
    ($(#[$attr:meta])* interface $interface:ident<$t1:ident, $t2:ident> $($rest:tt)*) => {
        RT_INTERFACE!($(#[$attr])* basic $interface<$t1,$t2> $($rest)*);
        unsafe impl<$t1: RtType, $t2: RtType> ::RtInterface for $interface<$t1,$t2> {}
        unsafe impl<$t1: RtType, $t2: RtType> ::RtClassInterface for $interface<$t1,$t2> {}
    };

    ($(#[$attr:meta])* interface $interface:ident<$t1:ident> $($rest:tt)*) => {
        RT_INTERFACE!($(#[$attr])* basic $interface<$t1> $($rest)*);
        unsafe impl<$t1: RtType> ::RtInterface for $interface<$t1> {}
        unsafe impl<$t1: RtType> ::RtClassInterface for $interface<$t1> {}
    };

    ($(#[$attr:meta])* interface $interface:ident $($rest:tt)*) => {
        RT_INTERFACE!($(#[$attr])* basic $interface $($rest)*);
        unsafe impl ::RtInterface for $interface {}
        unsafe impl ::RtClassInterface for $interface {}
    };

    ($(#[$attr:meta])* static interface $interface:ident<$t1:ident, $t2:ident> $($rest:tt)*) => {
        RT_INTERFACE!($(#[$attr])* basic $interface<$t1,$t2> $($rest)*);
        unsafe impl<$t1: RtType, $t2: RtType> ::RtInterface for $interface<$t1,$t2> {}
    };

    ($(#[$attr:meta])* static interface $interface:ident<$t1:ident> $($rest:tt)*) => {
        RT_INTERFACE!($(#[$attr])* basic $interface<$t1> $($rest)*);
        unsafe impl<$t1: RtType> ::RtInterface for $interface<$t1> {}
    };

    ($(#[$attr:meta])* static interface $interface:ident $($rest:tt)*) => {
        RT_INTERFACE!($(#[$attr])* basic $interface $($rest)*);
        unsafe impl ::RtInterface for $interface {}
    };

    // version with no methods
    ($(#[$attr:meta])* basic $interface:ident ($vtbl:ident) : $pinterface:ident ($pvtbl:ty) [$iid:ident]
        {}
    ) => {
        #[repr(C)] #[allow(missing_copy_implementations)] #[doc(hidden)]
        pub struct $vtbl {
            pub parent: $pvtbl
        }
        $(#[$attr])* #[repr(C)] #[allow(missing_copy_implementations)]
        pub struct $interface {
            lpVtbl: *const $vtbl
        }
        impl ComIid for $interface {
            #[inline] fn iid() -> &'static ::Guid { &$iid }
        }
        impl ComInterface for $interface {
            type Vtbl = $vtbl;
        }
        impl ::RtType for $interface {
            type In = $interface;
            type Abi = *mut $interface;
            type Out = ComPtr<$interface>;

            #[doc(hidden)] #[inline] unsafe fn unwrap(v: &Self::In) -> Self::Abi { v as *const _ as *mut _ }
            #[doc(hidden)] #[inline] unsafe fn uninitialized() -> Self::Abi { ::std::ptr::null_mut() }
            #[doc(hidden)] #[inline] unsafe fn wrap(v: Self::Abi) -> Self::Out { ComPtr::wrap(v) }
        }
        impl ::std::ops::Deref for $interface {
            type Target = $crate::$pinterface;
            #[inline]
            fn deref(&self) -> &$crate::$pinterface {
                unsafe { ::std::mem::transmute(self) }
            }
        }
        impl ::std::ops::DerefMut for $interface {
            #[inline]
            fn deref_mut(&mut self) -> &mut $crate::$pinterface {
                unsafe { ::std::mem::transmute(self) }
            }
        }
    };

    // version with methods, but without generic parameters
    ($(#[$attr:meta])* basic $interface:ident ($vtbl:ident) : $pinterface:ident ($pvtbl:ty) [$iid:ident]
        {$(
            $(#[cfg($cond_attr:meta)])* fn $method:ident(&self $(,$p:ident : $t:ty)*) -> $rtr:ty
        ),+}
    ) => {
        #[repr(C)] #[allow(missing_copy_implementations)] #[doc(hidden)]
        pub struct $vtbl {
            pub parent: $pvtbl
            $(, $(#[cfg($cond_attr)])* pub $method: unsafe extern "system" fn(
                This: *mut $interface
                $(,$p: $t)*
            ) -> $rtr)+
        }
        $(#[$attr])* #[repr(C)] #[allow(missing_copy_implementations)]
        pub struct $interface {
            lpVtbl: *const $vtbl
        }
        impl ComIid for $interface {
            #[inline] fn iid() -> &'static ::Guid { &$iid }
        }
        impl ComInterface for $interface {
            type Vtbl = $vtbl;
        }
        impl ::RtType for $interface {
            type In = $interface;
            type Abi = *mut $interface;
            type Out = ComPtr<$interface>;

            #[doc(hidden)] #[inline] unsafe fn unwrap(v: &Self::In) -> Self::Abi { v as *const _ as *mut _ }
            #[doc(hidden)] #[inline] unsafe fn uninitialized() -> Self::Abi { ::std::ptr::null_mut() }
            #[doc(hidden)] #[inline] unsafe fn wrap(v: Self::Abi) -> Self::Out { ComPtr::wrap(v) }
        }
        impl ::std::ops::Deref for $interface {
            type Target = $crate::$pinterface;
            #[inline]
            fn deref(&self) -> &$crate::$pinterface {
                unsafe { ::std::mem::transmute(self) }
            }
        }
        impl ::std::ops::DerefMut for $interface {
            #[inline]
            fn deref_mut(&mut self) -> &mut $crate::$pinterface {
                unsafe { ::std::mem::transmute(self) }
            }
        }
    };
    // The $iid is actually not necessary, because it refers to the uninstantiated version of the interface,
    // which is irrelevant at runtime (it is used to generate the IIDs of the parameterized interfaces).
    ($(#[$attr:meta])* basic $interface:ident<$t1:ident> ($vtbl:ident) : $pinterface:ident ($pvtbl:ty) [$iid:ident]
        {$(
            $(#[cfg($cond_attr:meta)])* fn $method:ident(&self $(,$p:ident : $t:ty)*) -> $rtr:ty
        ),+}
    ) => {
        #[repr(C)] #[allow(missing_copy_implementations)] #[doc(hidden)]
        pub struct $vtbl<$t1> where $t1: RtType {
            pub parent: $pvtbl
            $(, $(#[cfg($cond_attr)])* pub $method: unsafe extern "system" fn(
                This: *mut $interface<$t1>
                $(,$p: $t)*
            ) -> $rtr)+
        }
        $(#[$attr])* #[repr(C)] #[allow(missing_copy_implementations)]
        pub struct $interface<$t1> where $t1: RtType {
            lpVtbl: *const $vtbl<$t1>,
        }
        impl<$t1> ComInterface for $interface<$t1> where $t1: RtType {
            type Vtbl = $vtbl<$t1>;
        }
        impl<$t1> ::RtType for $interface<$t1> where $t1: RtType{
            type In = $interface<$t1>;
            type Abi = *mut $interface<$t1>;
            type Out = ComPtr<$interface<$t1>>;

            #[doc(hidden)] #[inline] unsafe fn unwrap(v: &Self::In) -> Self::Abi { v as *const _ as *mut _ }
            #[doc(hidden)] #[inline] unsafe fn uninitialized() -> Self::Abi { ::std::ptr::null_mut() }
            #[doc(hidden)] #[inline] unsafe fn wrap(v: Self::Abi) -> Self::Out { ComPtr::wrap(v) }
        }
        impl<$t1> ::std::ops::Deref for $interface<$t1> where $t1: RtType {
            type Target = $pinterface;
            #[inline]
            fn deref(&self) -> &$pinterface {
                unsafe { ::std::mem::transmute(self) }
            }
        }
        impl<$t1> ::std::ops::DerefMut for $interface<$t1> where $t1: RtType {
            #[inline]
            fn deref_mut(&mut self) -> &mut $pinterface {
                unsafe { ::std::mem::transmute(self) }
            }
        }
    };

    ($(#[$attr:meta])* basic $interface:ident<$t1:ident, $t2:ident> ($vtbl:ident) : $pinterface:ident ($pvtbl:ty) [$iid:ident]
        {$(
            $(#[cfg($cond_attr:meta)])* fn $method:ident(&self $(,$p:ident : $t:ty)*) -> $rtr:ty
        ),+}
    ) => {
        #[repr(C)] #[allow(missing_copy_implementations)] #[doc(hidden)]
        pub struct $vtbl<$t1, $t2> where $t1: RtType, $t2: RtType {
            pub parent: $pvtbl
            $(, $(#[cfg($cond_attr)])* pub $method: unsafe extern "system" fn(
                This: *mut $interface<$t1, $t2>
                $(,$p: $t)*
            ) -> $rtr)+
        }
        $(#[$attr])* #[repr(C)] #[allow(missing_copy_implementations)]
        pub struct $interface<$t1, $t2> where $t1: RtType, $t2: RtType {
            lpVtbl: *const $vtbl<$t1, $t2>,
        }
        impl<$t1, $t2> ComInterface for $interface<$t1, $t2> where $t1: RtType, $t2: RtType {
            type Vtbl = $vtbl<$t1, $t2>;
        }
        impl<$t1, $t2> ::RtType for $interface<$t1, $t2> where $t1: RtType, $t2: RtType {
            type In = $interface<$t1, $t2>;
            type Abi = *mut $interface<$t1, $t2>;
            type Out = ComPtr<$interface<$t1, $t2>>;

            #[doc(hidden)] #[inline] unsafe fn unwrap(v: &Self::In) -> Self::Abi { v as *const _ as *mut _ }
            #[doc(hidden)] #[inline] unsafe fn uninitialized() -> Self::Abi { ::std::ptr::null_mut() }
            #[doc(hidden)] #[inline] unsafe fn wrap(v: Self::Abi) -> Self::Out { ComPtr::wrap(v) }
        }
        impl<$t1, $t2> ::std::ops::Deref for $interface<$t1, $t2> where $t1: RtType, $t2: RtType {
            type Target = $pinterface;
            #[inline]
            fn deref(&self) -> &$pinterface {
                unsafe { ::std::mem::transmute(self) }
            }
        }
        impl<$t1, $t2> ::std::ops::DerefMut for $interface<$t1, $t2> where $t1: RtType, $t2: RtType {
            #[inline]
            fn deref_mut(&mut self) -> &mut $pinterface {
                unsafe { ::std::mem::transmute(self) }
            }
        }
    };
}

macro_rules! RT_DELEGATE {
    // without generic parameters
    (delegate $interface:ident ($vtbl:ident, $imp:ident) [$iid:ident] {
        $(#[cfg($cond_attr:meta)])* fn Invoke(&self $(,$p:ident : $t:ty)*) -> HRESULT
    }) => {
        RT_INTERFACE!{basic $interface($vtbl) : IUnknown(IUnknownVtbl) [$iid] {
            $(#[cfg($cond_attr)])* fn Invoke(&self $(,$p : $t)*) -> HRESULT
        }}

        $(#[cfg($cond_attr)])*
        impl $interface {
            #[inline] pub fn new<_F_>(f: _F_) -> ComPtr<$interface>
                where _F_: 'static + Send + FnMut($($t),*) -> Result<()>, $interface: ComIid {
                $imp::new(f).into_interface()
            }
        }

        $(#[cfg($cond_attr)])*
        struct $imp<_F_> where _F_: 'static + Send + FnMut($($t),*) -> Result<()> {
            invoke: _F_
        }

        $(#[cfg($cond_attr)])*
        impl<_F_> $imp<_F_>
            where $interface: ComIid, _F_: 'static + Send + FnMut($($t),*) -> Result<()>
        {
            #[inline]
            pub fn new(f: _F_) -> $imp<_F_> {
                $imp {
                    invoke: f,
                }
            }
        }

        $(#[cfg($cond_attr)])*
        impl<_F_> ::rt::handler::ComClass<$interface> for $imp<_F_>
            where $interface: ComIid, _F_: 'static + Send + FnMut($($t),*) -> Result<()>
        {
            #[inline]
            fn get_vtbl() -> $vtbl {
                $vtbl {
                    parent: IUnknownVtbl {
                        QueryInterface: ::rt::handler::ComReprHandler_QueryInterface::<$imp<_F_>, _>,
                        AddRef: ::rt::handler::ComRepr_AddRef::<$imp<_F_>>,
                        Release: ::rt::handler::ComRepr_Release::<$imp<_F_>>,
                    },
                    Invoke: {
                        unsafe extern "system" fn Invoke<_F_>(this_: *mut $interface $(,$p : $t)*) -> HRESULT
                            where $interface: ComIid, _F_: 'static + Send + FnMut($($t),*) -> Result<()>
                        {
                            let this: &mut $imp<_F_> = ::rt::handler::ComClass::from_interface(this_);
                            match (this.invoke)($($p),*) {
                                Ok(()) => S_OK,
                                Err(err) => err.as_hresult()
                            }
                        }
                        Invoke::<_F_>
                    }
                }
            }
        }

        /*impl<_F_> Drop for $imp<_F_>
            where _F_: 'static + Send + FnMut($($t),*) -> HRESULT
        {
            fn drop(&mut self) {
                println!("DROPPED {}<...>!", stringify!($imp));
            }
        }*/
    };

    // with generic parameters
    (delegate $interface:ident<$($ht:ident),+> ($vtbl:ident, $imp:ident) [$iid:ident] {
        $(#[cfg($cond_attr:meta)])* fn Invoke(&self $(,$p:ident : $t:ty)*) -> HRESULT
    }) => {
        RT_INTERFACE!{basic $interface<$($ht),+>($vtbl) : IUnknown(IUnknownVtbl) [$iid] {
            $(#[cfg($cond_attr)])* fn Invoke(&self $(,$p : $t)*) -> HRESULT
        }}

        impl<$($ht: RtType + 'static),+> $interface<$($ht),+> {
            #[inline] pub fn new<_F_>(f: _F_) -> ComPtr<$interface<$($ht),+>>
                where _F_: 'static + Send + FnMut($($t),*) -> Result<()>, $interface<$($ht),+>: ComIid {
                $imp::new(f).into_interface()
            }
        }

        struct $imp<$($ht: RtType),+ , _F_> where _F_: 'static + Send + FnMut($($t),*) -> Result<()> {
            invoke: _F_,
            phantom: ::std::marker::PhantomData<($($ht),+)>
        }

        impl<$($ht: RtType + 'static),+ , _F_> $imp<$($ht),+ , _F_>
            where $interface<$($ht),+>: ComIid, _F_: 'static + Send + FnMut($($t),*) -> Result<()>
        {
            #[inline]
            pub fn new(f: _F_) -> $imp<$($ht),+ , _F_> {
                $imp {
                    invoke: f,
                    phantom: ::std::marker::PhantomData
                }
            }
        }

        impl<$($ht: RtType + 'static),+ , _F_> ::rt::handler::ComClass<$interface<$($ht),+>> for $imp<$($ht),+ , _F_>
            where $interface<$($ht),+>: ComIid, _F_: 'static + Send + FnMut($($t),*) -> Result<()>
        {
            #[inline]
            fn get_vtbl() -> $vtbl<$($ht),+> {
                $vtbl::<$($ht),+> {
                    parent: IUnknownVtbl {
                        QueryInterface: ::rt::handler::ComReprHandler_QueryInterface::<$imp<$($ht),+ , _F_>, _>,
                        AddRef: ::rt::handler::ComRepr_AddRef::<$imp<$($ht),+ , _F_>>,
                        Release: ::rt::handler::ComRepr_Release::<$imp<$($ht),+ , _F_>>,
                    },
                    Invoke: {
                        unsafe extern "system" fn Invoke<$($ht: RtType + 'static),+ , _F_>(this_: *mut $interface<$($ht),+> $(,$p : $t)*) -> HRESULT
                            where $interface<$($ht),+>: ComIid, _F_: 'static + Send + FnMut($($t),*) -> Result<()>
                        {
                            let this: &mut $imp<$($ht),+ , _F_> = ::rt::handler::ComClass::from_interface(this_);
                            match (this.invoke)($($p),*) {
                                Ok(()) => S_OK,
                                Err(err) => err.as_hresult()
                            }
                        }
                        Invoke::<$($ht),+ , _F_>
                    }
                }
            }
        }

        /*impl<$($ht: RtType),+ , _F_> Drop for $imp<$($ht),+ , _F_>
            where _F_: 'static + Send + FnMut($($t),*) -> HRESULT
        {
            fn drop(&mut self) {
                println!("DROPPED {}<...>!", stringify!($imp));
            }
        }*/
    };
}

macro_rules! RT_CLASS {
    {static class $cls:ident} => {
        pub struct $cls; // does not exist at runtime and has no ABI representation
    };

    {class $cls:ident : $interface:ty} => {
        pub struct $cls($interface);
        unsafe impl ::RtInterface for $cls {}
        unsafe impl ::RtClassInterface for $cls {}
        impl ComInterface for $cls {
            type Vtbl = <$interface as ComInterface>::Vtbl;
        }
        impl ComIid for $cls {
            #[inline] fn iid() -> &'static ::Guid { <$interface as ComIid>::iid() }
        }
        impl ::RtType for $cls {
            type In = $cls;
            type Abi = *mut $cls;
            type Out = ComPtr<$cls>;
            
            #[doc(hidden)] #[inline] unsafe fn unwrap(v: &Self::In) -> Self::Abi { v as *const _ as *mut _ }
            #[doc(hidden)] #[inline] unsafe fn uninitialized() -> Self::Abi { ::std::ptr::null_mut() }
            #[doc(hidden)] #[inline] unsafe fn wrap(v: Self::Abi) -> Self::Out { ComPtr::wrap(v) }
        }
        impl ::std::ops::Deref for $cls {
            type Target = $interface;
            #[inline]
            fn deref(&self) -> &$interface {
                &self.0
            }
        }
        impl ::std::ops::DerefMut for $cls {
            #[inline]
            fn deref_mut(&mut self) -> &mut $interface {
                &mut self.0
            }
        }
    };
}

macro_rules! DEFINE_CLSID {
    ($clsname:ident($id:expr) [$idname:ident]) => {
        const $idname: &'static [u16] = $id; // Full name of the class as null-terminated UTF16 string
        impl ::RtNamedClass for $clsname {
            #[inline]
            fn name() -> &'static [u16] { $idname } 
        }
    }
}


macro_rules! RT_ENUM {
    {enum $name:ident : $t:ty { $($variant:ident ($longvariant:ident) = $value:expr,)+ }} => {
        #[repr(C)] #[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
        #[allow(non_upper_case_globals)]
        pub struct $name(pub $t);
        $(#[cfg(feature = "lang-compat")] pub const $longvariant: $name = $name($value);)+

        #[cfg(not(feature = "lang-compat"))]
        impl $name {
            $(pub const $variant: $name = $name($value);)+
        }
        unsafe impl ::RtValueType for $name {}
    };
}

macro_rules! RT_STRUCT {
    { struct $name:ident { $($field:ident: $ftype:ty,)* }} => {
        #[repr(C)] #[derive(Debug,Copy,Clone)]
        pub struct $name {
            $(pub $field: $ftype,)*
        }
        unsafe impl ::RtValueType for $name {}
    };
}

macro_rules! RT_PINTERFACE {
    (
        for $t:ty => [$l:expr, $w1:expr, $w2:expr, $b1:expr, $b2:expr, $b3:expr, $b4:expr, $b5:expr,
        $b6:expr, $b7:expr, $b8:expr] as $iid:ident
    ) => {
        DEFINE_IID!($iid, $l,$w1,$w2,$b1,$b2,$b3,$b4,$b5,$b6,$b7,$b8);
        impl ComIid for $t {
            #[inline] fn iid() -> &'static ::Guid { &$iid }
        }
    };
}

pub mod handler;
pub mod async;
pub mod gen; // import auto-generated definitions (has to come after macro definitions)

// FIXME: maybe better reexport from winapi?
DEFINE_IID!(IID_IInspectable, 0xAF86E2E0, 0xB12D, 0x4c6a, 0x9C, 0x5A, 0xD7, 0xAA, 0x65, 0x10, 0x1E, 0x90);
RT_INTERFACE!{
/// The `IInspectable` interface is the base interface for all Windows Runtime classes.
interface IInspectable(IInspectableVtbl): IUnknown(IUnknownVtbl) [IID_IInspectable]  {
    fn GetIids(&self, iidCount: *mut ULONG, iids: *mut *mut IID) -> HRESULT,
    fn GetRuntimeClassName(&self, className: *mut HSTRING) -> HRESULT,
    fn GetTrustLevel(&self, trustLevel: *mut ::TrustLevel) -> HRESULT
}}
impl IInspectable {
    /// Returns the interfaces that are implemented by the current Windows Runtime object.
    #[inline]
    pub fn get_iids(&self) -> ComArray<Guid> {
        let mut result = ::std::ptr::null_mut();
        let mut count = 0;
        let hr = unsafe { ((*self.lpVtbl).GetIids)(self as *const _ as *mut _, &mut count, &mut result) };
        assert_eq!(hr, S_OK);
        let result = result as *mut Guid; // convert from ::w::GUID to (binary compatible) Guid
        unsafe { ComArray::from_raw(count, result) }
    }

    /// Returns the trust level of the current Windows Runtime object.
    #[inline]
    pub fn get_trust_level(&self) -> ::TrustLevel {
        let mut result = unsafe { ::std::mem::zeroed() };
        let hr = unsafe { ((*self.lpVtbl).GetTrustLevel)(self as *const _ as *mut _, &mut result) };
        assert_eq!(hr, S_OK);
        result
    }
}

impl ::comptr::HiddenGetRuntimeClassName for IInspectable {
    #[inline]
    fn get_runtime_class_name(&self) -> HString {
        let mut result = ::std::ptr::null_mut();
        let hr = unsafe { ((*self.lpVtbl).GetRuntimeClassName)(self as *const _ as *mut _, &mut result) };
        assert_eq!(hr, S_OK);
        unsafe { HString::wrap(result) }
    }
}

DEFINE_IID!(IID_IActivationFactory, 0x00000035, 0x0000, 0x0000, 0xC0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x46);
RT_INTERFACE!{
interface IActivationFactory(IActivationFactoryVtbl): IInspectable(IInspectableVtbl) [IID_IActivationFactory]  {
    fn ActivateInstance(&self, instance: *mut *mut IInspectable) -> HRESULT
}}

impl IActivationFactory {
    #[inline]
    pub fn activate_instance(&self) -> ComPtr<IInspectable> {
        let mut result = ::std::ptr::null_mut();
        let hr = unsafe { ((*self.lpVtbl).ActivateInstance)(self as *const _ as *mut _, &mut result) };
        assert_eq!(hr, S_OK);
        unsafe { ComPtr::wrap(result) }
    }
}

DEFINE_IID!(IID_IMemoryBufferByteAccess, 0x5b0d3235, 0x4dba, 0x4d44, 0x86, 0x5e, 0x8f, 0x1d, 0x0e, 0x4f, 0xd0, 0x4d);
RT_INTERFACE!{
interface IMemoryBufferByteAccess(IMemoryBufferByteAccessVtbl) : IUnknown(IUnknownVtbl) [IID_IMemoryBufferByteAccess]  {
    fn GetBuffer(&self, value: *mut *mut u8, capacity: *mut u32) -> HRESULT
}}

impl IMemoryBufferByteAccess {
    /// Provides direct byte access to the memory buffer underlying an `IMemoryBuffer`.
    /// To use `IMemoryBufferByteAccess`, you first need to obtain an `IMemoryBufferReference`, then
    /// call `query_interface::<IMemoryBufferByteAccess>()` on that.
    ///
    /// This method is marked `unsafe`, because the buffer might be invalidated during its lifetime.
    /// Due to the nature of COM objects, the buffer might be referenced and modified or closed
    /// by some other instance, which Rust's type system cannot know about. The `Close` event on
    /// `IMemoryBufferReference` can be used to be notified when the buffer is closed.
    ///
    /// An empty slice is returned if the underlying buffer has already been closed.
    #[inline]
    pub unsafe fn get_buffer(&self) -> &[u8] {
        let mut ptr = ::std::ptr::null_mut();
        let mut capacity: u32 = 0;
        let hr = ((*self.lpVtbl).GetBuffer)(self as *const _ as *mut _, &mut ptr, &mut capacity);
        assert_eq!(hr, S_OK);
        if capacity == 0 {
            ptr = 1 as *mut u8; // null pointer is not allowed
        }
        ::std::slice::from_raw_parts(ptr, capacity as usize)
    }
}


/// Manages initialization and uninitialization of the Windows Runtime.
pub struct RuntimeContext {
    token: ()
}

impl RuntimeContext {
    /// Initializes the Windows Runtime. This must be called before any other operations can use
    /// the Windows Runtime. The Windows Runtime will be unitilized when the returned `RuntimeContext`
    /// is dropped or `uninit` is called explicitly. You have to make sure that this does not happen
    /// as long as any Windows Runtime object is still alive.
    #[inline]
    pub fn init() -> RuntimeContext {
        let hr = unsafe { RoInitialize(RO_INIT_MULTITHREADED) };
        assert!(hr == S_OK || hr == S_FALSE, "failed to call RoInitialize: error {}", hr);
        /*let mut f: ::w::UINT32 = 0;
        assert!(RoGetErrorReportingFlags(&mut f) == S_OK);
        println!("ErrorReportingFlags: {:?}", f);*/
        RuntimeContext { token: () }
    }
    
    /// Unitializes the Windows Runtime. This must not be called as long as any Windows Runtime
    /// object is still alive.
    #[inline]
    pub fn uninit(self) {
        drop(self);
    }
}

impl Drop for RuntimeContext {
    #[inline]
    fn drop(&mut self) {
        unsafe { RoUninitialize() };
    }
}