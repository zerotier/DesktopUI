#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use gtk_sys::{
    GtkContainer, GtkContainerPrivate, GtkMenu, GtkMenuPrivate, GtkMenuShell, GtkMenuShellPrivate,
    GtkStatusIcon, GtkStatusIconPrivate, GtkWidget, GtkWidgetPrivate,
};

use std::os::raw::*;

pub type guint32 = c_uint;
pub type gint64 = c_long;
pub type guint64 = c_ulong;
pub type gsize = c_ulong;
pub type gchar = c_char;
pub type glong = c_long;
pub type gint = c_int;
pub type gboolean = gint;
pub type gulong = c_ulong;
pub type guint = c_uint;
pub type gfloat = f32;
pub type gdouble = f64;
pub type gpointer = *mut c_void;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _GData {
    _unused: [u8; 0],
}
pub type GData = _GData;
pub type GSList = _GSList;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _GSList {
    pub data: gpointer,
    pub next: *mut GSList,
}
#[test]
fn bindgen_test_layout__GSList() {
    assert_eq!(
        ::std::mem::size_of::<_GSList>(),
        16usize,
        concat!("Size of: ", stringify!(_GSList))
    );
    assert_eq!(
        ::std::mem::align_of::<_GSList>(),
        8usize,
        concat!("Alignment of ", stringify!(_GSList))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_GSList>())).data as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(_GSList),
            "::",
            stringify!(data)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_GSList>())).next as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(_GSList),
            "::",
            stringify!(next)
        )
    );
}
pub type GType = gsize;
pub type GValue = _GValue;
pub type GTypeClass = _GTypeClass;
pub type GTypeInstance = _GTypeInstance;
#[doc = " GTypeClass:"]
#[doc = ""]
#[doc = " An opaque structure used as the base of all classes."]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _GTypeClass {
    pub g_type: GType,
}
#[test]
fn bindgen_test_layout__GTypeClass() {
    assert_eq!(
        ::std::mem::size_of::<_GTypeClass>(),
        8usize,
        concat!("Size of: ", stringify!(_GTypeClass))
    );
    assert_eq!(
        ::std::mem::align_of::<_GTypeClass>(),
        8usize,
        concat!("Alignment of ", stringify!(_GTypeClass))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_GTypeClass>())).g_type as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(_GTypeClass),
            "::",
            stringify!(g_type)
        )
    );
}
#[doc = " GTypeInstance:"]
#[doc = ""]
#[doc = " An opaque structure used as the base of all type instances."]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _GTypeInstance {
    pub g_class: *mut GTypeClass,
}
#[test]
fn bindgen_test_layout__GTypeInstance() {
    assert_eq!(
        ::std::mem::size_of::<_GTypeInstance>(),
        8usize,
        concat!("Size of: ", stringify!(_GTypeInstance))
    );
    assert_eq!(
        ::std::mem::align_of::<_GTypeInstance>(),
        8usize,
        concat!("Alignment of ", stringify!(_GTypeInstance))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_GTypeInstance>())).g_class as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(_GTypeInstance),
            "::",
            stringify!(g_class)
        )
    );
}
#[doc = " GValue:"]
#[doc = ""]
#[doc = " An opaque structure used to hold different types of values."]
#[doc = " The data within the structure has protected scope: it is accessible only"]
#[doc = " to functions within a #GTypeValueTable structure, or implementations of"]
#[doc = " the g_value_*() API. That is, code portions which implement new fundamental"]
#[doc = " types."]
#[doc = " #GValue users cannot make any assumptions about how data is stored"]
#[doc = " within the 2 element @data union, and the @g_type member should"]
#[doc = " only be accessed through the G_VALUE_TYPE() macro."]
#[repr(C)]
#[derive(Copy, Clone)]
pub struct _GValue {
    pub g_type: GType,
    pub data: [_GValue__bindgen_ty_1; 2usize],
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union _GValue__bindgen_ty_1 {
    pub v_int: gint,
    pub v_uint: guint,
    pub v_long: glong,
    pub v_ulong: gulong,
    pub v_int64: gint64,
    pub v_uint64: guint64,
    pub v_float: gfloat,
    pub v_double: gdouble,
    pub v_pointer: gpointer,
    _bindgen_union_align: u64,
}
#[test]
fn bindgen_test_layout__GValue__bindgen_ty_1() {
    assert_eq!(
        ::std::mem::size_of::<_GValue__bindgen_ty_1>(),
        8usize,
        concat!("Size of: ", stringify!(_GValue__bindgen_ty_1))
    );
    assert_eq!(
        ::std::mem::align_of::<_GValue__bindgen_ty_1>(),
        8usize,
        concat!("Alignment of ", stringify!(_GValue__bindgen_ty_1))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_GValue__bindgen_ty_1>())).v_int as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(_GValue__bindgen_ty_1),
            "::",
            stringify!(v_int)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_GValue__bindgen_ty_1>())).v_uint as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(_GValue__bindgen_ty_1),
            "::",
            stringify!(v_uint)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_GValue__bindgen_ty_1>())).v_long as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(_GValue__bindgen_ty_1),
            "::",
            stringify!(v_long)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_GValue__bindgen_ty_1>())).v_ulong as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(_GValue__bindgen_ty_1),
            "::",
            stringify!(v_ulong)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_GValue__bindgen_ty_1>())).v_int64 as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(_GValue__bindgen_ty_1),
            "::",
            stringify!(v_int64)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_GValue__bindgen_ty_1>())).v_uint64 as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(_GValue__bindgen_ty_1),
            "::",
            stringify!(v_uint64)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_GValue__bindgen_ty_1>())).v_float as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(_GValue__bindgen_ty_1),
            "::",
            stringify!(v_float)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_GValue__bindgen_ty_1>())).v_double as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(_GValue__bindgen_ty_1),
            "::",
            stringify!(v_double)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_GValue__bindgen_ty_1>())).v_pointer as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(_GValue__bindgen_ty_1),
            "::",
            stringify!(v_pointer)
        )
    );
}
#[test]
fn bindgen_test_layout__GValue() {
    assert_eq!(
        ::std::mem::size_of::<_GValue>(),
        24usize,
        concat!("Size of: ", stringify!(_GValue))
    );
    assert_eq!(
        ::std::mem::align_of::<_GValue>(),
        8usize,
        concat!("Alignment of ", stringify!(_GValue))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_GValue>())).g_type as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(_GValue),
            "::",
            stringify!(g_type)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_GValue>())).data as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(_GValue),
            "::",
            stringify!(data)
        )
    );
}
pub const GParamFlags_G_PARAM_READABLE: GParamFlags = 1;
pub const GParamFlags_G_PARAM_WRITABLE: GParamFlags = 2;
pub const GParamFlags_G_PARAM_READWRITE: GParamFlags = 3;
pub const GParamFlags_G_PARAM_CONSTRUCT: GParamFlags = 4;
pub const GParamFlags_G_PARAM_CONSTRUCT_ONLY: GParamFlags = 8;
pub const GParamFlags_G_PARAM_LAX_VALIDATION: GParamFlags = 16;
pub const GParamFlags_G_PARAM_STATIC_NAME: GParamFlags = 32;
pub const GParamFlags_G_PARAM_PRIVATE: GParamFlags = 32;
pub const GParamFlags_G_PARAM_STATIC_NICK: GParamFlags = 64;
pub const GParamFlags_G_PARAM_STATIC_BLURB: GParamFlags = 128;
pub const GParamFlags_G_PARAM_EXPLICIT_NOTIFY: GParamFlags = 1073741824;
pub const GParamFlags_G_PARAM_DEPRECATED: GParamFlags = -2147483648;
#[doc = " GParamFlags:"]
#[doc = " @G_PARAM_READABLE: the parameter is readable"]
#[doc = " @G_PARAM_WRITABLE: the parameter is writable"]
#[doc = " @G_PARAM_READWRITE: alias for %G_PARAM_READABLE | %G_PARAM_WRITABLE"]
#[doc = " @G_PARAM_CONSTRUCT: the parameter will be set upon object construction"]
#[doc = " @G_PARAM_CONSTRUCT_ONLY: the parameter can only be set upon object construction"]
#[doc = " @G_PARAM_LAX_VALIDATION: upon parameter conversion (see g_param_value_convert())"]
#[doc = "  strict validation is not required"]
#[doc = " @G_PARAM_STATIC_NAME: the string used as name when constructing the"]
#[doc = "  parameter is guaranteed to remain valid and"]
#[doc = "  unmodified for the lifetime of the parameter."]
#[doc = "  Since 2.8"]
#[doc = " @G_PARAM_STATIC_NICK: the string used as nick when constructing the"]
#[doc = "  parameter is guaranteed to remain valid and"]
#[doc = "  unmmodified for the lifetime of the parameter."]
#[doc = "  Since 2.8"]
#[doc = " @G_PARAM_STATIC_BLURB: the string used as blurb when constructing the"]
#[doc = "  parameter is guaranteed to remain valid and"]
#[doc = "  unmodified for the lifetime of the parameter."]
#[doc = "  Since 2.8"]
#[doc = " @G_PARAM_EXPLICIT_NOTIFY: calls to g_object_set_property() for this"]
#[doc = "   property will not automatically result in a \"notify\" signal being"]
#[doc = "   emitted: the implementation must call g_object_notify() themselves"]
#[doc = "   in case the property actually changes.  Since: 2.42."]
#[doc = " @G_PARAM_PRIVATE: internal"]
#[doc = " @G_PARAM_DEPRECATED: the parameter is deprecated and will be removed"]
#[doc = "  in a future version. A warning will be generated if it is used"]
#[doc = "  while running with G_ENABLE_DIAGNOSTIC=1."]
#[doc = "  Since 2.26"]
#[doc = ""]
#[doc = " Through the #GParamFlags flag values, certain aspects of parameters"]
#[doc = " can be configured. See also #G_PARAM_STATIC_STRINGS."]
pub type GParamFlags = i32;
pub type GParamSpec = _GParamSpec;
#[doc = " GParamSpec: (ref-func g_param_spec_ref_sink) (unref-func g_param_spec_uref) (set-value-func g_value_set_param) (get-value-func g_value_get_param)"]
#[doc = " @g_type_instance: private #GTypeInstance portion"]
#[doc = " @name: name of this parameter: always an interned string"]
#[doc = " @flags: #GParamFlags flags for this parameter"]
#[doc = " @value_type: the #GValue type for this parameter"]
#[doc = " @owner_type: #GType type that uses (introduces) this parameter"]
#[doc = ""]
#[doc = " All other fields of the GParamSpec struct are private and"]
#[doc = " should not be used directly."]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _GParamSpec {
    pub g_type_instance: GTypeInstance,
    pub name: *const gchar,
    pub flags: GParamFlags,
    pub value_type: GType,
    pub owner_type: GType,
    pub _nick: *mut gchar,
    pub _blurb: *mut gchar,
    pub qdata: *mut GData,
    pub ref_count: guint,
    pub param_id: guint,
}
#[test]
fn bindgen_test_layout__GParamSpec() {
    assert_eq!(
        ::std::mem::size_of::<_GParamSpec>(),
        72usize,
        concat!("Size of: ", stringify!(_GParamSpec))
    );
    assert_eq!(
        ::std::mem::align_of::<_GParamSpec>(),
        8usize,
        concat!("Alignment of ", stringify!(_GParamSpec))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_GParamSpec>())).g_type_instance as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(_GParamSpec),
            "::",
            stringify!(g_type_instance)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_GParamSpec>())).name as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(_GParamSpec),
            "::",
            stringify!(name)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_GParamSpec>())).flags as *const _ as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(_GParamSpec),
            "::",
            stringify!(flags)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_GParamSpec>())).value_type as *const _ as usize },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(_GParamSpec),
            "::",
            stringify!(value_type)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_GParamSpec>())).owner_type as *const _ as usize },
        32usize,
        concat!(
            "Offset of field: ",
            stringify!(_GParamSpec),
            "::",
            stringify!(owner_type)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_GParamSpec>()))._nick as *const _ as usize },
        40usize,
        concat!(
            "Offset of field: ",
            stringify!(_GParamSpec),
            "::",
            stringify!(_nick)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_GParamSpec>()))._blurb as *const _ as usize },
        48usize,
        concat!(
            "Offset of field: ",
            stringify!(_GParamSpec),
            "::",
            stringify!(_blurb)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_GParamSpec>())).qdata as *const _ as usize },
        56usize,
        concat!(
            "Offset of field: ",
            stringify!(_GParamSpec),
            "::",
            stringify!(qdata)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_GParamSpec>())).ref_count as *const _ as usize },
        64usize,
        concat!(
            "Offset of field: ",
            stringify!(_GParamSpec),
            "::",
            stringify!(ref_count)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_GParamSpec>())).param_id as *const _ as usize },
        68usize,
        concat!(
            "Offset of field: ",
            stringify!(_GParamSpec),
            "::",
            stringify!(param_id)
        )
    );
}
pub type GObject = _GObject;
pub type GObjectClass = _GObjectClass;
pub type GInitiallyUnowned = _GObject;
pub type GObjectConstructParam = _GObjectConstructParam;
#[doc = " GObject:"]
#[doc = ""]
#[doc = " All the fields in the GObject structure are private"]
#[doc = " to the #GObject implementation and should never be accessed directly."]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _GObject {
    pub g_type_instance: GTypeInstance,
    pub ref_count: guint,
    pub qdata: *mut GData,
}
#[test]
fn bindgen_test_layout__GObject() {
    assert_eq!(
        ::std::mem::size_of::<_GObject>(),
        24usize,
        concat!("Size of: ", stringify!(_GObject))
    );
    assert_eq!(
        ::std::mem::align_of::<_GObject>(),
        8usize,
        concat!("Alignment of ", stringify!(_GObject))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_GObject>())).g_type_instance as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(_GObject),
            "::",
            stringify!(g_type_instance)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_GObject>())).ref_count as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(_GObject),
            "::",
            stringify!(ref_count)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_GObject>())).qdata as *const _ as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(_GObject),
            "::",
            stringify!(qdata)
        )
    );
}
#[doc = " GObjectClass:"]
#[doc = " @g_type_class: the parent class"]
#[doc = " @constructor: the @constructor function is called by g_object_new () to"]
#[doc = "  complete the object initialization after all the construction properties are"]
#[doc = "  set. The first thing a @constructor implementation must do is chain up to the"]
#[doc = "  @constructor of the parent class. Overriding @constructor should be rarely"]
#[doc = "  needed, e.g. to handle construct properties, or to implement singletons."]
#[doc = " @set_property: the generic setter for all properties of this type. Should be"]
#[doc = "  overridden for every type with properties. If implementations of"]
#[doc = "  @set_property don't emit property change notification explicitly, this will"]
#[doc = "  be done implicitly by the type system. However, if the notify signal is"]
#[doc = "  emitted explicitly, the type system will not emit it a second time."]
#[doc = " @get_property: the generic getter for all properties of this type. Should be"]
#[doc = "  overridden for every type with properties."]
#[doc = " @dispose: the @dispose function is supposed to drop all references to other"]
#[doc = "  objects, but keep the instance otherwise intact, so that client method"]
#[doc = "  invocations still work. It may be run multiple times (due to reference"]
#[doc = "  loops). Before returning, @dispose should chain up to the @dispose method"]
#[doc = "  of the parent class."]
#[doc = " @finalize: instance finalization function, should finish the finalization of"]
#[doc = "  the instance begun in @dispose and chain up to the @finalize method of the"]
#[doc = "  parent class."]
#[doc = " @dispatch_properties_changed: emits property change notification for a bunch"]
#[doc = "  of properties. Overriding @dispatch_properties_changed should be rarely"]
#[doc = "  needed."]
#[doc = " @notify: the class closure for the notify signal"]
#[doc = " @constructed: the @constructed function is called by g_object_new() as the"]
#[doc = "  final step of the object creation process.  At the point of the call, all"]
#[doc = "  construction properties have been set on the object.  The purpose of this"]
#[doc = "  call is to allow for object initialisation steps that can only be performed"]
#[doc = "  after construction properties have been set.  @constructed implementors"]
#[doc = "  should chain up to the @constructed call of their parent class to allow it"]
#[doc = "  to complete its initialisation."]
#[doc = ""]
#[doc = " The class structure for the GObject type."]
#[doc = ""]
#[doc = " |[<!-- language=\"C\" -->"]
#[doc = " // Example of implementing a singleton using a constructor."]
#[doc = " static MySingleton *the_singleton = NULL;"]
#[doc = ""]
#[doc = " static GObject*"]
#[doc = " my_singleton_constructor (GType                  type,"]
#[doc = "                           guint                  n_construct_params,"]
#[doc = "                           GObjectConstructParam *construct_params)"]
#[doc = " {"]
#[doc = "   GObject *object;"]
#[doc = ""]
#[doc = "   if (!the_singleton)"]
#[doc = "     {"]
#[doc = "       object = G_OBJECT_CLASS (parent_class)->constructor (type,"]
#[doc = "                                                            n_construct_params,"]
#[doc = "                                                            construct_params);"]
#[doc = "       the_singleton = MY_SINGLETON (object);"]
#[doc = "     }"]
#[doc = "   else"]
#[doc = "     object = g_object_ref (G_OBJECT (the_singleton));"]
#[doc = ""]
#[doc = "   return object;"]
#[doc = " }"]
#[doc = " ]|"]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _GObjectClass {
    pub g_type_class: GTypeClass,
    pub construct_properties: *mut GSList,
    pub constructor: ::std::option::Option<
        unsafe extern "C" fn(
            type_: GType,
            n_construct_properties: guint,
            construct_properties: *mut GObjectConstructParam,
        ) -> *mut GObject,
    >,
    pub set_property: ::std::option::Option<
        unsafe extern "C" fn(
            object: *mut GObject,
            property_id: guint,
            value: *const GValue,
            pspec: *mut GParamSpec,
        ),
    >,
    pub get_property: ::std::option::Option<
        unsafe extern "C" fn(
            object: *mut GObject,
            property_id: guint,
            value: *mut GValue,
            pspec: *mut GParamSpec,
        ),
    >,
    pub dispose: ::std::option::Option<unsafe extern "C" fn(object: *mut GObject)>,
    pub finalize: ::std::option::Option<unsafe extern "C" fn(object: *mut GObject)>,
    pub dispatch_properties_changed: ::std::option::Option<
        unsafe extern "C" fn(object: *mut GObject, n_pspecs: guint, pspecs: *mut *mut GParamSpec),
    >,
    pub notify:
        ::std::option::Option<unsafe extern "C" fn(object: *mut GObject, pspec: *mut GParamSpec)>,
    pub constructed: ::std::option::Option<unsafe extern "C" fn(object: *mut GObject)>,
    pub flags: gsize,
    pub pdummy: [gpointer; 6usize],
}
#[test]
fn bindgen_test_layout__GObjectClass() {
    assert_eq!(
        ::std::mem::size_of::<_GObjectClass>(),
        136usize,
        concat!("Size of: ", stringify!(_GObjectClass))
    );
    assert_eq!(
        ::std::mem::align_of::<_GObjectClass>(),
        8usize,
        concat!("Alignment of ", stringify!(_GObjectClass))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_GObjectClass>())).g_type_class as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(_GObjectClass),
            "::",
            stringify!(g_type_class)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<_GObjectClass>())).construct_properties as *const _ as usize
        },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(_GObjectClass),
            "::",
            stringify!(construct_properties)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_GObjectClass>())).constructor as *const _ as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(_GObjectClass),
            "::",
            stringify!(constructor)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_GObjectClass>())).set_property as *const _ as usize },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(_GObjectClass),
            "::",
            stringify!(set_property)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_GObjectClass>())).get_property as *const _ as usize },
        32usize,
        concat!(
            "Offset of field: ",
            stringify!(_GObjectClass),
            "::",
            stringify!(get_property)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_GObjectClass>())).dispose as *const _ as usize },
        40usize,
        concat!(
            "Offset of field: ",
            stringify!(_GObjectClass),
            "::",
            stringify!(dispose)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_GObjectClass>())).finalize as *const _ as usize },
        48usize,
        concat!(
            "Offset of field: ",
            stringify!(_GObjectClass),
            "::",
            stringify!(finalize)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<_GObjectClass>())).dispatch_properties_changed as *const _
                as usize
        },
        56usize,
        concat!(
            "Offset of field: ",
            stringify!(_GObjectClass),
            "::",
            stringify!(dispatch_properties_changed)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_GObjectClass>())).notify as *const _ as usize },
        64usize,
        concat!(
            "Offset of field: ",
            stringify!(_GObjectClass),
            "::",
            stringify!(notify)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_GObjectClass>())).constructed as *const _ as usize },
        72usize,
        concat!(
            "Offset of field: ",
            stringify!(_GObjectClass),
            "::",
            stringify!(constructed)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_GObjectClass>())).flags as *const _ as usize },
        80usize,
        concat!(
            "Offset of field: ",
            stringify!(_GObjectClass),
            "::",
            stringify!(flags)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_GObjectClass>())).pdummy as *const _ as usize },
        88usize,
        concat!(
            "Offset of field: ",
            stringify!(_GObjectClass),
            "::",
            stringify!(pdummy)
        )
    );
}
#[doc = " GObjectConstructParam:"]
#[doc = " @pspec: the #GParamSpec of the construct parameter"]
#[doc = " @value: the value to set the parameter to"]
#[doc = ""]
#[doc = " The GObjectConstructParam struct is an auxiliary"]
#[doc = " structure used to hand #GParamSpec/#GValue pairs to the @constructor of"]
#[doc = " a #GObjectClass."]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _GObjectConstructParam {
    pub pspec: *mut GParamSpec,
    pub value: *mut GValue,
}
#[test]
fn bindgen_test_layout__GObjectConstructParam() {
    assert_eq!(
        ::std::mem::size_of::<_GObjectConstructParam>(),
        16usize,
        concat!("Size of: ", stringify!(_GObjectConstructParam))
    );
    assert_eq!(
        ::std::mem::align_of::<_GObjectConstructParam>(),
        8usize,
        concat!("Alignment of ", stringify!(_GObjectConstructParam))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_GObjectConstructParam>())).pspec as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(_GObjectConstructParam),
            "::",
            stringify!(pspec)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_GObjectConstructParam>())).value as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(_GObjectConstructParam),
            "::",
            stringify!(value)
        )
    );
}
pub const GdkScrollDirection_GDK_SCROLL_UP: GdkScrollDirection = 0;
pub const GdkScrollDirection_GDK_SCROLL_DOWN: GdkScrollDirection = 1;
pub const GdkScrollDirection_GDK_SCROLL_LEFT: GdkScrollDirection = 2;
pub const GdkScrollDirection_GDK_SCROLL_RIGHT: GdkScrollDirection = 3;
pub const GdkScrollDirection_GDK_SCROLL_SMOOTH: GdkScrollDirection = 4;
#[doc = " GdkScrollDirection:"]
#[doc = " @GDK_SCROLL_UP: the window is scrolled up."]
#[doc = " @GDK_SCROLL_DOWN: the window is scrolled down."]
#[doc = " @GDK_SCROLL_LEFT: the window is scrolled to the left."]
#[doc = " @GDK_SCROLL_RIGHT: the window is scrolled to the right."]
#[doc = " @GDK_SCROLL_SMOOTH: the scrolling is determined by the delta values"]
#[doc = "   in #GdkEventScroll. See gdk_event_get_scroll_deltas(). Since: 3.4"]
#[doc = ""]
#[doc = " Specifies the direction for #GdkEventScroll."]
pub type GdkScrollDirection = u32;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _GtkWidgetPrivate {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _GtkWidget {
    pub parent_instance: GInitiallyUnowned,
    pub priv_: *mut GtkWidgetPrivate,
}
#[test]
fn bindgen_test_layout__GtkWidget() {
    assert_eq!(
        ::std::mem::size_of::<_GtkWidget>(),
        32usize,
        concat!("Size of: ", stringify!(_GtkWidget))
    );
    assert_eq!(
        ::std::mem::align_of::<_GtkWidget>(),
        8usize,
        concat!("Alignment of ", stringify!(_GtkWidget))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_GtkWidget>())).parent_instance as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(_GtkWidget),
            "::",
            stringify!(parent_instance)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_GtkWidget>())).priv_ as *const _ as usize },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(_GtkWidget),
            "::",
            stringify!(priv_)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _GtkContainerPrivate {
    _unused: [u8; 0],
}
#[repr(C)]
pub struct _GtkContainer {
    pub widget: GtkWidget,
    pub priv_: *mut GtkContainerPrivate,
}
#[test]
fn bindgen_test_layout__GtkContainer() {
    assert_eq!(
        ::std::mem::size_of::<_GtkContainer>(),
        40usize,
        concat!("Size of: ", stringify!(_GtkContainer))
    );
    assert_eq!(
        ::std::mem::align_of::<_GtkContainer>(),
        8usize,
        concat!("Alignment of ", stringify!(_GtkContainer))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_GtkContainer>())).widget as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(_GtkContainer),
            "::",
            stringify!(widget)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_GtkContainer>())).priv_ as *const _ as usize },
        32usize,
        concat!(
            "Offset of field: ",
            stringify!(_GtkContainer),
            "::",
            stringify!(priv_)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _GtkMenuShellPrivate {
    _unused: [u8; 0],
}
#[repr(C)]
pub struct _GtkMenuShell {
    pub container: GtkContainer,
    pub priv_: *mut GtkMenuShellPrivate,
}
#[test]
fn bindgen_test_layout__GtkMenuShell() {
    assert_eq!(
        ::std::mem::size_of::<_GtkMenuShell>(),
        48usize,
        concat!("Size of: ", stringify!(_GtkMenuShell))
    );
    assert_eq!(
        ::std::mem::align_of::<_GtkMenuShell>(),
        8usize,
        concat!("Alignment of ", stringify!(_GtkMenuShell))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_GtkMenuShell>())).container as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(_GtkMenuShell),
            "::",
            stringify!(container)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_GtkMenuShell>())).priv_ as *const _ as usize },
        40usize,
        concat!(
            "Offset of field: ",
            stringify!(_GtkMenuShell),
            "::",
            stringify!(priv_)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _GtkMenuPrivate {
    _unused: [u8; 0],
}
#[repr(C)]
pub struct _GtkMenu {
    pub menu_shell: GtkMenuShell,
    pub priv_: *mut GtkMenuPrivate,
}
#[test]
fn bindgen_test_layout__GtkMenu() {
    assert_eq!(
        ::std::mem::size_of::<_GtkMenu>(),
        56usize,
        concat!("Size of: ", stringify!(_GtkMenu))
    );
    assert_eq!(
        ::std::mem::align_of::<_GtkMenu>(),
        8usize,
        concat!("Alignment of ", stringify!(_GtkMenu))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_GtkMenu>())).menu_shell as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(_GtkMenu),
            "::",
            stringify!(menu_shell)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_GtkMenu>())).priv_ as *const _ as usize },
        48usize,
        concat!(
            "Offset of field: ",
            stringify!(_GtkMenu),
            "::",
            stringify!(priv_)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _GtkStatusIconPrivate {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _GtkStatusIcon {
    pub parent_instance: GObject,
    pub priv_: *mut GtkStatusIconPrivate,
}
#[test]
fn bindgen_test_layout__GtkStatusIcon() {
    assert_eq!(
        ::std::mem::size_of::<_GtkStatusIcon>(),
        32usize,
        concat!("Size of: ", stringify!(_GtkStatusIcon))
    );
    assert_eq!(
        ::std::mem::align_of::<_GtkStatusIcon>(),
        8usize,
        concat!("Alignment of ", stringify!(_GtkStatusIcon))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_GtkStatusIcon>())).parent_instance as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(_GtkStatusIcon),
            "::",
            stringify!(parent_instance)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_GtkStatusIcon>())).priv_ as *const _ as usize },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(_GtkStatusIcon),
            "::",
            stringify!(priv_)
        )
    );
}
pub const AppIndicatorCategory_APP_INDICATOR_CATEGORY_APPLICATION_STATUS: AppIndicatorCategory = 0;
pub const AppIndicatorCategory_APP_INDICATOR_CATEGORY_COMMUNICATIONS: AppIndicatorCategory = 1;
pub const AppIndicatorCategory_APP_INDICATOR_CATEGORY_SYSTEM_SERVICES: AppIndicatorCategory = 2;
pub const AppIndicatorCategory_APP_INDICATOR_CATEGORY_HARDWARE: AppIndicatorCategory = 3;
pub const AppIndicatorCategory_APP_INDICATOR_CATEGORY_OTHER: AppIndicatorCategory = 4;
#[doc = " AppIndicatorCategory:"]
#[doc = " @APP_INDICATOR_CATEGORY_APPLICATION_STATUS: The indicator is used to display the status of the application."]
#[doc = " @APP_INDICATOR_CATEGORY_COMMUNICATIONS: The application is used for communication with other people."]
#[doc = " @APP_INDICATOR_CATEGORY_SYSTEM_SERVICES: A system indicator relating to something in the user's system."]
#[doc = " @APP_INDICATOR_CATEGORY_HARDWARE: An indicator relating to the user's hardware."]
#[doc = " @APP_INDICATOR_CATEGORY_OTHER: Something not defined in this enum, please don't use unless you really need it."]
#[doc = ""]
#[doc = " The category provides grouping for the indicators so that"]
#[doc = " users can find indicators that are similar together."]
pub type AppIndicatorCategory = u32;
pub const AppIndicatorStatus_APP_INDICATOR_STATUS_PASSIVE: AppIndicatorStatus = 0;
pub const AppIndicatorStatus_APP_INDICATOR_STATUS_ACTIVE: AppIndicatorStatus = 1;
pub const AppIndicatorStatus_APP_INDICATOR_STATUS_ATTENTION: AppIndicatorStatus = 2;
#[doc = " AppIndicatorStatus:"]
#[doc = " @APP_INDICATOR_STATUS_PASSIVE: The indicator should not be shown to the user."]
#[doc = " @APP_INDICATOR_STATUS_ACTIVE: The indicator should be shown in it's default state."]
#[doc = " @APP_INDICATOR_STATUS_ATTENTION: The indicator should show it's attention icon."]
#[doc = ""]
#[doc = " These are the states that the indicator can be on in"]
#[doc = " the user's panel.  The indicator by default starts"]
#[doc = " in the state @APP_INDICATOR_STATUS_PASSIVE and can be"]
#[doc = " shown by setting it to @APP_INDICATOR_STATUS_ACTIVE."]
pub type AppIndicatorStatus = u32;
pub type AppIndicator = _AppIndicator;
pub type AppIndicatorClass = _AppIndicatorClass;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _AppIndicatorPrivate {
    _unused: [u8; 0],
}
pub type AppIndicatorPrivate = _AppIndicatorPrivate;
#[doc = " AppIndicatorClass:"]
#[doc = " @parent_class: Mia familia"]
#[doc = " @new_icon: Slot for #AppIndicator::new-icon."]
#[doc = " @new_attention_icon: Slot for #AppIndicator::new-attention-icon."]
#[doc = " @new_status: Slot for #AppIndicator::new-status."]
#[doc = " @new_icon_theme_path: Slot for #AppIndicator::new-icon-theme-path"]
#[doc = " @new_label: Slot for #AppIndicator::new-label."]
#[doc = " @connection_changed: Slot for #AppIndicator::connection-changed."]
#[doc = " @scroll_event: Slot for #AppIndicator::scroll-event"]
#[doc = " @app_indicator_reserved_ats: Reserved for future use."]
#[doc = " @fallback: Function that gets called to make a #GtkStatusIcon when"]
#[doc = "            there is no Application Indicator area available."]
#[doc = " @unfallback: The function that gets called if an Application"]
#[doc = "              Indicator area appears after the fallback has been created."]
#[doc = " @app_indicator_reserved_1: Reserved for future use."]
#[doc = " @app_indicator_reserved_2: Reserved for future use."]
#[doc = " @app_indicator_reserved_3: Reserved for future use."]
#[doc = " @app_indicator_reserved_4: Reserved for future use."]
#[doc = " @app_indicator_reserved_5: Reserved for future use."]
#[doc = " @app_indicator_reserved_6: Reserved for future use."]
#[doc = ""]
#[doc = " The signals and external functions that make up the #AppIndicator"]
#[doc = " class object."]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _AppIndicatorClass {
    pub parent_class: GObjectClass,
    pub new_icon: ::std::option::Option<
        unsafe extern "C" fn(indicator: *mut AppIndicator, user_data: gpointer),
    >,
    pub new_attention_icon: ::std::option::Option<
        unsafe extern "C" fn(indicator: *mut AppIndicator, user_data: gpointer),
    >,
    pub new_status: ::std::option::Option<
        unsafe extern "C" fn(
            indicator: *mut AppIndicator,
            status: *const gchar,
            user_data: gpointer,
        ),
    >,
    pub new_icon_theme_path: ::std::option::Option<
        unsafe extern "C" fn(
            indicator: *mut AppIndicator,
            icon_theme_path: *const gchar,
            user_data: gpointer,
        ),
    >,
    pub new_label: ::std::option::Option<
        unsafe extern "C" fn(
            indicator: *mut AppIndicator,
            label: *const gchar,
            guide: *const gchar,
            user_data: gpointer,
        ),
    >,
    pub connection_changed: ::std::option::Option<
        unsafe extern "C" fn(
            indicator: *mut AppIndicator,
            connected: gboolean,
            user_data: gpointer,
        ),
    >,
    pub scroll_event: ::std::option::Option<
        unsafe extern "C" fn(
            indicator: *mut AppIndicator,
            delta: gint,
            direction: GdkScrollDirection,
            user_data: gpointer,
        ),
    >,
    pub app_indicator_reserved_ats: ::std::option::Option<unsafe extern "C" fn()>,
    pub fallback: ::std::option::Option<
        unsafe extern "C" fn(indicator: *mut AppIndicator) -> *mut GtkStatusIcon,
    >,
    pub unfallback: ::std::option::Option<
        unsafe extern "C" fn(indicator: *mut AppIndicator, status_icon: *mut GtkStatusIcon),
    >,
    pub app_indicator_reserved_1: ::std::option::Option<unsafe extern "C" fn()>,
    pub app_indicator_reserved_2: ::std::option::Option<unsafe extern "C" fn()>,
    pub app_indicator_reserved_3: ::std::option::Option<unsafe extern "C" fn()>,
    pub app_indicator_reserved_4: ::std::option::Option<unsafe extern "C" fn()>,
    pub app_indicator_reserved_5: ::std::option::Option<unsafe extern "C" fn()>,
    pub app_indicator_reserved_6: ::std::option::Option<unsafe extern "C" fn()>,
}
#[test]
fn bindgen_test_layout__AppIndicatorClass() {
    assert_eq!(
        ::std::mem::size_of::<_AppIndicatorClass>(),
        264usize,
        concat!("Size of: ", stringify!(_AppIndicatorClass))
    );
    assert_eq!(
        ::std::mem::align_of::<_AppIndicatorClass>(),
        8usize,
        concat!("Alignment of ", stringify!(_AppIndicatorClass))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_AppIndicatorClass>())).parent_class as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(_AppIndicatorClass),
            "::",
            stringify!(parent_class)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_AppIndicatorClass>())).new_icon as *const _ as usize },
        136usize,
        concat!(
            "Offset of field: ",
            stringify!(_AppIndicatorClass),
            "::",
            stringify!(new_icon)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<_AppIndicatorClass>())).new_attention_icon as *const _ as usize
        },
        144usize,
        concat!(
            "Offset of field: ",
            stringify!(_AppIndicatorClass),
            "::",
            stringify!(new_attention_icon)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_AppIndicatorClass>())).new_status as *const _ as usize },
        152usize,
        concat!(
            "Offset of field: ",
            stringify!(_AppIndicatorClass),
            "::",
            stringify!(new_status)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<_AppIndicatorClass>())).new_icon_theme_path as *const _ as usize
        },
        160usize,
        concat!(
            "Offset of field: ",
            stringify!(_AppIndicatorClass),
            "::",
            stringify!(new_icon_theme_path)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_AppIndicatorClass>())).new_label as *const _ as usize },
        168usize,
        concat!(
            "Offset of field: ",
            stringify!(_AppIndicatorClass),
            "::",
            stringify!(new_label)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<_AppIndicatorClass>())).connection_changed as *const _ as usize
        },
        176usize,
        concat!(
            "Offset of field: ",
            stringify!(_AppIndicatorClass),
            "::",
            stringify!(connection_changed)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_AppIndicatorClass>())).scroll_event as *const _ as usize },
        184usize,
        concat!(
            "Offset of field: ",
            stringify!(_AppIndicatorClass),
            "::",
            stringify!(scroll_event)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<_AppIndicatorClass>())).app_indicator_reserved_ats as *const _
                as usize
        },
        192usize,
        concat!(
            "Offset of field: ",
            stringify!(_AppIndicatorClass),
            "::",
            stringify!(app_indicator_reserved_ats)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_AppIndicatorClass>())).fallback as *const _ as usize },
        200usize,
        concat!(
            "Offset of field: ",
            stringify!(_AppIndicatorClass),
            "::",
            stringify!(fallback)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_AppIndicatorClass>())).unfallback as *const _ as usize },
        208usize,
        concat!(
            "Offset of field: ",
            stringify!(_AppIndicatorClass),
            "::",
            stringify!(unfallback)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<_AppIndicatorClass>())).app_indicator_reserved_1 as *const _
                as usize
        },
        216usize,
        concat!(
            "Offset of field: ",
            stringify!(_AppIndicatorClass),
            "::",
            stringify!(app_indicator_reserved_1)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<_AppIndicatorClass>())).app_indicator_reserved_2 as *const _
                as usize
        },
        224usize,
        concat!(
            "Offset of field: ",
            stringify!(_AppIndicatorClass),
            "::",
            stringify!(app_indicator_reserved_2)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<_AppIndicatorClass>())).app_indicator_reserved_3 as *const _
                as usize
        },
        232usize,
        concat!(
            "Offset of field: ",
            stringify!(_AppIndicatorClass),
            "::",
            stringify!(app_indicator_reserved_3)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<_AppIndicatorClass>())).app_indicator_reserved_4 as *const _
                as usize
        },
        240usize,
        concat!(
            "Offset of field: ",
            stringify!(_AppIndicatorClass),
            "::",
            stringify!(app_indicator_reserved_4)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<_AppIndicatorClass>())).app_indicator_reserved_5 as *const _
                as usize
        },
        248usize,
        concat!(
            "Offset of field: ",
            stringify!(_AppIndicatorClass),
            "::",
            stringify!(app_indicator_reserved_5)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<_AppIndicatorClass>())).app_indicator_reserved_6 as *const _
                as usize
        },
        256usize,
        concat!(
            "Offset of field: ",
            stringify!(_AppIndicatorClass),
            "::",
            stringify!(app_indicator_reserved_6)
        )
    );
}
#[doc = " AppIndicator:"]
#[doc = ""]
#[doc = " A application indicator represents the values that are needed to show a"]
#[doc = " unique status in the panel for an application.  In general, applications"]
#[doc = " should try to fit in the other indicators that are available on the"]
#[doc = " panel before using this.  But, sometimes it is necissary."]
#[doc = ""]
#[doc = "  Private fields"]
#[doc = " @parent: Parent object."]
#[doc = " @priv: Internal data."]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _AppIndicator {
    pub parent: GObject,
    pub priv_: *mut AppIndicatorPrivate,
}
#[test]
fn bindgen_test_layout__AppIndicator() {
    assert_eq!(
        ::std::mem::size_of::<_AppIndicator>(),
        32usize,
        concat!("Size of: ", stringify!(_AppIndicator))
    );
    assert_eq!(
        ::std::mem::align_of::<_AppIndicator>(),
        8usize,
        concat!("Alignment of ", stringify!(_AppIndicator))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_AppIndicator>())).parent as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(_AppIndicator),
            "::",
            stringify!(parent)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_AppIndicator>())).priv_ as *const _ as usize },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(_AppIndicator),
            "::",
            stringify!(priv_)
        )
    );
}
extern "C" {
    pub fn app_indicator_get_type() -> GType;
}
extern "C" {
    pub fn app_indicator_new(
        id: *const gchar,
        icon_name: *const gchar,
        category: AppIndicatorCategory,
    ) -> *mut AppIndicator;
}
extern "C" {
    pub fn app_indicator_new_with_path(
        id: *const gchar,
        icon_name: *const gchar,
        category: AppIndicatorCategory,
        icon_theme_path: *const gchar,
    ) -> *mut AppIndicator;
}
extern "C" {
    pub fn app_indicator_set_status(self_: *mut AppIndicator, status: AppIndicatorStatus);
}
extern "C" {
    pub fn app_indicator_set_attention_icon(self_: *mut AppIndicator, icon_name: *const gchar);
}
extern "C" {
    pub fn app_indicator_set_attention_icon_full(
        self_: *mut AppIndicator,
        icon_name: *const gchar,
        icon_desc: *const gchar,
    );
}
extern "C" {
    pub fn app_indicator_set_menu(self_: *mut AppIndicator, menu: *mut GtkMenu);
}
extern "C" {
    pub fn app_indicator_set_icon(self_: *mut AppIndicator, icon_name: *const gchar);
}
extern "C" {
    pub fn app_indicator_set_icon_full(
        self_: *mut AppIndicator,
        icon_name: *const gchar,
        icon_desc: *const gchar,
    );
}
extern "C" {
    pub fn app_indicator_set_label(
        self_: *mut AppIndicator,
        label: *const gchar,
        guide: *const gchar,
    );
}
extern "C" {
    pub fn app_indicator_set_icon_theme_path(
        self_: *mut AppIndicator,
        icon_theme_path: *const gchar,
    );
}
extern "C" {
    pub fn app_indicator_set_ordering_index(self_: *mut AppIndicator, ordering_index: guint32);
}
extern "C" {
    pub fn app_indicator_set_secondary_activate_target(
        self_: *mut AppIndicator,
        menuitem: *mut GtkWidget,
    );
}
extern "C" {
    pub fn app_indicator_set_title(self_: *mut AppIndicator, title: *const gchar);
}
extern "C" {
    pub fn app_indicator_get_id(self_: *mut AppIndicator) -> *const gchar;
}
extern "C" {
    pub fn app_indicator_get_category(self_: *mut AppIndicator) -> AppIndicatorCategory;
}
extern "C" {
    pub fn app_indicator_get_status(self_: *mut AppIndicator) -> AppIndicatorStatus;
}
extern "C" {
    pub fn app_indicator_get_icon(self_: *mut AppIndicator) -> *const gchar;
}
extern "C" {
    pub fn app_indicator_get_icon_desc(self_: *mut AppIndicator) -> *const gchar;
}
extern "C" {
    pub fn app_indicator_get_icon_theme_path(self_: *mut AppIndicator) -> *const gchar;
}
extern "C" {
    pub fn app_indicator_get_attention_icon(self_: *mut AppIndicator) -> *const gchar;
}
extern "C" {
    pub fn app_indicator_get_attention_icon_desc(self_: *mut AppIndicator) -> *const gchar;
}
extern "C" {
    pub fn app_indicator_get_title(self_: *mut AppIndicator) -> *const gchar;
}
extern "C" {
    pub fn app_indicator_get_menu(self_: *mut AppIndicator) -> *mut GtkMenu;
}
extern "C" {
    pub fn app_indicator_get_label(self_: *mut AppIndicator) -> *const gchar;
}
extern "C" {
    pub fn app_indicator_get_label_guide(self_: *mut AppIndicator) -> *const gchar;
}
extern "C" {
    pub fn app_indicator_get_ordering_index(self_: *mut AppIndicator) -> guint32;
}
extern "C" {
    pub fn app_indicator_get_secondary_activate_target(self_: *mut AppIndicator) -> *mut GtkWidget;
}
extern "C" {
    pub fn app_indicator_build_menu_from_desktop(
        self_: *mut AppIndicator,
        desktop_file: *const gchar,
        desktop_profile: *const gchar,
    );
}

